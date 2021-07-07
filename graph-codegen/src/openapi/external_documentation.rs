use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [External Documentation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#externalDocumentationObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ExternalDocumentation {
    /// A description of the target documentation. CommonMark syntax MAY be used
    /// for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// REQUIRED. The URL for the target documentation. This MUST be in the form
    /// of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
