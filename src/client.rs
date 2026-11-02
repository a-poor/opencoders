use crate::{
    Agents, BuildError, DecodeError, Error, HttpError, Location, Locations, ResponseTooLargeError,
    Result, Server, Sessions, TransportError,
};
use reqwest::{
    StatusCode, Url,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::de::DeserializeOwned;
use std::{fmt, sync::Arc, time::Duration};

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_RESPONSE_LIMIT: usize = 8 * 1024 * 1024;
const DEFAULT_DIAGNOSTIC_LIMIT: usize = 8 * 1024;

/// Cloneable client sharing an HTTP connection pool and immutable configuration.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    default_location: Location,
    base_url: Url,
    http: reqwest::Client,
    headers: HeaderMap,
    request_timeout: Option<Duration>,
    max_response_bytes: usize,
    max_error_body_bytes: usize,
}

impl Client {
    /// Construct a client with defaults. See [`ClientBuilder`] for configuration.
    pub fn new(base_url: impl Into<String>) -> std::result::Result<Self, BuildError> {
        Self::builder(base_url).build()
    }

    /// Configure a server base URL, optionally including a reverse-proxy prefix.
    ///
    /// For `https://host/opencode`, the server-info URL becomes
    /// `https://host/opencode/api/info`. Do not include the API's `/api` suffix.
    pub fn builder(base_url: impl Into<String>) -> ClientBuilder {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        ClientBuilder {
            default_location: Location::Unspecified,
            base_url: base_url.into(),
            http: None,
            headers,
            header_error: None,
            connect_timeout: None,
            request_timeout: Some(DEFAULT_REQUEST_TIMEOUT),
            max_response_bytes: DEFAULT_RESPONSE_LIMIT,
            max_error_body_bytes: DEFAULT_DIAGNOSTIC_LIMIT,
        }
    }

    /// Access server-wide operations.
    pub fn server(&self) -> Server<'_> {
        Server { client: self }
    }

    /// Resolve working locations on the server.
    pub fn location(&self) -> Locations<'_> {
        Locations { client: self }
    }

    /// Discover registered agents at a working location.
    pub fn agents(&self) -> Agents<'_> {
        Agents { client: self }
    }

    /// Browse sessions with explicit filters and cursor pagination.
    pub fn sessions(&self) -> Sessions<'_> {
        Sessions { client: self }
    }

    /// Configured default for location-aware routes. Server-wide routes ignore it.
    pub fn default_location(&self) -> &Location {
        &self.inner.default_location
    }

    /// The validated base URL, normalized to end in `/`.
    pub fn base_url(&self) -> &Url {
        &self.inner.base_url
    }

    pub(crate) async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        operation: &'static str,
        expected: StatusCode,
    ) -> Result<T> {
        self.get_json_at(path, operation, expected, &Location::Unspecified)
            .await
    }

    pub(crate) async fn get_json_at<T: DeserializeOwned>(
        &self,
        path: &str,
        operation: &'static str,
        expected: StatusCode,
        location: &Location,
    ) -> Result<T> {
        let query = match location {
            Location::Directory(directory) => vec![("location[directory]", directory.clone())],
            Location::Unspecified => Vec::new(),
        };
        self.get_json_query(path, operation, expected, &query).await
    }

    pub(crate) async fn get_json_query<T: DeserializeOwned>(
        &self,
        path: &str,
        operation: &'static str,
        expected: StatusCode,
        query: &[(&str, String)],
    ) -> Result<T> {
        // Route paths are crate-owned constants, relative to a validated directory URL.
        let mut url = self
            .inner
            .base_url
            .join(path)
            .expect("valid relative API route");
        if !query.is_empty() {
            url.query_pairs_mut()
                .extend_pairs(query.iter().map(|(key, value)| (*key, value.as_str())));
        }
        let mut request = self.inner.http.get(url).headers(self.inner.headers.clone());
        if let Some(timeout) = self.inner.request_timeout {
            request = request.timeout(timeout);
        }
        let mut response = request
            .send()
            .await
            .map_err(|source| Error::Transport(TransportError::new(operation, None, source)))?;
        let status = response.status();
        let success = status == expected;
        let limit = if success {
            self.inner.max_response_bytes
        } else {
            self.inner.max_error_body_bytes
        };
        let mut body = Vec::new();
        let mut truncated = false;
        while let Some(chunk) = response.chunk().await.map_err(|source| {
            Error::Transport(TransportError::new(operation, Some(status), source))
        })? {
            let remaining = limit - body.len();
            body.extend_from_slice(&chunk[..remaining.min(chunk.len())]);
            if chunk.len() > remaining {
                truncated = true;
                break;
            }
        }
        if !success {
            return Err(Error::Http(Box::new(HttpError::new(
                operation, status, body, truncated,
            ))));
        }
        if truncated {
            return Err(Error::ResponseTooLarge(ResponseTooLargeError {
                operation,
                status,
                limit,
            }));
        }
        serde_json::from_slice(&body).map_err(|source| {
            Error::Decode(DecodeError::new(
                operation,
                status,
                &body,
                self.inner.max_error_body_bytes,
                source,
            ))
        })
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Headers, URLs, and the supplied HTTP client may contain credentials.
        f.debug_struct("Client")
            .field("request_timeout", &self.inner.request_timeout)
            .field("max_response_bytes", &self.inner.max_response_bytes)
            .finish_non_exhaustive()
    }
}

/// Configuration for [`Client`]. Validation happens in [`Self::build`].
pub struct ClientBuilder {
    default_location: Location,
    base_url: String,
    http: Option<reqwest::Client>,
    headers: HeaderMap,
    header_error: Option<reqwest::header::InvalidHeaderValue>,
    connect_timeout: Option<Duration>,
    request_timeout: Option<Duration>,
    max_response_bytes: usize,
    max_error_body_bytes: usize,
}

impl ClientBuilder {
    /// Set the default directory selection for routes that accept a location.
    /// Defaults to [`Location::Unspecified`]. Per-call overrides do not change it.
    pub fn default_location(mut self, location: Location) -> Self {
        self.default_location = location;
        self
    }

    /// Merge default request headers. Later values override earlier values.
    ///
    /// SDK headers override defaults on a supplied reqwest client. All caller
    /// values are marked sensitive to prevent accidental debug formatting.
    pub fn default_headers(mut self, mut headers: HeaderMap) -> Self {
        for value in headers.values_mut() {
            value.set_sensitive(true);
        }
        self.headers.extend(headers);
        self
    }

    /// Set an Authorization bearer token. Invalid values fail in [`Self::build`].
    /// Custom authentication schemes can be supplied through default headers.
    pub fn bearer_auth(mut self, token: impl AsRef<str>) -> Self {
        match HeaderValue::from_str(&format!("Bearer {}", token.as_ref())) {
            Ok(mut value) => {
                value.set_sensitive(true);
                self.headers.insert(AUTHORIZATION, value);
            }
            Err(error) => self.header_error = Some(error),
        }
        self
    }

    /// Supply an existing HTTP client for custom TLS, proxies, DNS, or policies.
    ///
    /// Its connection, redirect, and retry policies are retained. Configure those
    /// on the supplied client; [`Self::connect_timeout`] cannot be combined with
    /// this option. SDK headers and a configured SDK request timeout take priority.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    /// Set connection establishment timeout for the default transport (10s default).
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Set the ordinary request deadline, including body reads (30s default).
    /// This overrides a supplied HTTP client's total-request timeout.
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }

    /// Remove the SDK request deadline. A supplied HTTP client's deadlines remain.
    pub fn without_request_timeout(mut self) -> Self {
        self.request_timeout = None;
        self
    }

    /// Limit buffered success bodies (8 MiB default), measured after transport decoding.
    pub fn max_response_bytes(mut self, limit: usize) -> Self {
        self.max_response_bytes = limit;
        self
    }

    /// Limit retained HTTP/decode diagnostic body prefixes (8 KiB default).
    /// Zero disables retaining diagnostic bodies. Truncation is explicit in errors.
    pub fn max_error_body_bytes(mut self, limit: usize) -> Self {
        self.max_error_body_bytes = limit;
        self
    }

    /// Validate configuration and construct the client without making a request.
    ///
    /// The default transport uses no redirects or automatic retries. The optional
    /// `rustls` Cargo feature is enabled by default for HTTPS.
    ///
    /// # Errors
    /// Returns [`BuildError`] for unsupported URLs, invalid authentication headers,
    /// conflicting connection settings, or transport initialization failures.
    pub fn build(self) -> std::result::Result<Client, BuildError> {
        let mut base_url = Url::parse(&self.base_url).map_err(|_| BuildError::InvalidBaseUrl)?;
        if !matches!(base_url.scheme(), "http" | "https")
            || base_url.host_str().is_none()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(BuildError::InvalidBaseUrl);
        }
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }
        if let Some(error) = self.header_error {
            return Err(BuildError::InvalidHeader(error));
        }
        let http = if let Some(http) = self.http {
            if self.connect_timeout.is_some() {
                return Err(BuildError::ConnectTimeoutWithHttpClient);
            }
            http
        } else {
            let builder = reqwest::Client::builder()
                .connect_timeout(self.connect_timeout.unwrap_or(DEFAULT_CONNECT_TIMEOUT))
                .redirect(reqwest::redirect::Policy::none())
                .retry(reqwest::retry::never())
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION")
                ));
            #[cfg(feature = "rustls")]
            let builder = builder.tls_backend_rustls();
            builder
                .build()
                .map_err(|source| BuildError::HttpClient(source.without_url()))?
        };
        Ok(Client {
            inner: Arc::new(Inner {
                default_location: self.default_location,
                base_url,
                http,
                headers: self.headers,
                request_timeout: self.request_timeout,
                max_response_bytes: self.max_response_bytes,
                max_error_body_bytes: self.max_error_body_bytes,
            }),
        })
    }
}

impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientBuilder")
            .field("has_http_client", &self.http.is_some())
            .field("request_timeout", &self.request_timeout)
            .finish_non_exhaustive()
    }
}
