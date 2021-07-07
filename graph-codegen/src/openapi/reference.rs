use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#referenceObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Reference {
    /// REQUIRED. The reference identifier. This MUST be in the form of a URI.
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    /// A short summary which by default SHOULD override that of the referenced
    /// component. If the referenced object-type does not allow a summary
    /// field, then this field has no effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A description which by default SHOULD override that of the referenced
    /// component. CommonMark syntax MAY be used for rich text
    /// representation. If the referenced object-type does not allow a
    /// description field, then this field has no effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Reference {
    pub fn is_upload_session(&self) -> bool {
        self.ref_.eq(&Some(
            "#/components/schemas/microsoft.graph.uploadSession".into(),
        ))
    }
}
