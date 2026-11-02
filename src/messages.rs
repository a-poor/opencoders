use crate::{
    Client, Result, StatusCode,
    models::{
        SessionId, SessionMessageId, SessionMessageResponse, SessionMessageType,
        SessionMessagesResponse,
    },
};

/// Timeline ordering for the first message page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MessageOrder {
    /// Oldest messages first.
    Asc,
    /// Newest messages first.
    Desc,
}

/// Explicit message-list filters. The default omits every query parameter.
///
/// Omit `order` when following a cursor, and retain the same `message_type`.
/// Setting a cursor does not silently change any other option.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct MessageListOptions {
    /// Requested page size. The server validates its range; v2.2.5 defaults to 50.
    pub limit: Option<u32>,
    /// First-page ordering; v2.2.5 defaults to descending.
    pub order: Option<MessageOrder>,
    /// Opaque previous/next cursor. An empty string is sent unchanged.
    pub cursor: Option<String>,
    /// Filter applied before pagination, encoded as `type`.
    pub message_type: Option<SessionMessageType>,
}
impl MessageListOptions {
    /// Set the requested page size.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    /// Set the first page's order.
    pub fn order(mut self, order: MessageOrder) -> Self {
        self.order = Some(order);
        self
    }
    /// Set a cursor without changing the other options.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
    /// Set the message-kind filter; repeat it on subsequent pages.
    pub fn message_type(mut self, message_type: SessionMessageType) -> Self {
        self.message_type = Some(message_type);
        self
    }
    fn query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(order) = self.order {
            query.push((
                "order",
                match order {
                    MessageOrder::Asc => "asc",
                    MessageOrder::Desc => "desc",
                }
                .into(),
            ));
        }
        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor.clone()));
        }
        if let Some(kind) = self.message_type {
            query.push(("type", kind.to_string()));
        }
        query
    }
}

/// Projected message history for one session, from [`crate::Sessions::messages`].
#[derive(Clone, Copy, Debug)]
pub struct SessionMessages<'a> {
    pub(crate) client: &'a Client,
    pub(crate) session_id: &'a SessionId,
}
impl SessionMessages<'_> {
    /// Retrieve one page with `session.message.list`.
    ///
    /// Preserves the data/cursor envelope and server order. No automatic pagination,
    /// retries, or cursor interpretation occur. The discovery location is ignored.
    /// In v2.2.5, combining a nonempty cursor with order returns an invalid-cursor
    /// error. Keep the same type filter; boundary cursors may lead to empty pages.
    ///
    /// # Errors
    /// Returns transport, unexpected HTTP status, decoding, or response-size errors.
    /// Only HTTP 200 is accepted. Invalid cursors and missing sessions expose typed errors.
    pub async fn list(&self, options: &MessageListOptions) -> Result<SessionMessagesResponse> {
        self.client
            .get_json_segments(
                "api/session",
                &[self.session_id.as_str(), "message"],
                "session.message.list",
                StatusCode::OK,
                &options.query(),
            )
            .await
    }
    /// Retrieve one projected message with `session.message.get`.
    ///
    /// Both IDs are encoded as individual path segments. Preserves the `data`
    /// envelope and additive fields; ignores the client's discovery location.
    ///
    /// # Errors
    /// Returns transport, unexpected HTTP status, decoding, or response-size errors.
    /// Only HTTP 200 is accepted. Missing messages expose [`crate::ApiError::MessageNotFound`];
    /// missing sessions expose [`crate::ApiError::SessionNotFound`].
    pub async fn get(&self, id: &SessionMessageId) -> Result<SessionMessageResponse> {
        self.client
            .get_json_segments(
                "api/session",
                &[self.session_id.as_str(), "message", id.as_str()],
                "session.message.get",
                StatusCode::OK,
                &[],
            )
            .await
    }
}
