use crate::{Client, Location, Result, StatusCode, models::*};

/// Options for direct-child directory listing. Paths are server paths.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct FileListOptions {
    /// Absolute or location-relative path. Omission uses the location directory.
    pub path: Option<String>,
}
impl FileListOptions {
    /// Set a path without local normalization; empty and omitted stay distinct.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
}

/// Ranked recursive search options. The server owns ranking and filtering.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FileFindOptions {
    /// Search text, including an explicitly empty string.
    pub query: String,
    /// Restrict results to files or directories.
    pub kind: Option<FileSystemEntryType>,
    /// Requested result limit; the server validates and may cap it.
    pub limit: Option<u64>,
}
impl FileFindOptions {
    /// Create a search with required query text.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            kind: None,
            limit: None,
        }
    }
    /// Filter by entry type.
    pub fn kind(mut self, kind: FileSystemEntryType) -> Self {
        self.kind = Some(kind);
        self
    }
    /// Set the result limit, preserving zero for server-side validation.
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Workspace filesystem operations, obtained from [`Client::files`].
#[derive(Clone, Copy, Debug)]
pub struct Files<'a> {
    pub(crate) client: &'a Client,
}
impl Files<'_> {
    /// List direct children using the client's location (`fs.list`).
    /// Preserves the location envelope and server ordering. The server allows
    /// listing parents/siblings; this does not change the client's location.
    pub async fn list(&self, options: &FileListOptions) -> Result<FileListResponse> {
        self.list_at(options, self.client.default_location().clone())
            .await
    }
    /// List at a per-call location; [`Location::Unspecified`] omits its query.
    pub async fn list_at(
        &self,
        options: &FileListOptions,
        location: Location,
    ) -> Result<FileListResponse> {
        let mut query = location.query();
        if let Some(path) = &options.path {
            query.push(("path", path.clone()));
        }
        self.client
            .get_json_query("api/fs/list", "fs.list", StatusCode::OK, &query)
            .await
    }
    /// Find recursively ranked entries at the client's location (`fs.find`).
    pub async fn find(&self, options: &FileFindOptions) -> Result<FileListResponse> {
        self.find_at(options, self.client.default_location().clone())
            .await
    }
    /// Find entries at a per-call location, preserving the response envelope.
    pub async fn find_at(
        &self,
        options: &FileFindOptions,
        location: Location,
    ) -> Result<FileListResponse> {
        let mut query = location.query();
        query.push(("query", options.query.clone()));
        if let Some(kind) = options.kind {
            query.push(("type", kind.to_string()));
        }
        if let Some(limit) = options.limit {
            query.push(("limit", limit.to_string()));
        }
        self.client
            .get_json_query("api/fs/find", "fs.find", StatusCode::OK, &query)
            .await
    }
}

/// Branch listing filters. Returned branch names retain server ordering.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct VcsBranchOptions {
    /// Optional name search; an empty search is transmitted explicitly.
    pub search: Option<String>,
    /// Requested maximum; v2.2.5 defaults to 50 and caps it at 100.
    pub limit: Option<u64>,
}
impl VcsBranchOptions {
    /// Set the branch-name search.
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }
    /// Set the requested limit; server validation applies.
    pub fn limit(mut self, value: u64) -> Self {
        self.limit = Some(value);
        self
    }
}

/// Explicit VCS diff mode and optional comparison settings.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct VcsDiffOptions {
    /// Working copy, branch changes, or committed changes.
    pub mode: VcsMode,
    /// Comparison base override; omission retains the server's default.
    pub base: Option<String>,
    /// Context lines; zero requests no context.
    pub context: Option<u64>,
    /// Representation of untracked files.
    pub untracked: Option<VcsUntracked>,
}
impl VcsDiffOptions {
    /// Select a required diff mode.
    pub fn new(mode: VcsMode) -> Self {
        Self {
            mode,
            base: None,
            context: None,
            untracked: None,
        }
    }
    /// Override the comparison base without saving it on the server.
    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }
    /// Set the number of context lines.
    pub fn context(mut self, value: u64) -> Self {
        self.context = Some(value);
        self
    }
    /// Set how untracked paths are represented.
    pub fn untracked(mut self, value: VcsUntracked) -> Self {
        self.untracked = Some(value);
        self
    }
}

/// Read-only version-control operations, obtained from [`Client::vcs`].
#[derive(Clone, Copy, Debug)]
pub struct Vcs<'a> {
    pub(crate) client: &'a Client,
}
impl Vcs<'_> {
    /// Read current/default branch and provider information (`vcs.get`).
    pub async fn get(&self) -> Result<VcsResponse> {
        self.get_at(self.client.default_location().clone()).await
    }
    /// Read VCS information at a per-call location.
    pub async fn get_at(&self, location: Location) -> Result<VcsResponse> {
        self.client
            .get_json_at("api/vcs", "vcs.get", StatusCode::OK, &location)
            .await
    }
    /// Infer a local review base (`vcs.base`). A present null `data` is preserved.
    /// The server may return null before the first commit or without base metadata.
    pub async fn base(&self) -> Result<VcsBaseResponse> {
        self.base_at(self.client.default_location().clone()).await
    }
    /// Infer a review base at a per-call location.
    pub async fn base_at(&self, location: Location) -> Result<VcsBaseResponse> {
        self.client
            .get_json_at("api/vcs/base", "vcs.base", StatusCode::OK, &location)
            .await
    }
    /// List uncommitted working-copy changes (`vcs.status`).
    pub async fn status(&self) -> Result<VcsStatusResponse> {
        self.status_at(self.client.default_location().clone()).await
    }
    /// List changes at a per-call location.
    pub async fn status_at(&self, location: Location) -> Result<VcsStatusResponse> {
        self.client
            .get_json_at("api/vcs/status", "vcs.status", StatusCode::OK, &location)
            .await
    }
    /// List local and remote branches (`vcs.branch.list`).
    pub async fn branches(&self, options: &VcsBranchOptions) -> Result<VcsBranchesResponse> {
        self.branches_at(options, self.client.default_location().clone())
            .await
    }
    /// List branches at a per-call location.
    pub async fn branches_at(
        &self,
        options: &VcsBranchOptions,
        location: Location,
    ) -> Result<VcsBranchesResponse> {
        let mut query = location.query();
        if let Some(search) = &options.search {
            query.push(("search", search.clone()));
        }
        if let Some(limit) = options.limit {
            query.push(("limit", limit.to_string()));
        }
        self.client
            .get_json_query("api/vcs/branch", "vcs.branch.list", StatusCode::OK, &query)
            .await
    }
    /// Read a diff using an explicit comparison mode (`vcs.diff`).
    /// Diff bodies retain their reviewed numeric constraints and extra fields.
    pub async fn diff(&self, options: &VcsDiffOptions) -> Result<VcsDiffResponse> {
        self.diff_at(options, self.client.default_location().clone())
            .await
    }
    /// Read a diff at a per-call location.
    pub async fn diff_at(
        &self,
        options: &VcsDiffOptions,
        location: Location,
    ) -> Result<VcsDiffResponse> {
        let mut query = location.query();
        query.push(("mode", options.mode.to_string()));
        if let Some(base) = &options.base {
            query.push(("base", base.clone()));
        }
        if let Some(context) = options.context {
            query.push(("context", context.to_string()));
        }
        if let Some(untracked) = options.untracked {
            query.push(("untracked", untracked.to_string()));
        }
        self.client
            .get_json_query("api/vcs/diff", "vcs.diff", StatusCode::OK, &query)
            .await
    }
}

/// Saved project worktree inventory, obtained from [`Client::worktrees`].
#[derive(Clone, Copy, Debug)]
pub struct Worktrees<'a> {
    pub(crate) client: &'a Client,
}
impl Worktrees<'_> {
    /// List the project's saved worktrees (`worktree.list`).
    /// Does not run discovery or activate a location. The default location is
    /// ignored; obtain a project ID from [`Client::location`].
    pub async fn list(&self, project_id: &ProjectId) -> Result<WorktreeList> {
        self.client
            .get_json_query(
                "api/worktree",
                "worktree.list",
                StatusCode::OK,
                &[("projectID", project_id.to_string())],
            )
            .await
    }
}
