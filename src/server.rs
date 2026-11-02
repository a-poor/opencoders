use crate::{Client, Result, StatusCode, models::ServerInfo};

/// Server-wide operations, obtained from [`Client::server`].
#[derive(Clone, Copy, Debug)]
pub struct Server<'a> {
    pub(crate) client: &'a Client,
}

impl Server<'_> {
    /// Retrieve server identity, advertised URLs, paths, and capabilities.
    ///
    /// Calls `GET /api/info` (`server.info`). This is a direct JSON response;
    /// it has no `data` envelope. Dropping the future cancels local consumption.
    ///
    /// # Errors
    /// Returns a transport error, an unexpected-status HTTP error, a decoding
    /// error, or a response-size error. Only HTTP 200 is accepted for this route.
    pub async fn info(&self) -> Result<ServerInfo> {
        self.client
            .get_json("api/info", "server.info", StatusCode::OK)
            .await
    }
}
