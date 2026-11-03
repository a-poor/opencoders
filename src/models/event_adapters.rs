//! Checked scalar/collection types used by generated event payloads.
use serde::{Deserialize, Serialize};

/// Integer within the inclusive bounds specified by the event schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct EventInteger<const MIN: u64, const MAX: u64>(u64);
impl<const MIN: u64, const MAX: u64> EventInteger<MIN, MAX> {
    /// Read the validated integer.
    pub fn get(self) -> u64 {
        self.0
    }
}
impl<const MIN: u64, const MAX: u64> TryFrom<u64> for EventInteger<MIN, MAX> {
    type Error = EventIntegerError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if (MIN..=MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(EventIntegerError)
        }
    }
}
/// A value outside the integer bounds of the pinned schema.
#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("integer outside schema bounds")]
pub struct EventIntegerError;
impl<'de, const MIN: u64, const MAX: u64> Deserialize<'de> for EventInteger<MIN, MAX> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let n = u64::deserialize(d)?;
        if (MIN..=MAX).contains(&n) {
            Ok(Self(n))
        } else {
            Err(serde::de::Error::custom(
                "event integer outside schema bounds",
            ))
        }
    }
}

/// The exact durable-event version specified by the pinned schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct EventVersion<const VERSION: u64>(u64);
impl<const VERSION: u64> EventVersion<VERSION> {
    /// Read the validated version.
    pub fn get(self) -> u64 {
        self.0
    }
}
impl<'de, const VERSION: u64> Deserialize<'de> for EventVersion<VERSION> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let n = serde_json::Number::deserialize(d)?;
        if n.as_f64() == Some(VERSION as f64) {
            Ok(Self(VERSION))
        } else {
            Err(serde::de::Error::custom("unexpected durable event version"))
        }
    }
}

/// A nonempty array of event payload items.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct NonEmptyEventItems<T>(Vec<T>);
impl<T> NonEmptyEventItems<T> {
    /// Inspect the validated items.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }
    /// Take ownership of the validated items.
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for NonEmptyEventItems<T> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = Vec::<T>::deserialize(d)?;
        if v.is_empty() {
            Err(serde::de::Error::custom("expected nonempty event items"))
        } else {
            Ok(Self(v))
        }
    }
}
