//! Checked scalars for the setup response schemas.
use serde::{Deserialize, Serialize};

/// Ordered configuration source: a document or a directory discovery root.
/// The document is boxed so small directory entries do not occupy its full size.
/// Debug delegates to the redacted generated payloads.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigEntry {
    /// Configuration values from a file or a virtual source.
    DocumentEncoded(Box<super::setup::ConfigDocumentEncoded>),
    /// Directory searched by configuration/domain discovery.
    DirectoryEncoded(super::setup::ConfigDirectoryEncoded),
}

/// Integer constrained by the inclusive bounds of its wire schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct SchemaInteger<const MIN: i64, const MAX: i64>(i64);
impl<const MIN: i64, const MAX: i64> SchemaInteger<MIN, MAX> {
    /// Read the validated integer without floating-point conversion.
    pub fn get(self) -> i64 {
        self.0
    }
}
impl<'de, const MIN: i64, const MAX: i64> Deserialize<'de> for SchemaInteger<MIN, MAX> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = i64::deserialize(d)?;
        if (MIN..=MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom("integer outside schema bounds"))
        }
    }
}
/// A boolean literal fixed by the schema, such as `false` to disable a feature.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct SchemaBoolean<const VALUE: bool>(bool);
impl<const VALUE: bool> SchemaBoolean<VALUE> {
    /// Read the validated literal.
    pub fn get(self) -> bool {
        self.0
    }
}
impl<'de, const VALUE: bool> Deserialize<'de> for SchemaBoolean<VALUE> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = bool::deserialize(d)?;
        if value == VALUE {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom("unexpected boolean literal"))
        }
    }
}
