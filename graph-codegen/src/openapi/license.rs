use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [License Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#licenseObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct License {
    /// REQUIRED. The license name used for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// An SPDX license expression for the API. The identifier field is mutually exclusive of
    /// the url field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    /// A URL to the license used for the API. This MUST be in the form of a URL. The url field
    /// is mutually exclusive of the identifier field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
