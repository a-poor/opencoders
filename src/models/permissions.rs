// Generated from openapi/opencode-v2.json by model-spike --permissions.
// Do not edit manually; adaptations live in tools/model-spike/src/permissions.rs.
///`PermissionRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PermissionRequest {
    pub action: ::std::string::String,
    pub id: super::PermissionId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub resources: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub save: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub source: ::std::option::Option<super::PermissionSource>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PermissionRequestListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PermissionRequestListResponse {
    pub data: ::std::vec::Vec<PermissionRequest>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PermissionSavedId`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[serde(transparent)]
pub struct PermissionSavedId(pub ::std::string::String);
impl ::std::ops::Deref for PermissionSavedId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PermissionSavedId> for ::std::string::String {
    fn from(value: PermissionSavedId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for PermissionSavedId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for PermissionSavedId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for PermissionSavedId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`PermissionSavedInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PermissionSavedInfo {
    pub action: ::std::string::String,
    pub id: PermissionSavedId,
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
    pub resource: ::std::string::String,
    pub time: PermissionSavedInfoTime,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PermissionSavedInfoTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PermissionSavedInfoTime {
    pub created: ::serde_json::Number,
    pub updated: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PermissionSavedListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PermissionSavedListResponse {
    pub data: ::std::vec::Vec<PermissionSavedInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PermissionSavedResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PermissionSavedResponse {
    pub data: PermissionSavedInfo,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PermissionSavedUpdateRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PermissionSavedUpdateRequest {
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub action: super::RequestField<::std::string::String>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub resource: super::RequestField<::std::string::String>,
}
///`PermissionSourceInput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", deny_unknown_fields)]
#[non_exhaustive]
pub enum PermissionSourceInput {
    #[serde(rename = "tool")]
    #[non_exhaustive]
    Tool {
        id: ::std::string::String,
        #[serde(rename = "messageID")]
        message_id: ::std::string::String,
    },
}
///`SessionPermissionCreateRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionPermissionCreateRequest {
    pub action: ::std::string::String,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub agent: super::RequestField<super::AgentId>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub id: super::RequestField<super::PermissionId>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    pub resources: ::std::vec::Vec<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub save: Option<::std::vec::Vec<::std::string::String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub source: Option<PermissionSourceInput>,
}
///`SessionPermissionCreateResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionPermissionCreateResponse {
    pub data: SessionPermissionCreateResponseData,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionPermissionCreateResponseData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionPermissionCreateResponseData {
    pub effect: super::PermissionEffect,
    pub id: super::PermissionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionPermissionListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionPermissionListResponse {
    pub data: ::std::vec::Vec<PermissionRequest>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionPermissionReplyRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionPermissionReplyRequest {
    pub decision: super::PermissionReply,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub message: super::RequestField<::std::string::String>,
}
///`SessionPermissionResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionPermissionResponse {
    pub data: PermissionRequest,
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
