use crate::auth::{OAuthCredential, OAuthReq};
use crate::grants::{GrantRequest, GrantType};
use graph_error::GraphFailure;
use std::error;
use std::error::Error;
use std::fmt;
use std::io::ErrorKind;

/// Error implementation for OAuth
#[derive(Debug)]
pub enum OAuthError {
    GraphFailure(GraphFailure),
}

impl OAuthError {
    pub fn error_kind(error_kind: ErrorKind, message: &str) -> GraphFailure {
        let e = std::io::Error::new(error_kind, message);
        GraphFailure::from(e)
    }

    pub fn none_error<T>(msg: &str) -> std::result::Result<T, GraphFailure> {
        Err(GraphFailure::error_kind(ErrorKind::NotFound, msg))
    }

    pub fn invalid_data<T>(msg: &str) -> std::result::Result<T, GraphFailure> {
        Err(OAuthError::error_kind(ErrorKind::InvalidData, msg))
    }

    pub fn error_from<T>(c: OAuthCredential) -> Result<T, GraphFailure> {
        Err(OAuthError::credential_error(c))
    }

    pub fn credential_error(c: OAuthCredential) -> GraphFailure {
        GraphFailure::error_kind(
            ErrorKind::NotFound,
            format!("MISSING OR INVALID: {:#?}", c).as_str(),
        )
    }

    pub fn grant_error<T>(grant: GrantType, grant_request: GrantRequest, msg: &str) -> OAuthReq<T> {
        let error_str = format!("There was an error for the grant: {:#?} when executing a request for: {:#?}\nError: {:#?}", grant, grant_request, msg);
        OAuthError::invalid_data(error_str.as_str())
    }
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OAuthError::GraphFailure(ref err) => write!(f, "Graph Failure: {}", err),
        }
    }
}

impl error::Error for OAuthError {
    fn description(&self) -> &str {
        match *self {
            OAuthError::GraphFailure(ref err) => err.description(),
        }
    }

    fn source<'a>(&'a self) -> Option<&(dyn Error + 'static)> {
        match *self {
            OAuthError::GraphFailure(ref err) => Some(err),
        }
    }
}

impl From<GraphFailure> for OAuthError {
    fn from(err: GraphFailure) -> Self {
        OAuthError::GraphFailure(err)
    }
}
