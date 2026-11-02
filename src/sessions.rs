use crate::{
    Client, Result, StatusCode,
    models::{AbsolutePath, ProjectId, RelativePath, SessionId, SessionsCursor, SessionsResponse},
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
    /// Clear `order` explicitly when following a cursor; retain applicable filters.
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

/// Session browsing operations, obtained from [`Client::sessions`].
#[derive(Clone, Copy, Debug)]
pub struct Sessions<'a> {
    pub(crate) client: &'a Client,
}

impl Sessions<'_> {
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
