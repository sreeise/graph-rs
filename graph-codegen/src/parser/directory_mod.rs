use from_as::*;
use graph_core::resource::ResourceIdentity;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

#[derive(
    Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, FromFile, AsFile,
)]
pub struct DirectoryModFile {
    pub(crate) resource_identity: ResourceIdentity,
    pub(crate) mod_name: String,
    pub(crate) use_all: bool,
}
