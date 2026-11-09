use crate::{
    Client, Error, Location, Result, StatusCode,
    models::{
        PtyConnectTokenResponse, PtyCreateRequest, PtyId, PtyListResponse, PtyResponse,
        PtyUpdateRequest,
    },
};
use reqwest::{
    Method, Url,
    header::{HeaderMap, HeaderName, HeaderValue},
};

/// Ordinary location-scoped pseudo-terminals, obtained from [`Client::ptys`].
/// Creation starts a process on the server. Removing a PTY terminates it.
/// Mutations are never retried by the default client; failure/cancellation after
/// sending a request can leave its outcome unknown.
#[derive(Clone, Copy, Debug)]
pub struct Ptys<'a> {
    pub(crate) client: &'a Client,
}
impl Ptys<'_> {
    pub(crate) fn url(&self, id: Option<&PtyId>, suffix: Option<&str>, location: &Location) -> Url {
        let mut segments = Vec::new();
        if let Some(id) = id {
            segments.push(id.as_str());
        }
        if let Some(suffix) = suffix {
            segments.push(suffix);
        }
        let mut url = self.client.segments_url("api/pty", &segments);
        let query = location.query();
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }
        url
    }
    /// List running and retained exited PTYs (`pty.list`).
    pub async fn list(&self) -> Result<PtyListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// List PTYs at a per-call location.
    pub async fn list_at(&self, location: Location) -> Result<PtyListResponse> {
        self.client
            .get_json_at("api/pty", "pty.list", StatusCode::OK, &location)
            .await
    }
    /// Start a PTY using the client's location (`pty.create`).
    /// Omitted properties retain server defaults; returned metadata includes the
    /// resolved location, process ID and PTY ID. Does not open a socket.
    pub async fn create(&self, request: &PtyCreateRequest) -> Result<PtyResponse> {
        self.create_at(request, self.client.default_location().clone())
            .await
    }
    /// Start a PTY at a per-call location; `cwd` controls the process directory.
    pub async fn create_at(
        &self,
        request: &PtyCreateRequest,
        location: Location,
    ) -> Result<PtyResponse> {
        let request = self.client.json_request(
            Method::POST,
            self.url(None, None, &location),
            "pty.create",
            request,
        )?;
        self.client
            .send_json(request, "pty.create", StatusCode::OK)
            .await
    }
    /// Inspect one PTY, including its exit code when exited (`pty.get`).
    pub async fn get(&self, id: &PtyId) -> Result<PtyResponse> {
        self.get_at(id, self.client.default_location().clone())
            .await
    }
    /// Inspect a PTY at a per-call location. Ownership is location-scoped.
    pub async fn get_at(&self, id: &PtyId, location: Location) -> Result<PtyResponse> {
        self.client
            .send_json(
                self.client
                    .request(Method::GET, self.url(Some(id), None, &location)),
                "pty.get",
                StatusCode::OK,
            )
            .await
    }
    /// Update a PTY title or resize its viewport (`pty.update`).
    /// Dimensions must be positive safe integers. The pinned server ignores an
    /// empty title and does not resize exited PTYs; requests retain empty strings.
    pub async fn update(&self, id: &PtyId, request: &PtyUpdateRequest) -> Result<PtyResponse> {
        self.update_at(id, request, self.client.default_location().clone())
            .await
    }
    /// Update a PTY at a per-call location.
    pub async fn update_at(
        &self,
        id: &PtyId,
        request: &PtyUpdateRequest,
        location: Location,
    ) -> Result<PtyResponse> {
        if request.size.as_ref().is_some_and(|size| {
            [size.rows, size.cols]
                .iter()
                .any(|value| !(1..=9_007_199_254_740_991).contains(value))
        }) {
            return Err(Error::Encode(crate::EncodeError {
                operation: "pty.update",
                source: serde::ser::Error::custom("PTY dimensions must be positive safe integers"),
            }));
        }
        let request = self.client.json_request(
            Method::PUT,
            self.url(Some(id), None, &location),
            "pty.update",
            request,
        )?;
        self.client
            .send_json(request, "pty.update", StatusCode::OK)
            .await
    }
    /// Terminate and remove a PTY (`pty.remove`). Only HTTP 204 is accepted.
    pub async fn remove(&self, id: &PtyId) -> Result<()> {
        self.remove_at(id, self.client.default_location().clone())
            .await
    }
    /// Terminate/remove a PTY at a per-call location.
    pub async fn remove_at(&self, id: &PtyId, location: Location) -> Result<()> {
        self.client
            .send_empty(
                self.client
                    .request(Method::DELETE, self.url(Some(id), None, &location)),
                "pty.remove",
            )
            .await
    }
    /// Mint a short-lived, single-use WebSocket ticket (`pty.connect.token`).
    /// Sends the required `x-opencode-ticket: 1` header. The ticket is scoped to
    /// the PTY and resolved location; use that location when connecting. Minting
    /// and consumption are explicit and never retried. Debug redacts the ticket.
    pub async fn connect_token(&self, id: &PtyId) -> Result<PtyConnectTokenResponse> {
        self.connect_token_at(id, self.client.default_location().clone())
            .await
    }
    /// Mint a ticket at a per-call location. Server origin restrictions apply.
    pub async fn connect_token_at(
        &self,
        id: &PtyId,
        location: Location,
    ) -> Result<PtyConnectTokenResponse> {
        let headers = HeaderMap::from_iter([(
            HeaderName::from_static("x-opencode-ticket"),
            HeaderValue::from_static("1"),
        )]);
        self.client
            .send_json(
                self.client
                    .request(
                        Method::POST,
                        self.url(Some(id), Some("connect-token"), &location),
                    )
                    .headers(headers),
                "pty.connect.token",
                StatusCode::OK,
            )
            .await
    }
}
