// Generated from openapi/opencode-v2.json by model-spike --tool-discovery.
// Do not edit manually; adaptations live in tools/model-spike/src/tool_discovery.rs.
///`ConnectionInfoCredential`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConnectionInfoCredential {
    pub id: CredentialId,
    pub label: ::std::string::String,
    pub method: ConnectionInfoCredentialMethod,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ConnectionStatus>,
    #[serde(rename = "type")]
    pub type_: ConnectionInfoCredentialType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConnectionInfoCredentialMethod`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum ConnectionInfoCredentialMethod {
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "oauth")]
    Oauth,
    #[serde(rename = "external")]
    External,
}
impl ::std::fmt::Display for ConnectionInfoCredentialMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Key => f.write_str("key"),
            Self::Oauth => f.write_str("oauth"),
            Self::External => f.write_str("external"),
        }
    }
}
impl ::std::str::FromStr for ConnectionInfoCredentialMethod {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "key" => Ok(Self::Key),
            "oauth" => Ok(Self::Oauth),
            "external" => Ok(Self::External),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConnectionInfoCredentialMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConnectionInfoCredentialMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConnectionInfoCredentialType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum ConnectionInfoCredentialType {
    #[serde(rename = "credential")]
    Credential,
}
impl ::std::fmt::Display for ConnectionInfoCredentialType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Credential => f.write_str("credential"),
        }
    }
}
impl ::std::str::FromStr for ConnectionInfoCredentialType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "credential" => Ok(Self::Credential),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConnectionInfoCredentialType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConnectionInfoCredentialType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConnectionInfoEnv`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConnectionInfoEnv {
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ConnectionStatus>,
    #[serde(rename = "type")]
    pub type_: ConnectionInfoEnvType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConnectionInfoEnvType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum ConnectionInfoEnvType {
    #[serde(rename = "env")]
    Env,
}
impl ::std::fmt::Display for ConnectionInfoEnvType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Env => f.write_str("env"),
        }
    }
}
impl ::std::str::FromStr for ConnectionInfoEnvType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "env" => Ok(Self::Env),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConnectionInfoEnvType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConnectionInfoEnvType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConnectionStatus`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConnectionStatus {
    pub message: ::std::string::String,
    pub status: ConnectionStatusStatus,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConnectionStatusStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum ConnectionStatusStatus {
    #[serde(rename = "needs_auth")]
    NeedsAuth,
}
impl ::std::fmt::Display for ConnectionStatusStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NeedsAuth => f.write_str("needs_auth"),
        }
    }
}
impl ::std::str::FromStr for ConnectionStatusStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "needs_auth" => Ok(Self::NeedsAuth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConnectionStatusStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConnectionStatusStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CredentialId`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct CredentialId(pub ::std::string::String);
impl ::std::ops::Deref for CredentialId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CredentialId> for ::std::string::String {
    fn from(value: CredentialId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for CredentialId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for CredentialId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for CredentialId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`IntegrationInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct IntegrationInfo {
    pub connections: ::std::vec::Vec<ConnectionInfo>,
    pub id: super::IntegrationId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub methods: ::std::vec::Vec<IntegrationMethod>,
    pub name: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`IntegrationListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct IntegrationListResponse {
    pub data: ::std::vec::Vec<IntegrationInfo>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`IntegrationMethodCommand`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct IntegrationMethodCommand {
    pub command: ::std::vec::Vec<::std::string::String>,
    pub id: IntegrationMethodId,
    pub label: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: IntegrationMethodCommandType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`IntegrationMethodCommandType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum IntegrationMethodCommandType {
    #[serde(rename = "command")]
    Command,
}
impl ::std::fmt::Display for IntegrationMethodCommandType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Command => f.write_str("command"),
        }
    }
}
impl ::std::str::FromStr for IntegrationMethodCommandType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "command" => Ok(Self::Command),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IntegrationMethodCommandType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IntegrationMethodCommandType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`IntegrationMethodEnv`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct IntegrationMethodEnv {
    pub names: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: IntegrationMethodEnvType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`IntegrationMethodEnvType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum IntegrationMethodEnvType {
    #[serde(rename = "env")]
    Env,
}
impl ::std::fmt::Display for IntegrationMethodEnvType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Env => f.write_str("env"),
        }
    }
}
impl ::std::str::FromStr for IntegrationMethodEnvType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "env" => Ok(Self::Env),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IntegrationMethodEnvType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IntegrationMethodEnvType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`IntegrationMethodExternal`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct IntegrationMethodExternal {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub form: ::std::option::Option<super::FormFields>,
    pub id: IntegrationMethodId,
    pub label: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: IntegrationMethodExternalType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`IntegrationMethodExternalType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum IntegrationMethodExternalType {
    #[serde(rename = "external")]
    External,
}
impl ::std::fmt::Display for IntegrationMethodExternalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::External => f.write_str("external"),
        }
    }
}
impl ::std::str::FromStr for IntegrationMethodExternalType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "external" => Ok(Self::External),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IntegrationMethodExternalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IntegrationMethodExternalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`IntegrationMethodId`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct IntegrationMethodId(pub ::std::string::String);
impl ::std::ops::Deref for IntegrationMethodId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IntegrationMethodId> for ::std::string::String {
    fn from(value: IntegrationMethodId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for IntegrationMethodId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for IntegrationMethodId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for IntegrationMethodId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`IntegrationMethodKey`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct IntegrationMethodKey {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub form: ::std::option::Option<super::FormFields>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: IntegrationMethodKeyType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`IntegrationMethodKeyType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum IntegrationMethodKeyType {
    #[serde(rename = "key")]
    Key,
}
impl ::std::fmt::Display for IntegrationMethodKeyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Key => f.write_str("key"),
        }
    }
}
impl ::std::str::FromStr for IntegrationMethodKeyType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "key" => Ok(Self::Key),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IntegrationMethodKeyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IntegrationMethodKeyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`IntegrationMethodOauth`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct IntegrationMethodOauth {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub form: ::std::option::Option<super::FormFields>,
    pub id: IntegrationMethodId,
    pub label: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: IntegrationMethodOauthType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`IntegrationMethodOauthType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum IntegrationMethodOauthType {
    #[serde(rename = "oauth")]
    Oauth,
}
impl ::std::fmt::Display for IntegrationMethodOauthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Oauth => f.write_str("oauth"),
        }
    }
}
impl ::std::str::FromStr for IntegrationMethodOauthType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "oauth" => Ok(Self::Oauth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for IntegrationMethodOauthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for IntegrationMethodOauthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`IntegrationResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct IntegrationResponse {
    pub data: IntegrationInfo,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpListResponse {
    pub data: ::std::vec::Vec<McpServer>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpResource`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpResource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType", skip_serializing_if = "::std::option::Option::is_none")]
    pub mime_type: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub server: ::std::string::String,
    pub uri: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpResourceCatalog`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpResourceCatalog {
    pub resources: ::std::vec::Vec<McpResource>,
    pub templates: ::std::vec::Vec<McpResourceTemplate>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpResourceCatalogResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpResourceCatalogResponse {
    pub data: McpResourceCatalog,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpResourceTemplate`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpResourceTemplate {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType", skip_serializing_if = "::std::option::Option::is_none")]
    pub mime_type: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub server: ::std::string::String,
    #[serde(rename = "uriTemplate")]
    pub uri_template: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpServer`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpServer {
    #[serde(
        rename = "integrationID",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub integration_id: ::std::option::Option<super::IntegrationId>,
    pub name: ::std::string::String,
    pub status: McpStatus,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpStatusConnected`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpStatusConnected {
    pub status: McpStatusConnectedStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpStatusConnectedStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum McpStatusConnectedStatus {
    #[serde(rename = "connected")]
    Connected,
}
impl ::std::fmt::Display for McpStatusConnectedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Connected => f.write_str("connected"),
        }
    }
}
impl ::std::str::FromStr for McpStatusConnectedStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "connected" => Ok(Self::Connected),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpStatusConnectedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpStatusConnectedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`McpStatusDisabled`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpStatusDisabled {
    pub status: McpStatusDisabledStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpStatusDisabledStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum McpStatusDisabledStatus {
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for McpStatusDisabledStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for McpStatusDisabledStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpStatusDisabledStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpStatusDisabledStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`McpStatusFailed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpStatusFailed {
    pub error: ::std::string::String,
    pub status: McpStatusFailedStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpStatusFailedStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum McpStatusFailedStatus {
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for McpStatusFailedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for McpStatusFailedStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpStatusFailedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpStatusFailedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`McpStatusNeedsAuth`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpStatusNeedsAuth {
    pub error: ::std::string::String,
    pub status: McpStatusNeedsAuthStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpStatusNeedsAuthStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum McpStatusNeedsAuthStatus {
    #[serde(rename = "needs_auth")]
    NeedsAuth,
}
impl ::std::fmt::Display for McpStatusNeedsAuthStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NeedsAuth => f.write_str("needs_auth"),
        }
    }
}
impl ::std::str::FromStr for McpStatusNeedsAuthStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "needs_auth" => Ok(Self::NeedsAuth),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpStatusNeedsAuthStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpStatusNeedsAuthStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`McpStatusPending`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpStatusPending {
    pub status: McpStatusPendingStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpStatusPendingStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum McpStatusPendingStatus {
    #[serde(rename = "pending")]
    Pending,
}
impl ::std::fmt::Display for McpStatusPendingStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => f.write_str("pending"),
        }
    }
}
impl ::std::str::FromStr for McpStatusPendingStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pending" => Ok(Self::Pending),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpStatusPendingStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpStatusPendingStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PluginFeatures`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginFeatures {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub gui: ::std::option::Option<super::SchemaBoolean<true>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub rpc: ::std::option::Option<super::SchemaBoolean<true>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub server: ::std::option::Option<super::SchemaBoolean<true>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub tui: ::std::option::Option<super::SchemaBoolean<true>>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginId`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct PluginId(pub ::std::string::String);
impl ::std::ops::Deref for PluginId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PluginId> for ::std::string::String {
    fn from(value: PluginId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for PluginId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for PluginId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for PluginId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`PluginInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginInfo {
    pub features: PluginFeatures,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<PluginId>,
    pub source: PluginSource,
    pub state: PluginState,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginListResponse {
    pub data: ::std::vec::Vec<PluginInfo>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginSourceBuiltin`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginSourceBuiltin {
    #[serde(rename = "type")]
    pub type_: PluginSourceBuiltinType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginSourceBuiltinType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum PluginSourceBuiltinType {
    #[serde(rename = "builtin")]
    Builtin,
}
impl ::std::fmt::Display for PluginSourceBuiltinType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Builtin => f.write_str("builtin"),
        }
    }
}
impl ::std::str::FromStr for PluginSourceBuiltinType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "builtin" => Ok(Self::Builtin),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginSourceBuiltinType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginSourceBuiltinType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PluginSourceLocal`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginSourceLocal {
    pub path: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: PluginSourceLocalType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginSourceLocalType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum PluginSourceLocalType {
    #[serde(rename = "local")]
    Local,
}
impl ::std::fmt::Display for PluginSourceLocalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Local => f.write_str("local"),
        }
    }
}
impl ::std::str::FromStr for PluginSourceLocalType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "local" => Ok(Self::Local),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginSourceLocalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginSourceLocalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PluginSourcePackage`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginSourcePackage {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub outdated: ::std::option::Option<super::SchemaBoolean<true>>,
    pub target: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: PluginSourcePackageType,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub updating: ::std::option::Option<super::SchemaBoolean<true>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginSourcePackageType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum PluginSourcePackageType {
    #[serde(rename = "package")]
    Package,
}
impl ::std::fmt::Display for PluginSourcePackageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Package => f.write_str("package"),
        }
    }
}
impl ::std::str::FromStr for PluginSourcePackageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "package" => Ok(Self::Package),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginSourcePackageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginSourcePackageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PluginSourceSdk`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginSourceSdk {
    #[serde(rename = "type")]
    pub type_: PluginSourceSdkType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginSourceSdkType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum PluginSourceSdkType {
    #[serde(rename = "sdk")]
    Sdk,
}
impl ::std::fmt::Display for PluginSourceSdkType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sdk => f.write_str("sdk"),
        }
    }
}
impl ::std::str::FromStr for PluginSourceSdkType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "sdk" => Ok(Self::Sdk),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginSourceSdkType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginSourceSdkType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PluginStateActive`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginStateActive {
    pub status: PluginStateActiveStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginStateActiveStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum PluginStateActiveStatus {
    #[serde(rename = "active")]
    Active,
}
impl ::std::fmt::Display for PluginStateActiveStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => f.write_str("active"),
        }
    }
}
impl ::std::str::FromStr for PluginStateActiveStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginStateActiveStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginStateActiveStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PluginStateFailed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct PluginStateFailed {
    pub error: ::std::string::String,
    #[serde(rename = "ref", skip_serializing_if = "::std::option::Option::is_none")]
    pub ref_: ::std::option::Option<::std::string::String>,
    pub status: PluginStateFailedStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PluginStateFailedStatus`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum PluginStateFailedStatus {
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for PluginStateFailedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for PluginStateFailedStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "failed" => Ok(Self::Failed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PluginStateFailedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PluginStateFailedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`WebSearchProvider`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct WebSearchProvider {
    pub id: super::WebSearchId,
    pub name: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`WebSearchProvidersResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct WebSearchProvidersResponse {
    pub data: ::std::vec::Vec<WebSearchProvider>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
/// Typed wire event or payload; malformed known tags fail decoding.
#[derive(Clone, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum IntegrationMethod {
    /// Wire kind `oauth`.
    Oauth(Box<IntegrationMethodOauth>),
    /// Wire kind `command`.
    Command(Box<IntegrationMethodCommand>),
    /// Wire kind `key`.
    Key(Box<IntegrationMethodKey>),
    /// Wire kind `external`.
    External(Box<IntegrationMethodExternal>),
    /// Wire kind `env`.
    Env(Box<IntegrationMethodEnv>),
}
impl<'de> ::serde::Deserialize<'de> for IntegrationMethod {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("oauth") => ::serde_json::from_value(raw).map(Self::Oauth),
            Some("command") => ::serde_json::from_value(raw).map(Self::Command),
            Some("key") => ::serde_json::from_value(raw).map(Self::Key),
            Some("external") => ::serde_json::from_value(raw).map(Self::External),
            Some("env") => ::serde_json::from_value(raw).map(Self::Env),
            _ => {
                return Err(
                    ::serde::de::Error::custom("unknown or missing event/payload type"),
                );
            }
        }
            .map_err(::serde::de::Error::custom)
    }
}
/// Typed wire event or payload; malformed known tags fail decoding.
#[derive(Clone, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConnectionInfo {
    /// Wire kind `credential`.
    Credential(Box<ConnectionInfoCredential>),
    /// Wire kind `env`.
    Env(Box<ConnectionInfoEnv>),
}
impl<'de> ::serde::Deserialize<'de> for ConnectionInfo {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("credential") => ::serde_json::from_value(raw).map(Self::Credential),
            Some("env") => ::serde_json::from_value(raw).map(Self::Env),
            _ => {
                return Err(
                    ::serde::de::Error::custom("unknown or missing event/payload type"),
                );
            }
        }
            .map_err(::serde::de::Error::custom)
    }
}
/// Typed wire event or payload; malformed known tags fail decoding.
#[derive(Clone, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum McpStatus {
    /// Wire kind `connected`.
    Connected(Box<McpStatusConnected>),
    /// Wire kind `pending`.
    Pending(Box<McpStatusPending>),
    /// Wire kind `disabled`.
    Disabled(Box<McpStatusDisabled>),
    /// Wire kind `failed`.
    Failed(Box<McpStatusFailed>),
    /// Wire kind `needs_auth`.
    NeedsAuth(Box<McpStatusNeedsAuth>),
}
impl<'de> ::serde::Deserialize<'de> for McpStatus {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("status").and_then(::serde_json::Value::as_str) {
            Some("connected") => ::serde_json::from_value(raw).map(Self::Connected),
            Some("pending") => ::serde_json::from_value(raw).map(Self::Pending),
            Some("disabled") => ::serde_json::from_value(raw).map(Self::Disabled),
            Some("failed") => ::serde_json::from_value(raw).map(Self::Failed),
            Some("needs_auth") => ::serde_json::from_value(raw).map(Self::NeedsAuth),
            _ => {
                return Err(
                    ::serde::de::Error::custom("unknown or missing event/payload type"),
                );
            }
        }
            .map_err(::serde::de::Error::custom)
    }
}
/// Typed wire event or payload; malformed known tags fail decoding.
#[derive(Clone, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PluginSource {
    /// Wire kind `builtin`.
    Builtin(Box<PluginSourceBuiltin>),
    /// Wire kind `package`.
    Package(Box<PluginSourcePackage>),
    /// Wire kind `local`.
    Local(Box<PluginSourceLocal>),
    /// Wire kind `sdk`.
    Sdk(Box<PluginSourceSdk>),
}
impl<'de> ::serde::Deserialize<'de> for PluginSource {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("builtin") => ::serde_json::from_value(raw).map(Self::Builtin),
            Some("package") => ::serde_json::from_value(raw).map(Self::Package),
            Some("local") => ::serde_json::from_value(raw).map(Self::Local),
            Some("sdk") => ::serde_json::from_value(raw).map(Self::Sdk),
            _ => {
                return Err(
                    ::serde::de::Error::custom("unknown or missing event/payload type"),
                );
            }
        }
            .map_err(::serde::de::Error::custom)
    }
}
/// Typed wire event or payload; malformed known tags fail decoding.
#[derive(Clone, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PluginState {
    /// Wire kind `active`.
    Active(Box<PluginStateActive>),
    /// Wire kind `failed`.
    Failed(Box<PluginStateFailed>),
}
impl<'de> ::serde::Deserialize<'de> for PluginState {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("status").and_then(::serde_json::Value::as_str) {
            Some("active") => ::serde_json::from_value(raw).map(Self::Active),
            Some("failed") => ::serde_json::from_value(raw).map(Self::Failed),
            _ => {
                return Err(
                    ::serde::de::Error::custom("unknown or missing event/payload type"),
                );
            }
        }
            .map_err(::serde::de::Error::custom)
    }
}
impl ::std::fmt::Debug for ConnectionInfoCredential {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConnectionInfoCredential { .. }")
    }
}
impl ::std::fmt::Debug for ConnectionInfoCredentialMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConnectionInfoCredentialMethod { .. }")
    }
}
impl ::std::fmt::Debug for ConnectionInfoCredentialType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConnectionInfoCredentialType { .. }")
    }
}
impl ::std::fmt::Debug for ConnectionInfoEnv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConnectionInfoEnv { .. }")
    }
}
impl ::std::fmt::Debug for ConnectionInfoEnvType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConnectionInfoEnvType { .. }")
    }
}
impl ::std::fmt::Debug for ConnectionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConnectionStatus { .. }")
    }
}
impl ::std::fmt::Debug for ConnectionStatusStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConnectionStatusStatus { .. }")
    }
}
impl ::std::fmt::Debug for CredentialId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialId { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationInfo { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationListResponse { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodCommand { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodCommandType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodCommandType { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodEnv {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodEnv { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodEnvType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodEnvType { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodExternal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodExternal { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodExternalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodExternalType { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodId { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodKey { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodKeyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodKeyType { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodOauth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodOauth { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethodOauthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethodOauthType { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationResponse { .. }")
    }
}
impl ::std::fmt::Debug for McpListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpListResponse { .. }")
    }
}
impl ::std::fmt::Debug for McpResource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpResource { .. }")
    }
}
impl ::std::fmt::Debug for McpResourceCatalog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpResourceCatalog { .. }")
    }
}
impl ::std::fmt::Debug for McpResourceCatalogResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpResourceCatalogResponse { .. }")
    }
}
impl ::std::fmt::Debug for McpResourceTemplate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpResourceTemplate { .. }")
    }
}
impl ::std::fmt::Debug for McpServer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpServer { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusConnected {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusConnected { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusConnectedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusConnectedStatus { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusDisabled {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusDisabled { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusDisabledStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusDisabledStatus { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusFailed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusFailed { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusFailedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusFailedStatus { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusNeedsAuth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusNeedsAuth { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusNeedsAuthStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusNeedsAuthStatus { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusPending {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusPending { .. }")
    }
}
impl ::std::fmt::Debug for McpStatusPendingStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatusPendingStatus { .. }")
    }
}
impl ::std::fmt::Debug for PluginFeatures {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginFeatures { .. }")
    }
}
impl ::std::fmt::Debug for PluginId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginId { .. }")
    }
}
impl ::std::fmt::Debug for PluginInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginInfo { .. }")
    }
}
impl ::std::fmt::Debug for PluginListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginListResponse { .. }")
    }
}
impl ::std::fmt::Debug for PluginSourceBuiltin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSourceBuiltin { .. }")
    }
}
impl ::std::fmt::Debug for PluginSourceBuiltinType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSourceBuiltinType { .. }")
    }
}
impl ::std::fmt::Debug for PluginSourceLocal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSourceLocal { .. }")
    }
}
impl ::std::fmt::Debug for PluginSourceLocalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSourceLocalType { .. }")
    }
}
impl ::std::fmt::Debug for PluginSourcePackage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSourcePackage { .. }")
    }
}
impl ::std::fmt::Debug for PluginSourcePackageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSourcePackageType { .. }")
    }
}
impl ::std::fmt::Debug for PluginSourceSdk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSourceSdk { .. }")
    }
}
impl ::std::fmt::Debug for PluginSourceSdkType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSourceSdkType { .. }")
    }
}
impl ::std::fmt::Debug for PluginStateActive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginStateActive { .. }")
    }
}
impl ::std::fmt::Debug for PluginStateActiveStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginStateActiveStatus { .. }")
    }
}
impl ::std::fmt::Debug for PluginStateFailed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginStateFailed { .. }")
    }
}
impl ::std::fmt::Debug for PluginStateFailedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginStateFailedStatus { .. }")
    }
}
impl ::std::fmt::Debug for WebSearchProvider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("WebSearchProvider { .. }")
    }
}
impl ::std::fmt::Debug for WebSearchProvidersResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("WebSearchProvidersResponse { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationMethod { .. }")
    }
}
impl ::std::fmt::Debug for ConnectionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConnectionInfo { .. }")
    }
}
impl ::std::fmt::Debug for McpStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpStatus { .. }")
    }
}
impl ::std::fmt::Debug for PluginSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginSource { .. }")
    }
}
impl ::std::fmt::Debug for PluginState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("PluginState { .. }")
    }
}
