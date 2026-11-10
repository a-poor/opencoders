//! Read-only application setup and discovery.
use crate::{Client, Location, Result, StatusCode, models::*};

/// Filters for the server-wide known-project inventory.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ProjectListOptions {
    /// Include archived projects when true. Omitted/false excludes them on v2.2.5.
    pub archived: Option<bool>,
}
impl ProjectListOptions {
    /// Include or exclude archived projects explicitly; false remains on the wire.
    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }
}
/// Known projects, obtained from [`Client::projects`].
#[derive(Clone, Copy, Debug)]
pub struct Projects<'a> {
    pub(crate) client: &'a Client,
}
impl Projects<'_> {
    /// Read the known-project inventory (HTTP 200, bare array), ignoring default
    /// location. On v2.2.5 results sort by descending activity then project ID;
    /// archived=true includes both active and archived projects. Preserves order.
    pub async fn list(&self, options: &ProjectListOptions) -> Result<Vec<Project>> {
        let query: Vec<_> = options
            .archived
            .map(|v| ("archived", v.to_string()))
            .into_iter()
            .collect();
        self.client
            .get_json_query("api/project", "project.list", StatusCode::OK, &query)
            .await
    }
}
/// Configuration source discovery, obtained from [`Client::config`].
#[derive(Clone, Copy, Debug)]
pub struct Config<'a> {
    pub(crate) client: &'a Client,
}
impl Config<'_> {
    /// Read ordered documents/discovery directories at the client's location.
    /// This is a bare array, not a merged effective configuration. Documents may
    /// lack paths and contain secrets. Debug is redacted; serialization is not.
    pub async fn get(&self) -> Result<Vec<ConfigEntry>> {
        self.get_at(self.client.default_location().clone()).await
    }
    /// Override location; `Unspecified` omits the query, retaining caller headers.
    /// Only HTTP 200 is accepted. Ordinary transport/HTTP/decode/size errors apply.
    pub async fn get_at(&self, location: Location) -> Result<Vec<ConfigEntry>> {
        self.client
            .get_json_at("api/config", "config.get", StatusCode::OK, &location)
            .await
    }
    /// List server shell candidates and their compatibility flags (bare array).
    /// Ignores default location; does not select or execute a shell. `acceptable`
    /// is the server's compatibility judgement, not proof of executability.
    pub async fn shells(&self) -> Result<Vec<ConfigShellOption>> {
        self.client
            .get_json("api/config/shell", "config.shells", StatusCode::OK)
            .await
    }
}
/// Provider availability and registered settings, obtained from [`Client::providers`].
#[derive(Clone, Copy, Debug)]
pub struct Providers<'a> {
    pub(crate) client: &'a Client,
}
impl Providers<'_> {
    /// List currently available providers at the client's location (HTTP 200).
    /// Availability depends on activation and connections; this is not a health
    /// check. Retains the resolved location and server order; does not call models.
    pub async fn list(&self) -> Result<ProviderListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// Override location; `Unspecified` omits the query without clearing headers.
    pub async fn list_at(&self, location: Location) -> Result<ProviderListResponse> {
        self.client
            .get_json_at("api/provider", "provider.list", StatusCode::OK, &location)
            .await
    }
    /// Inspect a registered provider, including one absent from the available list.
    /// May contain credentials in settings/headers/body; Debug is redacted.
    /// Missing providers return [`crate::ApiError::ProviderNotFound`].
    pub async fn get(&self, id: &ProviderId) -> Result<ProviderResponse> {
        self.get_at(id, self.client.default_location().clone())
            .await
    }
    /// Inspect at an explicit location. Empty IDs and dot segments fail before
    /// HTTP; other IDs are encoded as one path segment. Only HTTP 200 is accepted.
    pub async fn get_at(&self, id: &ProviderId, location: Location) -> Result<ProviderResponse> {
        if matches!(id.as_str(), "" | "." | "..") {
            return Err(crate::Error::Encode(crate::EncodeError {
                operation: "provider.get",
                source: serde::ser::Error::custom(
                    "provider ID cannot be represented as a path segment",
                ),
            }));
        }
        self.client
            .get_json_segments(
                "api/provider",
                &[id.as_str()],
                "provider.get",
                StatusCode::OK,
                &location.query(),
            )
            .await
    }
}
/// Location reference catalogs, obtained from [`Client::references`].
#[derive(Clone, Copy, Debug)]
pub struct References<'a> {
    pub(crate) client: &'a Client,
}
impl References<'_> {
    /// Read registered local/Git references at the client's location (HTTP 200).
    /// Hidden entries remain included. Paths belong to the server; a Git cache
    /// path does not establish that materialization succeeded. The SDK reads no files.
    pub async fn list(&self) -> Result<ReferenceListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// Override location, preserving the resolved location/data envelope and order.
    pub async fn list_at(&self, location: Location) -> Result<ReferenceListResponse> {
        self.client
            .get_json_at("api/reference", "reference.list", StatusCode::OK, &location)
            .await
    }
}
