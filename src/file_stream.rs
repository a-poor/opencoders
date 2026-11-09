use crate::{
    Client, Error, Files, Location, Result, StatusCode, StreamError, StreamErrorKind,
    TransportError,
};
use bytes::Bytes;
use futures_core::{Stream, stream::FusedStream};
use reqwest::{
    Method,
    header::{self, HeaderMap, HeaderValue},
};
use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

const OPERATION: &str = "fs.read";

/// Conditional/range headers and independent deadlines for a streamed file read.
/// The ordinary JSON response limit and request deadline do not apply. Injected
/// HTTP transport deadlines still apply. Headers are passed to the server;
/// malformed/multi-range syntax may result in a full HTTP 200 response.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FileReadOptions {
    /// Raw HTTP Range value, for example `bytes=0-1023`.
    pub range: Option<String>,
    /// HTTP If-Range validator.
    pub if_range: Option<String>,
    /// HTTP If-None-Match validator.
    pub if_none_match: Option<String>,
    /// HTTP If-Modified-Since date.
    pub if_modified_since: Option<String>,
    /// Deadline for response headers and bounded HTTP errors, default 30s.
    pub handshake_timeout: Option<Duration>,
    /// Maximum wait for the next body chunk; disabled by default.
    pub idle_timeout: Option<Duration>,
    /// Optional total byte budget. Streaming is unbounded in total by default;
    /// chunks are delivered only when the caller polls. The current transport
    /// chunk's allocation is outside this budget.
    pub max_bytes: Option<usize>,
}
impl Default for FileReadOptions {
    fn default() -> Self {
        Self {
            range: None,
            if_range: None,
            if_none_match: None,
            if_modified_since: None,
            handshake_timeout: Some(Duration::from_secs(30)),
            idle_timeout: None,
            max_bytes: None,
        }
    }
}
impl FileReadOptions {
    /// Set a raw Range header; inspect the returned status and Content-Range.
    pub fn range(mut self, value: impl Into<String>) -> Self {
        self.range = Some(value.into());
        self
    }
    /// Condition a range on a validator.
    pub fn if_range(mut self, value: impl Into<String>) -> Self {
        self.if_range = Some(value.into());
        self
    }
    /// Set the entity-tag condition.
    pub fn if_none_match(mut self, value: impl Into<String>) -> Self {
        self.if_none_match = Some(value.into());
        self
    }
    /// Set the modification-date condition.
    pub fn if_modified_since(mut self, value: impl Into<String>) -> Self {
        self.if_modified_since = Some(value.into());
        self
    }
    /// Set the response handshake deadline.
    pub fn handshake_timeout(mut self, value: Duration) -> Self {
        self.handshake_timeout = Some(value);
        self
    }
    /// Disable the SDK handshake deadline; injected transport policy still applies.
    pub fn without_handshake_timeout(mut self) -> Self {
        self.handshake_timeout = None;
        self
    }
    /// Set the maximum wait for the next body chunk.
    pub fn idle_timeout(mut self, value: Duration) -> Self {
        self.idle_timeout = Some(value);
        self
    }
    /// Limit the total number of bytes delivered from this response.
    pub fn max_bytes(mut self, value: usize) -> Self {
        self.max_bytes = Some(value);
        self
    }
}

impl Files<'_> {
    /// Stream a file relative to the client's default location (`fs.read`).
    /// The server enforces read confinement. Paths are encoded once, without
    /// normalization or local filesystem access. Exact `.`/`..` paths are rejected
    /// because URL segments cannot represent them verbatim.
    ///
    /// Returns 200/206 with a body, or 304/416 with headers and an empty stream.
    /// In particular, 416 is an inspectable range outcome, not an HTTP error here.
    /// Other statuses use bounded HTTP diagnostics. No retry or resume is added.
    pub async fn read(&self, path: &str, options: &FileReadOptions) -> Result<FileResponse> {
        self.read_at(path, options, self.client.default_location().clone())
            .await
    }
    /// Read a file at a per-call location. Errors and statuses match [`Self::read`].
    pub async fn read_at(
        &self,
        path: &str,
        options: &FileReadOptions,
        location: Location,
    ) -> Result<FileResponse> {
        if matches!(path, "" | "." | "..") {
            return Err(encode("file path cannot be represented as a URL segment"));
        }
        let mut url = self.client.segments_url("api/fs/read", &[path]);
        let query = location.query();
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/octet-stream"),
        );
        for (key, value) in [
            (header::RANGE, &options.range),
            (header::IF_RANGE, &options.if_range),
            (header::IF_NONE_MATCH, &options.if_none_match),
            (header::IF_MODIFIED_SINCE, &options.if_modified_since),
        ] {
            if let Some(value) = value {
                headers.insert(
                    key,
                    HeaderValue::from_str(value)
                        .map_err(|_| encode("invalid file request header"))?,
                );
            }
        }
        let response = self
            .client
            .open_stream_response(
                self.client.request(Method::GET, url).headers(headers),
                OPERATION,
                &[
                    StatusCode::OK,
                    StatusCode::PARTIAL_CONTENT,
                    StatusCode::NOT_MODIFIED,
                    StatusCode::RANGE_NOT_SATISFIABLE,
                ],
                options.handshake_timeout,
            )
            .await?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = if matches!(
            status,
            StatusCode::NOT_MODIFIED | StatusCode::RANGE_NOT_SATISFIABLE
        ) {
            FileStream { inner: None }
        } else {
            FileStream::new(response, options)
        };
        Ok(FileResponse {
            status,
            headers,
            body,
        })
    }
}

fn encode(message: &str) -> Error {
    Error::Encode(crate::EncodeError {
        operation: OPERATION,
        source: serde::ser::Error::custom(message),
    })
}

/// File status and headers together with a caller-driven byte stream.
/// Metadata remains available after EOF or an error. Debug omits headers/body.
pub struct FileResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: FileStream,
}
impl FileResponse {
    /// Exact HTTP status: 200, 206, 304, or 416.
    pub fn status(&self) -> StatusCode {
        self.status
    }
    /// All received HTTP headers, including validators, MIME and range metadata.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    /// Borrow the byte stream; a read error terminates it.
    pub fn body_mut(&mut self) -> &mut FileStream {
        &mut self.body
    }
    /// Take ownership of the byte stream, discarding the response metadata.
    pub fn into_body(self) -> FileStream {
        self.body
    }
}
impl fmt::Debug for FileResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileResponse")
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>;
/// A streamed file body, without a background task or automatic buffering.
/// Drop/close releases the connection. Cancelling `next()` preserves stream state
/// and an already-started idle deadline. Errors are emitted once, then EOF.
pub struct FileStream {
    inner: Option<ByteStream>,
}
impl FileStream {
    fn new(response: reqwest::Response, options: &FileReadOptions) -> Self {
        let status = response.status();
        let idle = options.idle_timeout;
        let budget = options.max_bytes;
        let inner = futures_util::stream::try_unfold(
            (response, budget),
            move |(mut response, remaining)| async move {
                let chunk = if let Some(timeout) = idle {
                    tokio::time::timeout(timeout, response.chunk())
                        .await
                        .map_err(|_| {
                            Error::Stream(StreamError::new(
                                OPERATION,
                                StreamErrorKind::IdleTimeout,
                                Some(status),
                            ))
                        })?
                } else {
                    response.chunk().await
                };
                let chunk = chunk.map_err(|error| {
                    Error::Transport(TransportError::new(OPERATION, Some(status), error))
                })?;
                let Some(chunk) = chunk else { return Ok(None) };
                let remaining = if let Some(remaining) = remaining {
                    Some(
                        remaining
                            .checked_sub(chunk.len())
                            .ok_or(Error::ResponseTooLarge(crate::ResponseTooLargeError {
                                operation: OPERATION,
                                status,
                                limit: budget.unwrap(),
                            }))?,
                    )
                } else {
                    None
                };
                Ok(Some((chunk, (response, remaining))))
            },
        );
        Self {
            inner: Some(Box::pin(inner)),
        }
    }
    /// Read the next byte chunk. Chunk boundaries have no file-format meaning.
    pub async fn next(&mut self) -> Option<Result<Bytes>> {
        futures_util::StreamExt::next(self).await
    }
    /// Release the connection immediately. Subsequent polls return EOF.
    pub fn close(&mut self) {
        self.inner = None;
    }
}
impl Stream for FileStream {
    type Item = Result<Bytes>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let Some(inner) = this.inner.as_mut() else {
            return Poll::Ready(None);
        };
        let result = inner.as_mut().poll_next(cx);
        if matches!(result, Poll::Ready(None | Some(Err(_)))) {
            this.inner = None;
        }
        result
    }
}
impl FusedStream for FileStream {
    fn is_terminated(&self) -> bool {
        self.inner.is_none()
    }
}
impl fmt::Debug for FileStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileStream")
            .field("closed", &self.inner.is_none())
            .finish_non_exhaustive()
    }
}

impl Client {
    pub(crate) async fn open_stream_response(
        &self,
        request: reqwest::RequestBuilder,
        operation: &'static str,
        allowed: &[StatusCode],
        timeout: Option<Duration>,
    ) -> Result<reqwest::Response> {
        let mut status = None;
        let handshake = async {
            let response = request
                .send()
                .await
                .map_err(|error| Error::Transport(TransportError::new(operation, None, error)))?;
            status = Some(response.status());
            if !allowed.contains(&response.status()) {
                return Err(self.stream_http_error(response, operation).await);
            }
            Ok(response)
        };
        if let Some(timeout) = timeout {
            tokio::time::timeout(timeout, handshake)
                .await
                .map_err(|_| {
                    Error::Stream(StreamError::new(
                        operation,
                        StreamErrorKind::HandshakeTimeout,
                        status,
                    ))
                })?
        } else {
            handshake.await
        }
    }
}
