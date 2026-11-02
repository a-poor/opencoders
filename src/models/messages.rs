// Generated from openapi/opencode-v2.json by model-spike --messages.
// Do not edit manually; adaptations live in tools/model-spike/src/messages.rs.
///`PromptAgentAttachment`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PromptAgentAttachment {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mention: ::std::option::Option<PromptMention>,
    pub name: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PromptBase64`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PromptBase64(::std::string::String);
impl ::std::ops::Deref for PromptBase64 {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PromptBase64> for ::std::string::String {
    fn from(value: PromptBase64) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PromptBase64 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PromptBase64 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptBase64 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PromptBase64 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`PromptFileAttachment`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PromptFileAttachment {
    pub data: PromptBase64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mention: ::std::option::Option<PromptMention>,
    pub mime: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub source: PromptFileSource,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PromptFileSource`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum PromptFileSource {
    #[non_exhaustive]
    Variant0 {
        #[serde(rename = "type")]
        type_: PromptFileSourceVariant0Type,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
    #[non_exhaustive]
    Variant1 {
        #[serde(rename = "type")]
        type_: PromptFileSourceVariant1Type,
        uri: ::std::string::String,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
///`PromptFileSourceVariant0Type`
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
pub enum PromptFileSourceVariant0Type {
    #[serde(rename = "inline")]
    Inline,
}
impl ::std::fmt::Display for PromptFileSourceVariant0Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inline => f.write_str("inline"),
        }
    }
}
impl ::std::str::FromStr for PromptFileSourceVariant0Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "inline" => Ok(Self::Inline),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptFileSourceVariant0Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptFileSourceVariant0Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PromptFileSourceVariant1Type`
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
pub enum PromptFileSourceVariant1Type {
    #[serde(rename = "uri")]
    Uri,
}
impl ::std::fmt::Display for PromptFileSourceVariant1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Uri => f.write_str("uri"),
        }
    }
}
impl ::std::str::FromStr for PromptFileSourceVariant1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "uri" => Ok(Self::Uri),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PromptFileSourceVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PromptFileSourceVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PromptMention`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PromptMention {
    pub end: ::serde_json::Number,
    pub start: ::serde_json::Number,
    pub text: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PromptSkillAttachment`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PromptSkillAttachment {
    pub id: SkillId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mention: ::std::option::Option<PromptMention>,
    pub name: SkillName,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAgentSelected`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAgentSelected {
    pub agent: super::AgentId,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous: ::std::option::Option<super::AgentId>,
    pub time: SessionMessageAgentSelectedTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageAgentSelectedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAgentSelectedTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAgentSelectedTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAgentSelectedType`
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
pub enum SessionMessageAgentSelectedType {
    #[serde(rename = "agent-switched")]
    AgentSwitched,
}
impl ::std::fmt::Display for SessionMessageAgentSelectedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AgentSwitched => f.write_str("agent-switched"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageAgentSelectedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agent-switched" => Ok(Self::AgentSwitched),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageAgentSelectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageAgentSelectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageAssistant`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistant {
    pub agent: super::AgentId,
    pub content: ::std::vec::Vec<SessionMessageAssistantContentItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cost: ::std::option::Option<super::MoneyUsd>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<SessionStructuredError>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub finish: ::std::option::Option<SessionMessageAssistantFinish>,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub model: super::ModelRef,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<SessionMessageNative>,
    #[serde(
        rename = "rawFinish",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub raw_finish: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub retry: ::std::option::Option<SessionMessageAssistantRetry>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub snapshot: ::std::option::Option<SessionMessageAssistantSnapshot>,
    pub time: SessionMessageAssistantTime,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokens: ::std::option::Option<super::TokenUsageInfo>,
    #[serde(rename = "type")]
    pub type_: SessionMessageAssistantType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
#[derive(::serde::Serialize, Clone, Debug)]
///`SessionMessageAssistantContentItem`
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionMessageAssistantContentItem {
    Text(SessionMessageAssistantText),
    Reasoning(SessionMessageAssistantReasoning),
    Tool(Box<SessionMessageAssistantTool>),
}
impl ::std::convert::From<SessionMessageAssistantText>
for SessionMessageAssistantContentItem {
    fn from(value: SessionMessageAssistantText) -> Self {
        Self::Text(value)
    }
}
impl ::std::convert::From<SessionMessageAssistantReasoning>
for SessionMessageAssistantContentItem {
    fn from(value: SessionMessageAssistantReasoning) -> Self {
        Self::Reasoning(value)
    }
}
impl ::std::convert::From<SessionMessageAssistantTool>
for SessionMessageAssistantContentItem {
    fn from(value: SessionMessageAssistantTool) -> Self {
        Self::Tool(Box::new(value))
    }
}
///`SessionMessageAssistantFinish`
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
pub enum SessionMessageAssistantFinish {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool-calls")]
    ToolCalls,
    #[serde(rename = "content-filter")]
    ContentFilter,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::fmt::Display for SessionMessageAssistantFinish {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stop => f.write_str("stop"),
            Self::Length => f.write_str("length"),
            Self::ToolCalls => f.write_str("tool-calls"),
            Self::ContentFilter => f.write_str("content-filter"),
            Self::Error => f.write_str("error"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageAssistantFinish {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stop" => Ok(Self::Stop),
            "length" => Ok(Self::Length),
            "tool-calls" => Ok(Self::ToolCalls),
            "content-filter" => Ok(Self::ContentFilter),
            "error" => Ok(Self::Error),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageAssistantFinish {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageAssistantFinish {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageAssistantReasoning`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistantReasoning {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<SessionMessageNative>,
    pub text: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time: ::std::option::Option<SessionMessageAssistantReasoningTime>,
    #[serde(rename = "type")]
    pub type_: SessionMessageAssistantReasoningType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAssistantReasoningTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistantReasoningTime {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub completed: ::std::option::Option<::serde_json::Number>,
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAssistantReasoningType`
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
pub enum SessionMessageAssistantReasoningType {
    #[serde(rename = "reasoning")]
    Reasoning,
}
impl ::std::fmt::Display for SessionMessageAssistantReasoningType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Reasoning => f.write_str("reasoning"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageAssistantReasoningType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "reasoning" => Ok(Self::Reasoning),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageAssistantReasoningType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageAssistantReasoningType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageAssistantRetry`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistantRetry {
    pub at: ::serde_json::Number,
    #[serde(deserialize_with = "super::deserialize_message_attempt")]
    pub attempt: u64,
    pub error: SessionStructuredError,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAssistantSnapshot`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistantSnapshot {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end: ::std::option::Option<super::SnapshotId>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub files: ::std::vec::Vec<super::RelativePath>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<super::SnapshotId>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAssistantText`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistantText {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<SessionMessageNative>,
    pub text: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: SessionMessageAssistantTextType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAssistantTextType`
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
pub enum SessionMessageAssistantTextType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for SessionMessageAssistantTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageAssistantTextType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageAssistantTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageAssistantTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageAssistantTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistantTime {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub completed: ::std::option::Option<::serde_json::Number>,
    pub created: ::serde_json::Number,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub streamed: ::std::option::Option<::serde_json::Number>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAssistantTool`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistantTool {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub executed: ::std::option::Option<bool>,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<SessionMessageNative>,
    pub state: SessionMessageAssistantToolState,
    pub time: SessionMessageAssistantToolTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageAssistantToolType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
#[derive(::serde::Serialize, Clone, Debug)]
///`SessionMessageAssistantToolState`
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionMessageAssistantToolState {
    Streaming(SessionMessageToolStateStreaming),
    Running(SessionMessageToolStateRunning),
    Completed(SessionMessageToolStateCompleted),
    Error(SessionMessageToolStateError),
}
impl ::std::convert::From<SessionMessageToolStateStreaming>
for SessionMessageAssistantToolState {
    fn from(value: SessionMessageToolStateStreaming) -> Self {
        Self::Streaming(value)
    }
}
impl ::std::convert::From<SessionMessageToolStateRunning>
for SessionMessageAssistantToolState {
    fn from(value: SessionMessageToolStateRunning) -> Self {
        Self::Running(value)
    }
}
impl ::std::convert::From<SessionMessageToolStateCompleted>
for SessionMessageAssistantToolState {
    fn from(value: SessionMessageToolStateCompleted) -> Self {
        Self::Completed(value)
    }
}
impl ::std::convert::From<SessionMessageToolStateError>
for SessionMessageAssistantToolState {
    fn from(value: SessionMessageToolStateError) -> Self {
        Self::Error(value)
    }
}
///`SessionMessageAssistantToolTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageAssistantToolTime {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub completed: ::std::option::Option<::serde_json::Number>,
    pub created: ::serde_json::Number,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ran: ::std::option::Option<::serde_json::Number>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageAssistantToolType`
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
pub enum SessionMessageAssistantToolType {
    #[serde(rename = "tool")]
    Tool,
}
impl ::std::fmt::Display for SessionMessageAssistantToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tool => f.write_str("tool"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageAssistantToolType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tool" => Ok(Self::Tool),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageAssistantToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageAssistantToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageAssistantType`
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
pub enum SessionMessageAssistantType {
    #[serde(rename = "assistant")]
    Assistant,
}
impl ::std::fmt::Display for SessionMessageAssistantType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assistant => f.write_str("assistant"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageAssistantType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assistant" => Ok(Self::Assistant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageAssistantType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageAssistantType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(::serde::Serialize, Clone, Debug)]
///`SessionMessageCompaction`
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionMessageCompaction {
    Running(SessionMessageCompactionRunning),
    Completed(SessionMessageCompactionCompleted),
    Failed(SessionMessageCompactionFailed),
}
impl ::std::convert::From<SessionMessageCompactionRunning> for SessionMessageCompaction {
    fn from(value: SessionMessageCompactionRunning) -> Self {
        Self::Running(value)
    }
}
impl ::std::convert::From<SessionMessageCompactionCompleted>
for SessionMessageCompaction {
    fn from(value: SessionMessageCompactionCompleted) -> Self {
        Self::Completed(value)
    }
}
impl ::std::convert::From<SessionMessageCompactionFailed> for SessionMessageCompaction {
    fn from(value: SessionMessageCompactionFailed) -> Self {
        Self::Failed(value)
    }
}
///`SessionMessageCompactionCompleted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageCompactionCompleted {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cost: ::std::option::Option<super::MoneyUsd>,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<super::ModelRef>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<SessionMessageNative>,
    pub reason: SessionMessageCompactionCompletedReason,
    pub status: SessionMessageCompactionCompletedStatus,
    pub summary: ::std::string::String,
    pub time: SessionMessageCompactionCompletedTime,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokens: ::std::option::Option<super::TokenUsageInfo>,
    #[serde(rename = "type")]
    pub type_: SessionMessageCompactionCompletedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageCompactionCompletedReason`
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
pub enum SessionMessageCompactionCompletedReason {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}
impl ::std::fmt::Display for SessionMessageCompactionCompletedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Manual => f.write_str("manual"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionCompletedReason {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "manual" => Ok(Self::Manual),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionCompletedReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionCompletedReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageCompactionCompletedStatus`
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
pub enum SessionMessageCompactionCompletedStatus {
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for SessionMessageCompactionCompletedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionCompletedStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionCompletedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionCompletedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageCompactionCompletedTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageCompactionCompletedTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageCompactionCompletedType`
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
pub enum SessionMessageCompactionCompletedType {
    #[serde(rename = "compaction")]
    Compaction,
}
impl ::std::fmt::Display for SessionMessageCompactionCompletedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Compaction => f.write_str("compaction"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionCompletedType {
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
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionCompletedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionCompletedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageCompactionFailed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageCompactionFailed {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cost: ::std::option::Option<super::MoneyUsd>,
    pub error: SessionStructuredError,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub reason: SessionMessageCompactionFailedReason,
    pub status: SessionMessageCompactionFailedStatus,
    pub time: SessionMessageCompactionFailedTime,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokens: ::std::option::Option<super::TokenUsageInfo>,
    #[serde(rename = "type")]
    pub type_: SessionMessageCompactionFailedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageCompactionFailedReason`
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
pub enum SessionMessageCompactionFailedReason {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}
impl ::std::fmt::Display for SessionMessageCompactionFailedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Manual => f.write_str("manual"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionFailedReason {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "manual" => Ok(Self::Manual),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionFailedReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionFailedReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageCompactionFailedStatus`
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
pub enum SessionMessageCompactionFailedStatus {
    #[serde(rename = "failed")]
    Failed,
}
impl ::std::fmt::Display for SessionMessageCompactionFailedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Failed => f.write_str("failed"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionFailedStatus {
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
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionFailedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionFailedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageCompactionFailedTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageCompactionFailedTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageCompactionFailedType`
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
pub enum SessionMessageCompactionFailedType {
    #[serde(rename = "compaction")]
    Compaction,
}
impl ::std::fmt::Display for SessionMessageCompactionFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Compaction => f.write_str("compaction"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionFailedType {
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
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageCompactionRunning`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageCompactionRunning {
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub reason: SessionMessageCompactionRunningReason,
    pub status: SessionMessageCompactionRunningStatus,
    pub summary: ::std::string::String,
    pub time: SessionMessageCompactionRunningTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageCompactionRunningType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageCompactionRunningReason`
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
pub enum SessionMessageCompactionRunningReason {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}
impl ::std::fmt::Display for SessionMessageCompactionRunningReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Manual => f.write_str("manual"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionRunningReason {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "manual" => Ok(Self::Manual),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionRunningReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionRunningReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageCompactionRunningStatus`
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
pub enum SessionMessageCompactionRunningStatus {
    #[serde(rename = "running")]
    Running,
}
impl ::std::fmt::Display for SessionMessageCompactionRunningStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Running => f.write_str("running"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionRunningStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "running" => Ok(Self::Running),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionRunningStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionRunningStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageCompactionRunningTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageCompactionRunningTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageCompactionRunningType`
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
pub enum SessionMessageCompactionRunningType {
    #[serde(rename = "compaction")]
    Compaction,
}
impl ::std::fmt::Display for SessionMessageCompactionRunningType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Compaction => f.write_str("compaction"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageCompactionRunningType {
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
impl ::std::convert::TryFrom<&str> for SessionMessageCompactionRunningType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageCompactionRunningType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageIdle`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageIdle {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<SessionStructuredError>,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub outcome: SessionMessageIdleOutcome,
    pub time: SessionMessageIdleTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageIdleType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageIdleOutcome`
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
pub enum SessionMessageIdleOutcome {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "interrupted")]
    Interrupted,
}
impl ::std::fmt::Display for SessionMessageIdleOutcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Succeeded => f.write_str("succeeded"),
            Self::Failed => f.write_str("failed"),
            Self::Interrupted => f.write_str("interrupted"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageIdleOutcome {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "succeeded" => Ok(Self::Succeeded),
            "failed" => Ok(Self::Failed),
            "interrupted" => Ok(Self::Interrupted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageIdleOutcome {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageIdleOutcome {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageIdleTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageIdleTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageIdleType`
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
pub enum SessionMessageIdleType {
    #[serde(rename = "idle")]
    Idle,
}
impl ::std::fmt::Display for SessionMessageIdleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Idle => f.write_str("idle"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageIdleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "idle" => Ok(Self::Idle),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageIdleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageIdleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageLocationSwitched`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageLocationSwitched {
    pub id: super::SessionMessageId,
    pub location: super::LocationPublicRef,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous: ::std::option::Option<SessionMessageLocationSwitchedPrevious>,
    #[serde(
        rename = "projectID",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_id: ::std::option::Option<super::ProjectId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subpath: ::std::option::Option<super::RelativePath>,
    pub time: SessionMessageLocationSwitchedTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageLocationSwitchedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageLocationSwitchedPrevious`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageLocationSwitchedPrevious {
    pub location: super::LocationPublicRef,
    #[serde(
        rename = "projectID",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_id: ::std::option::Option<super::ProjectId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subpath: ::std::option::Option<super::RelativePath>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageLocationSwitchedTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageLocationSwitchedTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageLocationSwitchedType`
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
pub enum SessionMessageLocationSwitchedType {
    #[serde(rename = "location-switched")]
    LocationSwitched,
}
impl ::std::fmt::Display for SessionMessageLocationSwitchedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LocationSwitched => f.write_str("location-switched"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageLocationSwitchedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "location-switched" => Ok(Self::LocationSwitched),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageLocationSwitchedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageLocationSwitchedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageModelSelected`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageModelSelected {
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub model: super::ModelRef,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous: ::std::option::Option<super::ModelRef>,
    pub time: SessionMessageModelSelectedTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageModelSelectedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageModelSelectedTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageModelSelectedTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageModelSelectedType`
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
pub enum SessionMessageModelSelectedType {
    #[serde(rename = "model-switched")]
    ModelSwitched,
}
impl ::std::fmt::Display for SessionMessageModelSelectedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ModelSwitched => f.write_str("model-switched"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageModelSelectedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "model-switched" => Ok(Self::ModelSwitched),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageModelSelectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageModelSelectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageNative`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SessionMessageNative(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for SessionMessageNative {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<SessionMessageNative>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: SessionMessageNative) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for SessionMessageNative {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`SessionMessageResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageResponse {
    pub data: super::SessionMessageInfo,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageShell`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageShell {
    pub command: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exit: ::std::option::Option<SessionMessageShellExit>,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<SessionMessageShellOutput>,
    #[serde(rename = "shellID")]
    pub shell_id: ShellId,
    pub status: SessionMessageShellStatus,
    pub time: SessionMessageShellTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageShellType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageShellExit`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionMessageShellExit {
    Number(::serde_json::Number),
    String(SessionMessageShellExitString),
}
impl ::std::convert::From<::serde_json::Number> for SessionMessageShellExit {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<SessionMessageShellExitString> for SessionMessageShellExit {
    fn from(value: SessionMessageShellExitString) -> Self {
        Self::String(value)
    }
}
///`SessionMessageShellExitString`
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
pub enum SessionMessageShellExitString {
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
    NaN,
}
impl ::std::fmt::Display for SessionMessageShellExitString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
            Self::NaN => f.write_str("NaN"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageShellExitString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            "NaN" => Ok(Self::NaN),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageShellExitString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageShellExitString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageShellOutput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageShellOutput {
    #[serde(deserialize_with = "super::deserialize_diff_count")]
    pub cursor: u64,
    pub output: ::std::string::String,
    #[serde(deserialize_with = "super::deserialize_diff_count")]
    pub size: u64,
    pub truncated: bool,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageShellStatus`
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
pub enum SessionMessageShellStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "exited")]
    Exited,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "killed")]
    Killed,
}
impl ::std::fmt::Display for SessionMessageShellStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Running => f.write_str("running"),
            Self::Exited => f.write_str("exited"),
            Self::Timeout => f.write_str("timeout"),
            Self::Killed => f.write_str("killed"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageShellStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "running" => Ok(Self::Running),
            "exited" => Ok(Self::Exited),
            "timeout" => Ok(Self::Timeout),
            "killed" => Ok(Self::Killed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageShellStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageShellStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageShellTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageShellTime {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub completed: ::std::option::Option<::serde_json::Number>,
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageShellType`
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
pub enum SessionMessageShellType {
    #[serde(rename = "shell")]
    Shell,
}
impl ::std::fmt::Display for SessionMessageShellType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Shell => f.write_str("shell"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageShellType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "shell" => Ok(Self::Shell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageShellType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageShellType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageSkill`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageSkill {
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub name: SkillName,
    pub skill: SkillId,
    pub text: ::std::string::String,
    pub time: SessionMessageSkillTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageSkillType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageSkillTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageSkillTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageSkillType`
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
pub enum SessionMessageSkillType {
    #[serde(rename = "skill")]
    Skill,
}
impl ::std::fmt::Display for SessionMessageSkillType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Skill => f.write_str("skill"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageSkillType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "skill" => Ok(Self::Skill),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageSkillType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageSkillType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageSynthetic`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageSynthetic {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub text: ::std::string::String,
    pub time: SessionMessageSyntheticTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageSyntheticType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageSyntheticTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageSyntheticTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageSyntheticType`
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
pub enum SessionMessageSyntheticType {
    #[serde(rename = "synthetic")]
    Synthetic,
}
impl ::std::fmt::Display for SessionMessageSyntheticType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Synthetic => f.write_str("synthetic"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageSyntheticType {
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
impl ::std::convert::TryFrom<&str> for SessionMessageSyntheticType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageSyntheticType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageSystem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageSystem {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub text: ::std::string::String,
    pub time: SessionMessageSystemTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageSystemType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageSystemTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageSystemTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageSystemType`
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
pub enum SessionMessageSystemType {
    #[serde(rename = "system")]
    System,
}
impl ::std::fmt::Display for SessionMessageSystemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::System => f.write_str("system"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageSystemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "system" => Ok(Self::System),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageSystemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageSystemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageToolStateCompleted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageToolStateCompleted {
    #[serde(deserialize_with = "super::deserialize_tool_content")]
    pub content: ::std::vec::Vec<ToolContent>,
    pub input: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub status: SessionMessageToolStateCompletedStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageToolStateCompletedStatus`
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
pub enum SessionMessageToolStateCompletedStatus {
    #[serde(rename = "completed")]
    Completed,
}
impl ::std::fmt::Display for SessionMessageToolStateCompletedStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => f.write_str("completed"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageToolStateCompletedStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageToolStateCompletedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageToolStateCompletedStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageToolStateError`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageToolStateError {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    #[serde(deserialize_with = "super::deserialize_tool_content")]
    pub content: ::std::vec::Vec<ToolContent>,
    pub error: SessionStructuredError,
    pub input: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub status: SessionMessageToolStateErrorStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageToolStateErrorStatus`
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
pub enum SessionMessageToolStateErrorStatus {
    #[serde(rename = "error")]
    Error,
}
impl ::std::fmt::Display for SessionMessageToolStateErrorStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => f.write_str("error"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageToolStateErrorStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageToolStateErrorStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageToolStateErrorStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageToolStateRunning`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageToolStateRunning {
    pub input: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub status: SessionMessageToolStateRunningStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageToolStateRunningStatus`
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
pub enum SessionMessageToolStateRunningStatus {
    #[serde(rename = "running")]
    Running,
}
impl ::std::fmt::Display for SessionMessageToolStateRunningStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Running => f.write_str("running"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageToolStateRunningStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "running" => Ok(Self::Running),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageToolStateRunningStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageToolStateRunningStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageToolStateStreaming`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageToolStateStreaming {
    pub input: ::std::string::String,
    pub status: SessionMessageToolStateStreamingStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageToolStateStreamingStatus`
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
pub enum SessionMessageToolStateStreamingStatus {
    #[serde(rename = "streaming")]
    Streaming,
}
impl ::std::fmt::Display for SessionMessageToolStateStreamingStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Streaming => f.write_str("streaming"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageToolStateStreamingStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "streaming" => Ok(Self::Streaming),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageToolStateStreamingStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SessionMessageToolStateStreamingStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageType`
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
pub enum SessionMessageType {
    #[serde(rename = "agent-switched")]
    AgentSwitched,
    #[serde(rename = "model-switched")]
    ModelSwitched,
    #[serde(rename = "location-switched")]
    LocationSwitched,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "synthetic")]
    Synthetic,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "skill")]
    Skill,
    #[serde(rename = "shell")]
    Shell,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "compaction")]
    Compaction,
    #[serde(rename = "idle")]
    Idle,
}
impl ::std::fmt::Display for SessionMessageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AgentSwitched => f.write_str("agent-switched"),
            Self::ModelSwitched => f.write_str("model-switched"),
            Self::LocationSwitched => f.write_str("location-switched"),
            Self::User => f.write_str("user"),
            Self::Synthetic => f.write_str("synthetic"),
            Self::System => f.write_str("system"),
            Self::Skill => f.write_str("skill"),
            Self::Shell => f.write_str("shell"),
            Self::Assistant => f.write_str("assistant"),
            Self::Compaction => f.write_str("compaction"),
            Self::Idle => f.write_str("idle"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agent-switched" => Ok(Self::AgentSwitched),
            "model-switched" => Ok(Self::ModelSwitched),
            "location-switched" => Ok(Self::LocationSwitched),
            "user" => Ok(Self::User),
            "synthetic" => Ok(Self::Synthetic),
            "system" => Ok(Self::System),
            "skill" => Ok(Self::Skill),
            "shell" => Ok(Self::Shell),
            "assistant" => Ok(Self::Assistant),
            "compaction" => Ok(Self::Compaction),
            "idle" => Ok(Self::Idle),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessageUser`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageUser {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub agents: ::std::vec::Vec<PromptAgentAttachment>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub files: ::std::vec::Vec<PromptFileAttachment>,
    pub id: super::SessionMessageId,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub skills: ::std::vec::Vec<PromptSkillAttachment>,
    pub text: ::std::string::String,
    pub time: SessionMessageUserTime,
    #[serde(rename = "type")]
    pub type_: SessionMessageUserType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageUserTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessageUserTime {
    pub created: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessageUserType`
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
pub enum SessionMessageUserType {
    #[serde(rename = "user")]
    User,
}
impl ::std::fmt::Display for SessionMessageUserType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
        }
    }
}
impl ::std::str::FromStr for SessionMessageUserType {
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
impl ::std::convert::TryFrom<&str> for SessionMessageUserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageUserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionMessagesResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessagesResponse {
    pub cursor: SessionMessagesResponseCursor,
    pub data: ::std::vec::Vec<super::SessionMessageInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionMessagesResponseCursor`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionMessagesResponseCursor {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionStructuredError`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionStructuredError {
    pub message: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub response: ::std::option::Option<SessionStructuredErrorResponse>,
    #[serde(
        rename = "retryAfterMs",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    #[serde(default, deserialize_with = "super::deserialize_message_retry_delay")]
    pub retry_after_ms: Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_message_status")]
    pub status: Option<u16>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionStructuredErrorResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionStructuredErrorResponse {
    pub body: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ShellId`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ShellId(::std::string::String);
impl ::std::ops::Deref for ShellId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ShellId> for ::std::string::String {
    fn from(value: ShellId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ShellId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^sh_").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^sh_\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ShellId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ShellId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ShellId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`SkillId`
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
pub struct SkillId(pub ::std::string::String);
impl ::std::ops::Deref for SkillId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SkillId> for ::std::string::String {
    fn from(value: SkillId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SkillId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for SkillId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for SkillId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`SkillName`
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
pub struct SkillName(pub ::std::string::String);
impl ::std::ops::Deref for SkillName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SkillName> for ::std::string::String {
    fn from(value: SkillName) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SkillName {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for SkillName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for SkillName {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
#[derive(::serde::Serialize, Clone, Debug)]
///`ToolContent`
#[serde(untagged)]
#[non_exhaustive]
pub enum ToolContent {
    TextContent(ToolTextContent),
    FileContent(ToolFileContent),
}
impl ::std::convert::From<ToolTextContent> for ToolContent {
    fn from(value: ToolTextContent) -> Self {
        Self::TextContent(value)
    }
}
impl ::std::convert::From<ToolFileContent> for ToolContent {
    fn from(value: ToolFileContent) -> Self {
        Self::FileContent(value)
    }
}
///`ToolFileContent`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ToolFileContent {
    pub mime: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ToolFileContentType,
    pub uri: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ToolFileContentType`
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
pub enum ToolFileContentType {
    #[serde(rename = "file")]
    File,
}
impl ::std::fmt::Display for ToolFileContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => f.write_str("file"),
        }
    }
}
impl ::std::str::FromStr for ToolFileContentType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolFileContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolFileContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ToolTextContent`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ToolTextContent {
    pub text: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ToolTextContentType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ToolTextContentType`
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
pub enum ToolTextContentType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for ToolTextContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for ToolTextContentType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ToolTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ToolTextContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
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
