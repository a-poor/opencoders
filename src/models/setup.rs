// Generated from openapi/opencode-v2.json by model-spike --setup.
// Do not edit manually; adaptations live in tools/model-spike/src/setup.rs.
///`ConfigAgentEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigAgentEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<ConfigAgentEncodedColor>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disabled: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<ConfigAgentEncodedMode>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<ConfigAgentEncodedModel>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<super::PermissionRuleset>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub request: ::std::option::Option<ConfigAgentEncodedRequest>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub steps: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub system: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigAgentEncodedColor`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigAgentEncodedColor(::std::string::String);
impl ::std::ops::Deref for ConfigAgentEncodedColor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigAgentEncodedColor> for ::std::string::String {
    fn from(value: ConfigAgentEncodedColor) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigAgentEncodedColor {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^#[0-9a-fA-F]{6}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#[0-9a-fA-F]{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigAgentEncodedColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigAgentEncodedColor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigAgentEncodedColor {
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
///`ConfigAgentEncodedMode`
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
pub enum ConfigAgentEncodedMode {
    #[serde(rename = "subagent")]
    Subagent,
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "all")]
    All,
}
impl ::std::fmt::Display for ConfigAgentEncodedMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Subagent => f.write_str("subagent"),
            Self::Primary => f.write_str("primary"),
            Self::All => f.write_str("all"),
        }
    }
}
impl ::std::str::FromStr for ConfigAgentEncodedMode {
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
impl ::std::convert::TryFrom<&str> for ConfigAgentEncodedMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigAgentEncodedMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigAgentEncodedModel`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigAgentEncodedModel {
    String(ConfigAgentEncodedModelString),
    Object {
        model: ConfigAgentEncodedModelObjectModel,
        #[serde(rename = "providerID")]
        provider_id: ConfigAgentEncodedModelObjectProviderId,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        variant: ::std::option::Option<ConfigAgentEncodedModelObjectVariant>,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
impl ::std::convert::From<ConfigAgentEncodedModelString> for ConfigAgentEncodedModel {
    fn from(value: ConfigAgentEncodedModelString) -> Self {
        Self::String(value)
    }
}
///`ConfigAgentEncodedModelObjectModel`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigAgentEncodedModelObjectModel(::std::string::String);
impl ::std::ops::Deref for ConfigAgentEncodedModelObjectModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigAgentEncodedModelObjectModel> for ::std::string::String {
    fn from(value: ConfigAgentEncodedModelObjectModel) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigAgentEncodedModelObjectModel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigAgentEncodedModelObjectModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigAgentEncodedModelObjectModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigAgentEncodedModelObjectModel {
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
///`ConfigAgentEncodedModelObjectProviderId`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigAgentEncodedModelObjectProviderId(::std::string::String);
impl ::std::ops::Deref for ConfigAgentEncodedModelObjectProviderId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigAgentEncodedModelObjectProviderId>
for ::std::string::String {
    fn from(value: ConfigAgentEncodedModelObjectProviderId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigAgentEncodedModelObjectProviderId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^/#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigAgentEncodedModelObjectProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigAgentEncodedModelObjectProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigAgentEncodedModelObjectProviderId {
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
///`ConfigAgentEncodedModelObjectVariant`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigAgentEncodedModelObjectVariant(::std::string::String);
impl ::std::ops::Deref for ConfigAgentEncodedModelObjectVariant {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigAgentEncodedModelObjectVariant>
for ::std::string::String {
    fn from(value: ConfigAgentEncodedModelObjectVariant) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigAgentEncodedModelObjectVariant {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigAgentEncodedModelObjectVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigAgentEncodedModelObjectVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigAgentEncodedModelObjectVariant {
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
///`ConfigAgentEncodedModelString`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigAgentEncodedModelString(::std::string::String);
impl ::std::ops::Deref for ConfigAgentEncodedModelString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigAgentEncodedModelString> for ::std::string::String {
    fn from(value: ConfigAgentEncodedModelString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigAgentEncodedModelString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^/#]+\\/[^#]+(?:#[^#]+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/#]+\\/[^#]+(?:#[^#]+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigAgentEncodedModelString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigAgentEncodedModelString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigAgentEncodedModelString {
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
///`ConfigAgentEncodedRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigAgentEncodedRequest {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub body: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigCommandEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigCommandEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub agent: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<ConfigCommandEncodedModel>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subagent: ::std::option::Option<bool>,
    ///Deprecated alias for subagent.
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subtask: ::std::option::Option<bool>,
    pub template: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigCommandEncodedModel`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigCommandEncodedModel {
    String(ConfigCommandEncodedModelString),
    Object {
        model: ConfigCommandEncodedModelObjectModel,
        #[serde(rename = "providerID")]
        provider_id: ConfigCommandEncodedModelObjectProviderId,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        variant: ::std::option::Option<ConfigCommandEncodedModelObjectVariant>,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
impl ::std::convert::From<ConfigCommandEncodedModelString>
for ConfigCommandEncodedModel {
    fn from(value: ConfigCommandEncodedModelString) -> Self {
        Self::String(value)
    }
}
///`ConfigCommandEncodedModelObjectModel`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigCommandEncodedModelObjectModel(::std::string::String);
impl ::std::ops::Deref for ConfigCommandEncodedModelObjectModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigCommandEncodedModelObjectModel>
for ::std::string::String {
    fn from(value: ConfigCommandEncodedModelObjectModel) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigCommandEncodedModelObjectModel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigCommandEncodedModelObjectModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigCommandEncodedModelObjectModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigCommandEncodedModelObjectModel {
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
///`ConfigCommandEncodedModelObjectProviderId`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigCommandEncodedModelObjectProviderId(::std::string::String);
impl ::std::ops::Deref for ConfigCommandEncodedModelObjectProviderId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigCommandEncodedModelObjectProviderId>
for ::std::string::String {
    fn from(value: ConfigCommandEncodedModelObjectProviderId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigCommandEncodedModelObjectProviderId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^/#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigCommandEncodedModelObjectProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigCommandEncodedModelObjectProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigCommandEncodedModelObjectProviderId {
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
///`ConfigCommandEncodedModelObjectVariant`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigCommandEncodedModelObjectVariant(::std::string::String);
impl ::std::ops::Deref for ConfigCommandEncodedModelObjectVariant {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigCommandEncodedModelObjectVariant>
for ::std::string::String {
    fn from(value: ConfigCommandEncodedModelObjectVariant) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigCommandEncodedModelObjectVariant {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigCommandEncodedModelObjectVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigCommandEncodedModelObjectVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigCommandEncodedModelObjectVariant {
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
///`ConfigCommandEncodedModelString`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigCommandEncodedModelString(::std::string::String);
impl ::std::ops::Deref for ConfigCommandEncodedModelString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigCommandEncodedModelString> for ::std::string::String {
    fn from(value: ConfigCommandEncodedModelString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigCommandEncodedModelString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^/#]+\\/[^#]+(?:#[^#]+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/#]+\\/[^#]+(?:#[^#]+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigCommandEncodedModelString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigCommandEncodedModelString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigCommandEncodedModelString {
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
///`ConfigCompactionPercent`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigCompactionPercent(::std::string::String);
impl ::std::ops::Deref for ConfigCompactionPercent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigCompactionPercent> for ::std::string::String {
    fn from(value: ConfigCompactionPercent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigCompactionPercent {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^([1-9]\\d?(\\.\\d+)?)%$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^([1-9]\\d?(\\.\\d+)?)%$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigCompactionPercent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigCompactionPercent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigCompactionPercent {
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
///`ConfigDirectoryEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigDirectoryEncoded {
    pub path: super::AbsolutePath,
    #[serde(rename = "type")]
    pub type_: ConfigDirectoryEncodedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigDirectoryEncodedType`
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
pub enum ConfigDirectoryEncodedType {
    #[serde(rename = "directory")]
    Directory,
}
impl ::std::fmt::Display for ConfigDirectoryEncodedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Directory => f.write_str("directory"),
        }
    }
}
impl ::std::str::FromStr for ConfigDirectoryEncodedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "directory" => Ok(Self::Directory),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigDirectoryEncodedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigDirectoryEncodedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigDocumentEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigDocumentEncoded {
    pub info: ConfigInfoEncoded,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<super::AbsolutePath>,
    #[serde(rename = "type")]
    pub type_: ConfigDocumentEncodedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigDocumentEncodedType`
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
pub enum ConfigDocumentEncodedType {
    #[serde(rename = "document")]
    Document,
}
impl ::std::fmt::Display for ConfigDocumentEncodedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Document => f.write_str("document"),
        }
    }
}
impl ::std::str::FromStr for ConfigDocumentEncodedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "document" => Ok(Self::Document),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigDocumentEncodedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigDocumentEncodedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigFormatterEntryEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigFormatterEntryEncoded {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub command: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub environment: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncoded {
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub agents: ::std::collections::HashMap<::std::string::String, ConfigAgentEncoded>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub commands: ::std::collections::HashMap<
        ::std::string::String,
        ConfigCommandEncoded,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compaction: ::std::option::Option<ConfigInfoEncodedCompaction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default_agent: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub enterprise: ::std::option::Option<ConfigInfoEncodedEnterprise>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub experimental: ::std::option::Option<ConfigInfoEncodedExperimental>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub formatter: ::std::option::Option<ConfigInfoEncodedFormatter>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub instructions: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lsp: ::std::option::Option<ConfigInfoEncodedLsp>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub mcp: ::std::option::Option<ConfigInfoEncodedMcp>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub media: ::std::option::Option<ConfigInfoEncodedMedia>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<ConfigInfoEncodedModel>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<super::PermissionRuleset>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub plugins: ::std::vec::Vec<ConfigInfoEncodedPluginsItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub policies: ::std::vec::Vec<ConfigInfoEncodedPoliciesItem>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub providers: ::std::collections::HashMap<
        ::std::string::String,
        ConfigProviderEncoded,
    >,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub references: ::std::collections::HashMap<
        ::std::string::String,
        ConfigInfoEncodedReferencesValue,
    >,
    #[serde(rename = "$schema", skip_serializing_if = "::std::option::Option::is_none")]
    pub schema: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub share: ::std::option::Option<ConfigInfoEncodedShare>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub shell: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub skills: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub snapshots: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timezone: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_output: ::std::option::Option<ConfigInfoEncodedToolOutput>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub update: ::std::option::Option<ConfigInfoEncodedUpdate>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub username: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub warming: ::std::option::Option<ConfigInfoEncodedWarming>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub watcher: ::std::option::Option<ConfigInfoEncodedWatcher>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub websearch: ::std::option::Option<ConfigInfoEncodedWebsearch>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub worktree: ::std::option::Option<ConfigWorktree>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedCompaction`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedCompaction {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub auto: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub buffer: ::std::option::Option<ConfigInfoEncodedCompactionBuffer>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub keep: ::std::option::Option<ConfigInfoEncodedCompactionKeep>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedCompactionBuffer`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedCompactionBuffer {
    ConfigCompactionPercent(ConfigCompactionPercent),
    Integer(super::SchemaInteger<0, 9007199254740991>),
}
impl ::std::convert::From<ConfigCompactionPercent>
for ConfigInfoEncodedCompactionBuffer {
    fn from(value: ConfigCompactionPercent) -> Self {
        Self::ConfigCompactionPercent(value)
    }
}
impl ::std::convert::From<super::SchemaInteger<0, 9007199254740991>>
for ConfigInfoEncodedCompactionBuffer {
    fn from(value: super::SchemaInteger<0, 9007199254740991>) -> Self {
        Self::Integer(value)
    }
}
///`ConfigInfoEncodedCompactionKeep`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedCompactionKeep {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokens: ::std::option::Option<super::SchemaInteger<0, 9007199254740991>>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedEnterprise`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedEnterprise {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedExperimental`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedExperimental {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub all_flags: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub artifacts: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub background_compaction: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub codemode_vm: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image_generate: ::std::option::Option<
        ConfigInfoEncodedExperimentalImageGenerate,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location_idle_timeout: ::std::option::Option<
        ConfigInfoEncodedExperimentalLocationIdleTimeout,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub media_blob: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub part_rows: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub policies: ::std::vec::Vec<ConfigInfoEncodedExperimentalPoliciesItem>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub portable_shell_scanner: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub schedules: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subagent_depth: ::std::option::Option<super::SchemaInteger<0, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub todo: ::std::option::Option<bool>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedExperimentalImageGenerate`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedExperimentalImageGenerate {
    Boolean(bool),
    Object {
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        model: ::std::option::Option<
            ConfigInfoEncodedExperimentalImageGenerateObjectModel,
        >,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
impl ::std::convert::From<bool> for ConfigInfoEncodedExperimentalImageGenerate {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
///`ConfigInfoEncodedExperimentalImageGenerateObjectModel`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedExperimentalImageGenerateObjectModel {
    String(ConfigInfoEncodedExperimentalImageGenerateObjectModelString),
    Object {
        model: ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel,
        #[serde(rename = "providerID")]
        provider_id: ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        variant: ::std::option::Option<
            ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant,
        >,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
impl ::std::convert::From<ConfigInfoEncodedExperimentalImageGenerateObjectModelString>
for ConfigInfoEncodedExperimentalImageGenerateObjectModel {
    fn from(value: ConfigInfoEncodedExperimentalImageGenerateObjectModelString) -> Self {
        Self::String(value)
    }
}
///`ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel(
    ::std::string::String,
);
impl ::std::ops::Deref
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<
    ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel,
> for ::std::string::String {
    fn from(
        value: ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel,
    ) -> Self {
        value.0
    }
}
impl ::std::str::FromStr
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel {
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
///`ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId(
    ::std::string::String,
);
impl ::std::ops::Deref
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<
    ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId,
> for ::std::string::String {
    fn from(
        value: ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId,
    ) -> Self {
        value.0
    }
}
impl ::std::str::FromStr
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^/#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId {
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
///`ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant(
    ::std::string::String,
);
impl ::std::ops::Deref
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<
    ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant,
> for ::std::string::String {
    fn from(
        value: ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant,
    ) -> Self {
        value.0
    }
}
impl ::std::str::FromStr
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant {
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
///`ConfigInfoEncodedExperimentalImageGenerateObjectModelString`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigInfoEncodedExperimentalImageGenerateObjectModelString(
    ::std::string::String,
);
impl ::std::ops::Deref for ConfigInfoEncodedExperimentalImageGenerateObjectModelString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigInfoEncodedExperimentalImageGenerateObjectModelString>
for ::std::string::String {
    fn from(value: ConfigInfoEncodedExperimentalImageGenerateObjectModelString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr
for ConfigInfoEncodedExperimentalImageGenerateObjectModelString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^/#]+\\/[^#]+(?:#[^#]+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/#]+\\/[^#]+(?:#[^#]+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for ConfigInfoEncodedExperimentalImageGenerateObjectModelString {
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
///`ConfigInfoEncodedExperimentalLocationIdleTimeout`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedExperimentalLocationIdleTimeout {
    String(::std::string::String),
    Boolean(super::SchemaBoolean<false>),
}
impl ::std::convert::From<super::SchemaBoolean<false>>
for ConfigInfoEncodedExperimentalLocationIdleTimeout {
    fn from(value: super::SchemaBoolean<false>) -> Self {
        Self::Boolean(value)
    }
}
///`ConfigInfoEncodedExperimentalPoliciesItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedExperimentalPoliciesItem {
    pub action: ConfigInfoEncodedExperimentalPoliciesItemAction,
    pub effect: ConfigInfoEncodedExperimentalPoliciesItemEffect,
    pub resource: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedExperimentalPoliciesItemAction`
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
pub enum ConfigInfoEncodedExperimentalPoliciesItemAction {
    #[serde(rename = "provider.use")]
    ProviderUse,
    #[serde(rename = "tool.use")]
    ToolUse,
    #[serde(rename = "integration.connect")]
    IntegrationConnect,
    #[serde(rename = "integration.use")]
    IntegrationUse,
    #[serde(rename = "mcp.use")]
    McpUse,
    #[serde(rename = "skill.use")]
    SkillUse,
    #[serde(rename = "plugin.use")]
    PluginUse,
    #[serde(rename = "websearch.use")]
    WebsearchUse,
}
impl ::std::fmt::Display for ConfigInfoEncodedExperimentalPoliciesItemAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProviderUse => f.write_str("provider.use"),
            Self::ToolUse => f.write_str("tool.use"),
            Self::IntegrationConnect => f.write_str("integration.connect"),
            Self::IntegrationUse => f.write_str("integration.use"),
            Self::McpUse => f.write_str("mcp.use"),
            Self::SkillUse => f.write_str("skill.use"),
            Self::PluginUse => f.write_str("plugin.use"),
            Self::WebsearchUse => f.write_str("websearch.use"),
        }
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedExperimentalPoliciesItemAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "provider.use" => Ok(Self::ProviderUse),
            "tool.use" => Ok(Self::ToolUse),
            "integration.connect" => Ok(Self::IntegrationConnect),
            "integration.use" => Ok(Self::IntegrationUse),
            "mcp.use" => Ok(Self::McpUse),
            "skill.use" => Ok(Self::SkillUse),
            "plugin.use" => Ok(Self::PluginUse),
            "websearch.use" => Ok(Self::WebsearchUse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedExperimentalPoliciesItemAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedExperimentalPoliciesItemAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigInfoEncodedExperimentalPoliciesItemEffect`
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
pub enum ConfigInfoEncodedExperimentalPoliciesItemEffect {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}
impl ::std::fmt::Display for ConfigInfoEncodedExperimentalPoliciesItemEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allow => f.write_str("allow"),
            Self::Deny => f.write_str("deny"),
        }
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedExperimentalPoliciesItemEffect {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow" => Ok(Self::Allow),
            "deny" => Ok(Self::Deny),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedExperimentalPoliciesItemEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedExperimentalPoliciesItemEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigInfoEncodedFormatter`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedFormatter {
    Boolean(bool),
    Object(
        ::std::collections::HashMap<::std::string::String, ConfigFormatterEntryEncoded>,
    ),
}
impl ::std::convert::From<bool> for ConfigInfoEncodedFormatter {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<
    ::std::collections::HashMap<::std::string::String, ConfigFormatterEntryEncoded>,
> for ConfigInfoEncodedFormatter {
    fn from(
        value: ::std::collections::HashMap<
            ::std::string::String,
            ConfigFormatterEntryEncoded,
        >,
    ) -> Self {
        Self::Object(value)
    }
}
///`ConfigInfoEncodedLsp`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedLsp {
    Boolean(bool),
    Object(
        ::std::collections::HashMap<
            ::std::string::String,
            ConfigInfoEncodedLspObjectValue,
        >,
    ),
}
impl ::std::convert::From<bool> for ConfigInfoEncodedLsp {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<
    ::std::collections::HashMap<::std::string::String, ConfigInfoEncodedLspObjectValue>,
> for ConfigInfoEncodedLsp {
    fn from(
        value: ::std::collections::HashMap<
            ::std::string::String,
            ConfigInfoEncodedLspObjectValue,
        >,
    ) -> Self {
        Self::Object(value)
    }
}
///`ConfigInfoEncodedLspObjectValue`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedLspObjectValue {
    Object {
        disabled: super::SchemaBoolean<true>,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
    ConfigLspServerEncoded(ConfigLspServerEncoded),
}
impl ::std::convert::From<ConfigLspServerEncoded> for ConfigInfoEncodedLspObjectValue {
    fn from(value: ConfigLspServerEncoded) -> Self {
        Self::ConfigLspServerEncoded(value)
    }
}
///`ConfigInfoEncodedMcp`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedMcp {
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub servers: ::std::collections::HashMap<
        ::std::string::String,
        ConfigInfoEncodedMcpServersValue,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<ConfigInfoEncodedMcpTimeout>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedMcpServersValue`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedMcpServersValue {
    LocalConfigEncoded(McpLocalConfigEncoded),
    RemoteConfigEncoded(McpRemoteConfigEncoded),
}
impl ::std::convert::From<McpLocalConfigEncoded> for ConfigInfoEncodedMcpServersValue {
    fn from(value: McpLocalConfigEncoded) -> Self {
        Self::LocalConfigEncoded(value)
    }
}
impl ::std::convert::From<McpRemoteConfigEncoded> for ConfigInfoEncodedMcpServersValue {
    fn from(value: McpRemoteConfigEncoded) -> Self {
        Self::RemoteConfigEncoded(value)
    }
}
///`ConfigInfoEncodedMcpTimeout`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedMcpTimeout {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub catalog: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub execution: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub startup: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedMedia`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedMedia {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<ConfigInfoEncodedMediaImage>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedMediaImage`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedMediaImage {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub auto_resize: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_base64_bytes: ::std::option::Option<
        super::SchemaInteger<1, 9007199254740991>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_height: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_width: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedModel`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedModel {
    String(ConfigInfoEncodedModelString),
    Object {
        model: ConfigInfoEncodedModelObjectModel,
        #[serde(rename = "providerID")]
        provider_id: ConfigInfoEncodedModelObjectProviderId,
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        variant: ::std::option::Option<ConfigInfoEncodedModelObjectVariant>,
        #[serde(flatten)]
        extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    },
}
impl ::std::convert::From<ConfigInfoEncodedModelString> for ConfigInfoEncodedModel {
    fn from(value: ConfigInfoEncodedModelString) -> Self {
        Self::String(value)
    }
}
///`ConfigInfoEncodedModelObjectModel`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigInfoEncodedModelObjectModel(::std::string::String);
impl ::std::ops::Deref for ConfigInfoEncodedModelObjectModel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigInfoEncodedModelObjectModel> for ::std::string::String {
    fn from(value: ConfigInfoEncodedModelObjectModel) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedModelObjectModel {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedModelObjectModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedModelObjectModel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigInfoEncodedModelObjectModel {
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
///`ConfigInfoEncodedModelObjectProviderId`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigInfoEncodedModelObjectProviderId(::std::string::String);
impl ::std::ops::Deref for ConfigInfoEncodedModelObjectProviderId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigInfoEncodedModelObjectProviderId>
for ::std::string::String {
    fn from(value: ConfigInfoEncodedModelObjectProviderId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedModelObjectProviderId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^/#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedModelObjectProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedModelObjectProviderId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigInfoEncodedModelObjectProviderId {
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
///`ConfigInfoEncodedModelObjectVariant`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigInfoEncodedModelObjectVariant(::std::string::String);
impl ::std::ops::Deref for ConfigInfoEncodedModelObjectVariant {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigInfoEncodedModelObjectVariant>
for ::std::string::String {
    fn from(value: ConfigInfoEncodedModelObjectVariant) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedModelObjectVariant {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^#]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^#]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedModelObjectVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedModelObjectVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigInfoEncodedModelObjectVariant {
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
///`ConfigInfoEncodedModelString`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigInfoEncodedModelString(::std::string::String);
impl ::std::ops::Deref for ConfigInfoEncodedModelString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigInfoEncodedModelString> for ::std::string::String {
    fn from(value: ConfigInfoEncodedModelString) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedModelString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[^/#]+\\/[^#]+(?:#[^#]+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[^/#]+\\/[^#]+(?:#[^#]+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedModelString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigInfoEncodedModelString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigInfoEncodedModelString {
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
///`ConfigInfoEncodedPluginsItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedPluginsItem {
    String(::std::string::String),
    ConfigPluginEntryEncoded(ConfigPluginEntryEncoded),
}
impl ::std::convert::From<ConfigPluginEntryEncoded> for ConfigInfoEncodedPluginsItem {
    fn from(value: ConfigPluginEntryEncoded) -> Self {
        Self::ConfigPluginEntryEncoded(value)
    }
}
///`ConfigInfoEncodedPoliciesItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedPoliciesItem {
    pub action: ConfigInfoEncodedPoliciesItemAction,
    pub effect: ConfigInfoEncodedPoliciesItemEffect,
    pub resource: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedPoliciesItemAction`
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
pub enum ConfigInfoEncodedPoliciesItemAction {
    #[serde(rename = "provider.use")]
    ProviderUse,
    #[serde(rename = "tool.use")]
    ToolUse,
    #[serde(rename = "integration.connect")]
    IntegrationConnect,
    #[serde(rename = "integration.use")]
    IntegrationUse,
    #[serde(rename = "mcp.use")]
    McpUse,
    #[serde(rename = "skill.use")]
    SkillUse,
    #[serde(rename = "plugin.use")]
    PluginUse,
    #[serde(rename = "websearch.use")]
    WebsearchUse,
}
impl ::std::fmt::Display for ConfigInfoEncodedPoliciesItemAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProviderUse => f.write_str("provider.use"),
            Self::ToolUse => f.write_str("tool.use"),
            Self::IntegrationConnect => f.write_str("integration.connect"),
            Self::IntegrationUse => f.write_str("integration.use"),
            Self::McpUse => f.write_str("mcp.use"),
            Self::SkillUse => f.write_str("skill.use"),
            Self::PluginUse => f.write_str("plugin.use"),
            Self::WebsearchUse => f.write_str("websearch.use"),
        }
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedPoliciesItemAction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "provider.use" => Ok(Self::ProviderUse),
            "tool.use" => Ok(Self::ToolUse),
            "integration.connect" => Ok(Self::IntegrationConnect),
            "integration.use" => Ok(Self::IntegrationUse),
            "mcp.use" => Ok(Self::McpUse),
            "skill.use" => Ok(Self::SkillUse),
            "plugin.use" => Ok(Self::PluginUse),
            "websearch.use" => Ok(Self::WebsearchUse),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedPoliciesItemAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedPoliciesItemAction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigInfoEncodedPoliciesItemEffect`
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
pub enum ConfigInfoEncodedPoliciesItemEffect {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}
impl ::std::fmt::Display for ConfigInfoEncodedPoliciesItemEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allow => f.write_str("allow"),
            Self::Deny => f.write_str("deny"),
        }
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedPoliciesItemEffect {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "allow" => Ok(Self::Allow),
            "deny" => Ok(Self::Deny),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedPoliciesItemEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ConfigInfoEncodedPoliciesItemEffect {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigInfoEncodedReferencesValue`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedReferencesValue {
    String(::std::string::String),
    ConfigReferenceGitEncoded(ConfigReferenceGitEncoded),
    ConfigReferenceLocalEncoded(ConfigReferenceLocalEncoded),
}
impl ::std::convert::From<ConfigReferenceGitEncoded>
for ConfigInfoEncodedReferencesValue {
    fn from(value: ConfigReferenceGitEncoded) -> Self {
        Self::ConfigReferenceGitEncoded(value)
    }
}
impl ::std::convert::From<ConfigReferenceLocalEncoded>
for ConfigInfoEncodedReferencesValue {
    fn from(value: ConfigReferenceLocalEncoded) -> Self {
        Self::ConfigReferenceLocalEncoded(value)
    }
}
///`ConfigInfoEncodedShare`
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
pub enum ConfigInfoEncodedShare {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for ConfigInfoEncodedShare {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Manual => f.write_str("manual"),
            Self::Auto => f.write_str("auto"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedShare {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "manual" => Ok(Self::Manual),
            "auto" => Ok(Self::Auto),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedShare {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigInfoEncodedShare {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigInfoEncodedToolOutput`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedToolOutput {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_bytes: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_lines: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedUpdate`
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
pub enum ConfigInfoEncodedUpdate {
    #[serde(rename = "disable")]
    Disable,
    #[serde(rename = "notify")]
    Notify,
    #[serde(rename = "auto")]
    Auto,
}
impl ::std::fmt::Display for ConfigInfoEncodedUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Disable => f.write_str("disable"),
            Self::Notify => f.write_str("notify"),
            Self::Auto => f.write_str("auto"),
        }
    }
}
impl ::std::str::FromStr for ConfigInfoEncodedUpdate {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "disable" => Ok(Self::Disable),
            "notify" => Ok(Self::Notify),
            "auto" => Ok(Self::Auto),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConfigInfoEncodedUpdate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigInfoEncodedUpdate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigInfoEncodedWarming`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedWarming {
    Boolean(bool),
    ConfigWarmingEncoded(ConfigWarmingEncoded),
}
impl ::std::convert::From<bool> for ConfigInfoEncodedWarming {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<ConfigWarmingEncoded> for ConfigInfoEncodedWarming {
    fn from(value: ConfigWarmingEncoded) -> Self {
        Self::ConfigWarmingEncoded(value)
    }
}
///`ConfigInfoEncodedWatcher`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigInfoEncodedWatcher {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub ignore: ::std::vec::Vec<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigInfoEncodedWebsearch`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigInfoEncodedWebsearch {
    Boolean(super::SchemaBoolean<false>),
    ConfigWebSearchInfoEncoded(ConfigWebSearchInfoEncoded),
}
impl ::std::convert::From<super::SchemaBoolean<false>> for ConfigInfoEncodedWebsearch {
    fn from(value: super::SchemaBoolean<false>) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<ConfigWebSearchInfoEncoded> for ConfigInfoEncodedWebsearch {
    fn from(value: ConfigWebSearchInfoEncoded) -> Self {
        Self::ConfigWebSearchInfoEncoded(value)
    }
}
///`ConfigLspServerEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigLspServerEncoded {
    pub command: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub env: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub initialization: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelCapabilities`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelCapabilities {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub input: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub output: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<ConfigModelReasoning>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub temperature: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tools: ::std::option::Option<bool>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelCostEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelCostEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache: ::std::option::Option<ConfigModelCostEncodedCache>,
    pub input: super::MoneyUsdPerMillionTokens,
    pub output: super::MoneyUsdPerMillionTokens,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tier: ::std::option::Option<ConfigModelCostEncodedTier>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelCostEncodedCache`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelCostEncodedCache {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub read: ::std::option::Option<super::MoneyUsdPerMillionTokens>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub write: ::std::option::Option<super::MoneyUsdPerMillionTokens>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelCostEncodedTier`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelCostEncodedTier {
    pub size: super::SchemaInteger<-9007199254740991, 9007199254740991>,
    #[serde(rename = "type")]
    pub type_: ConfigModelCostEncodedTierType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelCostEncodedTierType`
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
pub enum ConfigModelCostEncodedTierType {
    #[serde(rename = "context")]
    Context,
}
impl ::std::fmt::Display for ConfigModelCostEncodedTierType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Context => f.write_str("context"),
        }
    }
}
impl ::std::str::FromStr for ConfigModelCostEncodedTierType {
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
impl ::std::convert::TryFrom<&str> for ConfigModelCostEncodedTierType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigModelCostEncodedTierType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConfigModelEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelEncoded {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub body: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub capabilities: ::std::option::Option<ConfigModelCapabilities>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compatibility: ::std::option::Option<super::ModelCompatibility>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cost: ::std::option::Option<ConfigModelEncodedCost>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disabled: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub family: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<ConfigModelEncodedLimit>,
    #[serde(rename = "modelID", skip_serializing_if = "::std::option::Option::is_none")]
    pub model_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub package: ::std::option::Option<::std::string::String>,
    ///Release date as YYYY-MM-DD or YYYY-MM
    #[serde(
        rename = "releaseDate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub release_date: ::std::option::Option<ConfigModelEncodedReleaseDate>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<ConfigModelSettings>,
    #[serde(rename = "type", skip_serializing_if = "::std::option::Option::is_none")]
    pub type_: ::std::option::Option<super::ModelType>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub variants: ::std::vec::Vec<ConfigModelEncodedVariantsItem>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelEncodedCost`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConfigModelEncodedCost {
    ConfigModelCostEncoded(ConfigModelCostEncoded),
    Array(::std::vec::Vec<ConfigModelCostEncoded>),
}
impl ::std::convert::From<ConfigModelCostEncoded> for ConfigModelEncodedCost {
    fn from(value: ConfigModelCostEncoded) -> Self {
        Self::ConfigModelCostEncoded(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<ConfigModelCostEncoded>>
for ConfigModelEncodedCost {
    fn from(value: ::std::vec::Vec<ConfigModelCostEncoded>) -> Self {
        Self::Array(value)
    }
}
///`ConfigModelEncodedLimit`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelEncodedLimit {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub context: ::std::option::Option<
        super::SchemaInteger<-9007199254740991, 9007199254740991>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<
        super::SchemaInteger<-9007199254740991, 9007199254740991>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<
        super::SchemaInteger<-9007199254740991, 9007199254740991>,
    >,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///Release date as YYYY-MM-DD or YYYY-MM
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ConfigModelEncodedReleaseDate(::std::string::String);
impl ::std::ops::Deref for ConfigModelEncodedReleaseDate {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ConfigModelEncodedReleaseDate> for ::std::string::String {
    fn from(value: ConfigModelEncodedReleaseDate) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ConfigModelEncodedReleaseDate {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d{4}-\\d{2}(-\\d{2})?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d{4}-\\d{2}(-\\d{2})?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ConfigModelEncodedReleaseDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConfigModelEncodedReleaseDate {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigModelEncodedReleaseDate {
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
///`ConfigModelEncodedVariantsItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelEncodedVariantsItem {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub body: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    pub id: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<ConfigModelVariantSettings>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelReasoning`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Default)]
#[non_exhaustive]
pub struct ConfigModelReasoning {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<ConfigModelReasoningSubtype0>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<ConfigModelReasoningSubtype1>,
}
///`ConfigModelReasoningSubtype0`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelReasoningSubtype0 {
    pub supported: super::SchemaBoolean<false>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelReasoningSubtype1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelReasoningSubtype1 {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub budget: ::std::option::Option<ConfigModelReasoningSubtype1Budget>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub effort: ::std::vec::Vec<::std::string::String>,
    pub supported: super::SchemaBoolean<true>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub toggle: ::std::option::Option<bool>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelReasoningSubtype1Budget`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelReasoningSubtype1Budget {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<
        super::SchemaInteger<-9007199254740991, 9007199254740991>,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub min: ::std::option::Option<
        super::SchemaInteger<-9007199254740991, 9007199254740991>,
    >,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelSettings`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelSettings {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache: ::std::option::Option<super::ProviderCache>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compaction: ::std::option::Option<super::ProviderCompaction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding: ::std::option::Option<super::ProviderEncoding>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigModelVariantSettings`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigModelVariantSettings {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compaction: ::std::option::Option<super::ProviderCompaction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding: ::std::option::Option<super::ProviderEncoding>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigPluginEntryEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigPluginEntryEncoded {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub package: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigProviderEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigProviderEncoded {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub body: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub canonical: ::std::option::Option<super::ProviderId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compatibility: ::std::option::Option<ProviderCompatibility>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub env: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub models: ::std::collections::HashMap<::std::string::String, ConfigModelEncoded>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub package: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<ConfigProviderSettings>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigProviderSettings`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigProviderSettings {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cache: ::std::option::Option<super::ProviderCache>,
    #[serde(
        rename = "chunkTimeout",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub chunk_timeout: ::std::option::Option<super::ProviderTimeout>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compaction: ::std::option::Option<super::ProviderCompaction>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub encoding: ::std::option::Option<super::ProviderEncoding>,
    #[serde(
        rename = "headerTimeout",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub header_timeout: ::std::option::Option<super::ProviderTimeout>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<super::ProviderTimeout>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub transport: ::std::option::Option<super::ProviderTransport>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigReferenceGitEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigReferenceGitEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub repository: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigReferenceLocalEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigReferenceLocalEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub path: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(transparent)]
pub struct ConfigResponse(pub ::std::vec::Vec<super::ConfigEntry>);
impl ::std::ops::Deref for ConfigResponse {
    type Target = ::std::vec::Vec<super::ConfigEntry>;
    fn deref(&self) -> &::std::vec::Vec<super::ConfigEntry> {
        &self.0
    }
}
impl ::std::convert::From<ConfigResponse> for ::std::vec::Vec<super::ConfigEntry> {
    fn from(value: ConfigResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<super::ConfigEntry>> for ConfigResponse {
    fn from(value: ::std::vec::Vec<super::ConfigEntry>) -> Self {
        Self(value)
    }
}
///`ConfigShellOption`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigShellOption {
    pub acceptable: bool,
    pub name: ::std::string::String,
    pub path: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigShellsResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(transparent)]
pub struct ConfigShellsResponse(pub ::std::vec::Vec<ConfigShellOption>);
impl ::std::ops::Deref for ConfigShellsResponse {
    type Target = ::std::vec::Vec<ConfigShellOption>;
    fn deref(&self) -> &::std::vec::Vec<ConfigShellOption> {
        &self.0
    }
}
impl ::std::convert::From<ConfigShellsResponse> for ::std::vec::Vec<ConfigShellOption> {
    fn from(value: ConfigShellsResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<ConfigShellOption>> for ConfigShellsResponse {
    fn from(value: ::std::vec::Vec<ConfigShellOption>) -> Self {
        Self(value)
    }
}
///`ConfigWarmingEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigWarmingEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub interval: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub prompt: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigWebSearchInfoEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigWebSearchInfoEncoded {
    pub provider: WebSearchId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ConfigWorktree`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ConfigWorktree {
    pub directory: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`IntegrationId`
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
pub struct IntegrationId(pub ::std::string::String);
impl ::std::ops::Deref for IntegrationId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<IntegrationId> for ::std::string::String {
    fn from(value: IntegrationId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for IntegrationId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for IntegrationId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for IntegrationId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
///`McpLocalConfigEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpLocalConfigEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub codemode: ::std::option::Option<bool>,
    pub command: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cwd: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub environment: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pool: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<McpProtocol>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<McpLocalConfigEncodedTimeout>,
    #[serde(rename = "type")]
    pub type_: McpLocalConfigEncodedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpLocalConfigEncodedTimeout`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpLocalConfigEncodedTimeout {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub catalog: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub execution: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub startup: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpLocalConfigEncodedType`
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
pub enum McpLocalConfigEncodedType {
    #[serde(rename = "local")]
    Local,
}
impl ::std::fmt::Display for McpLocalConfigEncodedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Local => f.write_str("local"),
        }
    }
}
impl ::std::str::FromStr for McpLocalConfigEncodedType {
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
impl ::std::convert::TryFrom<&str> for McpLocalConfigEncodedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpLocalConfigEncodedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`McpOAuthConfigEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpOAuthConfigEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub auth_server_metadata_url: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub callback_port: ::std::option::Option<super::SchemaInteger<1, 65535>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub client_secret: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub redirect_uri: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///MCP protocol negotiation. "legacy" (default) opens with the initialize handshake and speaks protocol revisions up to 2025-11-25. "auto" probes for the 2026-07-28 revision and falls back to legacy when the server does not support it. "2026-07-28" requires that revision and fails otherwise.
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
pub enum McpProtocol {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "2026-07-28")]
    X20260728,
}
impl ::std::fmt::Display for McpProtocol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Legacy => f.write_str("legacy"),
            Self::Auto => f.write_str("auto"),
            Self::X20260728 => f.write_str("2026-07-28"),
        }
    }
}
impl ::std::str::FromStr for McpProtocol {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "legacy" => Ok(Self::Legacy),
            "auto" => Ok(Self::Auto),
            "2026-07-28" => Ok(Self::X20260728),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`McpRemoteConfigEncoded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpRemoteConfigEncoded {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub codemode: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub oauth: ::std::option::Option<McpRemoteConfigEncodedOauth>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pool: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol: ::std::option::Option<McpProtocol>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout: ::std::option::Option<McpRemoteConfigEncodedTimeout>,
    #[serde(rename = "type")]
    pub type_: McpRemoteConfigEncodedType,
    pub url: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpRemoteConfigEncodedOauth`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum McpRemoteConfigEncodedOauth {
    McpOAuthConfigEncoded(McpOAuthConfigEncoded),
    Boolean(super::SchemaBoolean<false>),
}
impl ::std::convert::From<McpOAuthConfigEncoded> for McpRemoteConfigEncodedOauth {
    fn from(value: McpOAuthConfigEncoded) -> Self {
        Self::McpOAuthConfigEncoded(value)
    }
}
impl ::std::convert::From<super::SchemaBoolean<false>> for McpRemoteConfigEncodedOauth {
    fn from(value: super::SchemaBoolean<false>) -> Self {
        Self::Boolean(value)
    }
}
///`McpRemoteConfigEncodedTimeout`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct McpRemoteConfigEncodedTimeout {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub catalog: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub execution: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub startup: ::std::option::Option<super::SchemaInteger<1, 9007199254740991>>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`McpRemoteConfigEncodedType`
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
pub enum McpRemoteConfigEncodedType {
    #[serde(rename = "remote")]
    Remote,
}
impl ::std::fmt::Display for McpRemoteConfigEncodedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Remote => f.write_str("remote"),
        }
    }
}
impl ::std::str::FromStr for McpRemoteConfigEncodedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "remote" => Ok(Self::Remote),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for McpRemoteConfigEncodedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for McpRemoteConfigEncodedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`Project`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct Project {
    pub canonical: super::AbsolutePath,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub commands: ::std::option::Option<ProjectCommands>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub icon: ::std::option::Option<ProjectIcon>,
    pub id: super::ProjectId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub sandboxes: ::std::vec::Vec<::std::string::String>,
    pub time: ProjectTime,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vcs: ::std::option::Option<ProjectVcs>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProjectCommands`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProjectCommands {
    ///Startup script to run when creating a new workspace (worktree)
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProjectIcon`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProjectIcon {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "override", skip_serializing_if = "::std::option::Option::is_none")]
    pub override_: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProjectListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(transparent)]
pub struct ProjectListResponse(pub ::std::vec::Vec<Project>);
impl ::std::ops::Deref for ProjectListResponse {
    type Target = ::std::vec::Vec<Project>;
    fn deref(&self) -> &::std::vec::Vec<Project> {
        &self.0
    }
}
impl ::std::convert::From<ProjectListResponse> for ::std::vec::Vec<Project> {
    fn from(value: ProjectListResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Project>> for ProjectListResponse {
    fn from(value: ::std::vec::Vec<Project>) -> Self {
        Self(value)
    }
}
///`ProjectTime`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProjectTime {
    pub active: super::SchemaInteger<0, 9007199254740991>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub archived: ::std::option::Option<super::SchemaInteger<0, 9007199254740991>>,
    pub created: super::SchemaInteger<0, 9007199254740991>,
    pub updated: super::SchemaInteger<0, 9007199254740991>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProjectVcs`
#[derive(::serde::Serialize, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProjectVcs(::std::string::String);
impl ::std::ops::Deref for ProjectVcs {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProjectVcs> for ::std::string::String {
    fn from(value: ProjectVcs) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProjectVcs {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z][a-z0-9._-]*$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z][a-z0-9._-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProjectVcs {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProjectVcs {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProjectVcs {
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
///`ProviderCompatibility`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProviderCompatibility {
    #[serde(
        rename = "cacheFormat",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cache_format: ::std::option::Option<super::ModelCacheFormat>,
    #[serde(
        rename = "maxTokensField",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_tokens_field: ::std::option::Option<super::ModelMaxTokensField>,
    #[serde(
        rename = "mediaInput",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub media_input: ::std::option::Option<super::ModelMediaInput>,
    #[serde(
        rename = "reasoningField",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reasoning_field: ::std::option::Option<super::ModelReasoningField>,
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
        rename = "supportsPromptCacheKey",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supports_prompt_cache_key: ::std::option::Option<bool>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProviderInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProviderInfo {
    pub activation: ProviderInfoActivation,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub body: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub canonical: ::std::option::Option<super::ProviderId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub compatibility: ::std::option::Option<ProviderCompatibility>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub headers: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    pub id: super::ProviderId,
    #[serde(
        rename = "integrationID",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub integration_id: ::std::option::Option<IntegrationId>,
    pub name: ::std::string::String,
    pub package: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub settings: ::std::option::Option<super::ProviderSettings>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProviderInfoActivation`
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
pub enum ProviderInfoActivation {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}
impl ::std::fmt::Display for ProviderInfoActivation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Enabled => f.write_str("enabled"),
            Self::Disabled => f.write_str("disabled"),
        }
    }
}
impl ::std::str::FromStr for ProviderInfoActivation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProviderInfoActivation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProviderInfoActivation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ProviderListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProviderListResponse {
    pub data: ::std::vec::Vec<ProviderInfo>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ProviderResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ProviderResponse {
    pub data: ProviderInfo,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ReferenceGitSource`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ReferenceGitSource {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<::std::string::String>,
    pub repository: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ReferenceGitSourceType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ReferenceGitSourceType`
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
pub enum ReferenceGitSourceType {
    #[serde(rename = "git")]
    Git,
}
impl ::std::fmt::Display for ReferenceGitSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Git => f.write_str("git"),
        }
    }
}
impl ::std::str::FromStr for ReferenceGitSourceType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "git" => Ok(Self::Git),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReferenceGitSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReferenceGitSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ReferenceInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ReferenceInfo {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub name: ::std::string::String,
    pub path: super::AbsolutePath,
    pub source: ReferenceSource,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ReferenceListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ReferenceListResponse {
    pub data: ::std::vec::Vec<ReferenceInfo>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ReferenceLocalSource`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ReferenceLocalSource {
    pub path: super::AbsolutePath,
    #[serde(rename = "type")]
    pub type_: ReferenceLocalSourceType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ReferenceLocalSourceType`
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
pub enum ReferenceLocalSourceType {
    #[serde(rename = "local")]
    Local,
}
impl ::std::fmt::Display for ReferenceLocalSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Local => f.write_str("local"),
        }
    }
}
impl ::std::str::FromStr for ReferenceLocalSourceType {
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
impl ::std::convert::TryFrom<&str> for ReferenceLocalSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReferenceLocalSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ReferenceSource`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ReferenceSource {
    LocalSource(ReferenceLocalSource),
    GitSource(ReferenceGitSource),
}
impl ::std::convert::From<ReferenceLocalSource> for ReferenceSource {
    fn from(value: ReferenceLocalSource) -> Self {
        Self::LocalSource(value)
    }
}
impl ::std::convert::From<ReferenceGitSource> for ReferenceSource {
    fn from(value: ReferenceGitSource) -> Self {
        Self::GitSource(value)
    }
}
///`WebSearchId`
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
pub struct WebSearchId(pub ::std::string::String);
impl ::std::ops::Deref for WebSearchId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WebSearchId> for ::std::string::String {
    fn from(value: WebSearchId) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::string::String> for WebSearchId {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::fmt::Display for WebSearchId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
impl ::std::str::FromStr for WebSearchId {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
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
impl ::std::fmt::Debug for ConfigAgentEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigAgentEncodedColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncodedColor { .. }")
    }
}
impl ::std::fmt::Debug for ConfigAgentEncodedMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncodedMode { .. }")
    }
}
impl ::std::fmt::Debug for ConfigAgentEncodedModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncodedModel { .. }")
    }
}
impl ::std::fmt::Debug for ConfigAgentEncodedModelObjectModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncodedModelObjectModel { .. }")
    }
}
impl ::std::fmt::Debug for ConfigAgentEncodedModelObjectProviderId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncodedModelObjectProviderId { .. }")
    }
}
impl ::std::fmt::Debug for ConfigAgentEncodedModelObjectVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncodedModelObjectVariant { .. }")
    }
}
impl ::std::fmt::Debug for ConfigAgentEncodedModelString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncodedModelString { .. }")
    }
}
impl ::std::fmt::Debug for ConfigAgentEncodedRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigAgentEncodedRequest { .. }")
    }
}
impl ::std::fmt::Debug for ConfigCommandEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigCommandEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigCommandEncodedModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigCommandEncodedModel { .. }")
    }
}
impl ::std::fmt::Debug for ConfigCommandEncodedModelObjectModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigCommandEncodedModelObjectModel { .. }")
    }
}
impl ::std::fmt::Debug for ConfigCommandEncodedModelObjectProviderId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigCommandEncodedModelObjectProviderId { .. }")
    }
}
impl ::std::fmt::Debug for ConfigCommandEncodedModelObjectVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigCommandEncodedModelObjectVariant { .. }")
    }
}
impl ::std::fmt::Debug for ConfigCommandEncodedModelString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigCommandEncodedModelString { .. }")
    }
}
impl ::std::fmt::Debug for ConfigCompactionPercent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigCompactionPercent { .. }")
    }
}
impl ::std::fmt::Debug for ConfigDirectoryEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigDirectoryEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigDirectoryEncodedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigDirectoryEncodedType { .. }")
    }
}
impl ::std::fmt::Debug for ConfigDocumentEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigDocumentEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigDocumentEncodedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigDocumentEncodedType { .. }")
    }
}
impl ::std::fmt::Debug for ConfigFormatterEntryEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigFormatterEntryEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedCompaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedCompaction { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedCompactionBuffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedCompactionBuffer { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedCompactionKeep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedCompactionKeep { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedEnterprise {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedEnterprise { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedExperimental {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedExperimental { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedExperimentalImageGenerate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedExperimentalImageGenerate { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedExperimentalImageGenerateObjectModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedExperimentalImageGenerateObjectModel { .. }")
    }
}
impl ::std::fmt::Debug
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(
            "ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectModel { .. }",
        )
    }
}
impl ::std::fmt::Debug
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(
            "ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectProviderId { .. }",
        )
    }
}
impl ::std::fmt::Debug
for ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(
            "ConfigInfoEncodedExperimentalImageGenerateObjectModelObjectVariant { .. }",
        )
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedExperimentalImageGenerateObjectModelString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedExperimentalImageGenerateObjectModelString { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedExperimentalLocationIdleTimeout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedExperimentalLocationIdleTimeout { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedExperimentalPoliciesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedExperimentalPoliciesItem { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedExperimentalPoliciesItemAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedExperimentalPoliciesItemAction { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedExperimentalPoliciesItemEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedExperimentalPoliciesItemEffect { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedFormatter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedFormatter { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedLsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedLsp { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedLspObjectValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedLspObjectValue { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedMcp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedMcp { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedMcpServersValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedMcpServersValue { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedMcpTimeout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedMcpTimeout { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedMedia {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedMedia { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedMediaImage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedMediaImage { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedModel { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedModelObjectModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedModelObjectModel { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedModelObjectProviderId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedModelObjectProviderId { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedModelObjectVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedModelObjectVariant { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedModelString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedModelString { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedPluginsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedPluginsItem { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedPoliciesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedPoliciesItem { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedPoliciesItemAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedPoliciesItemAction { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedPoliciesItemEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedPoliciesItemEffect { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedReferencesValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedReferencesValue { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedShare {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedShare { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedToolOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedToolOutput { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedUpdate { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedWarming {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedWarming { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedWatcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedWatcher { .. }")
    }
}
impl ::std::fmt::Debug for ConfigInfoEncodedWebsearch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigInfoEncodedWebsearch { .. }")
    }
}
impl ::std::fmt::Debug for ConfigLspServerEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigLspServerEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelCapabilities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelCapabilities { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelCostEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelCostEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelCostEncodedCache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelCostEncodedCache { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelCostEncodedTier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelCostEncodedTier { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelCostEncodedTierType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelCostEncodedTierType { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelEncodedCost {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelEncodedCost { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelEncodedLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelEncodedLimit { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelEncodedReleaseDate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelEncodedReleaseDate { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelEncodedVariantsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelEncodedVariantsItem { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelReasoning {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelReasoning { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelReasoningSubtype0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelReasoningSubtype0 { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelReasoningSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelReasoningSubtype1 { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelReasoningSubtype1Budget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelReasoningSubtype1Budget { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelSettings { .. }")
    }
}
impl ::std::fmt::Debug for ConfigModelVariantSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigModelVariantSettings { .. }")
    }
}
impl ::std::fmt::Debug for ConfigPluginEntryEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigPluginEntryEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigProviderEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigProviderEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigProviderSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigProviderSettings { .. }")
    }
}
impl ::std::fmt::Debug for ConfigReferenceGitEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigReferenceGitEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigReferenceLocalEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigReferenceLocalEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigResponse { .. }")
    }
}
impl ::std::fmt::Debug for ConfigShellOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigShellOption { .. }")
    }
}
impl ::std::fmt::Debug for ConfigShellsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigShellsResponse { .. }")
    }
}
impl ::std::fmt::Debug for ConfigWarmingEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigWarmingEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigWebSearchInfoEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigWebSearchInfoEncoded { .. }")
    }
}
impl ::std::fmt::Debug for ConfigWorktree {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ConfigWorktree { .. }")
    }
}
impl ::std::fmt::Debug for IntegrationId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("IntegrationId { .. }")
    }
}
impl ::std::fmt::Debug for McpLocalConfigEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpLocalConfigEncoded { .. }")
    }
}
impl ::std::fmt::Debug for McpLocalConfigEncodedTimeout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpLocalConfigEncodedTimeout { .. }")
    }
}
impl ::std::fmt::Debug for McpLocalConfigEncodedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpLocalConfigEncodedType { .. }")
    }
}
impl ::std::fmt::Debug for McpOAuthConfigEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpOAuthConfigEncoded { .. }")
    }
}
impl ::std::fmt::Debug for McpProtocol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpProtocol { .. }")
    }
}
impl ::std::fmt::Debug for McpRemoteConfigEncoded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpRemoteConfigEncoded { .. }")
    }
}
impl ::std::fmt::Debug for McpRemoteConfigEncodedOauth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpRemoteConfigEncodedOauth { .. }")
    }
}
impl ::std::fmt::Debug for McpRemoteConfigEncodedTimeout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpRemoteConfigEncodedTimeout { .. }")
    }
}
impl ::std::fmt::Debug for McpRemoteConfigEncodedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("McpRemoteConfigEncodedType { .. }")
    }
}
impl ::std::fmt::Debug for Project {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("Project { .. }")
    }
}
impl ::std::fmt::Debug for ProjectCommands {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProjectCommands { .. }")
    }
}
impl ::std::fmt::Debug for ProjectIcon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProjectIcon { .. }")
    }
}
impl ::std::fmt::Debug for ProjectListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProjectListResponse { .. }")
    }
}
impl ::std::fmt::Debug for ProjectTime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProjectTime { .. }")
    }
}
impl ::std::fmt::Debug for ProjectVcs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProjectVcs { .. }")
    }
}
impl ::std::fmt::Debug for ProviderCompatibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProviderCompatibility { .. }")
    }
}
impl ::std::fmt::Debug for ProviderInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProviderInfo { .. }")
    }
}
impl ::std::fmt::Debug for ProviderInfoActivation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProviderInfoActivation { .. }")
    }
}
impl ::std::fmt::Debug for ProviderListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProviderListResponse { .. }")
    }
}
impl ::std::fmt::Debug for ProviderResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ProviderResponse { .. }")
    }
}
impl ::std::fmt::Debug for ReferenceGitSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ReferenceGitSource { .. }")
    }
}
impl ::std::fmt::Debug for ReferenceGitSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ReferenceGitSourceType { .. }")
    }
}
impl ::std::fmt::Debug for ReferenceInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ReferenceInfo { .. }")
    }
}
impl ::std::fmt::Debug for ReferenceListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ReferenceListResponse { .. }")
    }
}
impl ::std::fmt::Debug for ReferenceLocalSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ReferenceLocalSource { .. }")
    }
}
impl ::std::fmt::Debug for ReferenceLocalSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ReferenceLocalSourceType { .. }")
    }
}
impl ::std::fmt::Debug for ReferenceSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("ReferenceSource { .. }")
    }
}
impl ::std::fmt::Debug for WebSearchId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("WebSearchId { .. }")
    }
}
