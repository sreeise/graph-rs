use crate::api_types::RequestTask;
use crate::openapi::{EitherT, Header, Link, MediaType, Reference};
use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Response Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#responseObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Response {
    /// REQUIRED. A short description of the response. CommonMark syntax MAY be
    /// used for rich text representation.
    pub description: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, EitherT<Header, Reference>>,

    /// A map containing descriptions of potential response payloads. The key is
    /// a media type or media type range and the value describes it. For
    /// responses that match multiple keys, only the most specific key is
    /// applicable. e.g. text/plain overrides text/*
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<String, EitherT<Link, Reference>>,
}

impl Response {
    pub fn response_body(&self) -> RequestTask {
        println!("{:#?}", self.content);
        if self.content.contains_key("application/json") {
            return RequestTask::Json;
        }

        Default::default()
    }

    pub fn is_upload_session(&self) -> bool {
        if let Some(media_type) = self.content.get("application/json") {
            return media_type.is_upload_session();
        }

        false
    }
}
