use from_as::*;
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

/// [Security Requirement Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#security-requirement-object)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct SecurityRequirement {
    #[serde(default)]
    security_schemes: HashMap<String, VecDeque<String>>,
}
