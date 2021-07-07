use crate::parser::{PathMap, RequestSet};
use from_as::*;
use inflector::Inflector;
use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResourceNames {
    pub names: BTreeSet<String>,
}

impl ResourceNames {
    pub fn new(names: BTreeSet<String>) -> ResourceNames {
        ResourceNames { names }
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.names.clone().into_iter().collect()
    }

    pub fn sort(&mut self) {
        let mut v: Vec<String> = self.to_vec();
        v = v.iter().map(|s| s.to_camel_case()).collect();
        v.sort();
        self.names = v.into_iter().collect();
    }
}

impl From<Vec<String>> for ResourceNames {
    fn from(paths: Vec<String>) -> Self {
        let mut resource = ResourceNames::new(BTreeSet::new());
        let mut names: Vec<String> = Vec::new();

        for path in paths.iter() {
            let mut vec: VecDeque<&str> = path.split('/').collect();
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
}

impl From<RequestSet> for ResourceNames {
    fn from(request_set: RequestSet) -> Self {
        ResourceNames::from(&request_set)
    }
}

impl From<&RequestSet> for ResourceNames {
    fn from(request_set: &RequestSet) -> Self {
        let mut paths: Vec<String> = Vec::new();
        for request_map in request_set.set.iter() {
            paths.push(request_map.path.to_string());
        }
        ResourceNames::from(paths)
    }
}

impl From<PathMap> for ResourceNames {
    fn from(path_map: PathMap) -> Self {
        let mut paths: Vec<String> = Vec::new();
        for (path, _p) in path_map.paths.iter() {
            paths.push(path.to_string());
        }

        ResourceNames::from(paths)
    }
}

impl From<HashMap<String, RequestSet>> for ResourceNames {
    fn from(map: HashMap<String, RequestSet>) -> Self {
        let mut resource_names = ResourceNames::default();
        for (name, _request_set) in map.iter() {
            resource_names.names.insert(name.to_camel_case());
        }
        resource_names
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResourceNameMapping {
    pub map: HashMap<String, BTreeSet<String>>,
}

impl ResourceNameMapping {
    pub fn new(map: HashMap<String, BTreeSet<String>>) -> ResourceNameMapping {
        ResourceNameMapping { map }
    }
}
