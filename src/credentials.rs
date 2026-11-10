use crate::{Client, Integrations, Location, Result, StatusCode, models::*};
use reqwest::Method;

/// Global stored credentials, obtained from [`Client::credentials`].
/// These routes ignore the SDK default location and have no per-location variants.
/// Returned fields and serialization contain secret values; Debug is redacted.
#[derive(Clone, Copy, Debug)]
pub struct Credentials<'a> {
    pub(crate) client: &'a Client,
}
impl Credentials<'_> {
    /// List every stored credential (`credential.list`), including secret values.
    /// Requires HTTP 200 and retains envelope/record extensions and server order.
    /// On v2.2.5 each integration's selected credential is its last listed entry.
    pub async fn list(&self) -> Result<CredentialListResponse> {
        self.client
            .get_json_query("api/credential", "credential.list", StatusCode::OK, &[])
            .await
    }
    /// Store a credential (`credential.create`), accepting only HTTP 200.
    /// Omitted activation selects it; false preserves an existing selection, but
    /// still selects the first credential. Omitted labels default to `default`.
    /// v2.2.5 checks global `integration.connect` policy, without requiring a
    /// registered integration. Existing IDs conflict. Never automatically retries.
    pub async fn create(&self, request: &CredentialCreateRequest) -> Result<CredentialResponse> {
        self.client
            .post_json(
                "api/credential",
                "credential.create",
                StatusCode::OK,
                request,
            )
            .await
    }
    /// Rename a credential (`credential.update`); this cannot change its value.
    /// Requires HTTP 204. Missing IDs succeed on v2.2.5. Never retries.
    pub async fn update(&self, id: &CredentialId, request: &CredentialUpdateRequest) -> Result<()> {
        validate_path_id(id.as_str(), "credential.update")?;
        self.client
            .patch_json_empty(
                "api/credential",
                &[id.as_str()],
                "credential.update",
                request,
            )
            .await
    }
    /// Select a credential (`credential.activate`), accepting only HTTP 204.
    /// Missing or already-selected IDs succeed on v2.2.5. Never retries.
    pub async fn activate(&self, id: &CredentialId) -> Result<()> {
        validate_path_id(id.as_str(), "credential.activate")?;
        let request = self.client.request(
            Method::POST,
            self.client
                .segments_url("api/credential", &[id.as_str(), "activate"]),
        );
        self.client.send_empty(request, "credential.activate").await
    }
    /// Delete a credential (`credential.remove`), accepting only HTTP 204.
    /// On v2.2.5 missing IDs succeed; deleting the selected credential chooses the
    /// newest remaining credential, or clears selection. Does not revoke tokens
    /// at their provider. Never retries, including after an ambiguous response.
    pub async fn remove(&self, id: &CredentialId) -> Result<()> {
        validate_path_id(id.as_str(), "credential.remove")?;
        self.client
            .delete_empty("api/credential", &[id.as_str()], "credential.remove")
            .await
    }
}

impl Integrations<'_> {
    /// Validate key-method answers and store a credential (`integration.connect.key`).
    /// Requires HTTP 204; does not return the new credential ID. On v2.2.5 this
    /// stores the key and supplied form metadata without testing provider access.
    /// Does not wait for plugin activation. Never automatically retries.
    pub async fn connect_key(
        &self,
        id: &IntegrationId,
        request: &IntegrationConnectKeyRequest,
    ) -> Result<()> {
        self.connect_key_at(id, request, self.client.default_location().clone())
            .await
    }
    /// Connect using the key method at an explicit location. Omitted and null
    /// answers/labels retain their wire distinction; the server chooses defaults.
    pub async fn connect_key_at(
        &self,
        id: &IntegrationId,
        request: &IntegrationConnectKeyRequest,
        location: Location,
    ) -> Result<()> {
        self.connect_direct(id, "key", "integration.connect.key", request, location)
            .await
    }
    /// Store an external credential reference (`integration.connect.external`).
    /// Requires HTTP 204. v2.2.5 validates method/form inputs and stores method ID
    /// and supplied metadata without resolving the external secret. It does not
    /// wait for plugin activation. Credential storage is global. Never retries.
    pub async fn connect_external(
        &self,
        id: &IntegrationId,
        request: &IntegrationConnectExternalRequest,
    ) -> Result<()> {
        self.connect_external_at(id, request, self.client.default_location().clone())
            .await
    }
    /// Connect using a named external method at a per-call location.
    pub async fn connect_external_at(
        &self,
        id: &IntegrationId,
        request: &IntegrationConnectExternalRequest,
        location: Location,
    ) -> Result<()> {
        self.connect_direct(
            id,
            "external",
            "integration.connect.external",
            request,
            location,
        )
        .await
    }
    async fn connect_direct<B: serde::Serialize>(
        &self,
        id: &IntegrationId,
        kind: &str,
        operation: &'static str,
        body: &B,
        location: Location,
    ) -> Result<()> {
        validate_path_id(id.as_str(), operation)?;
        let mut url = self
            .client
            .segments_url("api/integration", &[id.as_str(), "connect", kind]);
        let query = location.query();
        if !query.is_empty() {
            url.query_pairs_mut()
                .extend_pairs(query.iter().map(|(key, value)| (*key, value.as_str())));
        }
        let request = self
            .client
            .json_request(Method::POST, url, operation, body)?;
        self.client.send_empty(request, operation).await
    }
}
fn validate_path_id(id: &str, operation: &'static str) -> Result<()> {
    if matches!(id, "" | "." | "..") {
        return Err(crate::Error::Encode(crate::EncodeError {
            operation,
            source: serde::ser::Error::custom("ID cannot be represented as a path segment"),
        }));
    }
    Ok(())
}
