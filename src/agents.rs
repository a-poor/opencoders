use crate::{
    Client, Location, Result, StatusCode,
    models::{AgentId, AgentListResponse, AgentResponse},
};

/// Agent discovery operations, obtained from [`Client::agents`].
#[derive(Clone, Copy, Debug)]
pub struct Agents<'a> {
    pub(crate) client: &'a Client,
}

impl Agents<'_> {
    /// Retrieve a registered agent at the client's default location (`agent.get`).
    /// Retains the resolved location and the agent data envelope.
    ///
    /// # Errors
    /// Returns encode, transport, HTTP/API, decoding, or response-size errors. Only
    /// 200 is accepted. Missing agents expose [`crate::ApiError::AgentNotFound`].
    /// Empty IDs and the URL dot segments `.`/`..` cannot identify an agent path
    /// and fail before HTTP with an encode error.
    pub async fn get(&self, id: &AgentId) -> Result<AgentResponse> {
        self.get_at(id, self.client.default_location().clone())
            .await
    }
    /// Retrieve an agent at a per-call location. [`Location::Unspecified`] omits
    /// the location query without removing caller headers. Errors match [`Self::get`].
    pub async fn get_at(&self, id: &AgentId, location: Location) -> Result<AgentResponse> {
        if matches!(id.as_str(), "" | "." | "..") {
            return Err(crate::Error::Encode(crate::EncodeError {
                operation: "agent.get",
                source: serde::ser::Error::custom(
                    "agent ID cannot be represented as a path segment",
                ),
            }));
        }
        self.client
            .get_json_segments(
                "api/agent",
                &[id.as_str()],
                "agent.get",
                StatusCode::OK,
                &location.query(),
            )
            .await
    }

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
