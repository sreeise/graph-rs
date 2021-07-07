use crate::openapi::{EitherT, Encoding, Example, Reference, Schema};
use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Media Type Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#media-type-object)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct MediaType {
    // The schema defining the content of the request, response, or parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<EitherT<Schema, Reference>>,

    /// Example of the media type. The example object SHOULD be in the correct
    /// format as specified by the media type. The example field is mutually
    /// exclusive of the examples field. Furthermore, if referencing a
    /// schema which contains an example, the example value SHALL override
    /// the example provided by the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// Examples of the media type. Each example object SHOULD match the media
    /// type and specified schema if present. The examples field is mutually
    /// exclusive of the example field. Furthermore, if referencing a schema
    /// which contains an example, the examples value SHALL override the
    /// example provided by the schema.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, EitherT<Example, Reference>>,

    /// A map between a property name and its encoding information. The key,
    /// being the property name, MUST exist in the schema as a property. The
    /// encoding object SHALL only apply to requestBody objects when the
    /// media type is multipart or application/x-www-form-urlencoded.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub encoding: HashMap<String, Encoding>,
}

impl MediaType {
    pub fn is_upload_session(&self) -> bool {
        if let Some(either_t) = self.schema.as_ref() {
            if let Some(schema) = either_t.clone().into_left() {
                return schema.is_upload_session();
            }
        }

        false
    }
}
