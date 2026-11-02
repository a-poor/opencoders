//! Explicit event dispatch experiment using the reduced probe event set.

use crate::generated::{SessionTextDelta, V2EventServerConnected};
use serde::de::Error as _;
use serde_json::Value;

#[derive(Debug)]
pub enum Event {
    TextDelta(SessionTextDelta),
    Connected(V2EventServerConnected),
    Unknown(Value),
}

/// Decode the text of an SSE data field, not a JSON-quoted copy of that text.
pub fn decode_event_data(data: &str) -> Result<Event, serde_json::Error> {
    let value: Value = serde_json::from_str(data)?;
    let kind = value
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| serde_json::Error::custom("event must have a string type"))?;
    match kind {
        "session.text.delta" => serde_json::from_value(value).map(Event::TextDelta),
        "server.connected" => serde_json::from_value(value).map(Event::Connected),
        _ => Ok(Event::Unknown(value)),
    }
}
