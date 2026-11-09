use crate::StatusCode;
use std::fmt;

/// A terminal failure specific to streaming responses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamErrorKind {
    /// Headers or the bounded HTTP error body did not arrive before the deadline.
    HandshakeTimeout,
    /// No response-body bytes arrived before the configured idle deadline.
    IdleTimeout,
    /// HTTP 200 did not identify a text/event-stream body.
    InvalidContentType,
    /// An SSE block exceeded its configured byte budget.
    FrameTooLarge,
    /// The server emitted an effect/http-api/stream/failure frame.
    ServerFailure,
}

/// Bounded stream diagnostics. Default formatting omits server-supplied text.
#[derive(thiserror::Error)]
#[error("{operation}: stream failed ({kind:?})")]
pub struct StreamError {
    operation: &'static str,
    kind: StreamErrorKind,
    status: Option<StatusCode>,
    limit: Option<usize>,
    body: Vec<u8>,
    truncated: bool,
}
impl StreamError {
    pub(crate) fn new(
        operation: &'static str,
        kind: StreamErrorKind,
        status: Option<StatusCode>,
    ) -> Self {
        Self {
            operation,
            kind,
            status,
            limit: None,
            body: Vec::new(),
            truncated: false,
        }
    }
    pub(crate) fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    pub(crate) fn with_body(mut self, body: &[u8], limit: usize) -> Self {
        self.body = body[..body.len().min(limit)].to_vec();
        self.truncated = body.len() > limit;
        self
    }
    /// Failure category.
    pub fn kind(&self) -> StreamErrorKind {
        self.kind
    }
    /// Received HTTP status, if headers arrived.
    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }
    /// Upstream operation ID.
    pub fn operation(&self) -> &'static str {
        self.operation
    }
    /// Configured frame limit, for an oversized block.
    pub fn limit(&self) -> Option<usize> {
        self.limit
    }
    /// Raw server failure prefix. May contain sensitive data.
    pub fn body(&self) -> &[u8] {
        &self.body
    }
    /// Whether the server failure diagnostic was truncated.
    pub fn is_body_truncated(&self) -> bool {
        self.truncated
    }
}
impl fmt::Debug for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamError")
            .field("operation", &self.operation)
            .field("kind", &self.kind)
            .field("status", &self.status)
            .field("limit", &self.limit)
            .field("body_bytes", &self.body.len())
            .field("body_truncated", &self.truncated)
            .finish_non_exhaustive()
    }
}
