use crate::{
    Error, Location, Ptys, Result, StatusCode, StreamError, StreamErrorKind, TransportError,
    models::PtyId,
};
use futures_core::{Stream, stream::FusedStream};
use futures_util::{Sink, SinkExt, StreamExt};
use reqwest::{
    Method, Version,
    header::{self, HeaderMap, HeaderValue},
};
use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::{
        self, Message,
        protocol::{Role, WebSocketConfig},
    },
};

const OPERATION: &str = "pty.connect";
type Socket = WebSocketStream<reqwest::Upgraded>;

/// Options for an explicit ordinary PTY WebSocket connection.
#[derive(Clone)]
#[non_exhaustive]
pub struct PtyConnectOptions {
    /// Optional single-use ticket. Otherwise configured HTTP authentication is
    /// used. A ticket must match the resolved location and PTY used to mint it.
    pub ticket: Option<String>,
    /// Absolute UTF-16 code-unit cursor. Omission replays retained output; -1
    /// tails from the current end. Valid range is -1 through 2^53-1.
    pub cursor: Option<i64>,
    /// Deadline for the entire HTTP upgrade, default 30 seconds.
    pub handshake_timeout: Option<Duration>,
    /// Optional inactivity deadline while polling output, disabled by default.
    pub idle_timeout: Option<Duration>,
    /// Maximum incoming message/frame bytes and outgoing UTF-8 message bytes,
    /// default 8 MiB. The transport also uses fixed read/write buffers.
    pub max_message_bytes: usize,
}
impl Default for PtyConnectOptions {
    fn default() -> Self {
        Self {
            ticket: None,
            cursor: None,
            handshake_timeout: Some(Duration::from_secs(30)),
            idle_timeout: None,
            max_message_bytes: 8 * 1024 * 1024,
        }
    }
}
impl PtyConnectOptions {
    /// Supply a single-use ticket; no ticket is minted or retried implicitly.
    pub fn ticket(mut self, value: impl Into<String>) -> Self {
        self.ticket = Some(value.into());
        self
    }
    /// Choose an absolute output cursor or -1 for live output only.
    pub fn cursor(mut self, value: i64) -> Self {
        self.cursor = Some(value);
        self
    }
    /// Set the HTTP upgrade deadline.
    pub fn handshake_timeout(mut self, value: Duration) -> Self {
        self.handshake_timeout = Some(value);
        self
    }
    /// Disable the SDK handshake deadline; injected HTTP deadlines still apply.
    pub fn without_handshake_timeout(mut self) -> Self {
        self.handshake_timeout = None;
        self
    }
    /// Set the inactivity deadline while reading output.
    pub fn idle_timeout(mut self, value: Duration) -> Self {
        self.idle_timeout = Some(value);
        self
    }
    /// Set the per-message byte bound; zero and overflowing bounds are rejected.
    pub fn max_message_bytes(mut self, value: usize) -> Self {
        self.max_message_bytes = value;
        self
    }
}
impl fmt::Debug for PtyConnectOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PtyConnectOptions")
            .field("cursor", &self.cursor)
            .field("handshake_timeout", &self.handshake_timeout)
            .field("idle_timeout", &self.idle_timeout)
            .field("max_message_bytes", &self.max_message_bytes)
            .finish_non_exhaustive()
    }
}

/// A PTY transport or framing failure. Payloads, URLs and ticket values are omitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PtySocketErrorKind {
    /// HTTP 101 headers did not validate as the requested WebSocket upgrade.
    InvalidHandshake,
    /// A socket read/write or WebSocket protocol operation failed.
    Transport,
    /// A binary control message did not match the reviewed cursor protocol.
    InvalidMessage,
    /// An incoming/outgoing message exceeded its byte bound.
    MessageTooLarge,
    /// No activity arrived before the configured read deadline.
    IdleTimeout,
    /// Writing to an already closed connection was attempted.
    Closed,
}

/// Redacted PTY socket error. A socket error terminates this connection.
#[derive(Debug, thiserror::Error)]
#[error("pty.connect: socket failed ({kind:?})")]
pub struct PtySocketError {
    kind: PtySocketErrorKind,
}
impl PtySocketError {
    /// Failure category, without terminal data or authentication material.
    pub fn kind(&self) -> PtySocketErrorKind {
        self.kind
    }
}
fn failure(kind: PtySocketErrorKind) -> Error {
    Error::PtySocket(PtySocketError { kind })
}
fn socket_error(error: tungstenite::Error) -> Error {
    failure(match error {
        tungstenite::Error::Capacity(_) | tungstenite::Error::WriteBufferFull(_) => {
            PtySocketErrorKind::MessageTooLarge
        }
        tungstenite::Error::AlreadyClosed | tungstenite::Error::ConnectionClosed => {
            PtySocketErrorKind::Closed
        }
        _ => PtySocketErrorKind::Transport,
    })
}
fn encode(message: &str) -> Error {
    Error::Encode(crate::EncodeError {
        operation: OPERATION,
        source: serde::ser::Error::custom(message),
    })
}

impl Ptys<'_> {
    /// Open an independent PTY WebSocket using configured HTTP transport policies.
    /// Requires the `websocket` feature. Ordinary JSON deadlines do not apply;
    /// options bound the upgrade and idle reads. No background reader, reconnect,
    /// ticket minting, or input retransmission is added.
    pub async fn connect(&self, id: &PtyId, options: &PtyConnectOptions) -> Result<PtyConnection> {
        self.connect_at(id, options, self.client.default_location().clone())
            .await
    }
    /// Connect at a per-call location. With a ticket, use its resolved directory.
    /// The schema's boolean/200 success description is not the wire protocol:
    /// this method validates an HTTP/1.1 101 WebSocket upgrade.
    pub async fn connect_at(
        &self,
        id: &PtyId,
        options: &PtyConnectOptions,
        location: Location,
    ) -> Result<PtyConnection> {
        if options
            .cursor
            .is_some_and(|value| !(-1..=9_007_199_254_740_991).contains(&value))
        {
            return Err(encode("invalid PTY replay cursor"));
        }
        let write_limit = options
            .max_message_bytes
            .checked_add(1024)
            .filter(|_| options.max_message_bytes > 0)
            .ok_or_else(|| encode("invalid PTY message limit"))?;
        let mut url = self.url(Some(id), Some("connect"), &location);
        if let Some(cursor) = options.cursor {
            url.query_pairs_mut()
                .append_pair("cursor", &cursor.to_string());
        }
        if let Some(ticket) = &options.ticket {
            url.query_pairs_mut().append_pair("ticket", ticket);
        }
        let key = tungstenite::handshake::client::generate_key();
        let expected = tungstenite::handshake::derive_accept_key(key.as_bytes());
        let headers = HeaderMap::from_iter([
            (header::CONNECTION, HeaderValue::from_static("Upgrade")),
            (header::UPGRADE, HeaderValue::from_static("websocket")),
            (
                header::SEC_WEBSOCKET_VERSION,
                HeaderValue::from_static("13"),
            ),
            (
                header::SEC_WEBSOCKET_KEY,
                HeaderValue::from_str(&key).expect("generated ASCII key"),
            ),
        ]);
        let request = self
            .client
            .request(Method::GET, url)
            .version(Version::HTTP_11)
            .headers(headers);
        let mut status = None;
        let handshake = async {
            let response = request
                .send()
                .await
                .map_err(|error| Error::Transport(TransportError::new(OPERATION, None, error)))?;
            status = Some(response.status());
            if response.status() != StatusCode::SWITCHING_PROTOCOLS {
                return Err(self.client.stream_http_error(response, OPERATION).await);
            }
            let headers = response.headers();
            let token = |name: &str, value: &str| {
                headers
                    .get_all(name)
                    .iter()
                    .filter_map(|h| h.to_str().ok())
                    .flat_map(|h| h.split(','))
                    .any(|item| item.trim().eq_ignore_ascii_case(value))
            };
            if !token("connection", "upgrade")
                || !token("upgrade", "websocket")
                || headers.get_all(header::SEC_WEBSOCKET_ACCEPT).iter().count() != 1
                || headers
                    .get(header::SEC_WEBSOCKET_ACCEPT)
                    .and_then(|h| h.to_str().ok())
                    != Some(expected.as_str())
                || headers.contains_key(header::SEC_WEBSOCKET_PROTOCOL)
                || headers.contains_key(header::SEC_WEBSOCKET_EXTENSIONS)
            {
                return Err(failure(PtySocketErrorKind::InvalidHandshake));
            }
            let stream = response.upgrade().await.map_err(|error| {
                Error::Transport(TransportError::new(
                    OPERATION,
                    Some(StatusCode::SWITCHING_PROTOCOLS),
                    error,
                ))
            })?;
            let config = WebSocketConfig::default()
                .read_buffer_size(16 * 1024)
                .write_buffer_size(0)
                .max_write_buffer_size(write_limit)
                .max_message_size(Some(options.max_message_bytes))
                .max_frame_size(Some(options.max_message_bytes));
            let socket = WebSocketStream::from_raw_socket(stream, Role::Client, Some(config)).await;
            Ok(PtyConnection {
                socket: Some(socket),
                idle_timeout: options.idle_timeout,
                idle_deadline: None,
                pending_close: None,
                max_message_bytes: options.max_message_bytes,
            })
        };
        if let Some(timeout) = options.handshake_timeout {
            tokio::time::timeout(timeout, handshake)
                .await
                .map_err(|_| {
                    Error::Stream(StreamError::new(
                        OPERATION,
                        StreamErrorKind::HandshakeTimeout,
                        status,
                    ))
                })?
        } else {
            handshake.await
        }
    }
}

/// Terminal data or protocol metadata. Debug includes terminal text/close reasons.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PtyOutput {
    /// Raw terminal text; parsing ANSI and rendering remain application-owned.
    Data(String),
    /// Absolute output cursor after initial replay. For later live text, advance
    /// by UTF-16 code units (`text.encode_utf16().count()`), not UTF-8 bytes.
    /// Retention is bounded; the protocol does not signal a lost replay prefix.
    Cursor(u64),
    /// Peer close metadata. The process exit code must be obtained via `pty.get`.
    Closed {
        /// WebSocket close code, if supplied by the peer.
        code: Option<u16>,
        /// Peer-provided close reason.
        reason: String,
    },
}

/// Caller-driven PTY socket implementing `Stream` and `Sink<String>`.
/// Reading is cancellation-safe, including an already-started idle deadline.
/// Writes are not cancellation-safe: timeout/drop can leave partial delivery;
/// never replay input blindly. Dropping the socket detaches without terminating
/// the PTY. Use `Ptys::remove` to terminate the server process.
pub struct PtyConnection {
    socket: Option<Socket>,
    idle_timeout: Option<Duration>,
    idle_deadline: Option<Pin<Box<tokio::time::Sleep>>>,
    pending_close: Option<PtyOutput>,
    max_message_bytes: usize,
}
/// Writable half of a split PTY connection. Use `SinkExt` to send/flush/close.
pub type PtyWriter = futures_util::stream::SplitSink<PtyConnection, String>;
/// Readable half of a split PTY connection. Use `StreamExt` to poll output.
pub type PtyReader = futures_util::stream::SplitStream<PtyConnection>;
impl PtyConnection {
    /// Read the next terminal message. Errors are emitted once, then EOF.
    pub async fn next(&mut self) -> Option<Result<PtyOutput>> {
        StreamExt::next(self).await
    }
    /// Send and flush UTF-8 terminal input. This never retries on failure.
    /// Callers may impose a deadline, but cancelled writes have ambiguous delivery.
    pub async fn send(&mut self, text: impl Into<String>) -> Result<()> {
        SinkExt::send(self, text.into()).await
    }
    /// Send a close frame and release this connection. Does not kill the PTY.
    /// Polling this future drives the close write; it does not await a peer reply.
    pub async fn close(&mut self) -> Result<()> {
        SinkExt::close(self).await
    }
    /// Release the socket immediately without attempting a close handshake.
    pub fn disconnect(&mut self) {
        self.socket = None;
        self.idle_deadline = None;
        self.pending_close = None;
    }
    /// Split reading/writing for concurrent application tasks; creates no tasks.
    pub fn split(self) -> (PtyWriter, PtyReader) {
        StreamExt::split(self)
    }
    fn fail(&mut self, error: Error) -> Poll<Option<Result<PtyOutput>>> {
        self.disconnect();
        Poll::Ready(Some(Err(error)))
    }
    fn poll_deadline(&mut self, cx: &mut Context<'_>) -> bool {
        if self.idle_deadline.is_none() {
            self.idle_deadline = self
                .idle_timeout
                .map(|timeout| Box::pin(tokio::time::sleep(timeout)));
        }
        self.idle_deadline
            .as_mut()
            .is_some_and(|sleep| sleep.as_mut().poll(cx).is_ready())
    }
}
impl fmt::Debug for PtyConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PtyConnection")
            .field("closed", &self.socket.is_none())
            .finish_non_exhaustive()
    }
}
impl Stream for PtyConnection {
    type Item = Result<PtyOutput>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.socket.is_none() {
            return Poll::Ready(None);
        }
        if this.poll_deadline(cx) {
            return this.fail(failure(PtySocketErrorKind::IdleTimeout));
        }
        for _ in 0..32 {
            if this.pending_close.is_some() {
                match Pin::new(this.socket.as_mut().unwrap()).poll_flush(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(_) => {
                        let event = this.pending_close.take().unwrap();
                        this.disconnect();
                        return Poll::Ready(Some(Ok(event)));
                    }
                }
            }
            let message = match Pin::new(this.socket.as_mut().unwrap()).poll_next(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => {
                    this.disconnect();
                    return Poll::Ready(None);
                }
                Poll::Ready(Some(Err(error))) => return this.fail(socket_error(error)),
                Poll::Ready(Some(Ok(message))) => message,
            };
            this.idle_deadline = None;
            let output = match message {
                Message::Text(text) => PtyOutput::Data(text.to_string()),
                Message::Binary(bytes) => {
                    #[derive(serde::Deserialize)]
                    struct Marker {
                        cursor: u64,
                    }
                    let marker = bytes
                        .strip_prefix(&[0])
                        .and_then(|data| serde_json::from_slice::<Marker>(data).ok())
                        .filter(|marker| marker.cursor <= 9_007_199_254_740_991);
                    let Some(marker) = marker else {
                        return this.fail(failure(PtySocketErrorKind::InvalidMessage));
                    };
                    PtyOutput::Cursor(marker.cursor)
                }
                Message::Close(frame) => {
                    this.pending_close = Some(PtyOutput::Closed {
                        code: frame.as_ref().map(|f| u16::from(f.code)),
                        reason: frame.map(|f| f.reason.to_string()).unwrap_or_default(),
                    });
                    this.poll_deadline(cx);
                    continue;
                }
                Message::Ping(_) | Message::Pong(_) => {
                    this.poll_deadline(cx);
                    continue;
                }
                Message::Frame(_) => return this.fail(failure(PtySocketErrorKind::InvalidMessage)),
            };
            return Poll::Ready(Some(Ok(output)));
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
impl FusedStream for PtyConnection {
    fn is_terminated(&self) -> bool {
        self.socket.is_none()
    }
}
impl Sink<String> for PtyConnection {
    type Error = Error;
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();
        let Some(socket) = this.socket.as_mut() else {
            return Poll::Ready(Err(failure(PtySocketErrorKind::Closed)));
        };
        match Pin::new(socket).poll_ready(cx) {
            Poll::Ready(Err(error)) => {
                let error = socket_error(error);
                this.disconnect();
                Poll::Ready(Err(error))
            }
            result => result.map(|result| result.map_err(socket_error)),
        }
    }
    fn start_send(self: Pin<&mut Self>, text: String) -> Result<()> {
        let this = self.get_mut();
        if text.len() > this.max_message_bytes {
            this.disconnect();
            return Err(failure(PtySocketErrorKind::MessageTooLarge));
        }
        let Some(socket) = this.socket.as_mut() else {
            return Err(failure(PtySocketErrorKind::Closed));
        };
        if let Err(error) = Pin::new(socket).start_send(Message::Text(text.into())) {
            let error = socket_error(error);
            this.disconnect();
            return Err(error);
        }
        Ok(())
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();
        let Some(socket) = this.socket.as_mut() else {
            return Poll::Ready(Err(failure(PtySocketErrorKind::Closed)));
        };
        match Pin::new(socket).poll_flush(cx) {
            Poll::Ready(Err(error)) => {
                let error = socket_error(error);
                this.disconnect();
                Poll::Ready(Err(error))
            }
            result => result.map(|result| result.map_err(socket_error)),
        }
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();
        let Some(socket) = this.socket.as_mut() else {
            return Poll::Ready(Ok(()));
        };
        let result = Pin::new(socket).poll_close(cx);
        if result.is_ready() {
            this.disconnect();
        }
        result.map(|result| result.map_err(socket_error))
    }
}
