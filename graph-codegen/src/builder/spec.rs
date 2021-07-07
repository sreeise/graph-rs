use crate::{
    builder::{Client, ClientBuilder, ClientLinkSettings, StoredClient, StoredClientSet},
    parser::{
        filter::Filter, Modifier, Parser, ParserSettings, PathMap, RequestMap, RequestSet,
        ResourceNames, ResourceRequestMap,
    },
};
use graph_core::resource::ResourceIdentity;
use graph_http::iotools::create_dir;
use inflector::Inflector;
use std::{
    cell::{Ref, RefCell},
    collections::{BTreeMap, BTreeSet, HashMap},
    fs::OpenOptions,
    io::Write,
    str::FromStr,
};

#[derive(Default, Debug, Clone)]
pub struct SpecBuilder {
    pub(crate) parser: Parser,
    secondary_links: BTreeMap<String, Vec<String>>,
    build_with_modifier_filter: bool,
    dry_run: bool,
}

impl SpecBuilder {
    fn set_dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Builder {
    pub(crate) spec: RefCell<SpecBuilder>,
}

impl Builder {
    pub fn new(parser: Parser) -> Builder {
        Builder {
            spec: RefCell::new(SpecBuilder {
                parser,
                secondary_links: Default::default(),
                build_with_modifier_filter: false,
                dry_run: false,
            }),
        }
    }

    pub(crate) fn set_build_with_modifier_filter(&self, build_with_modifier_filter: bool) {
        self.spec.borrow_mut().build_with_modifier_filter = build_with_modifier_filter;
    }

    pub fn filter(&self, filter: Filter) -> PathMap {
        self.spec.borrow().parser.filter(filter)
    }

    pub fn generate_requests(&self) -> Vec<ResourceRequestMap> {
        let spec = self.spec.borrow();
        Builder::parser_build(&spec)
    }

    pub fn set_dry_run(&self, dry_run: bool) {
        self.spec.borrow_mut().set_dry_run(dry_run);
    }

    fn add_custom_clients(
        &self,
        name: &str,
        custom_methods: &HashMap<String, RequestSet>,
        client_map: &mut BTreeMap<String, Client>,
    ) {
        for (name, client) in client_map.iter_mut() {
            if let Some(request_set) = custom_methods.get(name) {
                let methods = request_set.methods();

                for (_name, methods) in methods.iter() {
                    let client_methods: BTreeSet<RequestMap> = methods.iter().cloned().collect();
                    client.extend_methods(client_methods);
                }
            }
        }

        if let Some(request_set) = custom_methods.get(name) {
            let request_set = request_set.clone();
            let methods = request_set.methods();

            for (name, methods) in methods.iter() {
                let client_methods: BTreeSet<RequestMap> = methods.iter().cloned().collect();

                client_map
                    .entry(name.to_string())
                    .and_modify(|client| client.extend_methods(client_methods.clone()))
                    .or_insert_with(|| Client::new(name.as_str(), client_methods.clone()));
            }
        }
    }

    // Temporary workaround to deal with the differences in the path
    // for drives when the resource comes from me, users, groups,
    // and sites.
    // For instance, the get_items methods for the drives resource
    // has a path that starts with /drives/drive-id/items
    // but the path for users is /users/user-id/drive/items
    // The users path has drive/items while the drive path is just items.
    fn fix_drive_methods(&self, methods: &mut Vec<RequestMap>) {
        let mat = "/drives/{{RID}}";
        let mat2 = "/drives/{{id}}";

        let empty_root = "{{drive_root}}";
        for request_map in methods {
            if request_map.path.starts_with(mat) || request_map.path.starts_with(mat2) {
                let mut reduce_param_count = false;
                let mat = {
                    if request_map.path.starts_with(mat) {
                        mat
                    } else {
                        reduce_param_count = true;
                        mat2
                    }
                };

                request_map.path = request_map.path.trim_start_matches(mat).to_string();

                if !request_map.path.ends_with("/drives") {
                    if request_map.path.starts_with('/') {
                        request_map.path =
                            format!("{{{{resource_drive_path}}}}{}", request_map.path);
                    } else {
                        request_map.path =
                            format!("{{{{resource_drive_path}}}}/{}", request_map.path);
                    }
                }

                request_map.path = request_map
                    .path
                    .replace("{{id2}}", "{{id}}")
                    .replace("{{id3}}", "{{id2}}")
                    .replace("{{id4}}", "{{id3}}");

                if request_map.path.is_empty() {
                    request_map.path = empty_root.into();
                }

                // Doing the exact same thing with the path in request map but we can't
                // just clone the path to the requests because we have a mutable
                // borrow of the Vec<RequestMap> and we need an immutable borrow
                // to clone.
                for request in request_map.iter_mut() {
                    request.has_rid = false;
                    request.path = request.path.trim_start_matches(mat).to_string();

                    if !request.path.ends_with("/drives") {
                        if request.path.starts_with('/') {
                            request.path = format!("{{{{resource_drive_path}}}}{}", request.path);
                        } else {
                            request.path = format!("{{{{resource_drive_path}}}}/{}", request.path);
                        }
                    }

                    request.path = request
                        .path
                        .replace("{{id2}}", "{{id}}")
                        .replace("{{id3}}", "{{id2}}")
                        .replace("{{id4}}", "{{id3}}");

                    if request.path.is_empty() {
                        request.path = empty_root.into();
                    }

                    if reduce_param_count && request.param_size > 0 {
                        request.param_size -= 1;
                    }
                }
            }
        }
    }

    pub fn build_client_map(&self, resource_map: &ResourceRequestMap) -> BTreeMap<String, Client> {
        let methods = resource_map.request_set.methods();
        let struct_links = resource_map.request_set.client_links();

        let mut clients: BTreeMap<String, Client> = BTreeMap::new();

        for (name, methods) in methods.iter() {
            let client_methods: BTreeSet<RequestMap> = methods.iter().cloned().collect();
            let mut client = Client::new(name.as_str(), client_methods);

            if let Some(client_link_settings) = resource_map.modifier.client_links.get(name) {
                client.extend_client_links(client_link_settings.clone());
            }

            println!("Current Name: {:#?}", name);
            let resource_identity = ResourceIdentity::from_str(name.as_str()).unwrap();
            if ParserSettings::is_registered_ident_client(resource_identity) {
                client.set_ident_client(true);
            }

            clients.insert(name.to_string(), client);
        }

        for (name, links) in struct_links.iter() {
            clients.entry(name.to_string()).and_modify(|client| {
                for link in links.iter() {
                    if client.get_client_link_setting(link).is_none() && name.ne(link.as_str()) {
                        let link_settings = ClientLinkSettings::new(link.as_str());
                        client.insert_client_link(link_settings);
                    }
                }
            });
        }

        clients
    }

    pub fn build_clients(&self) {
        let spec = self.spec.borrow();
        let resources = Builder::parser_build(&spec);
        let dry_run = spec.dry_run;

        for resource_map in resources.iter() {
            let imports = resource_map.get_imports();
            let mut methods = resource_map.request_set.methods();
            let mut directory_mods = BTreeSet::new();

            if let Some(directory_mod) = resource_map.modifier.directory_mod.as_ref() {
                directory_mods.insert(directory_mod.clone());
            }

            // Temporary workaround to deal with the differences in the path
            // for drives when the resource comes from me, users, groups,
            // and sites.
            for (_operation_id, request_map) in methods.iter_mut() {
                self.fix_drive_methods(request_map);
            }

            let mut clients = self.build_client_map(resource_map);

            if let Some(custom_methods) = resource_map.modifier.custom_methods.as_ref() {
                self.add_custom_clients(
                    resource_map.modifier.name.as_str(),
                    custom_methods,
                    &mut clients,
                );
            }

            let mut snake_casing = resource_map.modifier.name.to_snake_case();

            // The path /contacts is not related to me/contacts or users/user-id/contacts
            // but is really org contacts. The directory contacts is for me and users.
            if resource_map.modifier.name.eq("contacts") && clients.contains_key("orgContact") {
                snake_casing = "org_contact".to_string();
            }

            let dir = format!("./src/{}", snake_casing);
            let mod_file = format!("./src/{}/mod.rs", snake_casing);
            let file = format!("./src/{}/request.rs", snake_casing);
            let client_builder = ClientBuilder::new(imports, clients, directory_mods);

            println!("Client Builder: {:#?}", client_builder);
            println!("Building Client: {:#?}", snake_casing);
            println!("Directory: {:#?}", dir);
            println!("Mod file: {:#?}", mod_file);
            println!("Request file: {:#?}", file);

            if !dry_run {
                Builder::write(client_builder, dir, mod_file, file);
            }
        }
    }

    pub fn build_stored_clients(&self) -> StoredClientSet {
        let spec = self.spec.borrow();
        let resources = Builder::parser_build(&spec);
        let mut stored_client_set: BTreeSet<StoredClient> = BTreeSet::new();

        for resource_map in resources.iter() {
            let imports = resource_map.get_imports();
            let mut methods = resource_map.request_set.methods();
            let mut directory_mods = BTreeSet::new();

            if let Some(directory_mod) = resource_map.modifier.directory_mod.as_ref() {
                directory_mods.insert(directory_mod.clone());
            }

            // Temporary workaround to deal with the differences in the path
            // for drives when the resource comes from me, users, groups,
            // and sites.
            for (_operation_id, request_map) in methods.iter_mut() {
                self.fix_drive_methods(request_map);
            }

            let mut clients = self.build_client_map(resource_map);

            if let Some(custom_methods) = resource_map.modifier.custom_methods.as_ref() {
                self.add_custom_clients(
                    resource_map.modifier.name.as_str(),
                    custom_methods,
                    &mut clients,
                );
            }

            let mut snake_casing = resource_map.modifier.name.to_snake_case();

            // The path /contacts is not related to me/contacts or users/user-id/contacts
            // but is really org contacts. The directory contacts is for me and users.
            if resource_map.modifier.name.eq("contacts") && clients.contains_key("orgContact") {
                snake_casing = "org_contact".to_string();
            }

            let directory = format!("./src/{}", snake_casing);
            let mod_file = format!("./src/{}/mod.rs", snake_casing);
            let request_file = format!("./src/{}/request.rs", snake_casing);
            let client_builder = ClientBuilder::new(imports, clients, directory_mods);

            println!("Client Builder: {:#?}", client_builder);
            println!("Building Client: {:#?}", snake_casing);
            println!("Directory: {:#?}", directory);
            println!("Mod file: {:#?}", mod_file);
            println!("Request file: {:#?}", request_file);

            stored_client_set.insert(StoredClient::new(
                resource_map.modifier.resource_identity,
                client_builder,
                directory,
                mod_file,
                request_file,
            ));
        }
        StoredClientSet::from(stored_client_set)
    }

    pub fn write_stored_clients(stored_client_set: StoredClientSet) {
        for stored_client in stored_client_set.stored_client_set {
            println!("Client Builder: {:#?}", stored_client.client_builder);
            println!(
                "Building Client: {:#?}",
                stored_client.resource_identity.to_string()
            );
            println!("Directory: {:#?}", stored_client.directory);
            println!("Mod file: {:#?}", stored_client.mod_file);
            println!("Request file: {:#?}", stored_client.request_file);
            Builder::write(
                stored_client.client_builder,
                stored_client.directory,
                stored_client.mod_file,
                stored_client.request_file,
            );
        }
    }

    fn write(client_builder: ClientBuilder, dir: String, mod_file: String, request_file: String) {
        create_dir(dir).unwrap();

        let mut mod_file_buf = client_builder.build_mod_file();
        let mut file1 = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&mod_file)
            .unwrap();
        file1.write_all(mod_file_buf.as_mut()).unwrap();
        file1.sync_data().unwrap();

        let mut buf = client_builder.build();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&request_file)
            .unwrap();
        file.write_all(buf.as_mut()).unwrap();
        file.sync_data().unwrap();
    }

    fn parser_build(spec: &Ref<SpecBuilder>) -> Vec<ResourceRequestMap> {
        if !spec.build_with_modifier_filter {
            let path_map = spec.parser.path_map();
            let resource_names = ResourceNames::from(path_map);
            let vec = resource_names.to_vec();
            let vec_str: Vec<&str> = vec.iter().map(|s| s.as_str()).collect();
            let modifiers = Modifier::build_modifier_vec(&vec_str);
            spec.parser.set_modifiers(modifiers);
        }
        spec.parser.build()
    }

    pub fn generate_resource_names(&self) -> ResourceNames {
        ResourceNames::from(self.spec.borrow().parser.path_map())
    }
}
