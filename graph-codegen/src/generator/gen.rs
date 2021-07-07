use crate::{
    builder::{Builder, StoredClientSet},
    parser::{
        client_resource::ClientResource, error::ParseError, Modifier, Parser, ParserSettings,
        ParserSpec, PathMap, ResourceRequestMap,
    },
    traits::Parse,
};
use from_as::FromFile;
use graph_core::resource::ResourceIdentity;
use rayon::prelude::*;
use std::{cell::RefCell, convert::TryFrom, str::FromStr};

static API_V1_METADATA_URL_STR: &str = "https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/v1.0/openapi.yaml";

lazy_static! {
    static ref API_V1_METADATA_URL: reqwest::Url =
        reqwest::Url::parse(API_V1_METADATA_URL_STR).unwrap();
}

pub trait Generate<Clients> {
    fn generate(clients: Clients) -> Result<(), ParseError>;
    fn dry_run(clients: Clients) -> Result<(), ParseError>;
}

/// Generate the Graph API clients.
///
/// # Example
/// ```rust,ignore
/// use graph_codegen::generator::{Generator, Generate};
/// use graph_core::resource::ResourceIdentity;
///
/// Generator::generate(vec![
///     ResourceIdentity::Users,
///     ResourceIdentity::Me,
///     ResourceIdentity::Calendar,
///     ResourceIdentity::CalendarGroups
/// ]).unwrap();
/// ```
///
/// Not all clients in the ResourceIdentity enum can be generated. To see which
/// clients can be used to generate the API see ClientResource::try_from for
/// ResourceIdentity.
/// # Example
/// ```
/// # use graph_codegen::parser::client_resource::ClientResource;
/// # use std::convert::TryFrom;
/// # use graph_core::resource::ResourceIdentity;
/// let client_resource = ClientResource::try_from(ResourceIdentity::Users).unwrap();
/// println!("{:#?}", client_resource);
/// ```
///
/// You can also use the parser trait to do a more custom parsing.
/// # Example
/// ```rust,ignore
/// use graph_codegen::generator::Generator;
/// use graph_core::resource::ResourceIdentity;
/// use graph_codegen::parser::Parse;
/// use graph_codegen::parser::client_resource::ClientResource;
///
/// // The Main client resource filters requests based on the modifier given.
/// // The modifier should be what the starting path for requests you want to
/// // filter on. For instance, the client resource below will get all requests
/// // who's path starts with /drives.
/// let client_resource = ClientResource::Main { modifier: "/drives".to_string() };
///
/// // Add you OpenAPI for Microsoft Graph here. The file type can be one of JSON,
/// // yaml, or toml.
/// let path = std::path::Path::new("YOU_OPEN_API_FILE");
///
/// // Pass the path and client resource to the parse method. The generate_requests
/// // method returns a HashMap<String, RequestSet> where the key is the name of
/// // the current client (such as drives, users, me, etc.). See RequestSet for
/// // more info. You can also use the generate_clients method to build and create
/// // the generated rust code files. Note that the current settings can only be used
/// // to create files in this repository.
/// let builder = Generator::parse(path, client_resource).unwrap();
/// let map = builder.generate_requests();
/// pretty!(map);
/// ```
#[derive(Default, Debug)]
pub struct Generator {
    builder: Builder,
}

impl Generator {
    pub fn new(resource_identity: ResourceIdentity) -> Result<Generator, ParseError> {
        let client_resource = ClientResource::try_from(resource_identity)?;
        Ok(Generator {
            builder: Generator::parse(API_V1_METADATA_URL.clone(), client_resource)?,
        })
    }

    pub fn get_client_resource(resource_identity: ResourceIdentity) -> Option<ClientResource> {
        ClientResource::try_from(resource_identity).ok()
    }

    pub fn generate_requests(&self) -> Vec<ResourceRequestMap> {
        self.builder.generate_requests()
    }

    pub fn generate_clients(&self) {
        self.builder.build_clients();
    }

    pub fn get_parser() -> Result<Parser, ParseError> {
        Ok(Parser::try_from(API_V1_METADATA_URL.clone())?)
    }

    pub fn build_stored_clients(&self) -> StoredClientSet {
        self.builder.build_stored_clients()
    }

    pub fn write_stored_clients(stored_client_set: StoredClientSet) {
        Builder::write_stored_clients(stored_client_set);
    }
}

impl Parse<&std::path::Path> for Generator {
    type Error = ParseError;

    fn parse(
        parse_from: &std::path::Path,
        client_resource: ClientResource,
    ) -> Result<Builder, Self::Error> {
        match client_resource {
            ClientResource::Main { modifier } => {
                let mut path_map: PathMap = PathMap::from_file(parse_from)?;
                path_map.clean();

                let modifiers = Modifier::build_modifier_vec(&[&modifier]);
                let parser = Parser {
                    spec: RefCell::new(ParserSpec::parser_spec(path_map, modifiers)),
                };

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);

                Ok(builder)
            }
            ClientResource::Secondary {
                start_filter,
                modifier,
            } => {
                let path_map: PathMap = PathMap::from_file(parse_from)?;
                let mut path_map: PathMap = path_map.filter(start_filter).into();
                let path_map = path_map.clean_secondary(modifier.as_str());

                let modifiers = Modifier::build_modifier_vec(&[&modifier]);
                let parser = Parser {
                    spec: RefCell::new(ParserSpec::parser_spec(path_map, modifiers)),
                };

                if let Ok(resource_identity) = ResourceIdentity::from_str(modifier.as_str()) {
                    let mut spec = parser.spec.borrow_mut();
                    for filter in ParserSettings::path_filters(resource_identity).iter() {
                        spec.paths = spec.paths.filter(filter.clone()).into();
                    }
                }

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);

                Ok(builder)
            }
            ClientResource::MainResourceIdentity { modifier } => {
                let mut path_map: PathMap = PathMap::from_file(parse_from)?;
                path_map.clean();

                let modifiers = Modifier::build_modifier_vec_resource_identity(&[modifier]);
                let parser = Parser {
                    spec: RefCell::new(ParserSpec::parser_spec(path_map, modifiers)),
                };

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);

                Ok(builder)
            }
            ClientResource::SecondaryResourceIdentity {
                start_filter,
                modifier,
            } => {
                let path_map: PathMap = PathMap::from_file(parse_from)?;
                let mut path_map: PathMap = path_map.filter(start_filter).into();
                let path_map = path_map.clean_secondary(&modifier.to_string());
                let modifiers = Modifier::build_modifier_vec_resource_identity(&[modifier]);
                let parser_spec = ParserSpec::parser_spec(path_map, modifiers);

                let parser = Parser {
                    spec: RefCell::new(parser_spec),
                };

                {
                    let mut spec = parser.spec.borrow_mut();
                    for filter in ParserSettings::path_filters(modifier).iter() {
                        spec.paths = spec.paths.filter(filter.clone()).into();
                    }
                }

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);

                Ok(builder)
            }
        }
    }
}

impl Parse<reqwest::Url> for Generator {
    type Error = ParseError;

    fn parse(
        parse_from: reqwest::Url,
        client_resource: ClientResource,
    ) -> Result<Builder, Self::Error> {
        match client_resource {
            ClientResource::Main { modifier } => {
                let parser = Parser::try_from(parse_from)?;
                let modifiers = Modifier::build_modifier_vec(&[&modifier]);
                parser.set_modifiers(modifiers);

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);

                Ok(builder)
            }
            ClientResource::Secondary {
                start_filter,
                modifier,
            } => {
                let path_map = PathMap::try_from(parse_from)?;
                let mut path_map: PathMap = path_map.filter(start_filter).into();
                let path_map = path_map.clean_secondary(modifier.as_str());
                let modifiers = Modifier::build_modifier_vec(&[&modifier]);
                let parser_spec = ParserSpec::parser_spec(path_map, modifiers);

                let parser = Parser {
                    spec: RefCell::new(parser_spec),
                };

                if let Ok(resource_identity) = ResourceIdentity::from_str(modifier.as_str()) {
                    let mut spec = parser.spec.borrow_mut();
                    for filter in ParserSettings::path_filters(resource_identity).iter() {
                        spec.paths = spec.paths.filter(filter.clone()).into();
                    }
                }

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);

                Ok(builder)
            }
            ClientResource::MainResourceIdentity { modifier } => {
                let parser = Parser::try_from(parse_from)?;
                let modifiers = Modifier::build_modifier_vec_resource_identity(&[modifier]);
                parser.set_modifiers(modifiers);

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);

                Ok(builder)
            }
            ClientResource::SecondaryResourceIdentity {
                start_filter,
                modifier,
            } => {
                let path_map = PathMap::try_from(parse_from)?;
                let mut path_map: PathMap = path_map.filter(start_filter).into();
                let path_map = path_map.clean_secondary(&modifier.to_string());
                let modifiers = Modifier::build_modifier_vec_resource_identity(&[modifier]);
                let parser_spec = ParserSpec::parser_spec(path_map, modifiers);

                let parser = Parser {
                    spec: RefCell::new(parser_spec),
                };

                {
                    let mut spec = parser.spec.borrow_mut();
                    for filter in ParserSettings::path_filters(modifier).iter() {
                        spec.paths = spec.paths.filter(filter.clone()).into();
                    }
                }

                let builder = Builder::new(parser);
                builder.set_build_with_modifier_filter(true);

                Ok(builder)
            }
        }
    }
}

impl Generate<ResourceIdentity> for Generator {
    fn generate(resource_identity: ResourceIdentity) -> Result<(), ParseError> {
        let client_resource = ClientResource::try_from(resource_identity)?;
        let gen = Generator {
            builder: Generator::parse(API_V1_METADATA_URL.clone(), client_resource)?,
        };
        gen.builder.build_clients();
        Ok(())
    }

    fn dry_run(resource_identity: ResourceIdentity) -> Result<(), ParseError> {
        let client_resource = ClientResource::try_from(resource_identity)?;
        let gen = Generator {
            builder: Generator::parse(API_V1_METADATA_URL.clone(), client_resource)?,
        };
        // This tells the builder to stop before creating the files and writing to them.
        gen.builder.set_dry_run(true);
        gen.builder.build_clients();
        Ok(())
    }
}

impl Generate<Vec<ResourceIdentity>> for Generator {
    fn generate(vec: Vec<ResourceIdentity>) -> Result<(), ParseError> {
        vec.par_iter().for_each(|resource_identity| {
            Generator::generate(*resource_identity).unwrap();
        });
        Ok(())
    }

    fn dry_run(vec: Vec<ResourceIdentity>) -> Result<(), ParseError> {
        vec.par_iter().for_each(|resource_identity| {
            Generator::dry_run(*resource_identity).unwrap();
        });
        Ok(())
    }
}
