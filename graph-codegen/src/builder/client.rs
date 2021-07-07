use crate::{
    builder::{ClientLinkSettings, RegisterClient},
    parser::{DirectoryModFile, ParserSettings, Request, RequestMap, RequestType},
};
use bytes::{BufMut, BytesMut};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use inflector::Inflector;
use std::{
    collections::{BTreeMap, BTreeSet},
    convert::TryFrom,
    io::{Read, Write},
    str::FromStr,
};

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Client {
    name: String,
    client_links: BTreeSet<ClientLinkSettings>,
    methods: BTreeSet<RequestMap>,
    is_ident_client: bool,
}

impl Client {
    pub fn new(name: &str, methods: BTreeSet<RequestMap>) -> Client {
        Client {
            name: name.to_string(),
            client_links: Default::default(),
            methods,
            is_ident_client: false,
        }
    }

    pub fn insert_client_link(&mut self, client_link: ClientLinkSettings) {
        self.client_links.insert(client_link);
    }

    pub fn extend_client_links(&mut self, client_links: BTreeSet<ClientLinkSettings>) {
        self.client_links.extend(client_links);
    }

    pub fn set_ident_client(&mut self, value: bool) {
        self.is_ident_client = value;
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn client_link_settings(&self) -> &BTreeSet<ClientLinkSettings> {
        &self.client_links
    }

    pub fn methods(&self) -> &BTreeSet<RequestMap> {
        &self.methods
    }

    pub fn is_ident_client(&self) -> bool {
        self.is_ident_client
    }

    pub fn get_client_link_setting(&self, name: &str) -> Option<&ClientLinkSettings> {
        self.client_links
            .iter()
            .find(|link_settings| link_settings.name().eq(name))
    }

    pub fn extend_methods(&mut self, methods: BTreeSet<RequestMap>) {
        self.methods.extend(methods);
    }
}

#[derive(
    Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, FromFile, AsFile,
)]
pub struct ClientBuilder {
    imports: BTreeSet<String>,
    directory_mods: BTreeSet<DirectoryModFile>,
    clients: BTreeMap<String, Client>,
    #[serde(skip)]
    buf: BytesMut,
}

impl ClientBuilder {
    pub fn new(
        imports: BTreeSet<String>,
        clients: BTreeMap<String, Client>,
        directory_mods: BTreeSet<DirectoryModFile>,
    ) -> ClientBuilder {
        ClientBuilder {
            imports,
            directory_mods,
            clients,
            buf: BytesMut::with_capacity(1024),
        }
    }

    fn imports(&mut self) {
        let imports_vec: Vec<u8> = self
            .imports
            .iter()
            .map(|s| format!("use {};\n", s).into_bytes())
            .flatten()
            .collect();
        self.buf.extend(imports_vec);
        self.buf.put_u8(b'\n');
    }

    fn client_registrations(&mut self) {
        for (name, client) in self.clients.iter() {
            if let Ok(resource_identity) = ResourceIdentity::from_str(name.as_str()) {
                if let Some(custom_register_client) =
                    ParserSettings::custom_register_clients(resource_identity)
                {
                    self.buf.put(custom_register_client.as_bytes());
                    return;
                }
            }

            if !client.methods.is_empty() {
                if client.is_ident_client {
                    self.buf
                        .put(RegisterClient::IdentClient.format(client.name.as_str()));
                } else {
                    self.buf
                        .put(RegisterClient::BaseClient.format(client.name.as_str()));
                }
            }
        }
    }

    fn impl_start(&self, name: &str) -> String {
        format!(
            "\nimpl<'a, Client> {}<'a, Client> where Client: graph_http::RequestClient {{",
            name
        )
    }

    fn download_impl_start(&self, name: &str, is_async: bool) -> String {
        if is_async {
            format!("\nimpl<'a> {}<'a, AsyncHttpClient> {{", name)
        } else {
            format!("\nimpl<'a> {}<'a, BlockingHttpClient> {{", name)
        }
    }

    fn request(&self, request: &Request) -> String {
        if let Some(doc_comment) = request.doc.as_ref() {
            match request.request_type {
                RequestType::Normal => {
                    format!(
                        "\n\t{}!({{\n\t\tdoc: \"{}\",\n\t\tname: {},\n\t\tresponse: \
                         {},\n\t\tpath: \"{}\",\n\t\tparams: {},\n\t\thas_body: {}\n\t}});",
                        request.method.as_ref(),
                        doc_comment,
                        request.method_name.as_str(),
                        request.response.as_str(),
                        request.path.as_str(),
                        request.param_size,
                        request.has_body
                    )
                }
                RequestType::Upload => {
                    format!(
                        "\n\t{}!({{\n\t\tdoc: \"{}\",\n\t\tname: {},\n\t\tresponse: \
                         {},\n\t\tpath: \"{}\",\n\t\tparams: {},\n\t\tupload: true\n\t}});",
                        request.method.as_ref(),
                        doc_comment,
                        request.method_name.as_str(),
                        request.response.as_str(),
                        request.path.as_str(),
                        request.param_size,
                    )
                }
                RequestType::UploadSession => {
                    format!(
                        "\n\t{}!({{\n\t\tdoc: \"{}\",\n\t\tname: {},\n\t\tpath: \
                         \"{}\",\n\t\tparams: {},\n\t\thas_body: {},\n\t\tupload_session: \
                         true\n\t}});",
                        request.method.as_ref(),
                        doc_comment,
                        request.method_name.as_str(),
                        request.path.as_str(),
                        request.param_size,
                        request.has_body
                    )
                }
                RequestType::Download => {
                    format!(
                        "\n\tdownload!({{\n\t\tdoc: \"{}\",\n\t\tname: {},\n\t\tresponse: \
                         BlockingDownload,\n\t\tpath: \"{}\",\n\t\tparams: {}\n\t}});",
                        doc_comment,
                        request.method_name.as_str(),
                        request.path.as_str(),
                        request.param_size
                    )
                }
                RequestType::AsyncDownload => {
                    format!(
                        "\n\tasync_download!({{\n\t\tdoc: \"{}\",\n\t\tname: {},\n\t\tresponse: \
                         AsyncDownload,\n\t\tpath: \"{}\",\n\t\tparams: {}\n\t}});",
                        doc_comment,
                        request.method_name.as_str(),
                        request.path.as_str(),
                        request.param_size
                    )
                }
            }
        } else {
            match request.request_type {
                RequestType::Normal => {
                    format!(
                        "\n\t{}!({{\n\t\tname: {},\n\t\tresponse: {},\n\t\tpath: \
                         \"{}\",\n\t\tparams: {},\n\t\thas_body: {}\n\t}});",
                        request.method.as_ref(),
                        request.method_name.as_str(),
                        request.response.as_str(),
                        request.path.as_str(),
                        request.param_size,
                        request.has_body
                    )
                }
                RequestType::Upload => {
                    format!(
                        "\n\t{}!({{\n\t\tname: {},\n\t\tresponse: {},\n\t\tpath: \
                         \"{}\",\n\t\tparams: {},\n\t\tupload: true\n\t}});",
                        request.method.as_ref(),
                        request.method_name.as_str(),
                        request.response.as_str(),
                        request.path.as_str(),
                        request.param_size,
                    )
                }
                RequestType::UploadSession => {
                    format!(
                        "\n\t{}!({{\n\t\tname: {},\n\t\tpath: \"{}\",\n\t\tparams: \
                         {},\n\t\thas_body: {},\n\t\tupload_session: true\n\t}});",
                        request.method.as_ref(),
                        request.method_name.as_str(),
                        request.path.as_str(),
                        request.param_size,
                        request.has_body
                    )
                }
                RequestType::Download => {
                    format!(
                        "\n\tdownload!({{\n\t\tname: {},\n\t\tresponse: \
                         BlockingDownload,\n\t\tpath: \"{}\",\n\t\tparams: {}\n\t}});",
                        request.method_name.as_str(),
                        request.path.as_str(),
                        request.param_size
                    )
                }
                RequestType::AsyncDownload => {
                    format!(
                        "\n\tasync_download!({{\n\t\tname: {},\n\t\tresponse: \
                         AsyncDownload,\n\t\tpath: \"{}\",\n\t\tparams: {}\n\t}});",
                        request.method_name.as_str(),
                        request.path.as_str(),
                        request.param_size
                    )
                }
            }
        }
    }

    fn client_impl(&mut self) {
        for (_name, client) in self.clients.iter() {
            let name_pascal_casing = format!("{}Request", client.name.to_pascal_case());

            let impl_start = self.impl_start(&name_pascal_casing);
            self.buf.put(impl_start.as_bytes());

            for client_link in client.client_links.iter() {
                let link = client_link.format();
                self.buf.put(link.as_bytes());
            }

            for request_map in client.methods.iter() {
                for request in request_map.iter() {
                    if request.request_type.ne(&RequestType::Download)
                        && request.request_type.ne(&RequestType::AsyncDownload)
                    {
                        let r = self.request(request);
                        self.buf.put(r.as_bytes());
                    }
                }
            }
            self.buf.put_slice(b"\n}\n");
        }
    }

    fn download_client_impl(&mut self) {
        for (_name, client) in self.clients.iter() {
            let name_pascal_casing = format!("{}Request", client.name.to_pascal_case());

            for request_map in client.methods.iter() {
                for request in request_map.iter() {
                    match request.request_type {
                        RequestType::Download => {
                            let impl_start = self.download_impl_start(&name_pascal_casing, false);
                            self.buf.put(impl_start.as_bytes());
                            let r = self.request(request);
                            self.buf.put(r.as_bytes());
                            self.buf.put_slice(b"\n}\n");
                        }
                        RequestType::AsyncDownload => {
                            let impl_start = self.download_impl_start(&name_pascal_casing, true);
                            self.buf.put(impl_start.as_bytes());
                            let r = self.request(request);
                            self.buf.put(r.as_bytes());
                            self.buf.put_slice(b"\n}\n");
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn build_mod_file(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        if self.directory_mods.is_empty() {
            buf.put("mod request;\n\npub use request::*;".as_bytes());
        } else {
            let mut uses_buf = BytesMut::new();
            buf.put("mod request;\n".as_bytes());
            uses_buf.put("pub use request::*;\n".as_bytes());

            for directory_mod in self.directory_mods.iter() {
                buf.put(format!("mod {};\n", directory_mod.mod_name).as_bytes());
                if directory_mod.use_all {
                    buf.put(format!("pub use {}::*;", directory_mod.mod_name).as_bytes());
                }
            }

            buf.put_u8(b'\n');
            buf.put(uses_buf);
        }

        buf
    }

    pub fn build(mut self) -> BytesMut {
        self.buf.put("// GENERATED CODE\n\n".as_bytes());
        self.imports();
        self.client_registrations();
        self.client_impl();
        self.download_client_impl();
        self.buf
    }
}
