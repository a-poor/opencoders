//! Constructors for strict stable session workflow inputs.
use super::*;
impl SessionCommandRequest {
    /// Invoke a registered command with text, including an explicitly empty string.
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
            files: None,
            agents: None,
            skills: None,
            delivery: RequestField::Omitted,
        }
    }
    /// Set callback delivery mode; omitted/null defaults to steer on v2.2.5.
    pub fn delivery(mut self, value: SessionInboxDelivery) -> Self {
        self.delivery = RequestField::Value(value);
        self
    }
    /// Supply strict URI attachments; the SDK does not read/upload local files.
    pub fn files(mut self, value: Vec<PromptInputFileAttachment>) -> Self {
        self.files = Some(value);
        self
    }
    /// Supply agent attachments; an empty list remains explicit.
    pub fn agents(mut self, value: Vec<PromptInputAgentAttachment>) -> Self {
        self.agents = Some(value);
        self
    }
    /// Supply skill attachments; an empty list remains explicit.
    pub fn skills(mut self, value: Vec<PromptInputSkillAttachment>) -> Self {
        self.skills = Some(value);
        self
    }
}
impl SessionCompactRequest {
    /// Supply a durable input ID. Reconciliation belongs to the server; no SDK retry.
    pub fn id(mut self, value: SessionMessageId) -> Self {
        self.id = RequestField::Value(value);
        self
    }
    /// Set delivery mode independently of execution, which compaction wakes.
    pub fn delivery(mut self, value: SessionInboxDelivery) -> Self {
        self.delivery = RequestField::Value(value);
        self
    }
}
impl SessionRevertStageRequest {
    /// Stage this inclusive history boundary. File restoration defaults on upstream;
    /// call `.files(false)` for a history-only revert.
    pub fn new(message_id: SessionMessageId) -> Self {
        Self {
            message_id,
            files: RequestField::Omitted,
        }
    }
    /// Explicitly enable/disable file restoration for the new boundary.
    pub fn files(mut self, value: bool) -> Self {
        self.files = RequestField::Value(value);
        self
    }
}
