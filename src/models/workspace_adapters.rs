use super::{PtyCreateRequest, PtySize, PtyTicketConnectToken, PtyUpdateRequest};
use std::{collections::HashMap, fmt};

pub(super) fn deserialize_required_nullable<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    <Option<T> as serde::Deserialize>::deserialize(deserializer)
}

pub(super) fn deserialize_positive_integer<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<u64, D::Error> {
    let value = super::deserialize_pid(deserializer)?;
    if value == 0 {
        return Err(serde::de::Error::custom("expected a positive safe integer"));
    }
    Ok(value)
}

pub(super) fn deserialize_optional_safe_integer<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u64>, D::Error> {
    super::deserialize_pid(deserializer).map(Some)
}

impl PtyCreateRequest {
    /// Create an empty request using server defaults.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the executable path/name on the server.
    pub fn command(mut self, value: impl Into<String>) -> Self {
        self.command = Some(value.into());
        self
    }
    /// Set explicit arguments; an empty collection is distinct from omission.
    pub fn args(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// Set the working directory on the server.
    pub fn cwd(mut self, value: impl Into<String>) -> Self {
        self.cwd = Some(value.into());
        self
    }
    /// Set the requested display title.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Set environment overrides; the server also inherits its own environment.
    pub fn env(mut self, value: HashMap<String, String>) -> Self {
        self.env = Some(value);
        self
    }
}
impl fmt::Debug for PtyCreateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PtyCreateRequest").finish_non_exhaustive()
    }
}
impl fmt::Debug for PtyTicketConnectToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PtyTicketConnectToken")
            .field("expires_in", &self.expires_in)
            .finish_non_exhaustive()
    }
}
impl PtyUpdateRequest {
    /// Create an update with all fields omitted.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the title; an empty title is preserved on the wire.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }
    /// Set terminal dimensions. Invalid zero or oversized dimensions fail before HTTP.
    pub fn size(mut self, rows: u64, cols: u64) -> Self {
        self.size = Some(PtySize { rows, cols });
        self
    }
}
