use crate::openapi::MediaType;
use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Request Body Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#request-body-object)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestBody {
    /// A brief description of the request body. This could contain examples of
    /// use. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,

    /// REQUIRED. The content of the request body. The key is a media type or
    /// media type range and the value describes it. For requests that match
    /// multiple keys, only the most specific key is applicable. e.g.
    /// text/plain overrides text/*
    pub content: HashMap<String, MediaType>,

    /// Determines if the request body is required in the request. Defaults to
    /// false.
    #[serde(default)]
    pub required: bool,
}
