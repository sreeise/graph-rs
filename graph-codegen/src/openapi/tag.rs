use crate::openapi::ExternalDocumentation;
use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [Tag Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#tagObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// REQUIRED. The name of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A description for the tag. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional external documentation for this tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}
