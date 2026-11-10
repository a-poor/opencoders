//! Location-aware integration and tool discovery. Applications own refresh policy.
use crate::{Client, Location, Result, StatusCode, models::*};

/// Registered integrations and connection descriptors, from [`Client::integrations`].
#[derive(Clone, Copy, Debug)]
pub struct Integrations<'a> {
    pub(crate) client: &'a Client,
}
impl Integrations<'_> {
    /// List integrations (`integration.list`). On v2.2.5 waits for plugin activation,
    /// sorts by display name and filters denied `integration.connect` entries,
    /// except MCP integrations. Connection descriptors do not contain credential
    /// values; arbitrary metadata/forms can still be sensitive. Debug is redacted.
    pub async fn list(&self) -> Result<IntegrationListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// List at a per-call location, preserving response metadata and server order.
    pub async fn list_at(&self, location: Location) -> Result<IntegrationListResponse> {
        self.client
            .get_json_at(
                "api/integration",
                "integration.list",
                StatusCode::OK,
                &location,
            )
            .await
    }
    /// Inspect a registered integration (`integration.get`). The pinned handler
    /// does not wait for plugin activation or apply the list's policy filter.
    /// A missing registration returns [`crate::ApiError::IntegrationNotFound`].
    pub async fn get(&self, id: &IntegrationId) -> Result<IntegrationResponse> {
        self.get_at(id, self.client.default_location().clone())
            .await
    }
    /// Inspect at a per-call location. IDs are one encoded path segment; empty
    /// and dot-segment IDs fail locally. Only HTTP 200 is accepted.
    pub async fn get_at(
        &self,
        id: &IntegrationId,
        location: Location,
    ) -> Result<IntegrationResponse> {
        if matches!(id.as_str(), "" | "." | "..") {
            return Err(crate::Error::Encode(crate::EncodeError {
                operation: "integration.get",
                source: serde::ser::Error::custom(
                    "integration ID cannot be represented as a path segment",
                ),
            }));
        }
        self.client
            .get_json_segments(
                "api/integration",
                &[id.as_str()],
                "integration.get",
                StatusCode::OK,
                &location.query(),
            )
            .await
    }
}
/// Configured MCP servers and resource discovery, from [`Client::mcp`].
#[derive(Clone, Copy, Debug)]
pub struct Mcp<'a> {
    pub(crate) client: &'a Client,
}
impl Mcp<'_> {
    /// Inspect configured MCP server status (`mcp.list`), sorted by name on
    /// v2.2.5. Startup is asynchronous, so pending or initially empty snapshots
    /// are possible. This is not an aggregate readiness barrier.
    pub async fn list(&self) -> Result<McpListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// Inspect MCP servers at a per-call location.
    pub async fn list_at(&self, location: Location) -> Result<McpListResponse> {
        self.client
            .get_json_at("api/mcp", "mcp.list", StatusCode::OK, &location)
            .await
    }
    /// Retrieve connected servers' resources/templates (`mcp.resource.catalog`).
    /// This can contact MCP servers and invoke upstream retry/recovery. v2.2.5
    /// omits disconnected/failed catalog sources and tolerates unsupported
    /// template listing; empty data is not proof that every source was read.
    /// Does not read resource contents. Results retain server attribution/order.
    pub async fn resource_catalog(&self) -> Result<McpResourceCatalogResponse> {
        self.resource_catalog_at(self.client.default_location().clone())
            .await
    }
    /// Retrieve the aggregate catalog at a per-call location.
    pub async fn resource_catalog_at(
        &self,
        location: Location,
    ) -> Result<McpResourceCatalogResponse> {
        self.client
            .get_json_at(
                "api/mcp/resource",
                "mcp.resource.catalog",
                StatusCode::OK,
                &location,
            )
            .await
    }
}
/// Server plugin inventory, from [`Client::plugins`].
#[derive(Clone, Copy, Debug)]
pub struct Plugins<'a> {
    pub(crate) client: &'a Client,
}
impl Plugins<'_> {
    /// Read current plugin inventory (`plugin.list`), including failed entries.
    /// IDs may be absent for load failures. Flags are optional literal `true`;
    /// omission is preserved. Does not check for updates or await activation.
    pub async fn list(&self) -> Result<PluginListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// Read inventory at a per-call location, preserving server order and sources.
    pub async fn list_at(&self, location: Location) -> Result<PluginListResponse> {
        self.client
            .get_json_at("api/plugin", "plugin.list", StatusCode::OK, &location)
            .await
    }
}
/// Registered web-search providers, from [`Client::websearch`].
#[derive(Clone, Copy, Debug)]
pub struct WebSearch<'a> {
    pub(crate) client: &'a Client,
}
impl WebSearch<'_> {
    /// List provider IDs/names (`websearch.providers`). v2.2.5 sorts by name.
    /// This does not select a provider, check health, or perform a search; a
    /// registered provider can remain listed when web search is disabled.
    pub async fn providers(&self) -> Result<WebSearchProvidersResponse> {
        self.providers_at(self.client.default_location().clone())
            .await
    }
    /// List providers at a per-call location; cold plugin startup may be incomplete.
    pub async fn providers_at(&self, location: Location) -> Result<WebSearchProvidersResponse> {
        self.client
            .get_json_at(
                "api/websearch/provider",
                "websearch.providers",
                StatusCode::OK,
                &location,
            )
            .await
    }
}
