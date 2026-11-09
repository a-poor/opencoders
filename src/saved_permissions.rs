use crate::{
    Client, Permissions, Result, StatusCode,
    models::{
        PermissionSavedId, PermissionSavedListResponse, PermissionSavedResponse,
        PermissionSavedUpdateRequest, ProjectId,
    },
};
use reqwest::Method;

/// Project filter for saved rules; omission uses the server-resolved location's
/// project, not the SDK default location. This route has no pinned location query.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SavedPermissionListOptions {
    /// Explicit project ID. An empty string is sent unchanged; v2.2.5 treats it
    /// as an unfiltered database read, so prefer a concrete discovered project ID.
    pub project: Option<ProjectId>,
}
impl SavedPermissionListOptions {
    /// Filter by project ID. Use a concrete ID to avoid server location fallback.
    pub fn project(mut self, project: ProjectId) -> Self {
        self.project = Some(project);
        self
    }
}
/// Durable project allow rules, obtained from [`Permissions::saved`].
///
/// These are saved approvals, not pending requests or configured deny rules.
/// All calls ignore the SDK default location; listing without a project uses the
/// server's location fallback (including caller headers). Update/remove address
/// a rule globally by ID. Applications choose the scope and authorize mutations.
/// Empty IDs and URL dot segments `.`/`..` fail with an encode error before HTTP.
#[derive(Clone, Copy, Debug)]
pub struct SavedPermissions<'a> {
    pub(crate) client: &'a Client,
}
impl<'a> Permissions<'a> {
    /// Inspect, edit and remove durable project allow rules.
    pub fn saved(&self) -> SavedPermissions<'a> {
        SavedPermissions {
            client: self.client,
        }
    }
}
impl SavedPermissions<'_> {
    /// List saved rules (`permission.saved.list`), retaining order and envelope
    /// extensions. HTTP 200 is required. Omission does not mean every project.
    pub async fn list(
        &self,
        options: &SavedPermissionListOptions,
    ) -> Result<PermissionSavedListResponse> {
        let query = options
            .project
            .as_ref()
            .map(|id| vec![("projectID", id.to_string())])
            .unwrap_or_default();
        self.client
            .get_json_query(
                "api/permission/saved",
                "permission.saved.list",
                StatusCode::OK,
                &query,
            )
            .await
    }
    /// Change action/resource patterns, accepting HTTP 200 and retaining metadata.
    /// Omitted/null fields leave values unchanged on v2.2.5; empty strings are
    /// explicit values. Project and allow effect cannot be changed here.
    /// Conflicting project/action/resource triples return `Conflict` (409), and
    /// missing IDs return `PermissionSavedNotFound` (404). Edits affect future
    /// checks; already-pending requests are not automatically settled.
    /// No automatic retries, including after lost responses or partial effects.
    pub async fn update(
        &self,
        id: &PermissionSavedId,
        request: &PermissionSavedUpdateRequest,
    ) -> Result<PermissionSavedResponse> {
        validate_id(id, "permission.saved.update")?;
        let request = self.client.json_request(
            Method::PATCH,
            self.client
                .segments_url("api/permission/saved", &[id.as_str()]),
            "permission.saved.update",
            request,
        )?;
        self.client
            .send_json(request, "permission.saved.update", StatusCode::OK)
            .await
    }
    /// Remove a saved allow rule, accepting only HTTP 204. Missing IDs succeed on
    /// v2.2.5. Future checks lose this saved grant; configured rules still apply.
    /// Does not cancel active work or answer pending requests. Never retries.
    pub async fn remove(&self, id: &PermissionSavedId) -> Result<()> {
        validate_id(id, "permission.saved.remove")?;
        self.client
            .delete_empty(
                "api/permission/saved",
                &[id.as_str()],
                "permission.saved.remove",
            )
            .await
    }
}
fn validate_id(id: &PermissionSavedId, operation: &'static str) -> Result<()> {
    if matches!(id.as_str(), "" | "." | "..") {
        return Err(crate::Error::Encode(crate::EncodeError {
            operation,
            source: serde::ser::Error::custom(
                "saved permission ID cannot be represented as a path segment",
            ),
        }));
    }
    Ok(())
}
