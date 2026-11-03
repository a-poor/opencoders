use crate::{
    Client, Location, Result, Sessions, StatusCode,
    models::{
        PermissionId, PermissionRequestListResponse, SessionId, SessionPermissionCreateRequest,
        SessionPermissionCreateResponse, SessionPermissionListResponse,
        SessionPermissionReplyRequest, SessionPermissionResponse,
    },
};

/// Pending permission discovery. Applications own presentation and approval policy.
#[derive(Clone, Copy, Debug)]
pub struct Permissions<'a> {
    pub(crate) client: &'a Client,
}
impl Client {
    /// Discover pending permission requests by location.
    pub fn permissions(&self) -> Permissions<'_> {
        Permissions { client: self }
    }
}
impl Permissions<'_> {
    /// List pending requests at the default location (`permission.request.list`).
    /// Retains the location/data envelope, server order, and additional fields.
    /// This is a snapshot of in-memory pending requests, not durable history.
    pub async fn list(&self) -> Result<PermissionRequestListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// Override location for this call; `Unspecified` omits the query without
    /// removing caller headers. Returns HTTP/API, transport, decode, or size errors.
    pub async fn list_at(&self, location: Location) -> Result<PermissionRequestListResponse> {
        self.client
            .get_json_at(
                "api/permission/request",
                "permission.request.list",
                StatusCode::OK,
                &location,
            )
            .await
    }
}
/// Pending permissions owned by one session, resolved using its server location.
///
/// IDs are encoded as individual path segments. The client's discovery location
/// default is ignored. All calls use ordinary deadlines and bounded diagnostics.
#[derive(Clone, Copy, Debug)]
pub struct SessionPermissions<'a> {
    pub(crate) client: &'a Client,
    pub(crate) session_id: &'a SessionId,
}
impl<'a> Sessions<'a> {
    /// Evaluate, inspect, and answer permission requests for this session.
    pub fn permissions(&self, id: &'a SessionId) -> SessionPermissions<'a> {
        SessionPermissions {
            client: self.client,
            session_id: id,
        }
    }
}
impl SessionPermissions<'_> {
    /// List pending requests (`session.permission.list`), preserving the data
    /// envelope and server order. Requests can resolve between discovery and reply.
    pub async fn list(&self) -> Result<SessionPermissionListResponse> {
        self.client
            .get_json_segments(
                "api/session",
                &[self.session_id.as_str(), "permission"],
                "session.permission.list",
                StatusCode::OK,
                &[],
            )
            .await
    }
    /// Get a pending request (`session.permission.get`). Missing, resolved, and
    /// wrong-session requests expose [`crate::ApiError::PermissionNotFound`].
    pub async fn get(&self, id: &PermissionId) -> Result<SessionPermissionResponse> {
        self.client
            .get_json_segments(
                "api/session",
                &[self.session_id.as_str(), "permission", id.as_str()],
                "session.permission.get",
                StatusCode::OK,
                &[],
            )
            .await
    }
    /// Evaluate and possibly create a request (`session.permission.create`).
    ///
    /// HTTP 200 returns an immediate `allow`, `deny`, or `ask` effect and ID. Only
    /// `ask` creates a pending request; this does not execute the requested action
    /// or wait for approval. Nullable ID/agent fields preserve all three wire states.
    /// On v2.2.5 null behaves like omission. Duplicate pending IDs are not idempotent.
    ///
    /// Returns encode, transport, HTTP/API, decode, or size errors. A failure may
    /// follow successful creation. No automatic retry; injected policies remain.
    pub async fn create(
        &self,
        request: &SessionPermissionCreateRequest,
    ) -> Result<SessionPermissionCreateResponse> {
        self.client
            .post_json_segments(
                "api/session",
                &[self.session_id.as_str(), "permission"],
                "session.permission.create",
                request,
            )
            .await
    }
    /// Reply to a pending request (`session.permission.reply`), accepting HTTP 204.
    ///
    /// On v2.2.5 `once` resolves one request; `reject` also rejects other pending
    /// requests in that session, forwarding nonempty feedback. `always` can save
    /// the request's nonempty `save` patterns as project allow rules and resolve
    /// other now-allowed requests at this location, including other sessions.
    /// Without save patterns, always resolves only this request and saves nothing.
    /// Successful approval may unblock server work; it does not prove that work
    /// succeeded. No approval policy is imposed by this library.
    ///
    /// Missing/resolved/wrong-session requests expose `PermissionNotFound`.
    /// Failures can follow partial or complete effects. No automatic retry;
    /// injected HTTP policies remain effective. Dropping a future does not undo
    /// an accepted reply. Saved-rule management is not implemented here.
    pub async fn reply(
        &self,
        id: &PermissionId,
        request: &SessionPermissionReplyRequest,
    ) -> Result<()> {
        self.client
            .post_json_empty(
                "api/session",
                &[self.session_id.as_str(), "permission", id.as_str(), "reply"],
                "session.permission.reply",
                request,
            )
            .await
    }
}
