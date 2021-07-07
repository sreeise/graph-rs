use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [Contact Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#contactObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The URL pointing to the contact information. This MUST be in the form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The email address of the contact person/organization. This MUST be in the form of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
