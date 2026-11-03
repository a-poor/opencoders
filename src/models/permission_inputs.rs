//! Constructors for strict permission inputs. The caller chooses every decision.
use super::*;

impl SessionPermissionCreateRequest {
    /// Evaluate this action/resources pair. Empty strings/lists are sent as given.
    /// Other fields are omitted; only an `ask` result creates a pending request.
    pub fn new(action: impl Into<String>, resources: Vec<String>) -> Self {
        Self {
            action: action.into(),
            resources,
            id: RequestField::Omitted,
            agent: RequestField::Omitted,
            save: None,
            metadata: None,
            source: None,
        }
    }
    /// Supply a permission ID. This does not provide an idempotency guarantee;
    /// v2.2.5 rejects a duplicate pending ID. No automatic retry is performed.
    pub fn id(mut self, id: PermissionId) -> Self {
        self.id = RequestField::Value(id);
        self
    }
    /// Select the agent whose permission rules participate in evaluation.
    pub fn agent(mut self, agent: AgentId) -> Self {
        self.agent = RequestField::Value(agent);
        self
    }
    /// Patterns the server may save for an `always` reply. Empty stays explicit.
    pub fn save(mut self, save: Vec<String>) -> Self {
        self.save = Some(save);
        self
    }
    /// Attach arbitrary metadata. An empty map is sent unchanged.
    pub fn metadata(mut self, metadata: serde_json::Map<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }
    /// Describe the originating tool using strict input fields.
    pub fn source(mut self, source: PermissionSourceInput) -> Self {
        self.source = Some(source);
        self
    }
}
impl PermissionSourceInput {
    /// Identify the tool call. Both IDs are unconstrained strings in the schema.
    pub fn tool(message_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self::Tool {
            message_id: message_id.into(),
            id: id.into(),
        }
    }
}
impl SessionPermissionReplyRequest {
    /// Choose once, always, or reject explicitly. On v2.2.5 reject also rejects
    /// other requests in the session; always may save project rules and resolve
    /// matching pending requests in other sessions at the same location.
    pub fn new(decision: PermissionReply) -> Self {
        Self {
            decision,
            message: RequestField::Omitted,
        }
    }
    /// Supply feedback. Empty stays distinct from omission/null on the wire;
    /// the pinned server uses nonempty feedback when rejecting a request batch.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = RequestField::Value(message.into());
        self
    }
}
