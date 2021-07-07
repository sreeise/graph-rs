use crate::openapi::{Discriminator, EitherT, ExternalDocumentation, Reference, XML};
use from_as::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

/// [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#schemaObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<serde_json::Value>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    // pub format: Option<serde_json::Value>,
    pub format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,

    /// Regex pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    #[serde(default)]
    pub read_only: bool,

    #[serde(default)]
    pub write_only: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_of: Option<VecDeque<EitherT<Box<Schema>, Reference>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<VecDeque<EitherT<Box<Schema>, Reference>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_of: Option<VecDeque<EitherT<Box<Schema>, Reference>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<Schema>>,

    #[serde(default)]
    pub required: HashSet<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,

    #[serde(default)]
    pub unique_items: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<HashMap<String, Schema>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,

    #[serde(default)]
    pub additional_properties_allowed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Box<EitherT<bool, Schema>>>,

    /// Adds support for polymorphism. The discriminator is an object name that
    /// is used to differentiate between other schemas which may satisfy the
    /// payload description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<Discriminator>,

    /// Deprecated: The example property has been deprecated in favor of the
    /// JSON Schema examples keyword. Use of example is discouraged, and
    /// later versions of this specification may remove it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    #[serde(default)]
    pub enum_: VecDeque<serde_json::Value>,

    #[serde(default)]
    pub nullable: bool,

    /// Additional external documentation for this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,

    #[serde(default)]
    pub deprecated: bool,

    /// This MAY be used only on properties schemas. It has no effect on root
    /// schemas. Adds additional metadata to describe the XML representation
    /// of this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml: Option<XML>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,

    #[serde(default)]
    pub unresolved_reference: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_names: Option<Box<Schema>>,

    /// The value of "patternProperties" MUST be an object.  Each property name
    /// of this object SHOULD be a valid regular expression, according to the
    /// ECMA-262 regular expression dialect.  Each property value of this object
    /// MUST be a valid JSON Schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<serde_json::Value>,

    /// [Schema Re-Use With "$defs"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-00#section-8.2.4)
    #[serde(rename = "$defs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defs: Option<serde_json::Value>,

    /// The "$ref" keyword is an applicator that is used to reference a
    /// statically identified schema.
    ///
    /// The value of the "$ref" keyword MUST be a string which is a URI-
    /// Reference.  Resolved against the current URI base, it produces the
    /// URI of the schema to apply.  This resolution is safe to perform on
    /// schema load, as the process of evaluating an instance cannot change
    /// how the reference resolves.
    ///
    /// [Direct References with $ref](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-00#section-8.2.3.1)
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    /// The "$dynamicRef" keyword is an applicator that allows for deferring
    /// the full resolution until runtime, at which point it is resolved each
    /// time it is encountered while evaluating an instance.
    ///
    /// [Dynamic References with "$dynamicRef"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-00#section-8.2.3.2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_ref: Option<String>,

    /// This keyword reserves a location for comments from schema authors to
    /// readers or maintainers of the schema. [Comments with $comment](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-00#section-8.3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "if")]
    pub if_: Option<Box<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub then: Option<Box<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "else")]
    pub else_: Option<Box<Schema>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_schemas: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_items: Option<Box<VecDeque<Schema>>>,
}

impl Schema {
    pub fn is_upload_session(&self) -> bool {
        if let Some(any_of) = self.any_of.as_ref() {
            for either_t in any_of.iter() {
                if let Some(reference) = either_t.clone().into_right() {
                    if reference.is_upload_session() {
                        return true;
                    }
                }
            }
        }

        false
    }
}
