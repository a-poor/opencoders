use super::{Event, events::EventLogSynced};
use serde::{Deserialize, Serialize};

/// Experimental log payload. Known transient kinds and malformed known items fail
/// decoding; future kinds preserve their complete JSON without inferring a cursor.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionLogItem {
    /// One of the reviewed durable session events, reusing the global event model.
    Event(Box<Event>),
    /// Captured replay watermark. Not a transaction boundary with REST snapshots,
    /// a promise of retention, or proof that the application handled every event.
    Synced(EventLogSynced),
    /// Future log item. Applications must handle it before advancing checkpoints.
    Unknown(serde_json::Value),
}
impl<'de> Deserialize<'de> for SessionLogItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = serde_json::Value::deserialize(deserializer)?;
        if raw.get("type").and_then(serde_json::Value::as_str) == Some("log.synced") {
            return serde_json::from_value(raw)
                .map(Self::Synced)
                .map_err(serde::de::Error::custom);
        }
        match serde_json::from_value::<Event>(raw).map_err(serde::de::Error::custom)? {
            Event::Unknown(raw) => Ok(Self::Unknown(raw)),
            event if event.durable_sequence().is_some() => Ok(Self::Event(Box::new(event))),
            _ => Err(serde::de::Error::custom("non-durable event in session log")),
        }
    }
}
impl SessionLogItem {
    /// Sequence of a reviewed event or captured watermark. `None` represents an
    /// empty watermark or an unknown kind. Do not replace it with zero. Watermarks
    /// may exceed emitted events or be below a requested future `after` cursor.
    /// Persist a checkpoint only after handling its preceding items successfully.
    pub fn sequence(&self) -> Option<u64> {
        match self {
            Self::Event(e) => e.durable_sequence(),
            Self::Synced(s) => s.seq.as_ref().map(|v| v.0.get()),
            Self::Unknown(_) => None,
        }
    }
    /// Aggregate of a reviewed event or marker. No field is inferred for unknowns.
    pub fn aggregate_id(&self) -> Option<&str> {
        match self {
            Self::Event(e) => e.durable_aggregate_id(),
            Self::Synced(s) => Some(&s.aggregate_id),
            Self::Unknown(_) => None,
        }
    }
}
