use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use std::fmt;

/// Failure to construct a client. Input URLs and header values are not formatted.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BuildError {
    /// The URL is not HTTP(S), has no host, or contains credentials/query/fragment.
    #[error("base URL must be an absolute HTTP(S) URL without credentials, query, or fragment")]
    InvalidBaseUrl,
    /// An authentication header cannot be encoded as an HTTP header value.
    #[error("invalid authentication header value")]
    InvalidHeader(#[source] reqwest::header::InvalidHeaderValue),
    /// Connection configuration belongs to the supplied reqwest client.
    #[error("configure connect_timeout on the supplied HTTP client, not on both builders")]
    ConnectTimeoutWithHttpClient,
    /// The default HTTP transport could not be initialized.
    #[error("could not initialize HTTP client")]
    HttpClient(#[source] reqwest::Error),
}

/// API-operation failures. Default formatting omits response bodies and sources.
///
/// Explicit diagnostic accessors and [`std::error::Error::source`] may expose
/// server-supplied text. Treat that data as potentially sensitive.
#[derive(thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Sending the request or reading its body failed.
    #[error(transparent)]
    Transport(TransportError),
    /// The response status is not the operation's documented success status.
    #[error(transparent)]
    Http(Box<HttpError>),
    /// The success body does not match the response model.
    #[error(transparent)]
    Decode(DecodeError),
    /// A success body exceeded the configured byte limit.
    #[error(transparent)]
    ResponseTooLarge(ResponseTooLargeError),
}

impl Error {
    /// Whether the underlying request or response-body read timed out.
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Transport(error) if error.is_timeout())
    }

    /// HTTP status, when response headers were received.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Transport(error) => error.status(),
            Self::Http(error) => Some(error.status()),
            Self::Decode(error) => Some(error.status()),
            Self::ResponseTooLarge(error) => Some(error.status()),
        }
    }

    /// Upstream operation ID, for example `server.info`.
    pub fn operation(&self) -> &'static str {
        match self {
            Self::Transport(error) => error.operation(),
            Self::Http(error) => error.operation(),
            Self::Decode(error) => error.operation(),
            Self::ResponseTooLarge(error) => error.operation(),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => error.fmt(f),
            Self::Http(error) => fmt::Debug::fmt(error, f),
            Self::Decode(error) => error.fmt(f),
            Self::ResponseTooLarge(error) => error.fmt(f),
        }
    }
}

/// A network or response-body read failure with URL-free source diagnostics.
#[derive(thiserror::Error)]
#[error("{operation}: HTTP transport failed")]
pub struct TransportError {
    operation: &'static str,
    status: Option<StatusCode>,
    #[source]
    source: reqwest::Error,
}

impl TransportError {
    pub(crate) fn new(
        operation: &'static str,
        status: Option<StatusCode>,
        source: reqwest::Error,
    ) -> Self {
        Self {
            operation,
            status,
            source: source.without_url(),
        }
    }
    /// Upstream operation ID.
    pub fn operation(&self) -> &'static str {
        self.operation
    }
    /// Status received before the failure, if any.
    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }
    /// Whether reqwest classified the failure as a timeout.
    pub fn is_timeout(&self) -> bool {
        self.source.is_timeout()
    }
}

impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportError")
            .field("operation", &self.operation)
            .field("status", &self.status)
            .field("is_timeout", &self.is_timeout())
            .finish_non_exhaustive()
    }
}

/// An unexpected HTTP response with a bounded, explicitly accessible body.
#[derive(thiserror::Error)]
#[error("{operation}: unexpected HTTP status {status}")]
pub struct HttpError {
    operation: &'static str,
    status: StatusCode,
    body: Vec<u8>,
    truncated: bool,
    api_error: Option<ApiError>,
}

impl HttpError {
    pub(crate) fn new(
        operation: &'static str,
        status: StatusCode,
        body: Vec<u8>,
        truncated: bool,
    ) -> Self {
        let api_error = if truncated {
            None
        } else {
            serde_json::from_slice::<Value>(&body)
                .ok()
                .map(ApiError::from_json)
        };
        Self {
            operation,
            status,
            body,
            truncated,
            api_error,
        }
    }
    /// Upstream operation ID.
    pub fn operation(&self) -> &'static str {
        self.operation
    }
    /// The actual response status, including unexpected redirects or 2xx statuses.
    pub fn status(&self) -> StatusCode {
        self.status
    }
    /// Raw body prefix; may contain sensitive server-supplied text.
    pub fn body(&self) -> &[u8] {
        &self.body
    }
    /// Whether the server sent more bytes than the retained prefix.
    pub fn is_body_truncated(&self) -> bool {
        self.truncated
    }
    /// Parsed error JSON, when the complete retained body is valid JSON.
    pub fn api_error(&self) -> Option<&ApiError> {
        self.api_error.as_ref()
    }
}

impl fmt::Debug for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpError")
            .field("operation", &self.operation)
            .field("status", &self.status)
            .field("body_bytes", &self.body.len())
            .field("body_truncated", &self.truncated)
            .finish_non_exhaustive()
    }
}

/// Known server-info errors, with an escape hatch for future or malformed shapes.
#[non_exhaustive]
pub enum ApiError {
    /// The request could not be validated.
    InvalidRequest {
        /// Server explanation.
        message: String,
        /// Optional validation category.
        kind: Option<String>,
        /// Optional invalid field name.
        field: Option<String>,
    },
    /// Server authentication failed.
    Unauthorized {
        /// Server explanation.
        message: String,
    },
    /// JSON that does not match a known error shape. This includes malformed known tags.
    Unrecognized(Value),
}

impl ApiError {
    fn from_json(raw: Value) -> Self {
        #[derive(Deserialize)]
        #[serde(tag = "_tag")]
        enum Known {
            InvalidRequestError {
                message: String,
                kind: Option<String>,
                field: Option<String>,
            },
            UnauthorizedError {
                message: String,
            },
        }
        match serde_json::from_value::<Known>(raw.clone()) {
            Ok(Known::InvalidRequestError {
                message,
                kind,
                field,
            }) => Self::InvalidRequest {
                message,
                kind,
                field,
            },
            Ok(Known::UnauthorizedError { message }) => Self::Unauthorized { message },
            Err(_) => Self::Unrecognized(raw),
        }
    }
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidRequest { .. } => "InvalidRequest { .. }",
            Self::Unauthorized { .. } => "Unauthorized { .. }",
            Self::Unrecognized(_) => "Unrecognized { .. }",
        })
    }
}

/// A success response that could not be decoded, with bounded diagnostic bytes.
#[derive(thiserror::Error)]
#[error("{operation}: could not decode HTTP {status} response")]
pub struct DecodeError {
    operation: &'static str,
    status: StatusCode,
    body: Vec<u8>,
    truncated: bool,
    #[source]
    source: serde_json::Error,
}

impl DecodeError {
    pub(crate) fn new(
        operation: &'static str,
        status: StatusCode,
        body: &[u8],
        limit: usize,
        source: serde_json::Error,
    ) -> Self {
        Self {
            operation,
            status,
            body: body[..body.len().min(limit)].to_vec(),
            truncated: body.len() > limit,
            source,
        }
    }
    /// Upstream operation ID.
    pub fn operation(&self) -> &'static str {
        self.operation
    }
    /// Status of the success response that failed decoding.
    pub fn status(&self) -> StatusCode {
        self.status
    }
    /// Raw body prefix; may contain sensitive server-supplied text.
    pub fn body(&self) -> &[u8] {
        &self.body
    }
    /// Whether the diagnostic body prefix is truncated.
    pub fn is_body_truncated(&self) -> bool {
        self.truncated
    }
}

impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DecodeError")
            .field("operation", &self.operation)
            .field("status", &self.status)
            .field("body_bytes", &self.body.len())
            .field("body_truncated", &self.truncated)
            .finish_non_exhaustive()
    }
}

/// A success response exceeded the configured in-memory byte limit.
#[derive(Debug, thiserror::Error)]
#[error("{operation}: HTTP {status} response exceeds the {limit}-byte limit")]
pub struct ResponseTooLargeError {
    pub(crate) operation: &'static str,
    pub(crate) status: StatusCode,
    pub(crate) limit: usize,
}

impl ResponseTooLargeError {
    /// Upstream operation ID.
    pub fn operation(&self) -> &'static str {
        self.operation
    }
    /// Response status.
    pub fn status(&self) -> StatusCode {
        self.status
    }
    /// Configured maximum number of buffered response bytes.
    pub fn limit(&self) -> usize {
        self.limit
    }
}
