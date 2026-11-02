//! Constructors for strict prompt/inbox request models.

use super::*;

impl SessionPromptRequest {
    /// Submit this text, including an explicitly empty string. Other fields are
    /// omitted. On v2.2.5 omission defaults to steering and waking execution.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            id: RequestField::Omitted,
            delivery: RequestField::Omitted,
            resume: RequestField::Omitted,
            files: None,
            agents: None,
            skills: None,
            metadata: None,
        }
    }
    /// Supply a message ID. Reconciliation rules belong to the server; the SDK
    /// does not automatically retry even when an explicit ID is provided.
    pub fn id(mut self, id: SessionMessageId) -> Self {
        self.id = RequestField::Value(id);
        self
    }
    /// Set the delivery mode explicitly, independently of whether execution is woken.
    pub fn delivery(mut self, delivery: SessionInboxDelivery) -> Self {
        self.delivery = RequestField::Value(delivery);
        self
    }
    /// Whether to wake execution after admission. False does not pause a running loop.
    pub fn resume(mut self, resume: bool) -> Self {
        self.resume = RequestField::Value(resume);
        self
    }
    /// Supply server-resolved file URIs. An empty list is sent, not omitted.
    pub fn files(mut self, files: Vec<PromptInputFileAttachment>) -> Self {
        self.files = Some(files);
        self
    }
    /// Supply agent attachments. An empty list is sent, not omitted.
    pub fn agents(mut self, agents: Vec<PromptInputAgentAttachment>) -> Self {
        self.agents = Some(agents);
        self
    }
    /// Supply skill IDs for server-side preparation. An empty list is sent unchanged.
    pub fn skills(mut self, skills: Vec<PromptInputSkillAttachment>) -> Self {
        self.skills = Some(skills);
        self
    }
    /// Supply arbitrary JSON metadata. An empty map is sent, not omitted.
    pub fn metadata(mut self, metadata: serde_json::Map<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}
impl SessionInboxUpdateRequest {
    /// Change only delivery mode. Steering wakes execution on v2.2.5.
    pub fn new(delivery: SessionInboxDelivery) -> Self {
        Self { delivery }
    }
}
impl PromptInputFileAttachment {
    /// Reference a URI resolved by the server. No local file read/upload is performed.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            source: None,
            name: None,
            description: None,
            mention: None,
        }
    }
}
impl PromptInputAgentAttachment {
    /// Attach an agent by name, without changing the session's selected agent.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mention: None,
        }
    }
}
impl PromptInputSkillAttachment {
    /// Reference a skill for server-side lookup and preparation.
    pub fn new(id: SkillId) -> Self {
        Self { id, mention: None }
    }
}
impl PromptInputMention {
    /// Preserve the caller's mention coordinates and text without reinterpretation.
    pub fn new(
        start: serde_json::Number,
        end: serde_json::Number,
        text: impl Into<String>,
    ) -> Self {
        Self {
            start,
            end,
            text: text.into(),
        }
    }
}
impl PromptInputFileSource {
    /// Record a source URI separately from the attachment's resolvable input URI.
    pub fn uri(uri: impl Into<String>) -> Self {
        Self::Uri(uri.into())
    }
}

impl<'de> serde::Deserialize<'de> for PromptInputFileSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let object = raw
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("expected file source object"))?;
        match object.get("type").and_then(serde_json::Value::as_str) {
            Some("inline") if object.len() == 1 => Ok(Self::Inline),
            Some("uri") if object.len() == 2 => object
                .get("uri")
                .and_then(serde_json::Value::as_str)
                .map(|uri| Self::Uri(uri.to_owned()))
                .ok_or_else(|| serde::de::Error::custom("expected file source URI")),
            _ => Err(serde::de::Error::custom("unknown or malformed file source")),
        }
    }
}
