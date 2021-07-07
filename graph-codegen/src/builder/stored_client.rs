use crate::builder::ClientBuilder;
use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::{
    collections::BTreeSet,
    convert::TryFrom,
    io::{Read, Write},
};

#[derive(
    Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, FromFile, AsFile,
)]
pub struct StoredClient {
    pub(crate) resource_identity: ResourceIdentity,
    pub(crate) client_builder: ClientBuilder,
    pub(crate) directory: String,
    pub(crate) mod_file: String,
    pub(crate) request_file: String,
}

impl StoredClient {
    pub fn new(
        resource_identity: ResourceIdentity,
        client_builder: ClientBuilder,
        directory: String,
        mod_file: String,
        request_file: String,
    ) -> StoredClient {
        StoredClient {
            resource_identity,
            client_builder,
            directory,
            mod_file,
            request_file,
        }
    }

    pub fn write_json(&self) -> Result<(), FromAsError> {
        self.as_file_pretty(&format!(
            "./graph-codegen/clients/{}.json",
            self.resource_identity.to_string()
        ))
    }
}

#[derive(
    Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, FromFile, AsFile,
)]
pub struct StoredClientSet {
    pub(crate) stored_client_set: BTreeSet<StoredClient>,
}

impl StoredClientSet {
    pub fn new() -> StoredClientSet {
        StoredClientSet {
            stored_client_set: BTreeSet::new(),
        }
    }

    pub fn write_json(&self) -> Vec<Result<(), FromAsError>> {
        let mut errors = Vec::new();
        for client in self.stored_client_set.iter() {
            let result = client.write_json();
            println!(
                "Writing: {:#?}\nErrors: {:#?}",
                client.resource_identity.enum_string(),
                result
            );
            errors.push(result);
        }
        errors
    }
}

impl From<BTreeSet<StoredClient>> for StoredClientSet {
    fn from(stored_client_set: BTreeSet<StoredClient>) -> Self {
        StoredClientSet { stored_client_set }
    }
}
