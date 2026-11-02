//! Tag dispatch and constraints for projected message responses.

use super::*;
use serde::{Deserialize, Serialize};

/// A projected timeline message, selected by its wire `type` tag.
///
/// Unknown types and malformed known variants are decoding errors. Each payload
/// preserves additive fields. Payloads are boxed to keep timeline entries small.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionMessageInfo {
    /// Agent selection changed.
    AgentSelected(Box<SessionMessageAgentSelected>),
    /// Model selection changed.
    ModelSelected(Box<SessionMessageModelSelected>),
    /// Working location changed.
    LocationSwitched(Box<SessionMessageLocationSwitched>),
    /// User input, including attachments.
    User(Box<SessionMessageUser>),
    /// Synthetic input.
    Synthetic(Box<SessionMessageSynthetic>),
    /// System message.
    System(Box<SessionMessageSystem>),
    /// Skill instructions.
    Skill(Box<SessionMessageSkill>),
    /// Shell execution and captured output.
    Shell(Box<SessionMessageShell>),
    /// Assistant text, reasoning, and tool calls.
    Assistant(Box<SessionMessageAssistant>),
    /// Context compaction and its current state.
    Compaction(Box<SessionMessageCompaction>),
    /// Execution became idle.
    Idle(Box<SessionMessageIdle>),
}

macro_rules! tagged {
    ($name:ident, $key:literal, {$($tag:literal => $variant:ident),+ $(,)?}) => {
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error> {
                let raw=serde_json::Value::deserialize(deserializer)?;
                match raw.get($key).and_then(serde_json::Value::as_str) {
                    $(Some($tag)=>serde_json::from_value(raw).map(Self::$variant).map_err(serde::de::Error::custom),)+
                    _=>Err(serde::de::Error::custom(concat!("unknown or missing ",$key," in ",stringify!($name)))),
                }
            }
        }
    };
}

tagged!(SessionMessageInfo,"type",{
    "agent-switched"=>AgentSelected,"model-switched"=>ModelSelected,
    "location-switched"=>LocationSwitched,"user"=>User,"synthetic"=>Synthetic,
    "system"=>System,"skill"=>Skill,"shell"=>Shell,"assistant"=>Assistant,
    "compaction"=>Compaction,"idle"=>Idle
});
tagged!(SessionMessageCompaction,"status",{"running"=>Running,"completed"=>Completed,"failed"=>Failed});
tagged!(SessionMessageAssistantContentItem,"type",{"text"=>Text,"reasoning"=>Reasoning,"tool"=>Tool});
tagged!(SessionMessageAssistantToolState,"status",{"streaming"=>Streaming,"running"=>Running,"completed"=>Completed,"error"=>Error});
tagged!(ToolContent,"type",{"text"=>TextContent,"file"=>FileContent});

impl SessionMessageInfo {
    /// The shared typed message ID, regardless of message kind.
    pub fn id(&self) -> &SessionMessageId {
        match self {
            Self::AgentSelected(v) => &v.id,
            Self::ModelSelected(v) => &v.id,
            Self::LocationSwitched(v) => &v.id,
            Self::User(v) => &v.id,
            Self::Synthetic(v) => &v.id,
            Self::System(v) => &v.id,
            Self::Skill(v) => &v.id,
            Self::Shell(v) => &v.id,
            Self::Assistant(v) => &v.id,
            Self::Idle(v) => &v.id,
            Self::Compaction(v) => match v.as_ref() {
                SessionMessageCompaction::Running(v) => &v.id,
                SessionMessageCompaction::Completed(v) => &v.id,
                SessionMessageCompaction::Failed(v) => &v.id,
            },
        }
    }
    /// The wire kind, also usable as a message-list filter.
    pub fn message_type(&self) -> SessionMessageType {
        match self {
            Self::AgentSelected(_) => SessionMessageType::AgentSwitched,
            Self::ModelSelected(_) => SessionMessageType::ModelSwitched,
            Self::LocationSwitched(_) => SessionMessageType::LocationSwitched,
            Self::User(_) => SessionMessageType::User,
            Self::Synthetic(_) => SessionMessageType::Synthetic,
            Self::System(_) => SessionMessageType::System,
            Self::Skill(_) => SessionMessageType::Skill,
            Self::Shell(_) => SessionMessageType::Shell,
            Self::Assistant(_) => SessionMessageType::Assistant,
            Self::Compaction(_) => SessionMessageType::Compaction,
            Self::Idle(_) => SessionMessageType::Idle,
        }
    }
}

pub(super) fn deserialize_message_status<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<u16>, D::Error> {
    let value = u16::deserialize(d)?;
    if !(100..=599).contains(&value) {
        return Err(serde::de::Error::custom(
            "status must be between 100 and 599",
        ));
    }
    Ok(Some(value))
}
pub(super) fn deserialize_message_retry_delay<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<u64>, D::Error> {
    deserialize_diff_count(d).map(Some)
}
pub(super) fn deserialize_message_attempt<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<u64, D::Error> {
    let value = deserialize_diff_count(d)?;
    if value == 0 {
        return Err(serde::de::Error::custom("retry attempt must be positive"));
    }
    Ok(value)
}
pub(super) fn deserialize_tool_content<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Vec<ToolContent>, D::Error> {
    let value = Vec::<ToolContent>::deserialize(d)?;
    if value.is_empty() {
        return Err(serde::de::Error::custom(
            "tool content must contain at least one item",
        ));
    }
    Ok(value)
}
