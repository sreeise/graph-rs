use from_as::*;
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

/// [Server Variable Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#server-variable-object)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options
    /// are from a limited set. The array MUST NOT be empty.
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_: Option<VecDeque<String>>,

    /// REQUIRED. The default value to use for substitution, which SHALL be sent
    /// if an alternate value is not supplied. Note this behavior is
    /// different than the Schema Object's treatment of default values,
    /// because in those cases parameter values are optional. If the enum is
    /// defined, the value MUST exist in the enum's values.
    pub description: String,

    /// An optional description for the server variable. CommonMark syntax MAY
    /// be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
}
