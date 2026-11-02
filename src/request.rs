use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A request property's wire state: absent, JSON null, or a concrete value.
///
/// Generated request objects skip [`Self::Omitted`]. Serializing it outside a
/// skipped property returns an error; it never silently becomes JSON null.
/// `Null` preserves wire syntax and does not imply a server-side clearing policy.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RequestField<T> {
    /// Do not send the property.
    #[default]
    Omitted,
    /// Send an explicit JSON null.
    Null,
    /// Send the provided value.
    Value(T),
}

impl<T> RequestField<T> {
    /// Whether a containing request should omit this property.
    pub fn is_omitted(&self) -> bool {
        matches!(self, Self::Omitted)
    }
}

impl<T: Serialize> Serialize for RequestField<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Omitted => Err(serde::ser::Error::custom("omitted fields must be skipped")),
            Self::Null => serializer.serialize_none(),
            Self::Value(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RequestField<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}
