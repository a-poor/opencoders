use crate::{Client, Location, Result, Sessions, StatusCode, models::*};

/// Pending form discovery. Applications own rendering and explicit answers.
#[derive(Clone, Copy, Debug)]
pub struct Forms<'a> {
    pub(crate) client: &'a Client,
}
impl Client {
    /// Discover pending forms at a location.
    pub fn forms(&self) -> Forms<'_> {
        Forms { client: self }
    }
}
impl Forms<'_> {
    /// List pending forms at the client's default location (`form.list`).
    /// Preserves the location/data envelope and server order. This is an
    /// in-memory snapshot, not durable history; forms can settle after discovery.
    pub async fn list(&self) -> Result<FormListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// Override discovery location for this call. `Unspecified` omits the query
    /// without removing caller-supplied headers.
    pub async fn list_at(&self, location: Location) -> Result<FormListResponse> {
        self.client
            .get_json_at("api/form", "form.list", StatusCode::OK, &location)
            .await
    }
}

/// Forms owned by a session, resolved using the session's server location.
///
/// IDs are encoded as individual path segments; discovery location defaults are
/// ignored. Calls use ordinary request deadlines and bounded diagnostics.
/// Failures may be transport, HTTP/API, decoding, or response-size errors;
/// writes can also fail encoding. Mutations are never automatically retried.
#[derive(Clone, Copy, Debug)]
pub struct SessionForms<'a> {
    pub(crate) client: &'a Client,
    pub(crate) session_id: &'a SessionId,
}
impl<'a> Sessions<'a> {
    /// Create, inspect, answer, and cancel forms for this session.
    pub fn forms(&self, id: &'a SessionId) -> SessionForms<'a> {
        SessionForms {
            client: self.client,
            session_id: id,
        }
    }
}

/// Optional cancellation feedback, encoded in the DELETE query, never a body.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SessionFormCancelOptions {
    /// Omitted by default. An empty string is sent explicitly; null is unsupported.
    pub message: Option<String>,
}
impl SessionFormCancelOptions {
    /// Supply cancellation feedback, including an explicit empty string.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl SessionForms<'_> {
    /// List pending forms (`session.form.list`), preserving the response envelope.
    pub async fn list(&self) -> Result<SessionFormListResponse> {
        self.client
            .get_json_segments(
                "api/session",
                &[self.session_id.as_str(), "form"],
                "session.form.list",
                StatusCode::OK,
                &[],
            )
            .await
    }
    /// Get form fields and state (`session.form.get`). On v2.2.5 terminal states
    /// remain available for ten minutes in memory. Missing/expired/wrong-session
    /// IDs expose [`crate::ApiError::FormNotFound`]; restart/location teardown
    /// loses forms. Session existence is checked separately by the server.
    pub async fn get(&self, id: &FormId) -> Result<SessionFormResponse> {
        self.client
            .get_json_segments(
                "api/session",
                &[self.session_id.as_str(), "form", id.as_str()],
                "session.form.get",
                StatusCode::OK,
                &[],
            )
            .await
    }
    /// Create a pending form (`session.form.create`); returns immediately without
    /// starting a model call or waiting for an answer. Null ID behaves like
    /// omission on v2.2.5. Duplicate IDs, including retained terminal forms, are
    /// conflicts; caller-provided IDs do not make creation idempotent.
    ///
    /// The server validates duplicate keys and conditions on earlier fields.
    /// Applications own presentation, defaults, and answer collection. A failed
    /// response does not prove creation failed; inspect state before retrying.
    pub async fn create(
        &self,
        request: &SessionFormCreateRequest,
    ) -> Result<SessionFormCreateResponse> {
        self.client
            .post_json_segments(
                "api/session",
                &[self.session_id.as_str(), "form"],
                "session.form.create",
                request,
            )
            .await
    }
    /// Answer a pending form (`session.form.reply`), accepting HTTP 204.
    ///
    /// The server validates types, required/active fields, constraints, and options.
    /// Defaults are not implicit answers; external fields require `true`.
    /// Invalid answers leave the form pending. Settled forms expose
    /// [`crate::ApiError::FormAlreadySettled`]. Reply may unblock server work;
    /// successful acceptance does not prove that work succeeded. Failures or
    /// dropping this future cannot undo an accepted answer. No automatic retry.
    pub async fn reply(&self, id: &FormId, request: &SessionFormReplyRequest) -> Result<()> {
        self.client
            .post_json_empty(
                "api/session",
                &[self.session_id.as_str(), "form", id.as_str(), "reply"],
                "session.form.reply",
                request,
            )
            .await
    }
    /// Cancel a pending form (`session.form.cancel`), accepting HTTP 204.
    /// Feedback is retained in terminal state; v2.2.5 cancellation events omit it.
    /// Repeated cancellation or cancellation after answering is a settled-form
    /// conflict. No automatic retry; failures can follow an accepted cancellation.
    pub async fn cancel(&self, id: &FormId, options: &SessionFormCancelOptions) -> Result<()> {
        let query = options
            .message
            .as_ref()
            .map(|m| vec![("message".to_owned(), m.clone())])
            .unwrap_or_default();
        self.client
            .delete_empty_with_query(
                "api/session",
                &[self.session_id.as_str(), "form", id.as_str()],
                "session.form.cancel",
                &query,
            )
            .await
    }
}
