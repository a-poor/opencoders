//! Explicit inbox dispatch and the schema's non-null JSON payload.

use super::*;
use serde::{Deserialize, Serialize};

/// An arbitrary JSON value except null, used by opaque compaction payloads.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct NonNullValue(serde_json::Value);

impl TryFrom<serde_json::Value> for NonNullValue {
    type Error = &'static str;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        if value.is_null() {
            Err("expected non-null JSON")
        } else {
            Ok(Self(value))
        }
    }
}
impl From<NonNullValue> for serde_json::Value {
    fn from(value: NonNullValue) -> Self {
        value.0
    }
}
impl NonNullValue {
    /// Inspect the original JSON without losing its non-null invariant.
    pub fn as_value(&self) -> &serde_json::Value {
        &self.0
    }
}

/// Durable, pending work. Unknown kinds or malformed known variants fail decoding.
/// Payloads retain additional fields and are boxed to keep enum values small.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionInboxInfo {
    /// User input awaiting delivery.
    User(Box<SessionInboxUser>),
    /// Server-generated input awaiting delivery.
    Synthetic(Box<SessionInboxSynthetic>),
    /// Pending context compaction.
    Compaction(Box<SessionInboxCompaction>),
    /// Pending working-location change.
    Move(Box<SessionInboxMove>),
}
impl<'de> Deserialize<'de> for SessionInboxInfo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = serde_json::Value::deserialize(deserializer)?;
        match raw.get("type").and_then(serde_json::Value::as_str) {
            Some("user") => serde_json::from_value(raw).map(Self::User),
            Some("synthetic") => serde_json::from_value(raw).map(Self::Synthetic),
            Some("compaction") => serde_json::from_value(raw).map(Self::Compaction),
            Some("move") => serde_json::from_value(raw).map(Self::Move),
            _ => {
                return Err(serde::de::Error::custom(
                    "unknown or missing type in SessionInboxInfo",
                ));
            }
        }
        .map_err(serde::de::Error::custom)
    }
}
impl SessionInboxInfo {
    /// The shared message ID used for inbox updates/cancellation.
    pub fn id(&self) -> &SessionMessageId {
        match self {
            Self::User(v) => &v.id,
            Self::Synthetic(v) => &v.id,
            Self::Compaction(v) => &v.id,
            Self::Move(v) => &v.id,
        }
    }
    /// The owning session ID.
    pub fn session_id(&self) -> &SessionId {
        match self {
            Self::User(v) => &v.session_id,
            Self::Synthetic(v) => &v.session_id,
            Self::Compaction(v) => &v.session_id,
            Self::Move(v) => &v.session_id,
        }
    }
    /// The recorded delivery mode. It does not indicate execution state.
    pub fn delivery(&self) -> SessionInboxDelivery {
        match self {
            Self::User(v) => v.delivery,
            Self::Synthetic(v) => v.delivery,
            Self::Compaction(v) => v.delivery,
            Self::Move(v) => v.delivery,
        }
    }
}
