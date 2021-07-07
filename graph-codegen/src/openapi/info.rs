use crate::openapi::{Contact, License};
use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [Info Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#infoObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// REQUIRED. The title of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// A short summary of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A description of the API. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A URL to the Terms of Service for the API. This MUST be in the form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,

    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// REQUIRED. The version of the OpenAPI document (which is distinct from the OpenAPI
    /// Specification version or the API implementation version).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
