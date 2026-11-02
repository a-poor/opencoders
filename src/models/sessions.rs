// Generated from openapi/opencode-v2.json by model-spike --sessions.
// Do not edit manually; adaptations live in tools/model-spike/src/sessions.rs.
///`FileDiffInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FileDiffInfo {
    #[serde(deserialize_with = "super::deserialize_diff_count")]
    pub additions: u64,
    #[serde(deserialize_with = "super::deserialize_diff_count")]
    pub deletions: u64,
    pub file: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_more")]
    pub more: ::std::option::Option<bool>,
    pub patch: ::std::string::String,
    pub status: FileDiffInfoStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FileDiffInfoStatus`
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
pub enum FileDiffInfoStatus {
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "modified")]
    Modified,
}
impl ::std::fmt::Display for FileDiffInfoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Added => f.write_str("added"),
            Self::Deleted => f.write_str("deleted"),
            Self::Modified => f.write_str("modified"),
        }
    }
}
impl ::std::str::FromStr for FileDiffInfoStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "added" => Ok(Self::Added),
            "deleted" => Ok(Self::Deleted),
            "modified" => Ok(Self::Modified),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileDiffInfoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileDiffInfoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MoneyUsd`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MoneyUsd(pub ::serde_json::Number);
impl ::std::ops::Deref for MoneyUsd {
    type Target = ::serde_json::Number;
    fn deref(&self) -> &::serde_json::Number {
        &self.0
    }
}
impl ::std::convert::From<MoneyUsd> for ::serde_json::Number {
    fn from(value: MoneyUsd) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Number> for MoneyUsd {
    fn from(value: ::serde_json::Number) -> Self {
        Self(value)
    }
}
///`PermissionRuleInput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PermissionRuleInput {
    pub action: ::std::string::String,
    pub effect: super::PermissionEffect,
    pub resource: ::std::string::String,
}
///`PermissionRulesetInput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PermissionRulesetInput(pub ::std::vec::Vec<PermissionRuleInput>);
impl ::std::ops::Deref for PermissionRulesetInput {
    type Target = ::std::vec::Vec<PermissionRuleInput>;
    fn deref(&self) -> &::std::vec::Vec<PermissionRuleInput> {
        &self.0
    }
}
impl ::std::convert::From<PermissionRulesetInput>
for ::std::vec::Vec<PermissionRuleInput> {
    fn from(value: PermissionRulesetInput) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<PermissionRuleInput>>
for PermissionRulesetInput {
    fn from(value: ::std::vec::Vec<PermissionRuleInput>) -> Self {
        Self(value)
    }
}
///`RelativePath`
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
pub struct RelativePath(pub ::std::string::String);
impl ::std::ops::Deref for RelativePath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RelativePath> for ::std::string::String {
    fn from(value: RelativePath) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for RelativePath {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for RelativePath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for RelativePath {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`SessionCreateRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionCreateRequest {
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub agent: super::RequestField<super::AgentId>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub id: super::RequestField<SessionId>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub location: super::RequestField<SessionLocationInput>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub metadata: super::RequestField<SessionMetadata>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub model: super::RequestField<SessionModelInput>,
    #[serde(rename = "parentID")]
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub parent_id: super::RequestField<SessionId>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub permissions: super::RequestField<PermissionRulesetInput>,
    #[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]
    pub title: super::RequestField<::std::string::String>,
}
///`SessionForkBoundary`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SessionForkBoundary {
    #[non_exhaustive]
    Variant0 {
        #[serde(rename = "messageID")]
        message_id: SessionMessageId,
        #[serde(rename = "type")]
        type_: SessionForkBoundaryVariant0Type,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
    #[non_exhaustive]
    Variant1 {
        #[serde(rename = "messageID")]
        message_id: SessionMessageId,
        #[serde(rename = "type")]
        type_: SessionForkBoundaryVariant1Type,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
///`SessionForkBoundaryVariant0Type`
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
pub enum SessionForkBoundaryVariant0Type {
    #[serde(rename = "before")]
    Before,
}
impl ::std::fmt::Display for SessionForkBoundaryVariant0Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Before => f.write_str("before"),
        }
    }
}
impl ::std::str::FromStr for SessionForkBoundaryVariant0Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "before" => Ok(Self::Before),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionForkBoundaryVariant0Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionForkBoundaryVariant0Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionForkBoundaryVariant1Type`
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
pub enum SessionForkBoundaryVariant1Type {
    #[serde(rename = "through")]
    Through,
}
impl ::std::fmt::Display for SessionForkBoundaryVariant1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Through => f.write_str("through"),
        }
    }
}
impl ::std::str::FromStr for SessionForkBoundaryVariant1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "through" => Ok(Self::Through),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SessionForkBoundaryVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionForkBoundaryVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionId`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SessionId(::std::string::String);
impl ::std::ops::Deref for SessionId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SessionId> for ::std::string::String {
    fn from(value: SessionId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SessionId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^ses").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^ses\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SessionId {
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
///`SessionInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInfo {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub agent: ::std::option::Option<super::AgentId>,
    pub cost: MoneyUsd,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fork: ::std::option::Option<SessionInfoFork>,
    pub id: SessionId,
    pub location: super::LocationPublicRef,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<SessionMetadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<super::ModelRef>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub outcome: ::std::option::Option<SessionInfoOutcome>,
    #[serde(rename = "parentID", skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_id: ::std::option::Option<SessionId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<super::PermissionRuleset>,
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub revert: ::std::option::Option<SessionRevert>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subpath: ::std::option::Option<RelativePath>,
    pub time: SessionInfoTime,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    pub tokens: TokenUsageInfo,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInfoFork`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInfoFork {
    pub boundary: SessionForkBoundary,
    #[serde(rename = "sessionID")]
    pub session_id: SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionInfoOutcome`
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
pub enum SessionInfoOutcome {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "interrupted")]
    Interrupted,
}
impl ::std::fmt::Display for SessionInfoOutcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Succeeded => f.write_str("succeeded"),
            Self::Failed => f.write_str("failed"),
            Self::Interrupted => f.write_str("interrupted"),
        }
    }
}
impl ::std::str::FromStr for SessionInfoOutcome {
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
impl ::std::convert::TryFrom<&str> for SessionInfoOutcome {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionInfoOutcome {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionInfoTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInfoTime {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub archived: ::std::option::Option<::serde_json::Number>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub busy: ::std::option::Option<::serde_json::Number>,
    pub created: ::serde_json::Number,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub idle: ::std::option::Option<::serde_json::Number>,
    pub updated: ::serde_json::Number,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub viewed: ::std::option::Option<::serde_json::Number>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionLocationInput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionLocationInput {
    pub directory: super::AbsolutePath,
}
///`SessionMessageId`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SessionMessageId(::std::string::String);
impl ::std::ops::Deref for SessionMessageId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SessionMessageId> for ::std::string::String {
    fn from(value: SessionMessageId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SessionMessageId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^msg_").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^msg_\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SessionMessageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SessionMessageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SessionMessageId {
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
///`SessionMetadata`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SessionMetadata(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for SessionMetadata {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<SessionMetadata>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: SessionMetadata) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for SessionMetadata {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`SessionModelInput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionModelInput {
    pub id: ::std::string::String,
    #[serde(rename = "providerID")]
    pub provider_id: super::ProviderId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub variant: ::std::option::Option<::std::string::String>,
}
///`SessionResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionResponse {
    pub data: SessionInfo,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionRevert`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionRevert {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub files: ::std::vec::Vec<FileDiffInfo>,
    #[serde(rename = "messageID")]
    pub message_id: SessionMessageId,
    #[serde(rename = "partID", skip_serializing_if = "::std::option::Option::is_none")]
    pub part_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub snapshot: ::std::option::Option<SnapshotId>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///Opaque pagination cursor returned as cursor.previous or cursor.next in the previous response.
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
pub struct SessionsCursor(pub ::std::string::String);
impl ::std::ops::Deref for SessionsCursor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SessionsCursor> for ::std::string::String {
    fn from(value: SessionsCursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SessionsCursor {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for SessionsCursor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for SessionsCursor {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`SessionsResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionsResponse {
    pub cursor: SessionsResponseCursor,
    pub data: ::std::vec::Vec<SessionInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SessionsResponseCursor`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionsResponseCursor {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub next: ::std::option::Option<SessionsCursor>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous: ::std::option::Option<SessionsCursor>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`SnapshotId`
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
pub struct SnapshotId(pub ::std::string::String);
impl ::std::ops::Deref for SnapshotId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SnapshotId> for ::std::string::String {
    fn from(value: SnapshotId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for SnapshotId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for SnapshotId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for SnapshotId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`TokenUsageInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct TokenUsageInfo {
    pub cache: TokenUsageInfoCache,
    pub input: ::serde_json::Number,
    pub output: ::serde_json::Number,
    pub reasoning: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`TokenUsageInfoCache`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct TokenUsageInfoCache {
    pub read: ::serde_json::Number,
    pub write: ::serde_json::Number,
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
