use crate::{
    builder::ClientLinkSettings,
    parser::{
        filter::{
            Filter, ModifierMap, ResourceIdentityModifier, ResourceUrlReplacement,
            SecondaryModifierMap,
        },
        DirectoryModFile, ParserSettings, PathMap, RequestMap, RequestSet, ResourceRequestMap,
    },
    traits::Modify,
};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use reqwest::Url;
use std::{
    cell::{RefCell, RefMut},
    collections::{BTreeMap, BTreeSet, HashMap, VecDeque},
    convert::TryFrom,
    fmt::Debug,
    path::Path,
    str::FromStr,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Modifier {
    pub(crate) name: String,
    pub(crate) resource_identity: ResourceIdentity,
    pub(crate) modifier_map: ModifierMap,
    pub(crate) resource_url_modifier: Option<ResourceIdentityModifier>,
    pub(crate) client_links: BTreeMap<String, BTreeSet<ClientLinkSettings>>,
    pub(crate) secondary_modify_target: SecondaryModifierMap,
    pub(crate) custom_methods: Option<HashMap<String, RequestSet>>,
    pub(crate) filters: Vec<Filter>,
    pub(crate) imports: BTreeSet<String>,
    pub(crate) links_override: HashMap<String, Vec<String>>,
    pub(crate) directory_mod: Option<DirectoryModFile>,
}

impl Modifier {
    pub fn new(modifier_name: &str) -> Modifier {
        let resource_identity = ResourceIdentity::from_str(modifier_name).unwrap();
        Modifier::from(resource_identity)
    }

    pub fn build_modifier_vec(names: &[&str]) -> Vec<Modifier> {
        let mut vec: Vec<Modifier> = Vec::new();
        for name in names.iter() {
            vec.push(Modifier::new(name));
        }
        vec
    }

    pub fn build_modifier_vec_resource_identity(
        resource_identity_vec: &[ResourceIdentity],
    ) -> Vec<Modifier> {
        let mut vec: Vec<Modifier> = Vec::new();
        for resource_identity in resource_identity_vec {
            vec.push(Modifier::from(*resource_identity));
        }
        vec
    }
}

impl From<ResourceIdentity> for Modifier {
    fn from(resource_identity: ResourceIdentity) -> Self {
        let modifier_name = &resource_identity.to_string();
        let shorthand = &modifier_name[..modifier_name.len() - 1];
        let shorthand_name = format!("{}.{}", modifier_name, shorthand);
        let double_name = format!("{}.{}", modifier_name, modifier_name);

        let mut filters = ParserSettings::path_filters(resource_identity);
        let default_filters = ParserSettings::default_path_filters();
        filters.extend(default_filters);

        let mut modifier = Modifier {
            name: modifier_name.to_string(),
            resource_identity,
            modifier_map: ParserSettings::target_modifiers(resource_identity),
            resource_url_modifier: ParserSettings::resource_identity_modifier(resource_identity),
            client_links: ParserSettings::client_link_settings(resource_identity),
            secondary_modify_target: ParserSettings::secondary_modifier_map(resource_identity),
            custom_methods: ParserSettings::custom_methods(resource_identity),
            filters,
            imports: ParserSettings::imports(resource_identity),
            links_override: ParserSettings::links_override(resource_identity),
            directory_mod: ParserSettings::directory_mod(resource_identity),
        };

        modifier.modifier_map.operation_map("", modifier_name);
        modifier
            .modifier_map
            .operation_map(shorthand_name.as_str(), modifier_name);
        modifier
            .modifier_map
            .operation_map(double_name.as_str(), modifier_name);

        let handlebars_import = "handlebars::*";
        if ParserSettings::is_registered_ident_client(resource_identity) {
            modifier.imports.insert(handlebars_import.into());
        }

        if !modifier.imports.contains(handlebars_import) {
            for (name, _set) in modifier.client_links.iter() {
                // let ri = ResourceIdentity::from_str(name.as_str()).unwrap();

                if let Ok(ri) = ResourceIdentity::from_str(name.as_str()) {
                    if ParserSettings::is_registered_ident_client(ri) {
                        modifier.imports.insert(handlebars_import.into());
                    }
                }
            }
        }

        modifier
    }
}

impl<'a> From<&str> for Modifier {
    fn from(value: &str) -> Self {
        Modifier::from(ResourceIdentity::from_str(value).unwrap())
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParserSpec {
    pub(crate) paths: PathMap,
    pub(crate) modifiers: Vec<Modifier>,
}

impl ParserSpec {
    pub(crate) fn parser_spec(path_map: PathMap, modifiers: Vec<Modifier>) -> ParserSpec {
        ParserSpec {
            paths: path_map,
            modifiers,
        }
    }

    pub fn new(mut path_map: PathMap, modifiers: Vec<Modifier>) -> ParserSpec {
        path_map.clean();
        ParserSpec::parser_spec(path_map, modifiers)
    }

    pub fn parse<P: AsRef<Path>>(file: P) -> ParserSpec {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        path_map.clean();
        ParserSpec::parser_spec(path_map, Default::default())
    }

    pub fn parse_secondary<P: AsRef<Path>>(
        file: P,
        start_filter: Filter,
        secondary_name: &str,
    ) -> ParserSpec {
        let path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        let mut path_map: PathMap = path_map.filter(start_filter).into();
        let path_map = path_map.clean_secondary(secondary_name);
        ParserSpec::parser_spec(path_map, Default::default())
    }

    pub fn set_modifiers(&mut self, modifiers: Vec<Modifier>) {
        self.modifiers = modifiers;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Parser {
    pub(crate) spec: RefCell<ParserSpec>,
}

impl Parser {
    pub fn parse<P: AsRef<Path>>(file: P) -> Parser {
        Parser {
            spec: RefCell::new(ParserSpec::parse(file)),
        }
    }

    pub fn parse_filter<P: AsRef<str>>(file: P, filter: Filter) -> Parser {
        let mut path_map: PathMap = PathMap::from_file(file.as_ref()).unwrap();
        path_map.clean();

        Parser {
            spec: RefCell::new(ParserSpec {
                paths: path_map.filter(filter).into(),
                modifiers: Default::default(),
            }),
        }
    }

    pub fn path_map(&self) -> PathMap {
        self.spec.borrow().paths.clone()
    }

    pub fn set_path_map(&self, path_map: PathMap) {
        self.spec.borrow_mut().paths = path_map;
    }

    pub fn modifiers(&self) -> Vec<Modifier> {
        self.spec.borrow().modifiers.clone()
    }

    pub fn set_modifiers(&self, modifiers: Vec<Modifier>) {
        self.spec.borrow_mut().set_modifiers(modifiers);
    }

    pub fn filter(&self, filter: Filter) -> PathMap {
        let spec = self.spec.borrow();
        PathMap {
            paths: spec.paths.filter(filter),
        }
    }

    pub fn multi_filter(&self, filters: Vec<Filter>) -> PathMap {
        let spec = self.spec.borrow();
        let mut path_map = spec.paths.clone();

        for filter in filters.iter() {
            path_map = path_map.filter(filter.clone()).into();
        }

        path_map
    }

    fn use_filters_internal(&self, mut spec: RefMut<ParserSpec>, filters: Vec<Filter>) {
        let mut path_map = spec.paths.clone();

        for filter in filters.iter() {
            path_map = PathMap {
                paths: path_map.filter(filter.clone()),
            }
        }

        spec.paths = path_map;
    }

    pub fn use_filters(&self, filters: Vec<Filter>) {
        let spec = self.spec.borrow_mut();
        self.use_filters_internal(spec, filters);
    }

    pub fn build(&self) -> Vec<ResourceRequestMap> {
        let spec = self.spec.borrow();
        let modifiers = spec.modifiers.clone();
        let mut resource_requests: Vec<ResourceRequestMap> = Vec::new();

        for modifier in modifiers {
            let mut path_map: PathMap = spec
                .paths
                .filter(Filter::PathStartsWith(format!("/{}", modifier.name)))
                .into();

            path_map.transform_paths();
            for filter in modifier.filters.iter() {
                path_map = path_map.filter(filter.clone()).into();
            }

            let mut requests: VecDeque<RequestMap> = VecDeque::new();
            for (path, path_spec) in path_map.paths.iter() {
                let mut req_map = RequestMap {
                    path: path.clone(),
                    requests: path_spec.build_requests(&path, &modifier),
                };

                if let Some(url_modifier) = modifier.resource_url_modifier.as_ref() {
                    if url_modifier.matches(&req_map) {
                        url_modifier.modify(&mut req_map);
                    }

                    if url_modifier.modify_using_replacement()
                        && url_modifier.matches_replacement(&req_map)
                    {
                        url_modifier.modify(&mut req_map);
                    }
                }

                if let Some(request_map) = requests
                    .iter_mut()
                    .find(|r| r.path.eq(req_map.path.as_str()))
                {
                    request_map.requests.extend(req_map.requests);
                } else {
                    requests.push_back(req_map);
                }
            }

            let mut request_set = RequestSet::default();
            while let Some(req) = requests.pop_front() {
                request_set.join_inner_insert(req);
            }

            if let Some(url_modifier) = modifier.resource_url_modifier.as_ref() {
                url_modifier.modify(&mut request_set);
            }

            resource_requests.push(ResourceRequestMap::new(modifier, request_set));
            requests.clear();
        }

        resource_requests
    }
}

impl<'a> TryFrom<reqwest::Url> for Parser {
    type Error = reqwest::Error;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let response = reqwest::blocking::get(value)?;
        let s = response.text().unwrap();
        let path_map: PathMap = serde_yaml::from_str(s.as_str()).unwrap();
        Ok(Parser {
            spec: RefCell::new(ParserSpec::new(path_map, Default::default())),
        })
    }
}
