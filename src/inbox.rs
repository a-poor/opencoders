use crate::{
    Client, Result, StatusCode,
    models::{SessionId, SessionInboxResponse, SessionInboxUpdateRequest, SessionMessageId},
};

/// Pending session work, obtained from [`crate::Sessions::inbox`].
///
/// Uses session/item IDs encoded as individual path segments and ignores the
/// client's discovery location default. Calls never automatically paginate or retry.
#[derive(Clone, Copy, Debug)]
pub struct SessionInbox<'a> {
    pub(crate) client: &'a Client,
    pub(crate) session_id: &'a SessionId,
}

impl SessionInbox<'_> {
    /// List pending work (`session.inbox.list`), retaining the data envelope and
    /// server enqueue order. Includes user, synthetic, compaction, and move items.
    /// This is a snapshot, not message history or an execution-completion signal.
    ///
    /// # Errors
    /// Returns transport, HTTP/API, decoding, or response-size errors. Only HTTP 200 is accepted.
    pub async fn list(&self) -> Result<SessionInboxResponse> {
        self.client
            .get_json_segments(
                "api/session",
                &[self.session_id.as_str(), "inbox"],
                "session.inbox.list",
                StatusCode::OK,
                &[],
            )
            .await
    }

    /// Change a pending item's delivery mode (`session.inbox.update`). This does
    /// not edit its payload. `steer` wakes execution on v2.2.5; `queue` does not
    /// wake it, but also does not pause an already running loop.
    ///
    /// # Errors
    /// Returns encoding, transport, HTTP/API, or response-size errors. Only HTTP
    /// 204 is accepted. Unavailable items and requests for the already recorded
    /// mode expose [`crate::ApiError::Conflict`] on v2.2.5.
    /// Changes may have applied before a failure. Default transport retries are disabled.
    pub async fn update(
        &self,
        id: &SessionMessageId,
        request: &SessionInboxUpdateRequest,
    ) -> Result<()> {
        self.client
            .patch_json_empty(
                "api/session",
                &[self.session_id.as_str(), "inbox", id.as_str()],
                "session.inbox.update",
                request,
            )
            .await
    }

    /// Cancel pending work (`session.inbox.cancel`), accepting HTTP 204. Missing
    /// or already unavailable items are no-ops on v2.2.5; missing sessions fail.
    /// Cancellation does not interrupt work that has already been delivered.
    ///
    /// # Errors
    /// Returns transport, HTTP/API, or response-size errors. Cancellation may have
    /// applied before a failure. Default transport retries are disabled; injected
    /// client retry/redirect policies remain in effect for all mutations.
    pub async fn cancel(&self, id: &SessionMessageId) -> Result<()> {
        self.client
            .delete_empty(
                "api/session",
                &[self.session_id.as_str(), "inbox", id.as_str()],
                "session.inbox.cancel",
            )
            .await
    }
}
