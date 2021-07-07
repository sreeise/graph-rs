use from_as::*;
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Read, Write},
};

/// [OAuth Flow Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#oauthFlowObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlow {
    /// REQUIRED. The authorization URL to be used for this flow. This MUST be
    /// in the form of a URL. The OAuth2 standard requires the use of TLS.
    pub authorization_url: String,

    /// REQUIRED. The token URL to be used for this flow. This MUST be in the
    /// form of a URL. The OAuth2 standard requires the use of TLS.
    pub token_url: String,

    /// The URL to be used for obtaining refresh tokens. This MUST be in the
    /// form of a URL. The OAuth2 standard requires the use of TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,

    /// REQUIRED. The available scopes for the OAuth2 security scheme. A map
    /// between the scope name and a short description for it. The map MAY be
    /// empty.
    #[serde(default)]
    pub scopes: HashMap<String, String>,
}
