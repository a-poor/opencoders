use crate::{Client, Location, Result, StatusCode, models::AgentListResponse};

/// Agent discovery operations, obtained from [`Client::agents`].
#[derive(Clone, Copy, Debug)]
pub struct Agents<'a> {
    pub(crate) client: &'a Client,
}

impl Agents<'_> {
    /// List registered agents at the client's default location.
    ///
    /// Calls `GET /api/agent` (`agent.list`). The response retains the resolved
    /// location alongside the agent list; hidden agents are not filtered out.
    ///
    /// # Errors
    /// Returns transport, HTTP/API, decoding, or response-size errors. Only 200
    /// is accepted. Location and service failures expose typed API error details.
    pub async fn list(&self) -> Result<AgentListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }

    /// List agents at a per-call location, overriding the client default.
    /// [`Location::Unspecified`] omits the query; it does not remove caller headers.
    /// Errors are the same as [`Self::list`].
    pub async fn list_at(&self, location: Location) -> Result<AgentListResponse> {
        self.client
            .get_json_at("api/agent", "agent.list", StatusCode::OK, &location)
            .await
    }
}
