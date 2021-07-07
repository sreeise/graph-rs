use crate::openapi::ServerVariable;
use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Server Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#serverObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Server {
    /// REQUIRED. A URL to the target host. This URL supports Server Variables
    /// and MAY be relative, to indicate that the host location is relative
    /// to the location where the OpenAPI document is being served. Variable
    /// substitutions will be made when a variable is named in {brackets}.
    pub url: String,

    /// An optional string describing the host designated by the URL. CommonMark
    /// syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A map between a variable name and its value. The value is used for
    /// substitution in the server's URL template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, ServerVariable>>,
}
