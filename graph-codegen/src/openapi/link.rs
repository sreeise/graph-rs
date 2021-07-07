use crate::openapi::Server;
use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [Link Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#linkObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// A relative or absolute URI reference to an OAS operation. This field is
    /// mutually exclusive of the operationId field, and MUST point to an
    /// Operation Object. Relative operationRef values MAY be used to locate
    /// an existing Operation Object in the OpenAPI definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,

    /// The name of an existing, resolvable OAS operation, as defined with a
    /// unique operationId. This field is mutually exclusive of the operationRef
    /// field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,

    /// A map representing parameters to pass to an operation as specified with
    /// operationId or identified via operationRef. The key is the parameter
    /// name to be used, whereas the value can be a constant or an expression to
    /// be evaluated and passed to the linked operation. The parameter name can
    /// be qualified using the parameter location [{in}.]{name} for operations
    /// that use the same parameter name in different locations (e.g. path.id).
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, serde_json::Value>,

    /// A literal value or {expression} to use as a request body when calling
    /// the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<serde_json::Value>,

    /// A description of the link. CommonMark syntax MAY be used for rich text
    /// representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A server object to be used by the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}
