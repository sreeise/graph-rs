use crate::openapi::{EitherT, Example, Reference, Schema};
use from_as::*;
use std::collections::HashMap;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

/// [Header Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#header-object)
///
/// The Header Object follows the structure of the Parameter Object with the
/// following changes:
///
/// * name MUST NOT be specified, it is given in the corresponding headers map.
/// * in MUST NOT be specified, it is implicitly in header.
/// * All traits that are affected by the location MUST be applicable to a
///   location of header (for example, style).
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /// A brief description of the parameter. This could contain examples of
    /// use. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Determines whether this parameter is mandatory. If the parameter
    /// location is "path", this property is REQUIRED and its value MUST be
    /// true. Otherwise, the property MAY be included and its default value
    /// is false.
    #[serde(default)]
    pub required: bool,

    /// Specifies that a parameter is deprecated and SHOULD be transitioned out
    /// of usage. Default value is false
    #[serde(default)]
    pub deprecated: bool,

    /// Sets the ability to pass empty-valued parameters. This is valid only for
    /// query parameters and allows sending a parameter with an empty value.
    /// Default value is false. If style is used, and if behavior is n/a
    /// (cannot be serialized), the value of allowEmptyValue SHALL be ignored.
    /// Use of this property is NOT RECOMMENDED, as it is likely to be removed
    /// in a later revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,

    /// Describes how the parameter value will be serialized depending on the
    /// type of the parameter value. Default values (based on value of in):
    /// for query - form; for path - simple; for header - simple; for cookie
    /// - form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,

    /// When this is true, parameter values of type array or object generate
    /// separate parameters for each value of the array or key-value pair of
    /// the map. For other types of parameters this property has no effect.
    /// When style is form, the default value is true. For all other styles,
    /// the default value is false.
    #[serde(default)]
    pub explode: bool,

    /// Determines whether the parameter value SHOULD allow reserved characters,
    /// as defined by RFC3986 :/?#[]@!$&'()*+,;= to be included without
    /// percent-encoding. This property only applies to parameters with an
    /// in value of query. The default value is false.
    #[serde(default)]
    pub allow_reserved: bool,

    /// The schema defining the type used for the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    // pub schema: Option<serde_json::Value>,
    /// Example of the parameter's potential value. The example SHOULD match the
    /// specified schema and encoding properties if present. The example
    /// field is mutually exclusive of the examples field. Furthermore, if
    /// referencing a schema that contains an example, the example value
    /// SHALL override the example provided by the schema. To represent examples
    /// of media types that cannot naturally be represented in JSON or YAML, a
    /// string value can contain the example with escaping where necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// Examples of the parameter's potential value. Each example SHOULD contain
    /// a value in the correct format as specified in the parameter
    /// encoding. The examples field is mutually exclusive of the example
    /// field. Furthermore, if referencing a schema that contains an
    /// example, the examples value SHALL override the example provided by the
    /// schema.
    #[serde(default)]
    //#[serde(deserialize_with = "either_t_map_right_or_reference")]
    pub examples: HashMap<String, EitherT<Example, Reference>>,

    /// A map containing the representations for the parameter. The key is the
    /// media type and the value describes it. The map MUST only contain one
    /// entry.
    #[serde(default)]
    pub content: HashMap<String, serde_json::Value>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
