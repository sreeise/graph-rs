use crate::{GraphHeaders, GraphResult};
use async_trait::async_trait;
use reqwest::StatusCode;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::string::ToString;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InnerError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "request-id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

/// An error resource included in the error response returned from
/// Microsoft Graph.
///
/// [odata.error resource type](https://docs.microsoft.com/en-us/graph/errors#odataerror-resource-type)
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorStatus {
    /// An error code string for the error that occurred
    /// [Code Property](https://docs.microsoft.com/en-us/graph/errors#code-property)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// A developer ready message about the error that occurred. This should not be displayed to the user directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional. Additional error objects that may be more specific than the top level error.
    #[serde(rename = "innerError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorStatus>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphError {
    pub headers: Option<GraphHeaders>,

    #[serde(with = "http_serde::status_code")]
    pub code: StatusCode,

    pub error_message: ErrorMessage,
}

impl GraphError {
    pub fn new(
        headers: Option<GraphHeaders>,
        code: StatusCode,
        error_message: ErrorMessage,
    ) -> GraphError {
        GraphError {
            headers,
            code,
            error_message,
        }
    }

    pub fn set_headers(&mut self, headers: GraphHeaders) {
        self.headers = Some(headers);
    }

    pub fn set_error(&mut self, code: StatusCode) -> Result<(), GraphError> {
        self.code = code;
        Ok(())
    }

    pub fn set_error_message(&mut self, error_message: ErrorMessage) {
        self.error_message = error_message;
    }

    pub fn try_set_error_message(&mut self, result: GraphResult<ErrorMessage>) {
        if let Ok(error_message) = result {
            self.set_error_message(error_message);
        }
    }

    pub fn message(&self) -> Option<String> {
        self.error_message.error.as_ref()?.message.clone()
    }

    pub fn code_property(&self) -> Option<String> {
        self.error_message.error.as_ref()?.code.clone()
    }

    pub fn inner_error(&self) -> Option<&InnerError> {
        self.error_message.error.as_ref()?.inner_error.as_ref()
    }

    pub fn request_id(&self) -> Option<String> {
        self.error_message
            .error
            .as_ref()?
            .inner_error
            .as_ref()?
            .request_id
            .clone()
    }

    pub fn date(&self) -> Option<String> {
        self.error_message
            .error
            .as_ref()?
            .inner_error
            .as_ref()?
            .date
            .clone()
    }

    pub fn detailed_error_code(&self) -> Option<String> {
        self.error_message
            .error
            .as_ref()?
            .inner_error
            .as_ref()?
            .code
            .clone()
    }
}

impl Error for GraphError {
    fn source<'a>(&'a self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        if let Some(err) = self.error_message.error.as_ref() {
            if let Some(message) = err.message.as_ref() {
                return message.as_str();
            }
        }
        self.code.canonical_reason().unwrap_or_default()
    }
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nError Code: {:#?}\nError Message: {:#?}",
            &self.code,
            self.code.canonical_reason().unwrap_or_default()
        )
    }
}

impl Default for GraphError {
    fn default() -> Self {
        GraphError::new(None, StatusCode::BAD_REQUEST, Default::default())
    }
}

/// [Microsoft Graph API specific errors and HTTP status codes](https://docs.microsoft.com/en-us/graph/errors)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ErrorType {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    UnsupportedMediaType,
    RequestRangeNotSatisfiable,
    UnprocessableEntity,
    Locked,
    TooManyRequests,
    InternalServerError,
    NotImplemented,
    ServiceUnavailable,
    GatewayTimeout,
    InsufficientStorage,
    BandwidthLimitExceeded,
    UnknownError,
}

impl ErrorType {}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match *self {
            ErrorType::BadRequest => "Cannot process the request because it is malformed or incorrect.",
            ErrorType::Unauthorized => "Required authentication information is either missing or not valid for the resource.",
            ErrorType::Forbidden => "Access is denied to the requested resource. The user might not have enough permission.",
            ErrorType::NotFound => "The requested resource doesnt exist.",
            ErrorType::MethodNotAllowed => "The HTTP method in the request is not allowed on the resource.",
            ErrorType::NotAcceptable => "This service doesnt support the format requested in the Accept header.",
            ErrorType::Conflict => "The current state conflicts with what the request expects. For example, the specified parent folder might not exist",
            ErrorType::Gone => "The requested resource is no longer available at the server.",
            ErrorType::LengthRequired => "A Content-Length header is required on the request.",
            ErrorType::PreconditionFailed=> "A precondition provided in the request (such as an if-match header) does not match the resource's current state.",
            ErrorType::RequestEntityTooLarge => "The request size exceeds the maximum limit.",
            ErrorType::UnsupportedMediaType => "The content type of the request is a format that is not supported by the service.",
            ErrorType::RequestRangeNotSatisfiable => "The specified byte range is invalid or unavailable.",
            ErrorType::UnprocessableEntity => "Cannot process the request because it is semantically incorrect.",
            ErrorType::Locked => "The resource that is being accessed is locked.",
            ErrorType::TooManyRequests => "Client application has been throttled and should not attempt to repeat the request until an amount of time has elapsed.",
            ErrorType::InternalServerError => "There was an internal server error while processing the request.",
            ErrorType::NotImplemented => "The requested feature isn’t implemented.",
            ErrorType::ServiceUnavailable => "The service is temporarily unavailable. You may repeat the request after a delay. There may be a Retry-After header.",
            ErrorType::GatewayTimeout => "The server, while acting as a proxy, did not receive a timely response from the upstream server it needed to access in attempting to complete the request. May occur together with 503.",
            ErrorType::InsufficientStorage => "The maximum storage quota has been reached.",
            ErrorType::BandwidthLimitExceeded => "Your app has been throttled for exceeding the maximum bandwidth cap. Your app can retry the request again after more time has elapsed.",
            ErrorType::UnknownError => "Unknown error or failure",
        }
    }

    pub fn from_u16(num: u16) -> Option<ErrorType> {
        match num {
            400 => Some(ErrorType::BadRequest),
            401 => Some(ErrorType::Unauthorized),
            403 => Some(ErrorType::Forbidden),
            404 => Some(ErrorType::NotFound),
            405 => Some(ErrorType::MethodNotAllowed),
            406 => Some(ErrorType::NotAcceptable),
            409 => Some(ErrorType::Conflict),
            410 => Some(ErrorType::Gone),
            411 => Some(ErrorType::LengthRequired),
            412 => Some(ErrorType::PreconditionFailed),
            413 => Some(ErrorType::RequestEntityTooLarge),
            415 => Some(ErrorType::UnsupportedMediaType),
            416 => Some(ErrorType::RequestRangeNotSatisfiable),
            422 => Some(ErrorType::UnprocessableEntity),
            423 => Some(ErrorType::Locked),
            429 => Some(ErrorType::TooManyRequests),
            500 => Some(ErrorType::InternalServerError),
            501 => Some(ErrorType::NotImplemented),
            503 => Some(ErrorType::ServiceUnavailable),
            504 => Some(ErrorType::GatewayTimeout),
            507 => Some(ErrorType::InsufficientStorage),
            509 => Some(ErrorType::BandwidthLimitExceeded),
            _ => None,
        }
    }

    pub fn is_error(status: u16) -> bool {
        ErrorType::from_u16(status).is_some()
    }
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

pub trait WithGraphError: Sized {
    fn with_graph_error(self) -> Result<Self, GraphError>;
}

impl WithGraphError for reqwest::blocking::Response {
    fn with_graph_error(self) -> Result<Self, GraphError> {
        let code = self.status();
        if code.is_client_error() || code.is_server_error() {
            let headers = Some(GraphHeaders::from(&self));
            let error_message = self.json().unwrap_or_default();
            Err(GraphError {
                code,
                headers,
                error_message,
            })
        } else {
            Ok(self)
        }
    }
}
#[async_trait]
pub trait WithGraphErrorAsync: Sized {
    async fn with_graph_error(self) -> Result<Self, GraphError>;
}

#[async_trait]
impl WithGraphErrorAsync for reqwest::Response {
    async fn with_graph_error(self) -> Result<Self, GraphError> {
        let code = self.status();
        if code.is_client_error() || code.is_server_error() {
            let headers = Some(GraphHeaders::from(&self));
            let error_message = self.json().await.unwrap_or_default();
            Err(GraphError {
                code,
                headers,
                error_message,
            })
        } else {
            Ok(self)
        }
    }
}
