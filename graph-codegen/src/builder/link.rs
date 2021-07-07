use from_as::*;
use graph_core::resource::ResourceIdentity;
use inflector::Inflector;
use std::{
    convert::TryFrom,
    io::{Read, Write},
    str::FromStr,
};

#[derive(
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    FromFile,
    AsFile,
)]
pub struct ClientLinkSettings {
    name: String,
    method_name: Option<String>,
    custom_calls: Option<String>,
    has_id_param: bool,
    has_extend_path_id: bool,
    has_extend_path_ident: bool,
    has_set_resource_identity: bool,
    has_new_method_empty_id: bool,
    is_id_method_link: bool,
}

impl ClientLinkSettings {
    pub fn new(name: &str) -> ClientLinkSettings {
        ClientLinkSettings {
            name: name.to_string(),
            method_name: None,
            custom_calls: None,
            has_id_param: false,
            has_extend_path_id: false,
            has_extend_path_ident: false,
            has_set_resource_identity: false,
            has_new_method_empty_id: false,
            is_id_method_link: false,
        }
    }

    pub fn with_id_param(&mut self) -> &mut ClientLinkSettings {
        self.has_id_param = true;
        self
    }

    pub fn with_extend_path_id(&mut self) -> &mut ClientLinkSettings {
        self.has_extend_path_id = true;
        self
    }

    pub fn with_extend_path_ident(&mut self) -> &mut ClientLinkSettings {
        self.has_extend_path_ident = true;
        self
    }

    pub fn with_set_resource_identity(&mut self) -> &mut ClientLinkSettings {
        self.has_set_resource_identity = true;
        self
    }

    pub fn with_new_method_empty_id(&mut self) -> &mut ClientLinkSettings {
        self.has_new_method_empty_id = true;
        self
    }

    pub fn as_id_method_link(&mut self) -> &mut ClientLinkSettings {
        self.is_id_method_link = true;
        self
    }

    pub fn use_method_name(&mut self, name: &str) -> &mut ClientLinkSettings {
        self.method_name = Some(name.to_string());
        self
    }

    pub fn use_custom(&mut self, value: &str) -> &mut ClientLinkSettings {
        self.custom_calls = Some(value.to_string());
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    fn base_struct_name(&self) -> String {
        format!("{}Request", self.name.to_pascal_case())
    }

    fn method_link(&self) -> String {
        if let Some(name) = self.method_name.as_ref() {
            name.to_snake_case()
        } else {
            self.name.to_snake_case()
        }
    }

    fn method_start_string(&self) -> String {
        let pascal_casing = self.base_struct_name();
        let snake_casing = self.method_link();

        if self.is_id_method_link {
            format!(
                "\n\tpub fn id<ID: AsRef<str>>(&self, id: ID) -> {}<'a, Client> {{",
                pascal_casing
            )
        } else if self.has_id_param {
            format!(
                "\n\tpub fn {}<ID: AsRef<str>>(&self, id: ID) -> {}<'a, Client> {{",
                snake_casing, pascal_casing
            )
        } else {
            format!(
                "\n\tpub fn {}(&self) -> {}<'a, Client> {{",
                snake_casing, pascal_casing
            )
        }
    }

    fn extend_path_str(&self) -> &'static str {
        if self.has_extend_path_id && self.has_extend_path_ident {
            "self.client.request.extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);"
        } else if self.has_extend_path_ident {
            "self.client.request.extend_path(&[self.client.ident().as_ref()]);"
        } else if self.has_extend_path_id {
            "self.client.request.extend_path(&[self.id.as_str()]);"
        } else {
            ""
        }
    }

    fn resource_identity_str(&self) -> String {
        let camel_casing = self.name.to_camel_case();
        if self.has_set_resource_identity {
            if let Ok(resource_identity) = ResourceIdentity::from_str(camel_casing.as_str()) {
                return format!(
                    "self.client.set_ident({});",
                    resource_identity.enum_string()
                );
            }
        }

        Default::default()
    }

    fn client_new_string(&self) -> String {
        let pascal_casing = self.base_struct_name();

        if self.has_new_method_empty_id {
            return format!("{}::new(\"\", self.client)\n}}", pascal_casing);
        }

        if self.has_id_param || self.is_id_method_link {
            if self.is_id_method_link {
                let camel_casing = self.name.to_camel_case();
                if let Ok(resource_identity) = ResourceIdentity::from_str(camel_casing.as_str()) {
                    return format!(
                        "self.client.set_ident({});
                        {}::new(id.as_ref(), self.client)\n}}",
                        resource_identity.enum_string(),
                        pascal_casing
                    );
                }
            }

            format!("{}::new(id.as_ref(), self.client)\n}}", pascal_casing)
        } else {
            format!("{}::new(self.client)\n}}", pascal_casing)
        }
    }

    pub fn format(&self) -> String {
        let mut s = self.method_start_string();

        if let Some(custom) = self.custom_calls.as_ref() {
            s.push_str(custom.as_str());
        }

        s.push_str(self.extend_path_str());
        s.push_str(self.resource_identity_str().as_str());
        s.push_str(self.client_new_string().as_str());
        s
    }
}
