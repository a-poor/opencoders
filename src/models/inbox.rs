// Generated from openapi/opencode-v2.json by model-spike --inbox.
// Do not edit manually; adaptations live in tools/model-spike/src/inbox.rs.
///`PromptInputAgentAttachment`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PromptInputAgentAttachment {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub mention: Option<PromptInputMention>,
    pub name: ::std::string::String,
}
///`PromptInputFileAttachment`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PromptInputFileAttachment {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub description: Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub mention: Option<PromptInputMention>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub name: Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub source: Option<PromptInputFileSource>,
    pub uri: ::std::string::String,
}
///`PromptInputFileSource`
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "uri")]
#[non_exhaustive]
pub enum PromptInputFileSource {
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "uri")]
    Uri(::std::string::String),
}
///`PromptInputMention`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PromptInputMention {
    pub end: ::serde_json::Number,
    pub start: ::serde_json::Number,
    pub text: ::std::string::String,
}
///`PromptInputSkillAttachment`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PromptInputSkillAttachment {
    pub id: super::SkillId,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub mention: Option<PromptInputMention>,
}
///`SessionInboxCompaction`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxCompaction {
    pub delivery: SessionInboxDelivery,
    pub id: super::SessionMessageId,
    pub payload: super::NonNullValue,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub time: SessionInboxCompactionTime,
    #[serde(rename = "type")]
    pub type_: SessionInboxCompactionType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxCompactionTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxCompactionTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxCompactionType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum SessionInboxCompactionType {
    #[serde(rename = "compaction")]
    Compaction,
}
impl ::std::fmt::Display for SessionInboxCompactionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Compaction => f.write_str("compaction"),
        }
    }
}
impl ::std::str::FromStr for SessionInboxCompactionType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "compaction" => Ok(Self::Compaction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionInboxCompactionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionInboxCompactionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionInboxDelivery`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum SessionInboxDelivery {
    #[serde(rename = "steer")]
    Steer,
    #[serde(rename = "queue")]
    Queue,
}
impl ::std::fmt::Display for SessionInboxDelivery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Steer => f.write_str("steer"),
            Self::Queue => f.write_str("queue"),
        }
    }
}
impl ::std::str::FromStr for SessionInboxDelivery {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "steer" => Ok(Self::Steer),
            "queue" => Ok(Self::Queue),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionInboxDelivery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionInboxDelivery {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionInboxMove`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxMove {
    pub delivery: SessionInboxDelivery,
    pub id: super::SessionMessageId,
    pub payload: SessionInboxMovePayload,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub time: SessionInboxMoveTime,
    #[serde(rename = "type")]
    pub type_: SessionInboxMoveType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxMovePayload`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxMovePayload {
    pub location: super::LocationPublicRef,
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subpath: ::std::option::Option<super::RelativePath>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxMoveTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxMoveTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxMoveType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum SessionInboxMoveType {
    #[serde(rename = "move")]
    Move,
}
impl ::std::fmt::Display for SessionInboxMoveType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Move => f.write_str("move"),
        }
    }
}
impl ::std::str::FromStr for SessionInboxMoveType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "move" => Ok(Self::Move),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionInboxMoveType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionInboxMoveType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionInboxResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxResponse {
    pub data: ::std::vec::Vec<super::SessionInboxInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxSynthetic`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxSynthetic {
    pub delivery: SessionInboxDelivery,
    pub id: super::SessionMessageId,
    pub payload: SessionInboxSyntheticPayload,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub time: SessionInboxSyntheticTime,
    #[serde(rename = "type")]
    pub type_: SessionInboxSyntheticType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxSyntheticPayload`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxSyntheticPayload {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub text: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxSyntheticTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxSyntheticTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxSyntheticType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum SessionInboxSyntheticType {
    #[serde(rename = "synthetic")]
    Synthetic,
}
impl ::std::fmt::Display for SessionInboxSyntheticType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Synthetic => f.write_str("synthetic"),
        }
    }
}
impl ::std::str::FromStr for SessionInboxSyntheticType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "synthetic" => Ok(Self::Synthetic),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionInboxSyntheticType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionInboxSyntheticType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionInboxUpdateRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionInboxUpdateRequest {
    pub delivery: SessionInboxDelivery,
}
///`SessionInboxUser`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxUser {
    pub delivery: SessionInboxDelivery,
    pub id: super::SessionMessageId,
    pub payload: SessionInboxUserPayload,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub time: SessionInboxUserTime,
    #[serde(rename = "type")]
    pub type_: SessionInboxUserType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxUserPayload`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxUserPayload {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub agents: ::std::vec::Vec<super::PromptAgentAttachment>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub files: ::std::vec::Vec<super::PromptFileAttachment>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub skills: ::std::vec::Vec<super::PromptSkillAttachment>,
    pub text: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxUserTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxUserTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInboxUserType`
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
#[non_exhaustive]
pub enum SessionInboxUserType {
    #[serde(rename = "user")]
    User,
}
impl ::std::fmt::Display for SessionInboxUserType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
        }
    }
}
impl ::std::str::FromStr for SessionInboxUserType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionInboxUserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionInboxUserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionPromptRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionPromptRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub agents: Option<::std::vec::Vec<PromptInputAgentAttachment>>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub delivery: super::RequestField<SessionInboxDelivery>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub files: Option<::std::vec::Vec<PromptInputFileAttachment>>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub id: super::RequestField<super::SessionMessageId>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub resume: super::RequestField<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub skills: Option<::std::vec::Vec<PromptInputSkillAttachment>>,
    pub text: ::std::string::String,
}
///`SessionPromptResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionPromptResponse {
    pub data: SessionInboxUser,
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
