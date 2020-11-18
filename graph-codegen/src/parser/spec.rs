use crate::builder::ClientLinkSettings;
use crate::parser::filter::UrlMatchTarget::ResourceId;
use crate::parser::filter::{
    Filter, FilterIgnore, MatchTarget, ModifierMap, SecondaryModifierMap, UrlMatchTarget,
};
use crate::parser::{HttpMethod, ParserSettings, PathMap, Request, RequestMap, RequestSet};
use crate::traits::{Modify, RequestParser, RequestParserBuilder};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use rayon::prelude::*;
use reqwest::Url;
use serde::Serialize;
use std::cell::{RefCell, RefMut};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

#[derive(Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(default)]
pub struct ParserSpec {
    paths: PathMap,
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    requests: VecDeque<RequestMap>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    tag_map: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    operation_map: HashMap<String, String>,
    modify_target: ModifierMap,
    secondary_modify_target: SecondaryModifierMap,
    url_modify_target: HashSet<UrlMatchTarget>,
    modifiers: BTreeSet<String>,
    links_override: HashMap<String, Vec<String>>,
    secondary_links: HashMap<String, Vec<String>>,
    client_links: BTreeMap<String, BTreeSet<ClientLinkSettings>>,
}

impl ParserSpec {
    fn parser_spec(path_map: PathMap) -> ParserSpec {
        ParserSpec {
            paths: path_map,
            requests: Default::default(),
            tag_map: Default::default(),
            operation_map: Default::default(),
            modify_target: ModifierMap::with_capacity(30),
            secondary_modify_target: SecondaryModifierMap::with_capacity(15),
            url_modify_target: HashSet::with_capacity(15),
            modifiers: Default::default(),
            links_override: Default::default(),
            secondary_links: HashMap::with_capacity(10),
            client_links: Default::default(),
        }
    }

    pub fn new(mut path_map: PathMap) -> ParserSpec {
        path_map.clean();
        ParserSpec::parser_spec(path_map)
    }

    pub fn parse<P: AsRef<Path>>(file: P) -> ParserSpec {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        path_map.clean();
        ParserSpec::parser_spec(path_map)
    }

    pub fn parse_secondary<P: AsRef<Path>>(
        file: P,
        start_filter: Filter,
        secondary_name: &str,
    ) -> ParserSpec {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        let mut path_map: PathMap = path_map.filter(start_filter).into();
        let mut path_map = path_map.clean_secondary(secondary_name);
        ParserSpec::parser_spec(path_map)
    }
}

impl Debug for ParserSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserSpec")
            .field("tag_map", &self.tag_map)
            .field("operation_map", &self.operation_map)
            .field("modify_target", &self.modify_target)
            .field("url_modify_target", &self.url_modify_target)
            .field("modifiers", &self.modifiers)
            .field("links_override", &self.links_override)
            .finish()
    }
}

impl ParserSpec {
    pub fn modifier_map(&mut self) -> &mut ModifierMap {
        &mut self.modify_target
    }

    pub fn resource_modifier_set(&mut self) -> HashSet<UrlMatchTarget> {
        self.url_modify_target.clone()
    }

    pub fn add_client_link_settings(&mut self, name: &str, settings: ClientLinkSettings) {
        self.client_links
            .entry(name.to_string())
            .and_modify(|set| {
                set.insert(settings.clone());
            })
            .or_insert_with(|| {
                let mut set = BTreeSet::new();
                set.insert(settings.clone());
                set
            });
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(default)]
pub struct Parser {
    pub(crate) spec: RefCell<ParserSpec>,
}

impl Parser {
    pub fn parse<P: AsRef<Path>>(file: P) -> Parser {
        Parser {
            spec: RefCell::new(ParserSpec::parse(file)),
        }
    }

    pub fn parse_filter<P: AsRef<str>>(file: P, filter: Filter<'_>) -> Parser {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        path_map.clean();

        Parser {
            spec: RefCell::new(ParserSpec {
                paths: path_map.filter(filter).into(),
                requests: Default::default(),
                tag_map: Default::default(),
                operation_map: Default::default(),
                modify_target: ModifierMap::with_capacity(30),
                secondary_modify_target: SecondaryModifierMap::with_capacity(15),
                url_modify_target: HashSet::with_capacity(15),
                modifiers: Default::default(),
                links_override: Default::default(),
                secondary_links: HashMap::with_capacity(10),
                client_links: Default::default(),
            }),
        }
    }

    pub fn parse_secondary<P: AsRef<Path>>(
        file: P,
        start_filter: Filter,
        secondary_name: &str,
    ) -> Parser {
        let parser = Parser {
            spec: RefCell::new(ParserSpec::parse_secondary(
                file,
                start_filter,
                secondary_name,
            )),
        };

        if let Ok(resource_identity) = ResourceIdentity::from_str(secondary_name) {
            let mut spec = parser.spec.borrow_mut();
            for filter in ParserSettings::path_filters(resource_identity).iter() {
                spec.paths = spec.paths.filter(filter.clone()).into();
            }
        }

        parser
    }

    pub fn path_map(&self) -> PathMap {
        self.spec.borrow().paths.clone()
    }

    pub fn set_path_map(&self, path_map: PathMap) {
        self.spec.borrow_mut().paths = path_map;
    }

    pub fn resource_modifier_set(&self) -> HashSet<UrlMatchTarget> {
        self.spec.borrow().url_modify_target.clone()
    }

    pub fn secondary_links(&self) -> HashMap<String, Vec<String>> {
        self.spec.borrow().secondary_links.clone()
    }

    pub fn client_links(&self) -> BTreeMap<String, BTreeSet<ClientLinkSettings>> {
        self.spec.borrow().client_links.clone()
    }

    pub fn set_operation_map(&self, operation_map: HashMap<String, String>) {
        self.spec.borrow_mut().operation_map = operation_map;
    }

    pub fn add_operation_mapping(&self, original: &str, replace_with: &str) {
        self.spec
            .borrow_mut()
            .operation_map
            .insert(original.into(), replace_with.into());
    }

    pub fn add_tag_mapping(&self, original: &str, replace_with: &str) {
        self.spec
            .borrow_mut()
            .tag_map
            .insert(original.into(), replace_with.into());
    }

    pub fn add_match_modifier(&self, matcher: MatchTarget, modifier: Vec<MatchTarget>) {
        self.spec
            .borrow_mut()
            .modify_target
            .map
            .insert(matcher, modifier);
    }

    pub fn add_url_modifier(&self, modifier: UrlMatchTarget) {
        self.spec.borrow_mut().url_modify_target.insert(modifier);
    }

    pub fn modifier_map(&self) -> RefMut<ParserSpec> {
        self.spec.borrow_mut()
    }

    pub fn use_default_modifier(&self, name: &str) {
        self.use_default_modifiers(&[name]);
    }

    pub fn use_default_modifiers(&self, names: &[&str]) {
        let mut spec = self.spec.borrow_mut();

        for name in names.iter() {
            dbg!(name);
            let shorthand = &name[..name.len() - 1];
            let shorthand_name = format!("{}.{}", name, shorthand);
            let double_name = format!("{}.{}", name, name);
            let functions = format!("{}.Functions", name);
            let actions = format!("{}.Actions", name);

            spec.modify_target.map.insert(
                MatchTarget::OperationMap("".to_string()),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );
            spec.modify_target.map.insert(
                MatchTarget::OperationMap(shorthand_name),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );
            spec.modify_target.map.insert(
                MatchTarget::OperationMap(double_name),
                vec![MatchTarget::OperationMap(name.to_string())],
            );
            spec.modify_target.map.insert(
                MatchTarget::TagAndOperationMap(actions),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );
            spec.modify_target.map.insert(
                MatchTarget::TagAndOperationMap(functions),
                vec![MatchTarget::TagAndOperationMap(name.to_string())],
            );

            spec.modifiers.insert(name.to_string());
            let resource_identity = ResourceIdentity::from_str(name).unwrap();

            let filters = ParserSettings::path_filters(resource_identity);
            for filter in filters {
                spec.paths = spec.paths.filter(filter).into();
            }

            let settings = ParserSettings::client_link_settings(resource_identity);
            spec.client_links.extend(settings);

            let target_modifiers = ParserSettings::target_modifiers(resource_identity);
            spec.modify_target.map.extend(target_modifiers.map);

            let url_modifiers = ParserSettings::url_target_modifiers(resource_identity);
            spec.url_modify_target.extend(url_modifiers);

            let secondary_modifiers = ParserSettings::secondary_modifier_map(resource_identity);
            spec.secondary_modify_target
                .secondary_targets
                .extend(secondary_modifiers.secondary_targets);

            dbg!(&spec.modify_target);
        }

        // TODO: Change the inserts here to use the ModifierMap methods to cut down on code.
        // Modifiers that need to be explicitly declared.
        // The struct names for clients are generated based on the operation id
        // which is also modified when the clients are generated. This can result
        // in naming conflicts that is fixed by these modifiers.
        spec.modify_target.operation_map(
            "deviceManagement.detectedApps.managedDevices",
            "deviceManagement.detectedApps.appManagedDevices",
        );
        spec.modify_target.operation_map(
            "directoryObjects.microsoft.graph.administrativeUnit",
            "directoryObjects.administrativeUnits",
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationId("directory.administrativeUnits.delta.fa14".to_string()),
            vec![
                MatchTarget::OperationId("directoryObjects.administrativeUnits.delta".to_string()),
                MatchTarget::OperationMap("directoryObjects.administrativeUnits".to_string()),
            ],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationId("directoryRoles.delta.fa14".to_string()),
            vec![
                MatchTarget::OperationId("directoryRoles.delta".to_string()),
                MatchTarget::OperationMap("directoryRoles".to_string()),
            ],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationMap("policies.policyRoot".to_string()),
            vec![MatchTarget::OperationMap("policies".to_string())],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationMap("sites.contentTypes".to_string()),
            vec![MatchTarget::OperationMap("sites".to_string())],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationMap("sites.lists.contentTypes".to_string()),
            vec![MatchTarget::OperationMap("sites.lists".to_string())],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationMap("groupLifecyclePolicies.groupLifecyclePolicy".to_string()),
            vec![MatchTarget::OperationMap(
                "groupLifecyclePolicies".to_string(),
            )],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationMap("auditLogs.auditLogRoot".to_string()),
            vec![MatchTarget::OperationMap("auditLogs".to_string())],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationMap("policies.policyRoot".to_string()),
            vec![MatchTarget::OperationMap("policies".to_string())],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationMap("teams.primaryChannel.messages".to_string()),
            vec![MatchTarget::OperationMap(
                "teams.primaryChannel.primaryChannelMessages".to_string(),
            )],
        );
        spec.modify_target.map.insert(
            MatchTarget::OperationMap("teams.primaryChannel.tabs".to_string()),
            vec![MatchTarget::OperationMap(
                "teams.primaryChannel.primaryChannelTabs".to_string(),
            )],
        );

        /*
               // Secondary links and modifiers. These are api's that are used multiple times
               // such as calendars and calendar groups where we might have two resources
               // such as groups and users with the same ending path:
               // groups/{group-id}/calendars/{calendar-id}
               // users/{user-id}/calendars/{calendar-id}
               // We do not want each api implementation to have its own calendar struct
               // and methods to prevent repeated code. So we separate these out here
               // and add a link between them.
               spec.secondary_modify_target.insert(
                   "users.calendarGroups",
                   MatchTarget::OperationMap("calendarGroups".to_string()),
               );
               spec.secondary_modify_target.insert(
                   "users.calendarGroups",
                   MatchTarget::OperationId("calendarGroups".to_string()),
               );

               spec.secondary_modify_target.insert(
                   "users.calendars",
                   MatchTarget::OperationMap("calendars".to_string()),
               );
               spec.secondary_modify_target.insert(
                   "users.calendars",
                   MatchTarget::OperationId("calendars".to_string()),
               );

               spec.secondary_modify_target.insert(
                   "users.calendar",
                   MatchTarget::OperationMap("calendar".to_string()),
               );
               spec.secondary_modify_target.insert(
                   "users.calendar",
                   MatchTarget::OperationId("calendar".to_string()),
               );
               spec.secondary_modify_target.insert(
                   "users.events",
                   MatchTarget::OperationId("events".to_string()),
               );

        */
        // Modify that paths that have a resource id. See UrlMatchTarget
        // for more info.

        self.use_filters_internal(spec, ParserSettings::default_path_filters());
    }

    pub fn filter(&self, filter: Filter<'_>) -> PathMap {
        let spec = self.spec.borrow();
        PathMap {
            paths: spec.paths.filter(filter),
        }
    }

    pub fn multi_filter(&self, filters: Vec<Filter<'_>>) -> PathMap {
        let spec = self.spec.borrow();
        let mut path_map = spec.paths.clone();

        for filter in filters.iter() {
            path_map = path_map.filter(filter.clone()).into();
        }

        path_map
    }

    fn use_filter_internal(&self, mut spec: RefMut<ParserSpec>, filter: Filter<'_>) {
        spec.paths = spec.paths.filter(filter).into();
    }

    fn use_filters_internal(&self, mut spec: RefMut<ParserSpec>, filters: Vec<Filter<'_>>) {
        let mut path_map = spec.paths.clone();

        for filter in filters.iter() {
            path_map = PathMap {
                paths: path_map.filter(filter.clone()),
            }
        }

        spec.paths = path_map;
    }

    pub fn use_filters(&self, filters: Vec<Filter<'_>>) {
        let spec = self.spec.borrow_mut();
        self.use_filters_internal(spec, filters);
    }

    pub fn add_links_override(&self, spec_client_name: &str, links: &[&str]) {
        self.spec.borrow_mut().links_override.insert(
            spec_client_name.to_string(),
            links.iter().map(|s| s.to_string()).collect(),
        );
    }

    pub fn use_default_links_override(&self) {
        let mut spec = self.spec.borrow_mut();
        spec.links_override.insert(
            "directory".to_string(),
            [
                "directoryRoles",
                "directoryObjects",
                "directoryRoleTemplates",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );
    }

    pub fn get_links_override(&self) -> HashMap<String, Vec<String>> {
        self.spec.borrow().links_override.clone()
    }

    pub fn build(&self) -> HashMap<String, RequestSet> {
        let mut spec = self.spec.borrow_mut();
        let modifier = spec.modify_target.clone();
        let secondary_modifier = spec.secondary_modify_target.clone();
        let modifier_filters = spec.modifiers.clone();
        let url_modifiers = spec.url_modify_target.clone();
        let mut req_set_map = HashMap::new();

        let operation_mapping_fn = |request: &mut Request, modifier_filter: &str| {
            if request.operation_mapping.is_empty() {
                request.operation_mapping = modifier_filter.to_string();
            }
        };

        for modifier_filter in modifier_filters.iter() {
            let path_map: PathMap = spec
                .paths
                .filter(Filter::PathStartsWith(&format!("/{}", modifier_filter)))
                .into();

            for (path, path_spec) in path_map.paths.iter() {
                let mut req_map = RequestMap::default();
                let path = path.transform_path();
                req_map.path = path.clone();

                if let Some(operation) = path_spec.get.as_ref() {
                    let mut request = operation.build(path.clone(), &modifier, &secondary_modifier);
                    request.method = HttpMethod::GET;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                if let Some(operation) = path_spec.post.as_ref() {
                    let mut request = operation.build(path.clone(), &modifier, &secondary_modifier);
                    request.method = HttpMethod::POST;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                if let Some(operation) = path_spec.put.as_ref() {
                    let mut request = operation.build(path.clone(), &modifier, &secondary_modifier);
                    request.method = HttpMethod::PUT;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                if let Some(operation) = path_spec.patch.as_ref() {
                    let mut request = operation.build(path.clone(), &modifier, &secondary_modifier);
                    request.method = HttpMethod::PATCH;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                if let Some(operation) = path_spec.delete.as_ref() {
                    let mut request = operation.build(path.clone(), &modifier, &secondary_modifier);
                    request.method = HttpMethod::DELETE;
                    operation_mapping_fn(&mut request, modifier_filter.as_ref());
                    req_map.requests.push_back(request);
                }

                for modifier in url_modifiers.iter() {
                    if modifier.matches(&req_map) {
                        modifier.modify(&mut req_map);
                    }
                }

                if let Some(r) = spec
                    .requests
                    .iter_mut()
                    .find(|r| r.path.eq(req_map.path.as_str()))
                {
                    r.requests.extend(req_map.requests);
                } else {
                    spec.requests.push_back(req_map);
                }
            }

            let mut request_set = RequestSet::default();
            let mut requests = spec.requests.clone();
            while let Some(req) = requests.pop_front() {
                request_set.join_inner_insert(req);
            }

            for modifier in url_modifiers.iter() {
                modifier.modify(&mut request_set);
            }

            req_set_map.insert(modifier_filter.clone(), request_set);
            spec.requests.clear();
        }
        req_set_map
    }
}

impl TryFrom<reqwest::Url> for Parser {
    type Error = reqwest::Error;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value)?;
        let s = response.text().unwrap();
        dbg!(&s);
        let path_map: PathMap = serde_yaml::from_str(s.as_str()).unwrap();
        Ok(Parser {
            spec: RefCell::new(ParserSpec::new(path_map)),
        })
    }
}
