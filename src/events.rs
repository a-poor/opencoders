use crate::{
    Client, DecodeError, Error, Result, StatusCode, StreamError, StreamErrorKind, TransportError,
    models::Event, sse::Decoder,
};
use futures_core::{Stream, stream::FusedStream};
use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub(crate) const OPERATION: &str = "event.subscribe";

/// Options for one global, live-only event subscription.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EventOptions {
    /// Repeated `types` query values. Empty means all; initial connected events
    /// are sent regardless of filters. Strings permit future and RPC kinds.
    pub types: Vec<String>,
    /// Time to receive headers (and an HTTP error body), default 30 seconds.
    pub handshake_timeout: Option<Duration>,
    /// Maximum wait for the next body chunk, disabled by default. Heartbeats
    /// count as activity. A deadline starts when a body read begins and remains
    /// active if a pending `next()` call is cancelled.
    pub idle_timeout: Option<Duration>,
    /// Maximum bytes in an SSE block, default 8 MiB. Includes ignored fields
    /// and comments, counts CRLF as one terminator, and resets at each blank line.
    /// UTF-8 replacement may expand decoded strings by up to three times.
    /// The HTTP transport's current chunk allocation is outside this limit.
    pub max_event_bytes: usize,
}
impl Default for EventOptions {
    fn default() -> Self {
        Self {
            types: Vec::new(),
            handshake_timeout: Some(Duration::from_secs(30)),
            idle_timeout: None,
            max_event_bytes: 8 * 1024 * 1024,
        }
    }
}
impl EventOptions {
    /// Replace the event filters, preserving order and duplicates.
    pub fn types(mut self, types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.types = types.into_iter().map(Into::into).collect();
        self
    }
    /// Set the subscription handshake deadline.
    pub fn handshake_timeout(mut self, value: Duration) -> Self {
        self.handshake_timeout = Some(value);
        self
    }
    /// Disable the SDK handshake deadline; transport deadlines still apply.
    pub fn without_handshake_timeout(mut self) -> Self {
        self.handshake_timeout = None;
        self
    }
    /// Set a body inactivity deadline; comments and heartbeats reset it.
    pub fn idle_timeout(mut self, value: Duration) -> Self {
        self.idle_timeout = Some(value);
        self
    }
    /// Set the per-block byte budget, including ignored SSE fields.
    pub fn max_event_bytes(mut self, value: usize) -> Self {
        self.max_event_bytes = value;
        self
    }
}

/// Event subscription resource. Subscriptions span all server locations.
#[derive(Clone, Copy, Debug)]
pub struct Events<'a> {
    pub(crate) client: &'a Client,
}
impl Events<'_> {
    /// Open an independent live connection, without replay or automatic retries.
    ///
    /// The client's ordinary request timeout and response size limit do not
    /// apply. Injected reqwest client deadlines and policies still apply.
    /// No `Last-Event-ID` is synthesized. Reopening can miss or duplicate events;
    /// applications must refresh authoritative state after a disconnect.
    pub async fn subscribe(&self, options: &EventOptions) -> Result<EventStream> {
        let (response, diagnostic_limit) = self.client.open_event_response(options).await?;
        let state = State {
            response,
            decoder: Decoder::new(options.max_event_bytes),
            pending: bytes::Bytes::new(),
            idle_timeout: options.idle_timeout,
            frame_limit: options.max_event_bytes,
            diagnostic_limit,
        };
        let inner = futures_util::stream::try_unfold(state, |mut state| async move {
            loop {
                while !state.pending.is_empty() {
                    let b = state.pending[0];
                    use bytes::Buf;
                    state.pending.advance(1);
                    if let Some(frame) = state.decoder.push(b).map_err(|()| {
                        Error::Stream(
                            StreamError::new(StreamErrorKind::FrameTooLarge, Some(StatusCode::OK))
                                .with_limit(state.frame_limit),
                        )
                    })? {
                        if frame.event == "effect/http-api/stream/failure" {
                            return Err(Error::Stream(
                                StreamError::new(
                                    StreamErrorKind::ServerFailure,
                                    Some(StatusCode::OK),
                                )
                                .with_body(frame.data.as_bytes(), state.diagnostic_limit),
                            ));
                        }
                        let event = serde_json::from_str(&frame.data).map_err(|source| {
                            Error::Decode(DecodeError::new(
                                OPERATION,
                                StatusCode::OK,
                                frame.data.as_bytes(),
                                state.diagnostic_limit,
                                source,
                            ))
                        })?;
                        let message = EventMessage {
                            event,
                            id: frame.id,
                            event_name: frame.event,
                            retry: frame.retry,
                        };
                        return Ok(Some((message, state)));
                    }
                }
                let chunk = if let Some(timeout) = state.idle_timeout {
                    tokio::time::timeout(timeout, state.response.chunk())
                        .await
                        .map_err(|_| {
                            Error::Stream(StreamError::new(
                                StreamErrorKind::IdleTimeout,
                                Some(StatusCode::OK),
                            ))
                        })?
                } else {
                    state.response.chunk().await
                };
                match chunk.map_err(|source| {
                    Error::Transport(TransportError::new(OPERATION, Some(StatusCode::OK), source))
                })? {
                    Some(chunk) => state.pending = chunk,
                    None => return Ok(None),
                }
            }
        });
        Ok(EventStream {
            inner: Some(Box::pin(inner)),
        })
    }
}
struct State {
    response: reqwest::Response,
    decoder: Decoder,
    pending: bytes::Bytes,
    idle_timeout: Option<Duration>,
    frame_limit: usize,
    diagnostic_limit: usize,
}

/// Decoded payload and independent SSE framing metadata. Debug includes payloads.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EventMessage {
    /// Typed event or complete JSON for an unimplemented/future kind.
    pub event: Event,
    /// Last valid SSE `id`; unrelated to the JSON event's ID or durable sequence.
    /// `Some("")` means an explicit reset. The pinned server usually omits SSE IDs.
    pub id: Option<String>,
    /// SSE event name, default `message`; unrelated to the JSON `type`.
    pub event_name: String,
    /// Latest valid SSE retry hint. Retained as metadata only; never acted upon.
    /// Hints exceeding u64 milliseconds are ignored.
    pub retry: Option<Duration>,
}

/// Caller-driven subscription. Each instance owns a separate connection.
///
/// Clean EOF yields `None`; a failure yields one error then `None`. Both release
/// the connection. Dropping/closing cancels this subscription, not server work.
/// Cancelling a pending `next()` future leaves the subscription and parser intact.
/// No task, unbounded event queue, broadcast channel, or reconnect loop is spawned.
pub struct EventStream {
    inner: Option<Pin<Box<dyn Stream<Item = Result<EventMessage>> + Send>>>,
}
impl EventStream {
    /// Read the next event. Also implements [`Stream`] for stream combinators.
    pub async fn next(&mut self) -> Option<Result<EventMessage>> {
        futures_util::StreamExt::next(self).await
    }
    /// Release the connection immediately; subsequent reads return `None`.
    pub fn close(&mut self) {
        self.inner = None;
    }
}
impl Stream for EventStream {
    type Item = Result<EventMessage>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let Some(inner) = this.inner.as_mut() else {
            return Poll::Ready(None);
        };
        let result = inner.as_mut().poll_next(cx);
        if matches!(&result, Poll::Ready(None | Some(Err(_)))) {
            this.inner = None;
        }
        result
    }
}
impl FusedStream for EventStream {
    fn is_terminated(&self) -> bool {
        self.inner.is_none()
    }
}
impl fmt::Debug for EventStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventStream")
            .field("closed", &self.inner.is_none())
            .finish_non_exhaustive()
    }
}
