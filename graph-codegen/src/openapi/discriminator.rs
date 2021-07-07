use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Discriminator Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#discriminatorObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    // REQUIRED. The name of the property in the payload that will hold the discriminator value.
    pub property_name: String,

    // An object to hold mappings between payload values and schema names or references.
    #[serde(default)]
    pub mapping: HashMap<String, String>,
}
