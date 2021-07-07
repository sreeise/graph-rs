use crate::{
    openapi::{Reference, Schema},
    serde::{
        de::{DeserializeOwned, Error},
        Deserialize, Deserializer,
    },
};
use either::Either;
use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

#[derive(Debug, Clone, Serialize, FromFile, AsFile)]
pub struct EitherT<T, U> {
    either: Either<T, U>,
}

impl<T, U> EitherT<T, U> {
    pub fn left(t: T) -> EitherT<T, U> {
        EitherT {
            either: Either::Left(t),
        }
    }

    pub fn right(u: U) -> EitherT<T, U> {
        EitherT {
            either: Either::Right(u),
        }
    }

    pub fn is_left(&self) -> bool {
        self.either.is_left()
    }

    pub fn is_right(&self) -> bool {
        self.either.is_right()
    }

    pub fn into_either(self) -> Either<T, U> {
        self.either
    }

    pub fn into_left(self) -> Option<T> {
        self.either.left()
    }

    pub fn into_right(self) -> Option<U> {
        self.either.right()
    }

    pub fn left_or(self, other: T) -> T {
        self.either.left_or(other)
    }

    pub fn right_or(self, other: U) -> U {
        self.either.right_or(other)
    }

    pub fn either_as_ref(&self) -> Either<&T, &U> {
        self.either.as_ref()
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for EitherT<T, Reference> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        let map = value.as_object().ok_or_else(|| {
            D::Error::custom(&format!(
                "Expected struct matching either `T` or `Reference`. Found the following \
                 serde_json::Value:\n{:#?}\n",
                value
            ))
        })?;

        if map.contains_key("$ref") {
            let reference: Reference = serde_json::from_value(value).unwrap();
            Ok(EitherT::right(reference))
        } else {
            let t: T = serde_json::from_value(value).unwrap();
            Ok(EitherT::left(t))
        }
    }
}

impl<'de> Deserialize<'de> for EitherT<String, serde_json::Value> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        if value.is_string() {
            let s: String = serde_json::from_value(value).unwrap_or_default();
            Ok(EitherT::left(s))
        } else {
            let json: serde_json::Value = serde_json::from_value(value).unwrap_or_default();
            Ok(EitherT::right(json))
        }
    }
}

impl<'de> Deserialize<'de> for EitherT<bool, Schema> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        if value.is_boolean() {
            let b: bool = serde_json::from_value(value).unwrap_or_default();
            Ok(EitherT::left(b))
        } else {
            let schema: Schema = serde_json::from_value(value).unwrap_or_default();
            Ok(EitherT::right(schema))
        }
    }
}

impl<'de> Deserialize<'de> for EitherT<String, Schema> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        if value.is_string() {
            let s: String = serde_json::from_value(value).unwrap_or_default();
            Ok(EitherT::left(s))
        } else {
            let schema: Schema = serde_json::from_value(value).unwrap_or_default();
            Ok(EitherT::right(schema))
        }
    }
}
