use crate::{
    parser::{filter::*, HttpMethod, Modifier, Request, RequestType, ResponseType},
    traits::{RequestParser, RequestParserBuilder},
};
use from_as::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use reqwest::Url;
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

pub trait IsInPath {
    fn retain_is_in_path(&mut self);
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Response {
    #[serde(rename = "204")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_content: Option<ResponseObject>,
    #[serde(rename = "200")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<ResponseObject>,
    #[serde(rename = "201")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<ResponseObject>,
}

impl Response {
    pub fn get_type(&self) -> ResponseType {
        if self.no_content.is_some() {
            return ResponseType::NoContent;
        }

        if let Some(response_object) = self.ok.as_ref() {
            if response_object.is_array() {
                return ResponseType::Collection;
            }
        }

        if let Some(response_object) = self.created.as_ref() {
            if response_object.is_array() {
                return ResponseType::Collection;
            }
        }

        ResponseType::SerdeJson
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResponseObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
}

impl ResponseObject {
    pub fn is_array(&self) -> bool {
        if let Some(content) = self.content.as_ref() {
            content.is_array()
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Content {
    #[serde(rename = "application/json")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_json: Option<ApplicationJson>,
}

impl Content {
    pub fn is_array(&self) -> bool {
        if let Some(application_json) = self.application_json.as_ref() {
            application_json.is_array()
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ApplicationJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}

impl ApplicationJson {
    pub fn is_array(&self) -> bool {
        if let Some(schema) = self.schema.as_ref() {
            schema.is_array()
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Schema {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl Schema {
    pub fn is_array(&self) -> bool {
        if let Some(_type) = self._type.as_ref() {
            _type.eq("array")
        } else {
            false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<VecDeque<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub parameters: VecDeque<Parameter>,
    #[serde(rename = "requestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Response>,
}

impl Operation {
    pub fn param_size(&self) -> usize {
        self.parameters
            .iter()
            .filter(|param| param.is_in_path())
            .count()
    }

    pub fn has_body(&self) -> bool {
        self.request_body.is_some()
    }

    pub fn response_type(&self) -> ResponseType {
        if let Some(response) = self.responses.as_ref() {
            response.get_type()
        } else {
            ResponseType::SerdeJson
        }
    }
}

impl IsInPath for Option<Operation> {
    fn retain_is_in_path(&mut self) {
        if let Some(operation) = self.as_mut() {
            operation.parameters.retain_is_in_path();
        }
    }
}

impl RequestParser for Operation {
    fn method_name(&self) -> String {
        self.operation_id.method_name()
    }

    fn operation_mapping(&self) -> String {
        self.operation_id.operation_mapping()
    }

    fn transform_path(&self) -> String {
        Default::default()
    }

    fn links(&self) -> HashSet<String> {
        self.operation_mapping().links()
    }
}

impl RequestParserBuilder for Operation {
    /// The build method is where each individual request
    /// is parsed.
    fn build(&self, path: String, modifier: &Modifier, http_method: HttpMethod) -> Request {
        let mut request = Request {
            path,
            operation_id: self.operation_id.to_string(),
            operation_mapping: self.operation_id.operation_mapping(),
            method_name: self.method_name(),
            param_size: self.param_size(),
            has_body: self.has_body(),
            response: self.response_type(),
            doc: self.summary.clone().map(|s| format!("# {}", s)),
            ..Default::default()
        };

        if request.method.eq(&HttpMethod::DELETE) {
            request.response = ResponseType::NoContent;
        } else if request.method_name.starts_with("list") {
            request.response = ResponseType::Collection;
        } else if request.method_name.eq("delta") {
            request.response = ResponseType::Delta;
        } else if request.method_name.eq("create_upload_session") {
            request.response = ResponseType::UploadSession;
        }

        if request.response.eq(&ResponseType::UploadSession) {
            request.request_type = RequestType::UploadSession;
        }

        if let Some(tags) = self.tags.as_ref() {
            if let Some(tag) = tags.get(0) {
                request.tag = tag.to_string();
            }
        }

        request.modify(&modifier);
        request.method = http_method;

        if request.operation_mapping.is_empty() {
            request.operation_mapping = modifier.name.to_string();
        }
        request
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Path {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub parameters: VecDeque<Parameter>,
}

impl Path {
    pub fn build_requests(&self, path: &str, modifier: &Modifier) -> VecDeque<Request> {
        let mut requests = VecDeque::new();

        if let Some(request) =
            Path::build_request(&path, &modifier, self.get.as_ref(), HttpMethod::GET)
        {
            requests.push_back(request);
        }

        if let Some(request) =
            Path::build_request(&path, &modifier, self.post.as_ref(), HttpMethod::POST)
        {
            requests.push_back(request);
        }

        if let Some(request) =
            Path::build_request(&path, &modifier, self.put.as_ref(), HttpMethod::PUT)
        {
            requests.push_back(request);
        }

        if let Some(request) =
            Path::build_request(&path, &modifier, self.patch.as_ref(), HttpMethod::PATCH)
        {
            requests.push_back(request);
        }

        if let Some(request) =
            Path::build_request(&path, &modifier, self.delete.as_ref(), HttpMethod::DELETE)
        {
            requests.push_back(request);
        }
        requests
    }

    fn build_request(
        path: &str,
        modifier: &Modifier,
        operation: Option<&Operation>,
        http_method: HttpMethod,
    ) -> Option<Request> {
        operation
            .as_ref()
            .map(|operation| operation.build(path.to_string(), &modifier, http_method))
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Parameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Parameter {
    pub fn is_in_path(&self) -> bool {
        if let Some(in_) = self.in_.as_ref() {
            in_.eq("path")
        } else {
            false
        }
    }
}

impl IsInPath for VecDeque<Parameter> {
    fn retain_is_in_path(&mut self) {
        self.retain(|p| p.is_in_path());
    }
}

impl IsInPath for Option<VecDeque<Parameter>> {
    fn retain_is_in_path(&mut self) {
        if let Some(vec) = self.as_mut() {
            vec.retain_is_in_path();
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(default)]
pub struct PathMap {
    pub paths: BTreeMap<String, Path>,
}

impl PathMap {
    pub fn clean(&mut self) {
        self.retain_is_in_path();
    }

    pub fn clean_secondary(&mut self, secondary_name: &str) -> PathMap {
        let mut map2: BTreeMap<String, Path> = BTreeMap::new();
        for (path, path_struct) in self.paths.iter() {
            if let Some(index) = path.find(secondary_name) {
                let new_path = path[index - 1..].to_string();
                map2.insert(new_path, path_struct.clone());
            }
        }
        self.clean();
        map2.into()
    }

    pub fn transform_paths(&mut self) {
        let mut map: BTreeMap<String, Path> = BTreeMap::new();
        for (p, path) in self.paths.iter_mut() {
            map.insert(p.transform_path(), path.clone());
        }
        self.paths = map;
    }
}

impl TryFrom<reqwest::Url> for PathMap {
    type Error = reqwest::Error;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(url)?;
        let s = response.text().unwrap();
        let path_map: PathMap = serde_yaml::from_str(s.as_str()).unwrap();
        Ok(path_map)
    }
}

impl IsInPath for PathMap {
    fn retain_is_in_path(&mut self) {
        for (_s, path) in self.paths.iter_mut() {
            path.parameters.retain(|param| param.is_in_path());
            path.get.retain_is_in_path();
            path.put.retain_is_in_path();
            path.patch.retain_is_in_path();
            path.post.retain_is_in_path();
            path.delete.retain_is_in_path();
        }
    }
}

impl From<BTreeMap<String, Path>> for PathMap {
    fn from(paths: BTreeMap<String, Path>) -> Self {
        PathMap { paths }
    }
}

impl PathMap {
    pub fn filter(&self, filter: Filter) -> BTreeMap<String, Path> {
        match filter {
            Filter::PathStartsWith(filter) => self
                .paths
                .clone()
                .into_par_iter()
                .filter(|(path, _path_spec)| path.starts_with(filter.as_str()))
                .collect(),
            Filter::PathStartsWithMulti(vec) => self
                .paths
                .clone()
                .into_par_iter()
                .filter(|(path, _path_spec)| vec.iter().any(|s| path.starts_with(s)))
                .collect(),
            Filter::None => self.paths.clone(),
            Filter::PathEquals(filter) => self
                .paths
                .clone()
                .into_par_iter()
                .filter(|(path, _path_spec)| path.eq(filter.as_str()))
                .collect(),
            Filter::PathContains(filter) => self
                .paths
                .clone()
                .into_par_iter()
                .filter(|(path, _path_spec)| path.contains(filter.as_str()))
                .collect(),
            Filter::Regex(s) => {
                let regex = Regex::new(s.as_str()).unwrap();
                self.paths
                    .clone()
                    .into_iter()
                    .filter(|(path, _path_spec)| regex.is_match(path.as_ref()))
                    .collect()
            }
            Filter::IgnoreIf(filter_ignore) => match filter_ignore {
                FilterIgnore::PathContains(s) => self
                    .paths
                    .clone()
                    .into_par_iter()
                    .filter(|(path, _path_spec)| !path.contains(s.as_str()))
                    .collect(),
                FilterIgnore::PathStartsWith(s) => self
                    .paths
                    .clone()
                    .into_par_iter()
                    .filter(|(path, _path_spec)| !path.starts_with(s.as_str()))
                    .collect(),
                FilterIgnore::PathContainsMulti(vec) => {
                    let mut paths = self.paths.clone();
                    for s in vec.iter() {
                        paths = paths
                            .into_par_iter()
                            .filter(|(path, _path_spec)| !path.contains(s))
                            .collect();
                    }
                    paths
                }
                FilterIgnore::PathEquals(s) => self
                    .paths
                    .clone()
                    .into_par_iter()
                    .filter(|(path, _path_spec)| !path.eq(s.as_str()))
                    .collect(),
            },
            Filter::MultiFilter(vec) => {
                let mut map: BTreeMap<String, Path> = BTreeMap::new();
                let mut count = 0;
                for filter in vec.iter() {
                    if count == 0 {
                        map = self.filter(filter.clone());
                        count += 1;
                    } else {
                        map = PathMap { paths: map }.filter(filter.clone())
                    }
                }
                map
            }
        }
    }
}

impl IntoIterator for PathMap {
    type IntoIter = std::collections::btree_map::IntoIter<String, Path>;
    type Item = (String, Path);

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}
