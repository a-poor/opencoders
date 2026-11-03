//! Constructors and fluent conveniences for generated, strict request objects.

use super::{
    AgentId, PermissionEffect, PermissionRuleInput, PermissionRulesetInput, ProviderId,
    SessionCreateRequest, SessionForkRequest, SessionId, SessionLocationInput, SessionMessageId,
    SessionMetadata, SessionModelInput, SessionSwitchAgentRequest, SessionSwitchModelRequest,
    SessionUpdateRequest,
};
use crate::RequestField;

impl SessionForkRequest {
    /// Copy settled projected history strictly before this message on v2.2.5.
    /// The boundary message must belong to the source. Omission/null instead
    /// copies through the newest projected message; pending inbox work is not history.
    pub fn before(mut self, id: SessionMessageId) -> Self {
        self.before = RequestField::Value(id);
        self
    }
}

impl SessionLocationInput {
    /// Select a directory on the server; no local filesystem lookup is performed.
    pub fn new(directory: impl Into<String>) -> Self {
        Self {
            directory: directory.into().into(),
        }
    }
}

impl SessionSwitchAgentRequest {
    /// Select an agent by ID for subsequent provider turns.
    pub fn new(agent: AgentId) -> Self {
        Self { agent }
    }
}
impl SessionSwitchModelRequest {
    /// Select a model using a strict input reference; response extensions are not sent.
    pub fn new(model: SessionModelInput) -> Self {
        Self { model }
    }
}

impl SessionModelInput {
    /// Select a model using its catalog `id` and provider, not its native `modelID`.
    /// Session creation and selection requests do not execute the model.
    pub fn new(provider_id: ProviderId, id: impl Into<String>) -> Self {
        Self {
            provider_id,
            id: id.into(),
            variant: None,
        }
    }

    /// Select a named variant; `None` on the public field omits the property.
    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }
}

impl PermissionRuleInput {
    /// Construct an explicit permission rule without choosing an approval policy.
    pub fn new(
        action: impl Into<String>,
        effect: PermissionEffect,
        resource: impl Into<String>,
    ) -> Self {
        Self {
            action: action.into(),
            effect,
            resource: resource.into(),
        }
    }
}

impl SessionCreateRequest {
    /// Supply an explicit ID. This does not establish an idempotency guarantee.
    pub fn id(mut self, id: SessionId) -> Self {
        self.id = RequestField::Value(id);
        self
    }

    /// Create a linked child at its parent's location.
    pub fn parent_id(mut self, parent_id: SessionId) -> Self {
        self.parent_id = RequestField::Value(parent_id);
        self
    }

    /// Set the title, preserving even an explicitly empty string.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = RequestField::Value(title.into());
        self
    }

    /// Select an agent by its shared ID type.
    pub fn agent(mut self, agent: AgentId) -> Self {
        self.agent = RequestField::Value(agent);
        self
    }

    /// Select a model for the session.
    pub fn model(mut self, model: SessionModelInput) -> Self {
        self.model = RequestField::Value(model);
        self
    }

    /// Select the creation location explicitly in the JSON body.
    /// A concrete parent ID takes precedence on OpenCode v2.2.5.
    pub fn location(mut self, location: SessionLocationInput) -> Self {
        self.location = RequestField::Value(location);
        self
    }

    /// Set application metadata, including arbitrary nested JSON.
    pub fn metadata(mut self, metadata: impl Into<SessionMetadata>) -> Self {
        self.metadata = RequestField::Value(metadata.into());
        self
    }

    /// Set permission rules explicitly; an empty list is sent unchanged.
    pub fn permissions(mut self, permissions: impl Into<PermissionRulesetInput>) -> Self {
        self.permissions = RequestField::Value(permissions.into());
        self
    }
}

impl SessionUpdateRequest {
    /// Rename a session. On v2.2.5, an empty string requests generated titling
    /// instead and may invoke a model when the session has conversation history.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = RequestField::Value(title.into());
        self
    }

    /// Replace application metadata rather than merge it on v2.2.5.
    /// An empty map removes all metadata entries.
    pub fn metadata(mut self, metadata: impl Into<SessionMetadata>) -> Self {
        self.metadata = RequestField::Value(metadata.into());
        self
    }

    /// Replace the session's permission rules. An empty list is sent unchanged.
    pub fn permissions(mut self, permissions: impl Into<PermissionRulesetInput>) -> Self {
        self.permissions = RequestField::Value(permissions.into());
        self
    }

    /// Archive with `true` or restore with `false`.
    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = RequestField::Value(archived);
        self
    }
}
