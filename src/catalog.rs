use crate::{
    Client, Location, Result, StatusCode,
    models::{ModelDefaultResponse, ModelListResponse, ModelType},
};

/// Model discovery filters. Empty `types` omits the filter (chat on v2.2.5).
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct ModelListOptions {
    /// Model kinds sent as repeated `types` query parameters, preserving order and
    /// duplicates. Values are open strings; custom plugin types are supported.
    pub types: Vec<ModelType>,
}
impl ModelListOptions {
    /// Replace the requested types. Empty input restores the server default.
    /// Empty string elements are sent unchanged, distinct from an empty list.
    pub fn types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.types = types
            .into_iter()
            .map(|kind| ModelType::from(kind.into()))
            .collect();
        self
    }
}

/// Location-aware model discovery, obtained from [`Client::models`].
#[derive(Clone, Copy, Debug)]
pub struct Models<'a> {
    pub(crate) client: &'a Client,
}
impl Models<'_> {
    /// List the current model snapshot with `GET /api/model` (`model.list`).
    ///
    /// Retains the location/data envelope and server order. Uses the client's
    /// default location; does not wait for catalog changes or filter results locally.
    ///
    /// # Errors
    /// Returns transport, HTTP/API, decoding, or response-size errors. Only 200 is accepted.
    pub async fn list(&self, options: &ModelListOptions) -> Result<ModelListResponse> {
        self.list_at(options, self.client.default_location().clone())
            .await
    }
    /// List models at a per-call location. [`Location::Unspecified`] omits the
    /// query, retaining caller-supplied location headers. Errors match [`Self::list`].
    pub async fn list_at(
        &self,
        options: &ModelListOptions,
        location: Location,
    ) -> Result<ModelListResponse> {
        let mut query = location.query();
        query.extend(
            options
                .types
                .iter()
                .map(|kind| ("types", kind.as_str().to_owned())),
        );
        self.client
            .get_json_query("api/model", "model.list", StatusCode::OK, &query)
            .await
    }
    /// Get the default model (`model.default`) at the client's default location.
    /// The envelope's `data` is `None` when the server returns JSON null.
    /// This is discovery, not an instruction to clear a session's model.
    ///
    /// # Errors
    /// Returns transport, HTTP/API, decoding, or response-size errors. Only 200 is accepted.
    pub async fn default(&self) -> Result<ModelDefaultResponse> {
        self.default_at(self.client.default_location().clone())
            .await
    }
    /// Get the default model at a per-call location. Errors match [`Self::default`].
    pub async fn default_at(&self, location: Location) -> Result<ModelDefaultResponse> {
        self.client
            .get_json_at(
                "api/model/default",
                "model.default",
                StatusCode::OK,
                &location,
            )
            .await
    }
}
