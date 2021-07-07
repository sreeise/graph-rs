use crate::openapi::OAuthFlow;
use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [OAuth Flows Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#oauth-flows-object)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {
    /// Configuration for the OAuth Implicit flow
    #[serde(skip_serializing_if = "Option::is_none")]
    implicit: Option<OAuthFlow>,

    /// Configuration for the OAuth Resource Owner Password flow
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<OAuthFlow>,

    /// Configuration for the OAuth Client Credentials flow. Previously called
    /// application in OpenAPI 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    client_credentials: Option<OAuthFlow>,

    /// Configuration for the OAuth Authorization Code flow. Previously called
    /// accessCode in OpenAPI 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_code: Option<OAuthFlow>,
}
