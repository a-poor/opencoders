use crate::{Client, Result, StatusCode, models::LocationPublicInfo};

/// Location selection for routes that accept a working directory.
///
/// Paths refer to the server's filesystem. They are sent verbatim after URL
/// encoding, without consulting the local filesystem or normalizing path syntax.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Location {
    /// Omit the location query. Server fallback rules apply, including any
    /// application-supplied location headers, then the server's working directory.
    /// Used in an explicit override, this bypasses the client's configured location.
    #[default]
    Unspecified,
    /// Send this directory as `location[directory]`.
    Directory(String),
}

impl Location {
    /// Select a directory on the server without local path validation.
    pub fn directory(directory: impl Into<String>) -> Self {
        Self::Directory(directory.into())
    }
}

/// Location discovery operations, obtained from [`Client::location`].
#[derive(Clone, Copy, Debug)]
pub struct Locations<'a> {
    pub(crate) client: &'a Client,
}

impl Locations<'_> {
    /// Resolve the client's default location (`GET /api/location`, `location.get`).
    /// Returns the direct location/project response without a `data` envelope.
    ///
    /// # Errors
    /// Returns transport, HTTP/API, decoding, or response-size errors. Only 200
    /// is accepted. Missing locations expose [`crate::ApiError::LocationNotFound`].
    pub async fn get(&self) -> Result<LocationPublicInfo> {
        self.get_at(self.client.default_location().clone()).await
    }

    /// Resolve a location for this call, overriding the client default.
    /// [`Location::Unspecified`] omits the query; it does not remove caller headers.
    /// Errors are the same as [`Self::get`].
    pub async fn get_at(&self, location: Location) -> Result<LocationPublicInfo> {
        self.client
            .get_json_at("api/location", "location.get", StatusCode::OK, &location)
            .await
    }
}
