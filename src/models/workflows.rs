// Generated from openapi/opencode-v2.json by model-spike --workflows.
// Do not edit manually; adaptations live in tools/model-spike/src/workflows.rs.
///`CommandInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CommandInfo {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`CommandListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CommandListResponse {
    pub data: ::std::vec::Vec<CommandInfo>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionCommandRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionCommandRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub agents: Option<::std::vec::Vec<super::PromptInputAgentAttachment>>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub delivery: super::RequestField<super::SessionInboxDelivery>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub files: Option<::std::vec::Vec<super::PromptInputFileAttachment>>,
    pub name: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub skills: Option<::std::vec::Vec<super::PromptInputSkillAttachment>>,
    pub text: ::std::string::String,
}
///`SessionCompactRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionCompactRequest {
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub delivery: super::RequestField<super::SessionInboxDelivery>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub id: super::RequestField<super::SessionMessageId>,
}
///`SessionCompactResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionCompactResponse {
    pub data: super::SessionInboxCompaction,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionContextResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionContextResponse {
    pub data: ::std::vec::Vec<super::SessionMessageInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionDiffResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionDiffResponse {
    pub data: ::std::vec::Vec<super::FileDiffInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionRevertResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionRevertResponse {
    pub data: super::SessionRevert,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionRevertStageRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionRevertStageRequest {
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub files: super::RequestField<bool>,
    #[serde(rename = "messageID")]
    pub message_id: super::SessionMessageId,
}
///`SkillInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SkillInfo {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub autoinvoke: ::std::option::Option<bool>,
    pub content: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub id: super::SkillId,
    pub name: super::SkillName,
    pub path: super::AbsolutePath,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SkillListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SkillListResponse {
    pub data: ::std::vec::Vec<SkillInfo>,
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
