use crate::{
    Client, Result, StatusCode,
    models::{
        AbsolutePath, ProjectId, RelativePath, SessionCreateRequest, SessionForkRequest, SessionId,
        SessionResponse, SessionSwitchAgentRequest, SessionSwitchModelRequest,
        SessionUpdateRequest, SessionsCursor, SessionsResponse,
    },
};

/// Requested ordering for the first session page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SessionOrder {
    /// Oldest sessions first.
    Asc,
    /// Newest sessions first.
    Desc,
}

/// Restrict sessions by parent. An omitted filter leaves parent selection to the server.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ParentFilter {
    /// Root sessions only, encoded as the literal `parentID=null`.
    RootOnly,
    /// Sessions whose parent has this ID.
    ChildrenOf(SessionId),
}

/// Explicit filters for [`Sessions::list`]. Defaults omit every query parameter.
///
/// Fluent setters are conveniences; public fields allow clearing a filter with
/// `None`. Paths refer to the server and are never canonicalized locally.
/// The client default location does not apply to this server-wide list route.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SessionListOptions {
    /// Maximum requested item count. The server validates its accepted range.
    /// Omission uses the upstream default (currently the newest 50 sessions).
    pub limit: Option<u32>,
    /// Ordering for the first page. Omit when following a server cursor.
    pub order: Option<SessionOrder>,
    /// Server-side text search; an empty string is distinct from omission.
    pub search: Option<String>,
    /// Root-only or specific-parent selection; `None` omits `parentID`.
    pub parent: Option<ParentFilter>,
    /// Explicit archive filter; `None`, `Some(false)`, and `Some(true)` are distinct.
    pub archived: Option<bool>,
    /// Direct `directory` filter, not a nested location query.
    pub directory: Option<AbsolutePath>,
    /// Project filter using the shared project ID type.
    pub project: Option<ProjectId>,
    /// Relative subpath filter on the server.
    pub subpath: Option<RelativePath>,
    /// Opaque `next` or `previous` cursor from a response. Sent without interpretation.
    /// On OpenCode v2.2.5 it carries the original filters/order; clear it to change
    /// those. A cursor can lead to an empty page.
    pub cursor: Option<SessionsCursor>,
}

impl SessionListOptions {
    /// Set the requested item count; does not impose an undocumented server maximum.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    /// Set ordering for the first page.
    pub fn order(mut self, order: SessionOrder) -> Self {
        self.order = Some(order);
        self
    }
    /// Set server-side search text.
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
    /// Set a root-only or specific-parent filter.
    pub fn parent(mut self, parent: ParentFilter) -> Self {
        self.parent = Some(parent);
        self
    }
    /// Set the archive filter explicitly.
    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }
    /// Filter by directory, without applying the client's default location.
    pub fn directory(mut self, directory: impl Into<String>) -> Self {
        self.directory = Some(directory.into().into());
        self
    }
    /// Filter by a project ID, for example one returned by location discovery.
    pub fn project(mut self, project: ProjectId) -> Self {
        self.project = Some(project);
        self
    }
    /// Filter by a server-relative subpath.
    pub fn subpath(mut self, subpath: impl Into<String>) -> Self {
        self.subpath = Some(subpath.into().into());
        self
    }
    /// Set a next/previous cursor. Other filters, including order, remain unchanged.
    /// v2.2.5 uses the cursor's original filters/order; `limit` remains independent.
    pub fn cursor(mut self, cursor: SessionsCursor) -> Self {
        self.cursor = Some(cursor);
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
                    SessionOrder::Asc => "asc",
                    SessionOrder::Desc => "desc",
                }
                .into(),
            ));
        }
        if let Some(search) = &self.search {
            query.push(("search", search.clone()));
        }
        if let Some(parent) = &self.parent {
            query.push((
                "parentID",
                match parent {
                    ParentFilter::RootOnly => "null".into(),
                    ParentFilter::ChildrenOf(id) => id.as_str().to_owned(),
                },
            ));
        }
        if let Some(archived) = self.archived {
            query.push(("archived", archived.to_string()));
        }
        if let Some(directory) = &self.directory {
            query.push(("directory", directory.as_str().to_owned()));
        }
        if let Some(project) = &self.project {
            query.push(("project", project.as_str().to_owned()));
        }
        if let Some(subpath) = &self.subpath {
            query.push(("subpath", subpath.as_str().to_owned()));
        }
        if let Some(cursor) = &self.cursor {
            query.push(("cursor", cursor.as_str().to_owned()));
        }
        query
    }
}

/// Session lifecycle and browsing operations, obtained from [`Client::sessions`].
#[derive(Clone, Copy, Debug)]
pub struct Sessions<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Sessions<'a> {
    /// Durably admit input with `session.prompt`, returning its admitted user item.
    ///
    /// HTTP 200 acknowledges admission, not an assistant reply or execution completion.
    /// Work may already have left the inbox when the response arrives.
    /// On v2.2.5 omitted/null delivery means `steer`; omitted/null resume wakes the
    /// agent loop. Set `resume(false)` to admit without waking it. This does not
    /// pause an already running loop. Files refer to server-resolved URIs; the SDK
    /// neither reads nor uploads local files. The discovery location default is ignored.
    ///
    /// # Errors
    /// Returns encoding, transport, HTTP/API, decoding, or response-size errors.
    /// A failure can follow successful admission. The default transport never
    /// retries; injected HTTP-client policies remain in effect. Inspect pending
    /// work/history before resubmitting, including when supplying an explicit ID.
    pub async fn prompt(
        &self,
        id: &SessionId,
        request: &crate::models::SessionPromptRequest,
    ) -> Result<crate::models::SessionPromptResponse> {
        self.client
            .post_json_segments(
                "api/session",
                &[id.as_str(), "prompt"],
                "session.prompt",
                request,
            )
            .await
    }

    /// Inspect and manage durable work pending for this session.
    pub fn inbox(&self, id: &'a SessionId) -> crate::SessionInbox<'a> {
        crate::SessionInbox {
            client: self.client,
            session_id: id,
        }
    }

    /// Select the agent for subsequent provider turns (`session.switchAgent`).
    ///
    /// Sends a strict JSON request and accepts HTTP 204 without decoding JSON.
    /// Ignores the discovery location default. The server records the selection;
    /// success does not establish availability or invoke a model. On v2.2.5 even
    /// reselecting the same agent adds a history entry.
    ///
    /// # Errors
    /// Returns encode, transport, HTTP/API, or response-size errors. A failure can
    /// follow an applied selection. The default transport does not retry; injected
    /// client policies remain in effect. Missing sessions expose typed errors.
    pub async fn switch_agent(
        &self,
        id: &SessionId,
        request: &SessionSwitchAgentRequest,
    ) -> Result<()> {
        self.client
            .post_json_empty(
                "api/session",
                &[id.as_str(), "agent"],
                "session.switchAgent",
                request,
            )
            .await
    }
    /// Select the model/optional variant for subsequent turns (`session.switchModel`).
    ///
    /// Accepts HTTP 204; ignores the discovery location default. Use the catalog's
    /// `id` and `provider_id`, not its provider-native `model_id`. Success records
    /// a selection without proving execution readiness. Inputs cannot clear a
    /// selection with null. On v2.2.5, omitted variants become `"default"`; repeating
    /// the same provider/model/variant adds no history entry. Error and retry
    /// behavior matches [`Self::switch_agent`].
    pub async fn switch_model(
        &self,
        id: &SessionId,
        request: &SessionSwitchModelRequest,
    ) -> Result<()> {
        self.client
            .post_json_empty(
                "api/session",
                &[id.as_str(), "model"],
                "session.switchModel",
                request,
            )
            .await
    }

    /// Browse projected message history for one session.
    pub fn messages(&self, id: &'a SessionId) -> crate::SessionMessages<'a> {
        crate::SessionMessages {
            client: self.client,
            session_id: id,
        }
    }

    /// Create a session with `POST /api/session` (`session.create`).
    ///
    /// Returns the full `data` envelope. An empty default request sends `{}`.
    /// Location is selected explicitly in the request body; the client's discovery
    /// location default is ignored. A concrete parent ID uses the parent's location.
    /// No prompt is submitted. Nullable fields preserve their exact wire states.
    ///
    /// The default transport neither retries nor follows redirects. After a timeout,
    /// disconnect, or unreadable response, creation may already have succeeded;
    /// do not blindly resubmit. A supplied HTTP client's retry/redirect policy is
    /// retained, and supplying an ID does not promise idempotency.
    ///
    /// # Errors
    /// Returns encode (before sending), transport, unexpected HTTP status, decoding,
    /// or response-size errors. Only 200 is accepted. A missing parent may expose
    /// [`crate::ApiError::SessionNotFound`].
    pub async fn create(&self, request: &SessionCreateRequest) -> Result<SessionResponse> {
        self.client
            .post_json("api/session", "session.create", StatusCode::OK, request)
            .await
    }

    /// Fork settled projected history into a new session (`session.fork`).
    ///
    /// Omitted/null `before` copies through the newest projected message; a value
    /// excludes that boundary message. v2.2.5 skips unfinished assistant, shell,
    /// and compaction entries. Copied messages receive new IDs. Pending inbox work
    /// is not copied, and this call does not start model execution.
    ///
    /// The fork uses the source's location and current configuration/instructions,
    /// even for an earlier boundary; discovery location defaults are ignored.
    /// On v2.2.5 the result has `fork` provenance and no `parentID`, so removing the
    /// source does not recursively remove this independent session. Usage resets.
    /// Returns the full `data` envelope, including extension fields.
    ///
    /// # Errors
    /// An empty source returns [`crate::ApiError::InvalidRequest`] with kind
    /// `empty_session`; an explicit missing/wrong-session boundary returns
    /// [`crate::ApiError::MessageNotFound`]. Forking before the first existing
    /// message is allowed and creates an empty fork. Missing sources expose
    /// [`crate::ApiError::SessionNotFound`].
    ///
    /// Only HTTP 200 is accepted. Ordinary deadlines and bounded diagnostics apply;
    /// encode, transport, HTTP/API, decode, or response-size errors may occur.
    /// A failed response can follow successful creation. No automatic retry or
    /// idempotency key is provided; injected HTTP-client policies remain effective.
    pub async fn fork(
        &self,
        id: &SessionId,
        request: &SessionForkRequest,
    ) -> Result<SessionResponse> {
        self.client
            .post_json_segments(
                "api/session",
                &[id.as_str(), "fork"],
                "session.fork",
                request,
            )
            .await
    }

    /// Retrieve a session with `GET /api/session/{sessionID}` (`session.get`).
    ///
    /// The ID is encoded as one path segment; the client's location default is
    /// ignored. Returns the full `data` envelope, including extension fields.
    ///
    /// # Errors
    /// Returns transport, unexpected HTTP status, decoding, or response-size errors.
    /// Only 200 is accepted. Missing sessions expose [`crate::ApiError::SessionNotFound`].
    pub async fn get(&self, id: &SessionId) -> Result<SessionResponse> {
        self.client
            .get_json_segment("api/session", id.as_str(), "session.get", StatusCode::OK)
            .await
    }

    /// Update a session with `PATCH /api/session/{sessionID}` (`session.update`).
    ///
    /// Returns `()` for HTTP 204 without decoding JSON. The request preserves
    /// omitted/null/value states; client default location is ignored. An empty
    /// title requests server-generated titling and can invoke a model on v2.2.5.
    /// On that version, omission/null leave fields unchanged, while metadata and
    /// permission values replace their fields. `{}` and `[]` clear those contents.
    /// A failed response may follow partially or fully applied changes; the default
    /// transport does not retry. Injected HTTP-client policies are retained.
    ///
    /// # Errors
    /// Returns encode (before sending), transport, unexpected HTTP status, or response-size errors.
    /// Only 204 is accepted. Missing sessions/locations expose typed API errors.
    pub async fn update(&self, id: &SessionId, request: &SessionUpdateRequest) -> Result<()> {
        self.client
            .patch_json_empty("api/session", &[id.as_str()], "session.update", request)
            .await
    }

    /// Delete a session and its descendants with
    /// `DELETE /api/session/{sessionID}` (`session.remove`).
    ///
    /// Returns `()` for HTTP 204 without decoding JSON. Sends no request body and
    /// ignores the client default location. Deletion is recursive and irreversible;
    /// applications own any confirmation policy. A failed response can follow an
    /// applied deletion. The default transport does not retry; injected policies
    /// are retained. Repeating deletion may return a missing-session error.
    ///
    /// # Errors
    /// Returns transport, unexpected HTTP status, or response-size errors. Only 204 is accepted.
    /// Missing sessions expose [`crate::ApiError::SessionNotFound`].
    pub async fn remove(&self, id: &SessionId) -> Result<()> {
        self.client
            .delete_empty("api/session", &[id.as_str()], "session.remove")
            .await
    }

    /// Retrieve one page with `GET /api/session` (`session.list`).
    ///
    /// Returns the `data`/`cursor` envelope, preserving server order and extension
    /// fields. No automatic requests, sorting, filtering, or cursor decoding occur.
    /// Client default location is ignored; use the explicit directory/project filters.
    /// All supplied options are sent, including order if combined with a cursor.
    ///
    /// # Errors
    /// Returns transport, unexpected HTTP status, decoding, or response-size errors.
    /// Only 200 is accepted. Invalid cursors expose [`crate::ApiError::InvalidCursor`].
    pub async fn list(&self, options: &SessionListOptions) -> Result<SessionsResponse> {
        self.client
            .get_json_query(
                "api/session",
                "session.list",
                StatusCode::OK,
                &options.query(),
            )
            .await
    }
}
