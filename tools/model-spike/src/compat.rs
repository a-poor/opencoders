//! Small, explicit compatibility experiments; not public client types.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "Value", into = "Value")]
pub struct NonNullValue(Value);

impl TryFrom<Value> for NonNullValue {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            Err("expected non-null JSON")
        } else {
            Ok(Self(value))
        }
    }
}

impl From<NonNullValue> for Value {
    fn from(value: NonNullValue) -> Self {
        value.0
    }
}

/// A request-shape experiment for the actual session update `title` field.
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTitlePatch {
    #[serde(default, skip_serializing_if = "Field::is_omitted")]
    pub title: Field<String>,
}

/// Preserve all three wire states, without assuming what clearing means server-side.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Field<T> {
    #[default]
    Omitted,
    Null,
    Value(T),
}

impl<T> Field<T> {
    pub fn is_omitted(&self) -> bool {
        matches!(self, Self::Omitted)
    }
}

impl<T: Serialize> Serialize for Field<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Omitted => Err(serde::ser::Error::custom("omitted fields must be skipped")),
            Self::Null => serializer.serialize_none(),
            Self::Value(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Field<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}
