use crate::{
    Client, Error, Location, Result, Sessions, StatusCode,
    models::{
        SessionEnvironmentRequest, SessionId, SessionShellRequest, ShellCreateRequest, ShellId,
        ShellListResponse, ShellOutputResponse, ShellResponse,
    },
};
use reqwest::{Method, Url};
use std::time::Duration;

/// Location-scoped non-interactive commands, obtained from [`Client::shells`].
/// Mutations are never retried by the default transport. Cancellation/HTTP errors
/// after submission can leave their outcome unknown; keep returned IDs to inspect.
#[derive(Clone, Copy, Debug)]
pub struct Shells<'a> {
    pub(crate) client: &'a Client,
}

/// Absolute byte cursor and page size for [`Shells::output`].
/// The pinned implementation accepts nonnegative safe integers. SDK values above
/// 2^53−1 fail before sending. Omission uses cursor 0 and limit 65536 bytes.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ShellOutputOptions {
    /// Absolute byte offset, not a character index.
    pub cursor: Option<u64>,
    /// Bytes requested. Prefer positive sizes; v2.2.5 can fail on zero-sized reads.
    pub limit: Option<u64>,
}
impl ShellOutputOptions {
    /// Select an absolute byte cursor, preserving explicit zero.
    pub fn cursor(mut self, cursor: u64) -> Self {
        self.cursor = Some(cursor);
        self
    }
    /// Select the maximum bytes to read, preserving explicit zero.
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// HTTP deadline for [`Sessions::shell`], independent of ordinary client requests.
/// Default: no SDK deadline. Injected reqwest deadlines still apply. Timeout or
/// dropping the future stops local waiting only; the server owns completion.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SessionShellOptions {
    /// Total HTTP deadline, including bounded error-body reads.
    pub timeout: Option<Duration>,
}
impl SessionShellOptions {
    /// Set this call's HTTP deadline; this does not set a process timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    /// Add no SDK deadline; supplied HTTP-client deadlines remain effective.
    pub fn without_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }
}
impl Shells<'_> {
    fn url(&self, id: Option<&ShellId>, output: bool, location: &Location) -> Url {
        let mut segments = Vec::new();
        if let Some(id) = id {
            segments.push(id.as_str());
        }
        if output {
            segments.push("output");
        }
        let mut url = self.client.segments_url("api/shell", &segments);
        let query = location.query();
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }
        url
    }
    /// Start a command (`shell.create`). Returns after registration, before exit.
    pub async fn create(&self, request: &ShellCreateRequest) -> Result<ShellResponse> {
        self.create_at(request, self.client.default_location().clone())
            .await
    }
    /// Start a command at a per-call location; `cwd` selects its working directory.
    pub async fn create_at(
        &self,
        request: &ShellCreateRequest,
        location: Location,
    ) -> Result<ShellResponse> {
        let request = self.client.json_request(
            Method::POST,
            self.url(None, false, &location),
            "shell.create",
            request,
        )?;
        self.client
            .send_json(request, "shell.create", StatusCode::OK)
            .await
    }
    /// Inspect a command (`shell.get`), including retained exited records.
    pub async fn get(&self, id: &ShellId) -> Result<ShellResponse> {
        self.get_at(id, self.client.default_location().clone())
            .await
    }
    /// Inspect a command at a per-call location. Records are process-local;
    /// v2.2.5 retains at most 25 exited commands per location.
    pub async fn get_at(&self, id: &ShellId, location: Location) -> Result<ShellResponse> {
        self.client
            .send_json(
                self.client
                    .request(Method::GET, self.url(Some(id), false, &location)),
                "shell.get",
                StatusCode::OK,
            )
            .await
    }
    /// List currently running commands only (`shell.list`).
    pub async fn list(&self) -> Result<ShellListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// List currently running commands at a per-call location.
    pub async fn list_at(&self, location: Location) -> Result<ShellListResponse> {
        self.client
            .get_json_at("api/shell", "shell.list", StatusCode::OK, &location)
            .await
    }
    /// Read merged stdout/stderr (`shell.output`). Compare cursor with size to
    /// discover more output; `truncated` is not a pagination flag. Each page is
    /// decoded as UTF-8 by the server, so split characters can become U+FFFD.
    pub async fn output(
        &self,
        id: &ShellId,
        options: &ShellOutputOptions,
    ) -> Result<ShellOutputResponse> {
        self.output_at(id, options, self.client.default_location().clone())
            .await
    }
    /// Read output at a per-call location. No automatic polling or text repair.
    pub async fn output_at(
        &self,
        id: &ShellId,
        options: &ShellOutputOptions,
        location: Location,
    ) -> Result<ShellOutputResponse> {
        let mut url = self.url(Some(id), true, &location);
        for (key, value) in [("cursor", options.cursor), ("limit", options.limit)] {
            if let Some(value) = value {
                if value > 9_007_199_254_740_991 {
                    return Err(Error::Encode(crate::EncodeError {
                        operation: "shell.output",
                        source: serde::ser::Error::custom(
                            "shell output offset/limit exceeds safe integer range",
                        ),
                    }));
                }
                url.query_pairs_mut().append_pair(key, &value.to_string());
            }
        }
        self.client
            .send_json(
                self.client.request(Method::GET, url),
                "shell.output",
                StatusCode::OK,
            )
            .await
    }
    /// Terminate/remove a command and its retained output (`shell.remove`).
    /// HTTP 204 acknowledges removal; process cleanup can still be finishing.
    pub async fn remove(&self, id: &ShellId) -> Result<()> {
        self.remove_at(id, self.client.default_location().clone())
            .await
    }
    /// Remove at a per-call location. v2.2.5 also returns 204 for an unknown ID.
    pub async fn remove_at(&self, id: &ShellId, location: Location) -> Result<()> {
        self.client
            .send_empty(
                self.client
                    .request(Method::DELETE, self.url(Some(id), false, &location)),
                "shell.remove",
            )
            .await
    }
}
impl Sessions<'_> {
    /// Execute a user command in the session directory (`session.shell`). Waits
    /// for completion and requires HTTP 204; a nonzero process exit still returns
    /// 204. Observe `session.shell.started/ended` and shell message history for the
    /// outcome. On v2.2.5 completion also parks a synthetic inbox notification
    /// without waking a model. A supplied message ID does not make retries safe.
    /// Discovery location defaults are ignored. The server continues after client
    /// disconnection; use the shell ID with [`Shells::remove`] to terminate.
    pub async fn shell(
        &self,
        id: &SessionId,
        request: &SessionShellRequest,
        options: &SessionShellOptions,
    ) -> Result<()> {
        let request = self.client.json_request(
            Method::POST,
            self.client
                .segments_url("api/session", &[id.as_str(), "shell"]),
            "session.shell",
            request,
        )?;
        self.client
            .send_response_with_timeout(
                request,
                "session.shell",
                StatusCode::NO_CONTENT,
                options.timeout,
            )
            .await
            .map(|_| ())
    }
    /// Replace the local command environment for a session (`session.environment`).
    /// The map replaces previously installed values; `{}` does not restore server
    /// inheritance. v2.2.5 keeps it in memory and injects `TERM` and
    /// `OPENCODE_TERMINAL` into commands. Remote workspaces do not use this map.
    /// Does not change already running processes. Requires HTTP 204, never retried.
    pub async fn environment(
        &self,
        id: &SessionId,
        request: &SessionEnvironmentRequest,
    ) -> Result<()> {
        let request = self.client.json_request(
            Method::PUT,
            self.client
                .segments_url("api/session", &[id.as_str(), "environment"]),
            "session.environment",
            request,
        )?;
        self.client.send_empty(request, "session.environment").await
    }
}
