use crate::{
    parser::{Request, RequestMap, RequestSet},
    traits::{Modify, INTERNAL_PATH_ID},
};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub enum FilterIgnore {
    PathContains(String),
    PathContainsMulti(Vec<String>),
    PathStartsWith(String),
    PathEquals(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub enum Filter {
    None,
    PathStartsWith(String),
    PathStartsWithMulti(Vec<String>),
    PathEquals(String),
    PathContains(String),
    Regex(String),
    IgnoreIf(FilterIgnore),
    MultiFilter(Vec<Filter>),
}

/// Modifies the paths that start with a resource and id by replacing
/// that part of the path for the client sdk generation. An example
/// would be where we have the path `/drives/{{id}}/items`. Passing the
/// value of 'drives' to this modifier would change the path to
/// `/drives/{{{RID}}/items`
pub trait ResourceUrlReplacement: Modify<RequestSet> + Modify<RequestMap> {
    fn name(&self) -> String;
    fn replacement(&self) -> String;

    fn modify_using_replacement(&self) -> bool;

    fn formatted(&self) -> String {
        format!("/{}/{{{{id}}}}", self.name())
    }

    fn formatted_replacement(&self) -> String {
        format!("/{}/{{{{id}}}}", self.replacement())
    }

    fn matches(&self, request_map: &RequestMap) -> bool {
        request_map.path.starts_with(&self.formatted())
    }

    fn matches_replacement(&self, request_map: &RequestMap) -> bool {
        request_map.path.starts_with(&self.formatted_replacement())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub struct ResourceIdentityModifier {
    resource_identity: ResourceIdentity,
    modify_using_replacement: bool,
}

impl ResourceUrlReplacement for ResourceIdentityModifier {
    fn name(&self) -> String {
        self.resource_identity.to_string()
    }

    fn replacement(&self) -> String {
        let name = self.name();

        if name.ends_with('s') {
            let replacement = &name[..name.len() - 1];
            replacement.into()
        } else {
            let mut replacement = name;
            replacement.push('s');
            replacement
        }
    }

    fn modify_using_replacement(&self) -> bool {
        self.modify_using_replacement
    }
}

impl ResourceIdentityModifier {
    pub fn new(
        resource_identity: ResourceIdentity,
        modify_using_replacement: bool,
    ) -> ResourceIdentityModifier {
        ResourceIdentityModifier {
            resource_identity,
            modify_using_replacement,
        }
    }
}

impl Modify<RequestMap> for ResourceIdentityModifier {
    fn modify(&self, value: &mut RequestMap) {
        let path = value
            .path
            .replace("{{id}}", "{{RID}}")
            .replace("{{id2}}", "{{id}}")
            .replace("{{id3}}", "{{id2}}")
            .replace("{{id4}}", "{{id3}}");
        value.path = path.clone();
        for request in value.requests.iter_mut() {
            request.path = path.clone();
            request.has_rid = true;
            if request.param_size > 0 {
                request.param_size -= 1;
            }
        }
    }
}

impl Modify<RequestSet> for ResourceIdentityModifier {
    fn modify(&self, value: &mut RequestSet) {
        let formatted = self.formatted();
        let mut matches = false;
        for request_map in value.set.iter() {
            if request_map
                .path
                .starts_with(&formatted.replace("id", "RID"))
            {
                matches = true;
                break;
            }
        }

        if matches {
            let (mut rid_request_set, non_rid_set) = value.split_on_resource_id();
            let mut request_map_vec: Vec<RequestMap> = non_rid_set.set.into_iter().collect();
            let replacement = self.replacement();

            for request_map in request_map_vec.iter_mut() {
                let param_size = INTERNAL_PATH_ID.captures_iter(&request_map.path).count();
                for request in request_map.requests.iter_mut() {
                    if let Some(index) = request.operation_mapping.find('.') {
                        request
                            .operation_mapping
                            .replace_range(0..index, &replacement);
                    } else {
                        request.operation_mapping = replacement.to_string();
                    }

                    request.param_size = param_size;
                    if request.path.contains("RID") && request.param_size > 0 {
                        request.param_size -= 1;
                    }
                }
            }

            rid_request_set.set.extend(request_map_vec);
            value.set = rid_request_set.set;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct SecondaryTarget {
    match_target: MatchTarget,
    pat: String,
}

impl SecondaryTarget {
    pub fn new(pat: &str, match_target: MatchTarget) -> SecondaryTarget {
        SecondaryTarget {
            match_target,
            pat: pat.to_string(),
        }
    }

    pub fn modify(&self, request: &mut Request) {
        if self.match_target.contains(request) {
            self.match_target.modify_contains(&self.pat, request);
        }
    }
}

impl From<MatchTarget> for SecondaryTarget {
    fn from(match_target: MatchTarget) -> Self {
        SecondaryTarget::new("", match_target)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct SecondaryModifierMap {
    pub secondary_targets: VecDeque<SecondaryTarget>,
}

impl SecondaryModifierMap {
    pub fn with_capacity(capacity: usize) -> SecondaryModifierMap {
        SecondaryModifierMap {
            secondary_targets: VecDeque::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, pat: &str, match_target: MatchTarget) {
        self.secondary_targets
            .push_back(SecondaryTarget::new(pat, match_target));
    }

    pub fn insert_operation_mapping(&mut self, pat: &str, match_target: &str) {
        self.secondary_targets.push_back(SecondaryTarget::new(
            pat,
            MatchTarget::OperationMap(match_target.to_string()),
        ));
    }

    pub fn insert_operation_id(&mut self, pat: &str, match_target: &str) {
        self.secondary_targets.push_back(SecondaryTarget::new(
            pat,
            MatchTarget::OperationId(match_target.to_string()),
        ));
    }

    pub fn insert_operation_map_and_id(&mut self, pat: &str, match_target: &str) {
        self.insert_operation_mapping(pat, match_target);
        self.insert_operation_id(pat, match_target);
    }

    pub fn modify(&self, request: &mut Request) {
        for target in self.secondary_targets.iter() {
            target.modify(request);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub enum MatchTarget {
    Tag(String),
    OperationId(String),
    OperationMap(String),
    TagAndOperationMap(String),
    TagOrOperationMap(String),
}

impl MatchTarget {
    pub fn matches(&self, request: &Request) -> bool {
        match self {
            MatchTarget::OperationMap(s) => {
                if request.operation_mapping.eq(s.as_str()) {
                    return true;
                }
            }
            MatchTarget::Tag(s) => {
                if request.tag.eq(s.as_str()) {
                    return true;
                }
            }
            MatchTarget::TagAndOperationMap(s) => {
                if request.tag.eq(s.as_str()) && request.operation_mapping.eq(s.as_str()) {
                    return true;
                }
            }
            MatchTarget::TagOrOperationMap(s) => {
                if request.tag.eq(s.as_str()) || request.operation_mapping.eq(s.as_str()) {
                    return true;
                }
            }
            MatchTarget::OperationId(s) => {
                if request.operation_id.eq(s.as_str()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn modify(&self, request: &mut Request) {
        match self {
            MatchTarget::OperationMap(s) => {
                request.operation_mapping = s.to_string();
            }
            MatchTarget::Tag(s) => {
                request.tag = s.to_string();
            }
            MatchTarget::TagAndOperationMap(s) => {
                request.tag = s.to_string();
                request.operation_mapping = s.to_string();
            }
            MatchTarget::TagOrOperationMap(s) => {
                request.tag = s.to_string();
                request.operation_mapping = s.to_string();
            }
            MatchTarget::OperationId(s) => {
                request.operation_id = s.to_string();
            }
        }
    }

    pub fn contains(&self, request: &Request) -> bool {
        match self {
            MatchTarget::Tag(s) => request.tag.contains(s.as_str()),
            MatchTarget::OperationId(s) => request.operation_id.contains(s.as_str()),
            MatchTarget::OperationMap(s) => request.operation_mapping.contains(s.as_str()),
            MatchTarget::TagAndOperationMap(s) => {
                request.tag.contains(s.as_str()) && request.operation_mapping.contains(s.as_str())
            }
            MatchTarget::TagOrOperationMap(s) => {
                request.tag.contains(s.as_str()) || request.operation_mapping.contains(s.as_str())
            }
        }
    }

    fn modify_contains(&self, pat: &str, request: &mut Request) {
        match self {
            MatchTarget::OperationMap(s) => {
                request.operation_mapping = request.operation_mapping.replace(pat, s);
            }
            MatchTarget::Tag(s) => {
                request.tag = request.tag.replace(pat, s);
            }
            MatchTarget::TagAndOperationMap(s) => {
                request.tag = request.tag.replace(pat, s);
                request.operation_mapping = request.operation_mapping.replace(pat, s);
            }
            MatchTarget::TagOrOperationMap(s) => {
                request.tag = request.tag.replace(pat, s);
                request.operation_mapping = request.operation_mapping.replace(pat, s);
            }
            MatchTarget::OperationId(s) => {
                request.operation_id = request.operation_id.replace(pat, s);
            }
        }
        request.param_size = INTERNAL_PATH_ID.captures_iter(&request.path).count();
        if request.path.contains("RID") && request.param_size > 0 {
            request.param_size -= 1;
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ModifierMap {
    pub map: HashMap<MatchTarget, Vec<MatchTarget>>,
}

impl ModifierMap {
    pub fn with_capacity(size: usize) -> ModifierMap {
        ModifierMap {
            map: HashMap::with_capacity(size),
        }
    }

    pub fn insert(&mut self, from: MatchTarget, to: Vec<MatchTarget>) {
        let to_clone = to.clone();
        self.map
            .entry(from)
            .and_modify(|vec| {
                vec.extend(to_clone);
            })
            .or_insert(to);
    }

    pub fn operation_map(&mut self, from: &str, to: &str) {
        self.insert(
            MatchTarget::OperationMap(from.to_string()),
            vec![MatchTarget::OperationMap(to.to_string())],
        );
    }

    pub fn operation_id(&mut self, from: &str, to: &str) {
        self.insert(
            MatchTarget::OperationId(from.to_string()),
            vec![MatchTarget::OperationId(to.to_string())],
        );
    }
}
