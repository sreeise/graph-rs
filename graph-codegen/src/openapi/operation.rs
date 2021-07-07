use crate::api_types::RequestMetadata;
use crate::parser::HttpMethod;
use crate::{
    openapi::{
        EitherT, ExternalDocumentation, Parameter, Reference, RequestBody, Responses,
        SecurityRequirement, Server,
    },
    traits::RequestParser,
};
use from_as::*;
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

/// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#operation-object)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for
    /// logical grouping of operations by resources or any other qualifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<VecDeque<String>>,

    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A verbose explanation of the operation behavior. CommonMark syntax MAY
    /// be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    /// Unique string used to identify the operation. The id MUST be unique
    /// among all operations described in the API. The operationId value is
    /// case-sensitive. Tools and libraries MAY use the operationId to
    /// uniquely identify an operation, therefore, it is RECOMMENDED to
    /// follow common programming naming conventions.
    #[serde(rename = "operationId")]
    pub operation_id: String,

    /// A list of parameters that are applicable for this operation. If a
    /// parameter is already defined at the Path Item, the new definition
    /// will override it but can never remove it. The list MUST NOT include
    /// duplicated parameters. A unique parameter is defined by a
    /// combination of a name and location. The list can use the Reference
    /// Object to link to parameters that are defined at the OpenAPI Object's
    /// components/parameters.
    #[serde(default)]
    #[serde(skip_serializing_if = "VecDeque::is_empty")]
    pub parameters: VecDeque<EitherT<Parameter, Reference>>,

    /// The request body applicable for this operation. The requestBody is fully
    /// supported in HTTP methods where the HTTP 1.1 specification RFC7231
    /// has explicitly defined semantics for request bodies. In other cases
    /// where the HTTP spec is vague (such as GET, HEAD and
    /// DELETE), requestBody is permitted but does not have well-defined
    /// semantics and SHOULD be avoided if possible.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<EitherT<RequestBody, Reference>>,

    /// The list of possible responses as they are returned from executing this
    /// operation.
    pub responses: Responses,

    /// A map of possible out-of band callbacks related to the parent operation.
    /// The key is a unique identifier for the Callback Object. Each value
    /// in the map is a Callback Object that describes a request that may be
    /// initiated by the API provider and the expected responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<HashMap<String, serde_json::Value>>,

    // pub callbacks: Option<HashMap<String, Either<Either<PathItem, Reference>, Reference>>>,
    /// Declares this operation to be deprecated. Consumers SHOULD refrain from
    /// usage of the declared operation. Default value is false.
    #[serde(default)]
    pub deprecated: bool,

    /// Each name MUST correspond to a security scheme which is declared in the
    /// Security Schemes under the Components Object. If the security scheme
    /// is of type "oauth2" or "openIdConnect", then the value is a list of
    /// scope names required for the execution, and the list MAY be empty if
    /// authorization does not require a specified scope. For other security
    /// scheme types, the array MAY contain a list of role names which are
    /// required for the execution, but are not otherwise defined or
    /// exchanged in-band.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityRequirement>,

    /// An alternative server array to service this operation. If an alternative
    /// server object is specified at the Path Item Object or Root level, it
    /// will be overridden by this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<VecDeque<Server>>,
}

impl Operation {
    pub fn fn_name(&self) -> String {
        self.operation_id.method_name()
    }

    pub fn parameters(&self) -> VecDeque<Parameter> {
        self.parameters
            .iter()
            .filter(|either| either.is_left())
            .map(|either| either.either_as_ref().left())
            .flatten()
            .cloned()
            .collect()
    }

    pub fn path_parameter_size(&self) -> usize {
        self.parameters()
            .iter()
            .filter(|parameter| parameter.is_path())
            .count()
    }

    pub fn path_parameters(&self) -> VecDeque<String> {
        self.parameters()
            .iter()
            .filter(|p| p.is_path())
            .map(|p| p.name.clone())
            .flatten()
            .collect()
    }

    pub fn has_body(&self) -> bool {
        self.request_body.is_some()
    }

    pub fn request_metadata(&self, http_method: HttpMethod) -> RequestMetadata {
        RequestMetadata {
            has_body: self.has_body(),
            request_task: self.responses.response_body(self.operation_id.as_str()),
            operation_id: self.operation_id.to_string(),
            operation_mapping: self.operation_id.operation_mapping(),
            http_method,
            doc: self.summary.clone(),
        }
    }
}
