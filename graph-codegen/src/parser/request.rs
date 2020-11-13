use crate::parser::filter::ModifierMap;
use crate::parser::{ResourceNameMapping, ResourceNames};
use crate::traits::{HashMapExt, RequestParser};
use from_as::*;
use inflector::Inflector;
use rayon::prelude::*;
use std::collections::hash_set::{Difference, Iter};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
    PATCH,
    TRACE,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

impl AsRef<str> for HttpMethod {
    fn as_ref(&self) -> &str {
        match self {
            HttpMethod::GET => "get",
            HttpMethod::PUT => "put",
            HttpMethod::POST => "post",
            HttpMethod::DELETE => "delete",
            HttpMethod::PATCH => "patch",
            HttpMethod::TRACE => "trace",
        }
    }
}

impl From<HttpMethod> for reqwest::Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::GET => reqwest::Method::GET,
            HttpMethod::PUT => reqwest::Method::PUT,
            HttpMethod::POST => reqwest::Method::POST,
            HttpMethod::DELETE => reqwest::Method::DELETE,
            HttpMethod::PATCH => reqwest::Method::PATCH,
            HttpMethod::TRACE => reqwest::Method::TRACE,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, FromFile, AsFile, Hash)]
pub enum ResponseType {
    SerdeJson,
    Collection,
    NoContent,
    Delta,
    UploadSession,
}

impl ResponseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseType::Collection => "Collection<serde_json::Value>",
            ResponseType::Delta => "DeltaPhantom<serde_json::Value>",
            ResponseType::NoContent => "GraphResponse<Content>",
            ResponseType::SerdeJson => "serde_json::Value",
            ResponseType::UploadSession => "UploadSessionClient<Client>",
        }
    }

    pub fn as_imports(&self) -> HashSet<String> {
        let mut set: HashSet<String> = HashSet::new();
        match self {
            ResponseType::Collection => {
                set.insert("graph_http::types::Collection".into());
            },
            ResponseType::Delta => {
                set.insert("graph_http::types::DeltaPhantom".into());
            },
            ResponseType::NoContent => {
                set.insert("graph_http::types::Content".into());
                set.insert("graph_http::GraphResponse".into());
            },
            ResponseType::UploadSession => {
                set.insert("graph_http::UploadSessionClient".into());
            },
            _ => {},
        }
        set
    }
}

impl ToString for ResponseType {
    fn to_string(&self) -> String {
        self.as_str().into()
    }
}

impl Default for ResponseType {
    fn default() -> Self {
        ResponseType::SerdeJson
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Request {
    pub path: String,
    pub method: HttpMethod,
    pub method_name: String,
    pub param_size: usize,
    pub has_body: bool,
    pub has_rid: bool,
    pub response: ResponseType,
    pub tag: String,
    pub operation_id: String,
    pub operation_mapping: String,
    pub doc: Option<String>,
}

impl Request {
    pub fn modify(&mut self, map: &ModifierMap) {
        for (mat, modify_vec) in map.map.iter() {
            if mat.matches(self) {
                for modifier in modify_vec.iter() {
                    modifier.modify(self);
                }
            }
        }
    }
}

impl RequestParser for Request {
    fn method_name(&self) -> String {
        self.method_name.to_string()
    }

    fn operation_mapping(&self) -> String {
        self.operation_mapping.to_string()
    }

    fn transform_path(&self) -> String {
        unimplemented!()
    }

    fn links(&self) -> HashSet<String> {
        self.operation_mapping.links()
    }
}

pub struct ReqSet {
    set: HashSet<Request>,
}

impl ReqSet {
    pub fn difference<'a>(
        &'a self,
        request_set: &'a HashSet<Request>,
    ) -> Difference<'a, Request, std::collections::hash_map::RandomState> {
        self.set.difference(&request_set)
    }
}

/// RequestMap holds a list of requests that correspond to a URL path
#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestMap {
    pub path: String,
    pub requests: VecDeque<Request>,
}

impl PartialEq for RequestMap {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(other.path.as_str())
    }
}

impl Eq for RequestMap {}

impl Hash for RequestMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.requests.hash(state);
    }
}

impl IntoIterator for RequestMap {
    type Item = Request;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.requests.into_iter()
    }
}

impl RequestMap {
    pub fn get_imports(&self) -> HashSet<String> {
        let mut imports: HashSet<String> = HashSet::new();
        for request in self.requests.iter() {
            imports.extend(request.response.as_imports());
        }
        imports
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, Request> {
        self.requests.iter()
    }

    pub fn extend_struct_links(&self, set: &mut HashSet<String>) {
        for request in self.iter() {
            set.extend(request.links());
        }
    }

    pub fn difference(&self, request_map: RequestMap) -> Vec<Request> {
        let set1: HashSet<Request> = self.requests.clone().into_iter().collect();
        let set2: HashSet<Request> = request_map.requests.into_iter().collect();
        set1.difference(&set2).cloned().collect()
    }
}

/// RequestSet holds a set of unique RequestMap objects.
#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestSet {
    pub set: HashSet<RequestMap>,
}

impl RequestSet {
    pub fn new(set: HashSet<RequestMap>) -> RequestSet {
        RequestSet { set }
    }

    pub fn get(&self, path: &str) -> Option<RequestMap> {
        self.set.iter().find(|rm| rm.path.eq(path)).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn join_inner_insert(&mut self, request_map: RequestMap) {
        if self.set.contains(&request_map) {
            let mut req_map = self.set.get(&request_map).cloned().unwrap();
            for request in request_map.requests.iter() {
                if req_map.requests.iter().find(|r| r.eq(&request)).is_none() {
                    req_map.requests.push_back(request.clone());
                }
            }

            self.set.insert(req_map);
        } else {
            self.set.insert(request_map);
        }
    }

    pub fn resource_names(&self) -> ResourceNames {
        let mut resource = ResourceNames::new(BTreeSet::new());
        let mut names: Vec<String> = Vec::new();

        for request_map in self.set.iter() {
            let mut vec: VecDeque<&str> = request_map.path.split('/').collect();
            vec.retain(|s| !s.is_empty());
            if let Some(name) = vec.pop_front() {
                if !name.is_empty() {
                    names.push(name.to_camel_case());
                }
            }
        }

        names.sort();
        for name in names.iter() {
            resource.names.insert(name.to_string());
        }

        resource
    }

    pub fn resource_name_mapping(&self) -> ResourceNameMapping {
        let mut resource_map = ResourceNameMapping::new(HashMap::new());
        for request_map in self.set.iter() {
            for request in request_map.requests.iter() {
                if request.operation_mapping.contains('.') {
                    let mut v: VecDeque<&str> = request.operation_mapping.split('.').collect();
                    v.retain(|s| !s.is_empty());

                    if v.len() >= 2 {
                        let first = v.pop_front().unwrap();
                        let value = v.pop_front().map(|s| s.to_string()).unwrap();
                        resource_map
                            .map
                            .entry_modify_insert(first.to_string(), value);
                    }
                }
            }
        }
        resource_map
    }

    pub fn group_by_operation_mapping(&self) -> HashMap<String, Vec<RequestMap>> {
        let mut map: HashMap<String, Vec<RequestMap>> = HashMap::new();
        for request_map in self.set.iter() {
            if let Some(request) = request_map.requests.get(0) {
                let operation_mapping = request.operation_mapping.to_string();
                map.entry_modify_insert(operation_mapping, request_map.clone());
            }
        }
        map
    }

    pub fn group_by_operation_id(&self) -> HashMap<String, Vec<Request>> {
        let mut map: HashMap<String, Vec<Request>> = HashMap::new();
        for request_map in self.set.iter() {
            for request in request_map.iter() {
                if let Some(index) = request.operation_id.rfind('.') {
                    map.entry_modify_insert(request.operation_id[..index].to_string(), request);
                } else {
                    map.entry_modify_insert(request.operation_id.to_string(), request);
                }
            }
        }
        map
    }

    pub fn group_by_operation_mapping_name(&self) -> HashMap<String, Vec<RequestMap>> {
        let mut map: HashMap<String, Vec<RequestMap>> = HashMap::new();

        for request_map in self.set.iter() {
            if let Some(request) = request_map.requests.get(0) {
                if request.operation_mapping.contains('.') {
                    let mut vec_operation_mapping: VecDeque<&str> =
                        request.operation_mapping.split('.').collect();
                    vec_operation_mapping.retain(|s| !s.is_empty());
                    let last = vec_operation_mapping.pop_back().unwrap();
                    map.entry_modify_insert(last.to_string(), request_map.clone());
                } else {
                    let operation_mapping = request.operation_mapping.to_string();
                    map.entry_modify_insert(operation_mapping, request_map.clone());
                }
            }
        }
        map
    }

    /// Takes the operation mapping such as users.planner.plans
    /// and creates the list of individual links between structs:
    /// users.planner, planner.plans
    pub fn method_links(&self) -> (HashSet<String>, HashMap<String, Vec<String>>) {
        let mut secondary_set = HashSet::new();

        for request_map in self.iter() {
            request_map.extend_struct_links(&mut secondary_set);
        }

        (
            RequestSet::struct_names(&secondary_set),
            RequestSet::struct_links(&secondary_set),
        )
    }

    /// Splits the operation id for each request in the RequestMap
    /// and returns a unique set of struct names that are used
    /// to create the different client structs.
    fn struct_names(links: &HashSet<String>) -> HashSet<String> {
        let mut set: HashSet<String> = HashSet::new();
        for link in links.iter() {
            if link.contains('.') {
                let mut names: Vec<&str> = link.split('.').collect();
                names.retain(|s| !s.is_empty());
                for name in names.iter() {
                    set.insert(name.to_string());
                }
            } else {
                set.insert(link.to_string());
            }
        }
        set
    }

    /// Creates a hash map of each struct and the client structs
    /// it links too.
    ///
    /// # Example
    ///
    /// Say we have the following operation id's or operation mappings:
    ///     groups.calendar.calendarView
    ///     groups.calendarView
    ///     groups.drive
    ///
    /// {
    ///     "groups": [
    ///         "calendar",
    ///         "calendarView",
    ///         "drive"
    ///     ],
    ///     "calendar": [
    ///         "calendarView"
    ///     ]
    /// }
    fn struct_links(links: &HashSet<String>) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut vec: Vec<&str> = links.iter().map(|s| s.as_str()).collect();
        vec.sort();

        for link in vec.iter() {
            if link.contains('.') {
                let mut vec: VecDeque<&str> = link.split('.').collect();
                vec.retain(|l| !l.is_empty());
                let first = vec.pop_front().unwrap();
                let last = vec.pop_front().unwrap();
                map.entry_modify_insert(first.to_string(), last.to_string());
            } else {
                map.insert(link.to_string(), vec![]);
            }
        }
        map
    }

    pub fn get_imports(&self) -> HashSet<String> {
        let mut imports_vec: HashSet<String> = HashSet::new();
        for request_map in self.set.iter() {
            imports_vec.extend(request_map.get_imports());
        }
        imports_vec
    }

    pub fn iter(&self) -> Iter<'_, RequestMap> {
        self.set.iter()
    }

    /// Split the requests into two RequestSet groupings where the
    /// first group is all requests that require a resource id in the
    /// path such as /groups/{group-id} and the second RequestSet
    /// is all requests that do not require a resource id in the path.
    pub fn split_on_resource_id(&self) -> (RequestSet, RequestSet) {
        let mut request_set1 = RequestSet::default();
        let mut request_set2 = RequestSet::default();

        for request_map in self.set.iter() {
            if request_map.requests.iter().any(|req| req.has_rid) {
                request_set1.set.insert(request_map.clone());
            } else {
                request_set2.set.insert(request_map.clone());
            }
        }

        (request_set1, request_set2)
    }

    pub fn difference<'a>(
        &'a self,
        request_set: &'a RequestSet,
    ) -> Difference<'a, RequestMap, std::collections::hash_map::RandomState> {
        self.set.difference(&request_set.set)
    }
}

impl IntoIterator for RequestSet {
    type Item = RequestMap;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

// This is mainly used to output a serializable struct with
// the request sets grouped by operation mapping.
#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ApiImpl {
    pub requests: HashMap<String, RequestSet>,
}

impl From<HashMap<String, RequestSet>> for ApiImpl {
    fn from(requests: HashMap<String, RequestSet>) -> Self {
        ApiImpl { requests }
    }
}
