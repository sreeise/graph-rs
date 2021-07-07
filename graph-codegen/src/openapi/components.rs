use crate::openapi::{
    EitherT, Example, Header, Link, Parameter, PathItem, Reference, RequestBody, Response, Schema,
    SecurityScheme,
};
use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Components Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#componentsObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    /// An object to hold reusable Schema Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub schemas: HashMap<String, Schema>,

    /// An object to hold reusable Response Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub responses: HashMap<String, EitherT<Response, Reference>>,

    /// An object to hold reusable Parameter Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, EitherT<Parameter, Reference>>,

    /// An object to hold reusable Example Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, EitherT<Example, Reference>>,

    /// An object to hold reusable Request Body Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub request_bodies: HashMap<String, EitherT<RequestBody, Reference>>,

    /// An object to hold reusable Header Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, EitherT<Header, Reference>>,

    /// An object to hold reusable Security Scheme Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub security_schemes: HashMap<String, EitherT<SecurityScheme, Reference>>,

    /// An object to hold reusable Link Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<String, EitherT<Link, Reference>>,

    /// An object to hold reusable Callback Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub callbacks: HashMap<String, EitherT<serde_json::Value, Reference>>,

    /// An object to hold reusable Path Item Objects.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub path_items: HashMap<String, EitherT<PathItem, Reference>>,
}
