// Generated from openapi/opencode-v2.json by model-spike --discovery.
// Do not edit manually; adaptations live in tools/model-spike/src/discovery.rs.
///`AbsolutePath`
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
pub struct AbsolutePath(pub ::std::string::String);
impl ::std::ops::Deref for AbsolutePath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AbsolutePath> for ::std::string::String {
    fn from(value: AbsolutePath) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for AbsolutePath {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for AbsolutePath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for AbsolutePath {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`AgentColor`
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
pub struct AgentColor(pub ::std::string::String);
impl ::std::ops::Deref for AgentColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgentColor> for ::std::string::String {
    fn from(value: AgentColor) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for AgentColor {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for AgentColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for AgentColor {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`AgentId`
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
pub struct AgentId(pub ::std::string::String);
impl ::std::ops::Deref for AgentId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AgentId> for ::std::string::String {
    fn from(value: AgentId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for AgentId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for AgentId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`AgentInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct AgentInfo {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<AgentColor>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub hidden: bool,
    pub id: AgentId,
    pub mode: AgentInfoMode,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<ModelRef>,
    pub name: ::std::string::String,
    pub permissions: PermissionRuleset,
    pub request: ProviderRequest,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_steps")]
    pub steps: Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`AgentInfoMode`
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
pub enum AgentInfoMode {
    #[serde(rename = "subagent")]
    Subagent,
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "all")]
    All,
}
impl ::std::fmt::Display for AgentInfoMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Subagent => f.write_str("subagent"),
            Self::Primary => f.write_str("primary"),
            Self::All => f.write_str("all"),
        }
    }
}
impl ::std::str::FromStr for AgentInfoMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "subagent" => Ok(Self::Subagent),
            "primary" => Ok(Self::Primary),
            "all" => Ok(Self::All),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AgentInfoMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AgentInfoMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AgentListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct AgentListResponse {
    pub data: ::std::vec::Vec<AgentInfo>,
    pub location: LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`AgentResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct AgentResponse {
    pub data: AgentInfo,
    pub location: LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`LocationPublicInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct LocationPublicInfo {
    pub directory: AbsolutePath,
    pub project: LocationPublicInfoProject,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`LocationPublicInfoProject`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct LocationPublicInfoProject {
    pub canonical: AbsolutePath,
    pub directory: AbsolutePath,
    pub id: ProjectId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`LocationPublicRef`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct LocationPublicRef {
    pub directory: AbsolutePath,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelCacheFormat`
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
pub enum ModelCacheFormat {
    #[serde(rename = "anthropic")]
    Anthropic,
    #[serde(rename = "alibaba")]
    Alibaba,
}
impl ::std::fmt::Display for ModelCacheFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Anthropic => f.write_str("anthropic"),
            Self::Alibaba => f.write_str("alibaba"),
        }
    }
}
impl ::std::str::FromStr for ModelCacheFormat {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "anthropic" => Ok(Self::Anthropic),
            "alibaba" => Ok(Self::Alibaba),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelCacheFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelCacheFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ModelCapabilities`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelCapabilities {
    pub input: ::std::vec::Vec<::std::string::String>,
    pub output: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<bool>,
    pub tools: bool,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelCompatibility`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelCompatibility {
    #[serde(
        rename = "cacheFormat",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cache_format: ::std::option::Option<ModelCacheFormat>,
    #[serde(
        rename = "maxTokensField",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_tokens_field: ::std::option::Option<ModelMaxTokensField>,
    #[serde(
        rename = "mediaInput",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub media_input: ::std::option::Option<ModelMediaInput>,
    #[serde(
        rename = "reasoningField",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reasoning_field: ::std::option::Option<ModelReasoningField>,
    #[serde(
        rename = "requireAssistantAfterTool",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub require_assistant_after_tool: ::std::option::Option<bool>,
    #[serde(
        rename = "requireFinishReason",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub require_finish_reason: ::std::option::Option<bool>,
    #[serde(
        rename = "requireReasoning",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub require_reasoning: ::std::option::Option<bool>,
    #[serde(
        rename = "streamUsage",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stream_usage: ::std::option::Option<bool>,
    #[serde(
        rename = "supportsEffortUpdates",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supports_effort_updates: ::std::option::Option<bool>,
    #[serde(
        rename = "supportsPromptCacheKey",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supports_prompt_cache_key: ::std::option::Option<bool>,
    #[serde(
        rename = "supportsThinkingBlockBinding",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supports_thinking_block_binding: ::std::option::Option<bool>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelCost`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelCost {
    pub cache: ModelCostCache,
    pub input: MoneyUsdPerMillionTokens,
    pub output: MoneyUsdPerMillionTokens,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tier: ::std::option::Option<ModelCostTier>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelCostCache`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelCostCache {
    pub read: MoneyUsdPerMillionTokens,
    pub write: MoneyUsdPerMillionTokens,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelCostTier`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelCostTier {
    #[serde(deserialize_with = "super::deserialize_model_integer")]
    pub size: i64,
    #[serde(rename = "type")]
    pub type_: ModelCostTierType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelCostTierType`
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
pub enum ModelCostTierType {
    #[serde(rename = "context")]
    Context,
}
impl ::std::fmt::Display for ModelCostTierType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Context => f.write_str("context"),
        }
    }
}
impl ::std::str::FromStr for ModelCostTierType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "context" => Ok(Self::Context),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelCostTierType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelCostTierType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ModelDefaultResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelDefaultResponse {
    #[serde(deserialize_with = "super::deserialize_nullable_model")]
    pub data: ::std::option::Option<ModelInfo>,
    pub location: LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ModelInfo {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub body: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub canonical: ::std::option::Option<ProviderId>,
    pub capabilities: ModelCapabilities,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compatibility: ::std::option::Option<ModelCompatibility>,
    pub cost: ::std::vec::Vec<ModelCost>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub family: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    pub id: ::std::string::String,
    pub limit: ModelInfoLimit,
    #[serde(rename = "modelID")]
    pub model_id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub package: ::std::option::Option<::std::string::String>,
    #[serde(rename = "providerID")]
    pub provider_id: ProviderId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<ModelSettings>,
    pub status: ModelInfoStatus,
    pub time: ModelInfoTime,
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<ModelType>,
    pub variants: ::std::vec::Vec<ModelVariant>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelInfoLimit`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelInfoLimit {
    #[serde(deserialize_with = "super::deserialize_model_integer")]
    pub context: i64,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_model_integer")]
    pub input: Option<i64>,
    #[serde(deserialize_with = "super::deserialize_model_integer")]
    pub output: i64,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelInfoStatus`
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
pub enum ModelInfoStatus {
    #[serde(rename = "alpha")]
    Alpha,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "active")]
    Active,
}
impl ::std::fmt::Display for ModelInfoStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alpha => f.write_str("alpha"),
            Self::Beta => f.write_str("beta"),
            Self::Deprecated => f.write_str("deprecated"),
            Self::Active => f.write_str("active"),
        }
    }
}
impl ::std::str::FromStr for ModelInfoStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "alpha" => Ok(Self::Alpha),
            "beta" => Ok(Self::Beta),
            "deprecated" => Ok(Self::Deprecated),
            "active" => Ok(Self::Active),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelInfoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelInfoStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ModelInfoTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelInfoTime {
    pub released: ::serde_json::Number,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelListResponse {
    pub data: ::std::vec::Vec<ModelInfo>,
    pub location: LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelMaxTokensField`
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
pub enum ModelMaxTokensField {
    #[serde(rename = "max_completion_tokens")]
    MaxCompletionTokens,
    #[serde(rename = "max_tokens")]
    MaxTokens,
}
impl ::std::fmt::Display for ModelMaxTokensField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MaxCompletionTokens => f.write_str("max_completion_tokens"),
            Self::MaxTokens => f.write_str("max_tokens"),
        }
    }
}
impl ::std::str::FromStr for ModelMaxTokensField {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "max_completion_tokens" => Ok(Self::MaxCompletionTokens),
            "max_tokens" => Ok(Self::MaxTokens),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelMaxTokensField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelMaxTokensField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ModelMediaInput`
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
pub enum ModelMediaInput {
    #[serde(rename = "assets")]
    Assets,
    #[serde(rename = "state")]
    State,
    #[serde(rename = "root")]
    Root,
}
impl ::std::fmt::Display for ModelMediaInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Assets => f.write_str("assets"),
            Self::State => f.write_str("state"),
            Self::Root => f.write_str("root"),
        }
    }
}
impl ::std::str::FromStr for ModelMediaInput {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "assets" => Ok(Self::Assets),
            "state" => Ok(Self::State),
            "root" => Ok(Self::Root),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ModelMediaInput {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ModelMediaInput {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ModelReasoningField`
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
pub struct ModelReasoningField(pub ::std::string::String);
impl ::std::ops::Deref for ModelReasoningField {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModelReasoningField> for ::std::string::String {
    fn from(value: ModelReasoningField) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ModelReasoningField {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ModelReasoningField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ModelReasoningField {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`ModelRef`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelRef {
    pub id: ::std::string::String,
    #[serde(rename = "providerID")]
    pub provider_id: ProviderId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub variant: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelSettings`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelSettings {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache: ::std::option::Option<ProviderCache>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compaction: ::std::option::Option<ProviderCompaction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding: ::std::option::Option<ProviderEncoding>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelType`
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
pub struct ModelType(pub ::std::string::String);
impl ::std::ops::Deref for ModelType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ModelType> for ::std::string::String {
    fn from(value: ModelType) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ModelType {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ModelType {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`ModelVariant`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ModelVariant {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub body: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<ModelVariantSettings>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ModelVariantSettings`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ModelVariantSettings {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compaction: ::std::option::Option<ProviderCompaction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding: ::std::option::Option<ProviderEncoding>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`MoneyUsdPerMillionTokens`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MoneyUsdPerMillionTokens(pub ::serde_json::Number);
impl ::std::ops::Deref for MoneyUsdPerMillionTokens {
    type Target = ::serde_json::Number;
    fn deref(&self) -> &::serde_json::Number {
        &self.0
    }
}
impl ::std::convert::From<MoneyUsdPerMillionTokens> for ::serde_json::Number {
    fn from(value: MoneyUsdPerMillionTokens) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Number> for MoneyUsdPerMillionTokens {
    fn from(value: ::serde_json::Number) -> Self {
        Self(value)
    }
}
///`PermissionEffect`
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
pub enum PermissionEffect {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "ask")]
    Ask,
}
impl ::std::fmt::Display for PermissionEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allow => f.write_str("allow"),
            Self::Deny => f.write_str("deny"),
            Self::Ask => f.write_str("ask"),
        }
    }
}
impl ::std::str::FromStr for PermissionEffect {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow" => Ok(Self::Allow),
            "deny" => Ok(Self::Deny),
            "ask" => Ok(Self::Ask),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PermissionEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PermissionEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PermissionRule`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PermissionRule {
    pub action: ::std::string::String,
    pub effect: PermissionEffect,
    pub resource: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PermissionRuleset`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PermissionRuleset(pub ::std::vec::Vec<PermissionRule>);
impl ::std::ops::Deref for PermissionRuleset {
    type Target = ::std::vec::Vec<PermissionRule>;
    fn deref(&self) -> &::std::vec::Vec<PermissionRule> {
        &self.0
    }
}
impl ::std::convert::From<PermissionRuleset> for ::std::vec::Vec<PermissionRule> {
    fn from(value: PermissionRuleset) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<PermissionRule>> for PermissionRuleset {
    fn from(value: ::std::vec::Vec<PermissionRule>) -> Self {
        Self(value)
    }
}
///`ProjectId`
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
pub struct ProjectId(pub ::std::string::String);
impl ::std::ops::Deref for ProjectId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectId> for ::std::string::String {
    fn from(value: ProjectId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ProjectId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ProjectId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`ProviderCache`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ProviderCache {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl: ::std::option::Option<ProviderCacheTtl>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub warming: ::std::option::Option<ProviderCacheWarming>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProviderCacheTtl`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProviderCacheTtl(::std::string::String);
impl ::std::ops::Deref for ProviderCacheTtl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProviderCacheTtl> for ::std::string::String {
    fn from(value: ProviderCacheTtl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProviderCacheTtl {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[+-]?(?:\\d+(?:\\.\\d+)?|\\.\\d+)(?:[Ee][+-]?\\d+)?(?:ms|s|m|h|d)$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[+-]?(?:\\d+(?:\\.\\d+)?|\\.\\d+)(?:[Ee][+-]?\\d+)?(?:ms|s|m|h|d)$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProviderCacheTtl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProviderCacheTtl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProviderCacheTtl {
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
///`ProviderCacheWarming`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ProviderCacheWarming {
    Boolean(bool),
    Object {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        duration: ::std::option::Option<::std::string::String>,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        interval: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
impl ::std::convert::From<bool> for ProviderCacheWarming {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
///`ProviderCompaction`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ProviderCompaction {
    Variant0 {
        #[serde(rename = "type")]
        type_: ProviderCompactionVariant0Type,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
    Variant1 {
        #[serde(rename = "type")]
        type_: ProviderCompactionVariant1Type,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
///`ProviderCompactionVariant0Type`
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
pub enum ProviderCompactionVariant0Type {
    #[serde(rename = "summary")]
    Summary,
}
impl ::std::fmt::Display for ProviderCompactionVariant0Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Summary => f.write_str("summary"),
        }
    }
}
impl ::std::str::FromStr for ProviderCompactionVariant0Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "summary" => Ok(Self::Summary),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProviderCompactionVariant0Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProviderCompactionVariant0Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ProviderCompactionVariant1Type`
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
pub enum ProviderCompactionVariant1Type {
    #[serde(rename = "native")]
    Native,
}
impl ::std::fmt::Display for ProviderCompactionVariant1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Native => f.write_str("native"),
        }
    }
}
impl ::std::str::FromStr for ProviderCompactionVariant1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "native" => Ok(Self::Native),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProviderCompactionVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProviderCompactionVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ProviderEncoding`
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
pub enum ProviderEncoding {
    #[serde(rename = "zstd")]
    Zstd,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "none")]
    None,
}
impl ::std::fmt::Display for ProviderEncoding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Zstd => f.write_str("zstd"),
            Self::Gzip => f.write_str("gzip"),
            Self::None => f.write_str("none"),
        }
    }
}
impl ::std::str::FromStr for ProviderEncoding {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "zstd" => Ok(Self::Zstd),
            "gzip" => Ok(Self::Gzip),
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProviderEncoding {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProviderEncoding {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ProviderId`
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
pub struct ProviderId(pub ::std::string::String);
impl ::std::ops::Deref for ProviderId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProviderId> for ::std::string::String {
    fn from(value: ProviderId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for ProviderId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for ProviderId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`ProviderRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProviderRequest {
    pub body: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    pub settings: ProviderSettings,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProviderSettings`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProviderSettings {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache: ::std::option::Option<ProviderCache>,
    #[serde(
        rename = "chunkTimeout",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub chunk_timeout: ::std::option::Option<super::ProviderTimeout>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compaction: ::std::option::Option<ProviderCompaction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding: ::std::option::Option<ProviderEncoding>,
    #[serde(
        rename = "headerTimeout",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub header_timeout: ::std::option::Option<super::ProviderTimeout>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<super::ProviderTimeout>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transport: ::std::option::Option<ProviderTransport>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProviderTransport`
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
pub enum ProviderTransport {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "websocket")]
    Websocket,
}
impl ::std::fmt::Display for ProviderTransport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Http => f.write_str("http"),
            Self::Websocket => f.write_str("websocket"),
        }
    }
}
impl ::std::str::FromStr for ProviderTransport {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "http" => Ok(Self::Http),
            "websocket" => Ok(Self::Websocket),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProviderTransport {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProviderTransport {
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
