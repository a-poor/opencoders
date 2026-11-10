// Generated from openapi/opencode-v2.json by model-spike --credentials.
// Do not edit manually; adaptations live in tools/model-spike/src/credentials.rs.
///`CredentialCreateRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CredentialCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub activate: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub id: Option<super::CredentialId>,
    #[serde(rename = "integrationID")]
    pub integration_id: super::IntegrationId,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub label: Option<String>,
    pub value: CredentialValueInput,
}
///`CredentialEntry`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct CredentialEntry {
    pub active: bool,
    pub id: super::CredentialId,
    #[serde(rename = "integrationID")]
    pub integration_id: super::IntegrationId,
    pub label: ::std::string::String,
    pub value: CredentialValue,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`CredentialExternal`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct CredentialExternal {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(rename = "methodID")]
    pub method_id: super::IntegrationMethodId,
    #[serde(rename = "type")]
    pub type_: CredentialExternalType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`CredentialExternalInput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CredentialExternalInput {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(rename = "methodID")]
    pub method_id: super::IntegrationMethodId,
    #[serde(rename = "type")]
    pub type_: CredentialExternalInputType,
}
///`CredentialExternalInputType`
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
pub enum CredentialExternalInputType {
    #[serde(rename = "external")]
    External,
}
impl ::std::fmt::Display for CredentialExternalInputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::External => f.write_str("external"),
        }
    }
}
impl ::std::str::FromStr for CredentialExternalInputType {
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
impl ::std::convert::TryFrom<&str> for CredentialExternalInputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CredentialExternalInputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CredentialExternalType`
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
pub enum CredentialExternalType {
    #[serde(rename = "external")]
    External,
}
impl ::std::fmt::Display for CredentialExternalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::External => f.write_str("external"),
        }
    }
}
impl ::std::str::FromStr for CredentialExternalType {
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
impl ::std::convert::TryFrom<&str> for CredentialExternalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CredentialExternalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CredentialKey`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct CredentialKey {
    pub key: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(rename = "type")]
    pub type_: CredentialKeyType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`CredentialKeyInput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CredentialKeyInput {
    pub key: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(rename = "type")]
    pub type_: CredentialKeyInputType,
}
///`CredentialKeyInputType`
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
pub enum CredentialKeyInputType {
    #[serde(rename = "key")]
    Key,
}
impl ::std::fmt::Display for CredentialKeyInputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Key => f.write_str("key"),
        }
    }
}
impl ::std::str::FromStr for CredentialKeyInputType {
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
impl ::std::convert::TryFrom<&str> for CredentialKeyInputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CredentialKeyInputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CredentialKeyType`
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
pub enum CredentialKeyType {
    #[serde(rename = "key")]
    Key,
}
impl ::std::fmt::Display for CredentialKeyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Key => f.write_str("key"),
        }
    }
}
impl ::std::str::FromStr for CredentialKeyType {
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
impl ::std::convert::TryFrom<&str> for CredentialKeyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CredentialKeyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CredentialListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct CredentialListResponse {
    pub data: ::std::vec::Vec<CredentialEntry>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`CredentialOauth`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct CredentialOauth {
    pub access: ::std::string::String,
    pub expires: super::CredentialExpires,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(rename = "methodID")]
    pub method_id: super::IntegrationMethodId,
    pub refresh: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: CredentialOauthType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`CredentialOauthInput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CredentialOauthInput {
    pub access: ::std::string::String,
    pub expires: super::CredentialExpires,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(rename = "methodID")]
    pub method_id: super::IntegrationMethodId,
    pub refresh: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: CredentialOauthInputType,
}
///`CredentialOauthInputType`
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
pub enum CredentialOauthInputType {
    #[serde(rename = "oauth")]
    Oauth,
}
impl ::std::fmt::Display for CredentialOauthInputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Oauth => f.write_str("oauth"),
        }
    }
}
impl ::std::str::FromStr for CredentialOauthInputType {
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
impl ::std::convert::TryFrom<&str> for CredentialOauthInputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CredentialOauthInputType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CredentialOauthType`
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
pub enum CredentialOauthType {
    #[serde(rename = "oauth")]
    Oauth,
}
impl ::std::fmt::Display for CredentialOauthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Oauth => f.write_str("oauth"),
        }
    }
}
impl ::std::str::FromStr for CredentialOauthType {
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
impl ::std::convert::TryFrom<&str> for CredentialOauthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CredentialOauthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CredentialResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct CredentialResponse {
    pub data: CredentialEntry,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`CredentialUpdateRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CredentialUpdateRequest {
    pub label: ::std::string::String,
}
///`IntegrationConnectExternalRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct IntegrationConnectExternalRequest {
    #[serde(default, skip_serializing_if = "crate::RequestField::is_omitted")]
    pub answer: crate::RequestField<super::FormAnswer>,
    #[serde(default, skip_serializing_if = "crate::RequestField::is_omitted")]
    pub label: crate::RequestField<String>,
    #[serde(rename = "methodID")]
    pub method_id: super::IntegrationMethodId,
}
///`IntegrationConnectKeyRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct IntegrationConnectKeyRequest {
    #[serde(default, skip_serializing_if = "crate::RequestField::is_omitted")]
    pub answer: crate::RequestField<super::FormAnswer>,
    pub key: ::std::string::String,
    #[serde(default, skip_serializing_if = "crate::RequestField::is_omitted")]
    pub label: crate::RequestField<String>,
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
pub enum CredentialValue {
    /// Wire kind `oauth`.
    Oauth(Box<CredentialOauth>),
    /// Wire kind `key`.
    Key(Box<CredentialKey>),
    /// Wire kind `external`.
    External(Box<CredentialExternal>),
}
impl<'de> ::serde::Deserialize<'de> for CredentialValue {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("oauth") => ::serde_json::from_value(raw).map(Self::Oauth),
            Some("key") => ::serde_json::from_value(raw).map(Self::Key),
            Some("external") => ::serde_json::from_value(raw).map(Self::External),
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
pub enum CredentialValueInput {
    /// Wire kind `oauth`.
    Oauth(Box<CredentialOauthInput>),
    /// Wire kind `key`.
    Key(Box<CredentialKeyInput>),
    /// Wire kind `external`.
    External(Box<CredentialExternalInput>),
}
impl<'de> ::serde::Deserialize<'de> for CredentialValueInput {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("oauth") => ::serde_json::from_value(raw).map(Self::Oauth),
            Some("key") => ::serde_json::from_value(raw).map(Self::Key),
            Some("external") => ::serde_json::from_value(raw).map(Self::External),
            _ => {
                return Err(
                    ::serde::de::Error::custom("unknown or missing event/payload type"),
                );
            }
        }
            .map_err(::serde::de::Error::custom)
    }
}
impl ::std::fmt::Debug for CredentialCreateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialCreateRequest { .. }")
    }
}
impl ::std::fmt::Debug for CredentialEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialEntry { .. }")
    }
}
impl ::std::fmt::Debug for CredentialExternal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialExternal { .. }")
    }
}
impl ::std::fmt::Debug for CredentialExternalInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialExternalInput { .. }")
    }
}
impl ::std::fmt::Debug for CredentialExternalInputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialExternalInputType { .. }")
    }
}
impl ::std::fmt::Debug for CredentialExternalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialExternalType { .. }")
    }
}
impl ::std::fmt::Debug for CredentialKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialKey { .. }")
    }
}
impl ::std::fmt::Debug for CredentialKeyInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialKeyInput { .. }")
    }
}
impl ::std::fmt::Debug for CredentialKeyInputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialKeyInputType { .. }")
    }
}
impl ::std::fmt::Debug for CredentialKeyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialKeyType { .. }")
    }
}
impl ::std::fmt::Debug for CredentialListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialListResponse { .. }")
    }
}
impl ::std::fmt::Debug for CredentialOauth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialOauth { .. }")
    }
}
impl ::std::fmt::Debug for CredentialOauthInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialOauthInput { .. }")
    }
}
impl ::std::fmt::Debug for CredentialOauthInputType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialOauthInputType { .. }")
    }
}
impl ::std::fmt::Debug for CredentialOauthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialOauthType { .. }")
    }
}
impl ::std::fmt::Debug for CredentialResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialResponse { .. }")
    }
}
impl ::std::fmt::Debug for CredentialUpdateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialUpdateRequest { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationConnectExternalRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationConnectExternalRequest { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationConnectKeyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationConnectKeyRequest { .. }")
    }
}
impl ::std::fmt::Debug for CredentialValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialValue { .. }")
    }
}
impl ::std::fmt::Debug for CredentialValueInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("CredentialValueInput { .. }")
    }
}
