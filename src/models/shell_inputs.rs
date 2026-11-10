use super::{SessionEnvironmentRequest, SessionMessageId, SessionShellRequest, ShellCreateRequest};
use crate::RequestField;
use serde::Serialize;
use std::collections::HashMap;

impl ShellCreateRequest {
    /// Execute a command string using the server's selected shell.
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            cwd: None,
            timeout: None,
            metadata: None,
        }
    }
    /// Set the process working directory. v2.2.5 treats an empty string as omitted.
    pub fn cwd(mut self, cwd: impl Into<String>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }
    /// Set a server execution timeout in milliseconds; zero/omission disables it.
    /// Values above 2^53−1 are rejected during serialization, before sending.
    pub fn timeout_ms(mut self, milliseconds: u64) -> Self {
        self.timeout = Some(milliseconds);
        self
    }
    /// Attach opaque metadata echoed by records/events. On v2.2.5 a `sessionID`
    /// tag also selects that session's local environment if one was installed.
    pub fn metadata(mut self, metadata: serde_json::Map<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}
pub(super) fn serialize_timeout<S: serde::Serializer>(
    value: &Option<u64>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if value.is_some_and(|n| n > 9_007_199_254_740_991) {
        return Err(serde::ser::Error::custom(
            "shell timeout exceeds safe integer range",
        ));
    }
    value.serialize(serializer)
}
impl SessionShellRequest {
    /// Run a user command and record its shell history plus a parked synthetic
    /// completion notification. This does not wake model execution on v2.2.5.
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            id: RequestField::Omitted,
        }
    }
    /// Supply the shell message ID. This is not a deduplication/retry guarantee.
    /// Assign `RequestField::Null` directly to explicitly send null.
    pub fn id(mut self, id: SessionMessageId) -> Self {
        self.id = RequestField::Value(id);
        self
    }
}
impl SessionEnvironmentRequest {
    /// Replace the session's local command environment. An empty map is an
    /// explicit empty replacement, not a reset to the server's environment.
    /// The server injects terminal markers and the selected shell may add values.
    pub fn new(variables: HashMap<String, String>) -> Self {
        Self { variables }
    }
}
