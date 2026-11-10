//! Constructors for strict credential and direct-connection requests.
use super::{
    CredentialId, EventInteger, EventIntegerError, FormAnswer, IntegrationId, IntegrationMethodId,
    credentials::*,
};
use crate::RequestField;

/// Nonnegative integer expiry bounded by the pinned credential schema.
pub type CredentialExpires = EventInteger<0, 9007199254740991>;

impl CredentialValueInput {
    /// Store a key without invoking an integration connection method.
    pub fn key(key: impl Into<String>) -> Self {
        Self::Key(Box::new(CredentialKeyInput {
            key: key.into(),
            metadata: None,
            type_: CredentialKeyInputType::Key,
        }))
    }
    /// Store a reference to an external credential method without resolving it.
    pub fn external(method_id: IntegrationMethodId) -> Self {
        Self::External(Box::new(CredentialExternalInput {
            method_id,
            metadata: None,
            type_: CredentialExternalInputType::External,
        }))
    }
    /// Store caller-supplied OAuth tokens, checking the expiry's schema bounds.
    /// This does not initiate, complete or refresh an OAuth flow.
    pub fn oauth(
        method_id: IntegrationMethodId,
        refresh: impl Into<String>,
        access: impl Into<String>,
        expires: u64,
    ) -> Result<Self, EventIntegerError> {
        Ok(Self::Oauth(Box::new(CredentialOauthInput {
            method_id,
            refresh: refresh.into(),
            access: access.into(),
            expires: expires.try_into()?,
            metadata: None,
            type_: CredentialOauthInputType::Oauth,
        })))
    }
    /// Attach arbitrary JSON object metadata, retaining an explicit empty object.
    pub fn metadata(mut self, metadata: serde_json::Map<String, serde_json::Value>) -> Self {
        match &mut self {
            Self::Key(v) => v.metadata = Some(metadata),
            Self::Oauth(v) => v.metadata = Some(metadata),
            Self::External(v) => v.metadata = Some(metadata),
        }
        self
    }
}
impl CredentialCreateRequest {
    /// Store a credential globally. Omitted activation selects the new credential.
    pub fn new(integration_id: IntegrationId, value: CredentialValueInput) -> Self {
        Self {
            integration_id,
            value,
            id: None,
            label: None,
            activate: None,
        }
    }
    /// Supply an ID. Existing IDs conflict; this does not make creation retry-safe.
    pub fn id(mut self, id: CredentialId) -> Self {
        self.id = Some(id);
        self
    }
    /// Supply a label, preserving an explicit empty string.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// False preserves an existing selection; the first credential still activates.
    pub fn activate(mut self, activate: bool) -> Self {
        self.activate = Some(activate);
        self
    }
}
impl CredentialUpdateRequest {
    /// Replace only the label, preserving an explicit empty string.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}
impl IntegrationConnectKeyRequest {
    /// Run the integration's key method with omitted answers and label.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            answer: RequestField::Omitted,
            label: RequestField::Omitted,
        }
    }
    /// Supply typed form answers. An empty map remains explicit.
    pub fn answer(mut self, answer: FormAnswer) -> Self {
        self.answer = RequestField::Value(answer);
        self
    }
    /// Supply a label instead of the pinned server's generated default.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = RequestField::Value(label.into());
        self
    }
}
impl IntegrationConnectExternalRequest {
    /// Run the named external method with omitted answers and label.
    pub fn new(method_id: IntegrationMethodId) -> Self {
        Self {
            method_id,
            answer: RequestField::Omitted,
            label: RequestField::Omitted,
        }
    }
    /// Supply typed form answers. The server validates method-specific fields.
    pub fn answer(mut self, answer: FormAnswer) -> Self {
        self.answer = RequestField::Value(answer);
        self
    }
    /// Supply a label instead of the pinned server's generated default.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = RequestField::Value(label.into());
        self
    }
}
