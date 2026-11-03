// Generated from openapi/opencode-v2.json by model-spike --events.
// Do not edit manually; adaptations live in tools/model-spike/src/event_models.rs.
///`EventFormCancelled`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormCancelled {
    pub created: ::serde_json::Number,
    pub data: EventFormCancelledData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventFormCancelledType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormCancelledData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormCancelledData {
    pub id: FormId,
    #[serde(rename = "sessionID")]
    pub session_id: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormCancelledType`
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
pub enum EventFormCancelledType {
    #[serde(rename = "form.cancelled")]
    FormCancelled,
}
impl ::std::fmt::Display for EventFormCancelledType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FormCancelled => f.write_str("form.cancelled"),
        }
    }
}
impl ::std::str::FromStr for EventFormCancelledType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "form.cancelled" => Ok(Self::FormCancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormCancelledType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormCancelledType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormCreated`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormCreated {
    pub created: ::serde_json::Number,
    pub data: EventFormCreatedData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventFormCreatedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormCreatedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormCreatedData {
    pub form: FormInfo1,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormCreatedType`
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
pub enum EventFormCreatedType {
    #[serde(rename = "form.created")]
    FormCreated,
}
impl ::std::fmt::Display for EventFormCreatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FormCreated => f.write_str("form.created"),
        }
    }
}
impl ::std::str::FromStr for EventFormCreatedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "form.created" => Ok(Self::FormCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldBoolean`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormFieldBoolean {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: EventFormFieldBooleanType,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormFieldBooleanType`
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
pub enum EventFormFieldBooleanType {
    #[serde(rename = "boolean")]
    Boolean,
}
impl ::std::fmt::Display for EventFormFieldBooleanType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Boolean => f.write_str("boolean"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldBooleanType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "boolean" => Ok(Self::Boolean),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldBooleanType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormFieldBooleanType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldExternal`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormFieldExternal {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub key: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: EventFormFieldExternalType,
    pub url: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormFieldExternalType`
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
pub enum EventFormFieldExternalType {
    #[serde(rename = "external")]
    External,
}
impl ::std::fmt::Display for EventFormFieldExternalType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::External => f.write_str("external"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldExternalType {
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
impl ::std::convert::TryFrom<&str> for EventFormFieldExternalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormFieldExternalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldInteger`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormFieldInteger {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<EventFormFieldIntegerDefault>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<EventFormFieldIntegerMaximum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<EventFormFieldIntegerMinimum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: EventFormFieldIntegerType,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormFieldIntegerDefault`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventFormFieldIntegerDefault {
    Number(::serde_json::Number),
    String(EventFormFieldIntegerDefaultString),
}
impl ::std::convert::From<::serde_json::Number> for EventFormFieldIntegerDefault {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<EventFormFieldIntegerDefaultString>
for EventFormFieldIntegerDefault {
    fn from(value: EventFormFieldIntegerDefaultString) -> Self {
        Self::String(value)
    }
}
///`EventFormFieldIntegerDefaultString`
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
pub enum EventFormFieldIntegerDefaultString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for EventFormFieldIntegerDefaultString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldIntegerDefaultString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldIntegerDefaultString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventFormFieldIntegerDefaultString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldIntegerMaximum`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventFormFieldIntegerMaximum {
    Number(::serde_json::Number),
    String(EventFormFieldIntegerMaximumString),
}
impl ::std::convert::From<::serde_json::Number> for EventFormFieldIntegerMaximum {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<EventFormFieldIntegerMaximumString>
for EventFormFieldIntegerMaximum {
    fn from(value: EventFormFieldIntegerMaximumString) -> Self {
        Self::String(value)
    }
}
///`EventFormFieldIntegerMaximumString`
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
pub enum EventFormFieldIntegerMaximumString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for EventFormFieldIntegerMaximumString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldIntegerMaximumString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldIntegerMaximumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventFormFieldIntegerMaximumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldIntegerMinimum`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventFormFieldIntegerMinimum {
    Number(::serde_json::Number),
    String(EventFormFieldIntegerMinimumString),
}
impl ::std::convert::From<::serde_json::Number> for EventFormFieldIntegerMinimum {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<EventFormFieldIntegerMinimumString>
for EventFormFieldIntegerMinimum {
    fn from(value: EventFormFieldIntegerMinimumString) -> Self {
        Self::String(value)
    }
}
///`EventFormFieldIntegerMinimumString`
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
pub enum EventFormFieldIntegerMinimumString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for EventFormFieldIntegerMinimumString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldIntegerMinimumString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldIntegerMinimumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventFormFieldIntegerMinimumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldIntegerType`
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
pub enum EventFormFieldIntegerType {
    #[serde(rename = "integer")]
    Integer,
}
impl ::std::fmt::Display for EventFormFieldIntegerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Integer => f.write_str("integer"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldIntegerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "integer" => Ok(Self::Integer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldIntegerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormFieldIntegerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldMultiselect`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormFieldMultiselect {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub default: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(rename = "maxItems", skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub max_items: ::std::option::Option<super::EventInteger<0, 9007199254740991>>,
    #[serde(rename = "minItems", skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub min_items: ::std::option::Option<super::EventInteger<0, 9007199254740991>>,
    pub options: ::std::vec::Vec<FormOption>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: EventFormFieldMultiselectType,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormFieldMultiselectType`
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
pub enum EventFormFieldMultiselectType {
    #[serde(rename = "multiselect")]
    Multiselect,
}
impl ::std::fmt::Display for EventFormFieldMultiselectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Multiselect => f.write_str("multiselect"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldMultiselectType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "multiselect" => Ok(Self::Multiselect),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldMultiselectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormFieldMultiselectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldNumber`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormFieldNumber {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<EventFormFieldNumberDefault>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<EventFormFieldNumberMaximum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<EventFormFieldNumberMinimum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: EventFormFieldNumberType,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormFieldNumberDefault`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventFormFieldNumberDefault {
    Number(::serde_json::Number),
    String(EventFormFieldNumberDefaultString),
}
impl ::std::convert::From<::serde_json::Number> for EventFormFieldNumberDefault {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<EventFormFieldNumberDefaultString>
for EventFormFieldNumberDefault {
    fn from(value: EventFormFieldNumberDefaultString) -> Self {
        Self::String(value)
    }
}
///`EventFormFieldNumberDefaultString`
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
pub enum EventFormFieldNumberDefaultString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for EventFormFieldNumberDefaultString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldNumberDefaultString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldNumberDefaultString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventFormFieldNumberDefaultString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldNumberMaximum`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventFormFieldNumberMaximum {
    Number(::serde_json::Number),
    String(EventFormFieldNumberMaximumString),
}
impl ::std::convert::From<::serde_json::Number> for EventFormFieldNumberMaximum {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<EventFormFieldNumberMaximumString>
for EventFormFieldNumberMaximum {
    fn from(value: EventFormFieldNumberMaximumString) -> Self {
        Self::String(value)
    }
}
///`EventFormFieldNumberMaximumString`
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
pub enum EventFormFieldNumberMaximumString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for EventFormFieldNumberMaximumString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldNumberMaximumString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldNumberMaximumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventFormFieldNumberMaximumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldNumberMinimum`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventFormFieldNumberMinimum {
    Number(::serde_json::Number),
    String(EventFormFieldNumberMinimumString),
}
impl ::std::convert::From<::serde_json::Number> for EventFormFieldNumberMinimum {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<EventFormFieldNumberMinimumString>
for EventFormFieldNumberMinimum {
    fn from(value: EventFormFieldNumberMinimumString) -> Self {
        Self::String(value)
    }
}
///`EventFormFieldNumberMinimumString`
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
pub enum EventFormFieldNumberMinimumString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for EventFormFieldNumberMinimumString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldNumberMinimumString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldNumberMinimumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventFormFieldNumberMinimumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldNumberType`
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
pub enum EventFormFieldNumberType {
    #[serde(rename = "number")]
    Number,
}
impl ::std::fmt::Display for EventFormFieldNumberType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Number => f.write_str("number"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldNumberType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "number" => Ok(Self::Number),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldNumberType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormFieldNumberType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldString`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormFieldString {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<EventFormFieldStringFormat>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(
        rename = "maxLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub max_length: ::std::option::Option<super::EventInteger<0, 9007199254740991>>,
    #[serde(
        rename = "minLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub min_length: ::std::option::Option<super::EventInteger<0, 9007199254740991>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub options: ::std::vec::Vec<FormOption>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pattern: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub placeholder: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: EventFormFieldStringType,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormFieldStringFormat`
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
pub enum EventFormFieldStringFormat {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date-time")]
    DateTime,
}
impl ::std::fmt::Display for EventFormFieldStringFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Email => f.write_str("email"),
            Self::Uri => f.write_str("uri"),
            Self::Date => f.write_str("date"),
            Self::DateTime => f.write_str("date-time"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldStringFormat {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "email" => Ok(Self::Email),
            "uri" => Ok(Self::Uri),
            "date" => Ok(Self::Date),
            "date-time" => Ok(Self::DateTime),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldStringFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormFieldStringFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormFieldStringType`
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
pub enum EventFormFieldStringType {
    #[serde(rename = "string")]
    String,
}
impl ::std::fmt::Display for EventFormFieldStringType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::String => f.write_str("string"),
        }
    }
}
impl ::std::str::FromStr for EventFormFieldStringType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "string" => Ok(Self::String),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormFieldStringType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormFieldStringType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventFormReplied`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormReplied {
    pub created: ::serde_json::Number,
    pub data: EventFormRepliedData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventFormRepliedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormRepliedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventFormRepliedData {
    pub answer: FormAnswer1,
    pub id: FormId,
    #[serde(rename = "sessionID")]
    pub session_id: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventFormRepliedType`
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
pub enum EventFormRepliedType {
    #[serde(rename = "form.replied")]
    FormReplied,
}
impl ::std::fmt::Display for EventFormRepliedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FormReplied => f.write_str("form.replied"),
        }
    }
}
impl ::std::str::FromStr for EventFormRepliedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "form.replied" => Ok(Self::FormReplied),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventFormRepliedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventFormRepliedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventId`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventId(::std::string::String);
impl ::std::ops::Deref for EventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventId> for ::std::string::String {
    fn from(value: EventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^evt_").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^evt_\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventId {
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
///`EventInboxItemCompaction`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventInboxItemCompaction {
    pub delivery: super::SessionInboxDelivery,
    pub payload: SessionInboxCompactionPayload,
    #[serde(rename = "type")]
    pub type_: EventInboxItemCompactionType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventInboxItemCompactionType`
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
pub enum EventInboxItemCompactionType {
    #[serde(rename = "compaction")]
    Compaction,
}
impl ::std::fmt::Display for EventInboxItemCompactionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Compaction => f.write_str("compaction"),
        }
    }
}
impl ::std::str::FromStr for EventInboxItemCompactionType {
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
impl ::std::convert::TryFrom<&str> for EventInboxItemCompactionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventInboxItemCompactionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventInboxItemMove`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventInboxItemMove {
    pub delivery: super::SessionInboxDelivery,
    pub payload: SessionInboxMovePayload1,
    #[serde(rename = "type")]
    pub type_: EventInboxItemMoveType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventInboxItemMoveType`
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
pub enum EventInboxItemMoveType {
    #[serde(rename = "move")]
    Move,
}
impl ::std::fmt::Display for EventInboxItemMoveType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Move => f.write_str("move"),
        }
    }
}
impl ::std::str::FromStr for EventInboxItemMoveType {
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
impl ::std::convert::TryFrom<&str> for EventInboxItemMoveType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventInboxItemMoveType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventInboxItemSynthetic`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventInboxItemSynthetic {
    pub delivery: super::SessionInboxDelivery,
    pub payload: super::SessionInboxSyntheticPayload,
    #[serde(rename = "type")]
    pub type_: EventInboxItemSyntheticType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventInboxItemSyntheticType`
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
pub enum EventInboxItemSyntheticType {
    #[serde(rename = "synthetic")]
    Synthetic,
}
impl ::std::fmt::Display for EventInboxItemSyntheticType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Synthetic => f.write_str("synthetic"),
        }
    }
}
impl ::std::str::FromStr for EventInboxItemSyntheticType {
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
impl ::std::convert::TryFrom<&str> for EventInboxItemSyntheticType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventInboxItemSyntheticType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventInboxItemUser`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventInboxItemUser {
    pub delivery: super::SessionInboxDelivery,
    pub payload: super::SessionInboxUserPayload,
    #[serde(rename = "type")]
    pub type_: EventInboxItemUserType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventInboxItemUserType`
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
pub enum EventInboxItemUserType {
    #[serde(rename = "user")]
    User,
}
impl ::std::fmt::Display for EventInboxItemUserType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
        }
    }
}
impl ::std::str::FromStr for EventInboxItemUserType {
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
impl ::std::convert::TryFrom<&str> for EventInboxItemUserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventInboxItemUserType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventPermissionAsked`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventPermissionAsked {
    pub created: ::serde_json::Number,
    pub data: EventPermissionAskedData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventPermissionAskedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventPermissionAskedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventPermissionAskedData {
    pub action: ::std::string::String,
    pub id: PermissionId,
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
    pub source: ::std::option::Option<PermissionSource>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventPermissionAskedType`
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
pub enum EventPermissionAskedType {
    #[serde(rename = "permission.asked")]
    PermissionAsked,
}
impl ::std::fmt::Display for EventPermissionAskedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PermissionAsked => f.write_str("permission.asked"),
        }
    }
}
impl ::std::str::FromStr for EventPermissionAskedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "permission.asked" => Ok(Self::PermissionAsked),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventPermissionAskedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventPermissionAskedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventPermissionReplied`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventPermissionReplied {
    pub created: ::serde_json::Number,
    pub data: EventPermissionRepliedData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventPermissionRepliedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventPermissionRepliedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventPermissionRepliedData {
    pub reply: PermissionReply,
    #[serde(rename = "requestID")]
    pub request_id: PermissionId,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventPermissionRepliedType`
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
pub enum EventPermissionRepliedType {
    #[serde(rename = "permission.replied")]
    PermissionReplied,
}
impl ::std::fmt::Display for EventPermissionRepliedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PermissionReplied => f.write_str("permission.replied"),
        }
    }
}
impl ::std::str::FromStr for EventPermissionRepliedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "permission.replied" => Ok(Self::PermissionReplied),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventPermissionRepliedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventPermissionRepliedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventRpc`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventRpc {
    pub created: ::serde_json::Number,
    pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub id: EventId,
    pub location: super::LocationPublicRef,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(rename = "type")]
    pub type_: EventRpcType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventRpcType`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventRpcType(::std::string::String);
impl ::std::ops::Deref for EventRpcType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventRpcType> for ::std::string::String {
    fn from(value: EventRpcType) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventRpcType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^rpc\\.[\\s\\S]*?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^rpc\\.[\\s\\S]*?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventRpcType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventRpcType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventRpcType {
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
///`EventSeq`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EventSeq(pub super::EventInteger<0, 9007199254740991>);
impl ::std::ops::Deref for EventSeq {
    type Target = super::EventInteger<0, 9007199254740991>;
    fn deref(&self) -> &super::EventInteger<0, 9007199254740991> {
        &self.0
    }
}
impl ::std::convert::From<EventSeq> for super::EventInteger<0, 9007199254740991> {
    fn from(value: EventSeq) -> Self {
        value.0
    }
}
impl ::std::convert::From<super::EventInteger<0, 9007199254740991>> for EventSeq {
    fn from(value: super::EventInteger<0, 9007199254740991>) -> Self {
        Self(value)
    }
}
///`EventServerConnected`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventServerConnected {
    pub data: super::NonNullValue,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<super::LocationPublicRef>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    #[serde(rename = "type")]
    pub type_: EventServerConnectedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventServerConnectedType`
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
pub enum EventServerConnectedType {
    #[serde(rename = "server.connected")]
    ServerConnected,
}
impl ::std::fmt::Display for EventServerConnectedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ServerConnected => f.write_str("server.connected"),
        }
    }
}
impl ::std::str::FromStr for EventServerConnectedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "server.connected" => Ok(Self::ServerConnected),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventServerConnectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventServerConnectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionAgentSelected`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionAgentSelected {
    pub created: ::serde_json::Number,
    pub data: EventSessionAgentSelectedData,
    pub durable: EventSessionAgentSelectedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionAgentSelectedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionAgentSelectedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionAgentSelectedData {
    pub agent: super::AgentId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous: ::std::option::Option<super::AgentId>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionAgentSelectedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionAgentSelectedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionAgentSelectedType`
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
pub enum EventSessionAgentSelectedType {
    #[serde(rename = "session.agent.selected")]
    SessionAgentSelected,
}
impl ::std::fmt::Display for EventSessionAgentSelectedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionAgentSelected => f.write_str("session.agent.selected"),
        }
    }
}
impl ::std::str::FromStr for EventSessionAgentSelectedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.agent.selected" => Ok(Self::SessionAgentSelected),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionAgentSelectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionAgentSelectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionArchived`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionArchived {
    pub created: ::serde_json::Number,
    pub data: EventSessionArchivedData,
    pub durable: EventSessionArchivedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionArchivedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionArchivedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionArchivedData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionArchivedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionArchivedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionArchivedType`
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
pub enum EventSessionArchivedType {
    #[serde(rename = "session.archived")]
    SessionArchived,
}
impl ::std::fmt::Display for EventSessionArchivedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionArchived => f.write_str("session.archived"),
        }
    }
}
impl ::std::str::FromStr for EventSessionArchivedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.archived" => Ok(Self::SessionArchived),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionArchivedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionArchivedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionCompactionDelta`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionDelta {
    pub created: ::serde_json::Number,
    pub data: EventSessionCompactionDeltaData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionCompactionDeltaType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionDeltaData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionDeltaData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub text: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionDeltaType`
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
pub enum EventSessionCompactionDeltaType {
    #[serde(rename = "session.compaction.delta")]
    SessionCompactionDelta,
}
impl ::std::fmt::Display for EventSessionCompactionDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCompactionDelta => f.write_str("session.compaction.delta"),
        }
    }
}
impl ::std::str::FromStr for EventSessionCompactionDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.compaction.delta" => Ok(Self::SessionCompactionDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionCompactionDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionCompactionDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionCompactionEnded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionEnded {
    pub created: ::serde_json::Number,
    pub data: EventSessionCompactionEndedData,
    pub durable: EventSessionCompactionEndedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionCompactionEndedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionEndedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionEndedData {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cost: ::std::option::Option<super::MoneyUsd>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<super::ModelRef>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    pub reason: EventSessionCompactionEndedDataReason,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub text: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokens: ::std::option::Option<super::TokenUsageInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionEndedDataReason`
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
pub enum EventSessionCompactionEndedDataReason {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}
impl ::std::fmt::Display for EventSessionCompactionEndedDataReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Manual => f.write_str("manual"),
        }
    }
}
impl ::std::str::FromStr for EventSessionCompactionEndedDataReason {
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
impl ::std::convert::TryFrom<&str> for EventSessionCompactionEndedDataReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionCompactionEndedDataReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionCompactionEndedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionEndedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionEndedType`
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
pub enum EventSessionCompactionEndedType {
    #[serde(rename = "session.compaction.ended")]
    SessionCompactionEnded,
}
impl ::std::fmt::Display for EventSessionCompactionEndedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCompactionEnded => f.write_str("session.compaction.ended"),
        }
    }
}
impl ::std::str::FromStr for EventSessionCompactionEndedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.compaction.ended" => Ok(Self::SessionCompactionEnded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionCompactionEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionCompactionEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionCompactionFailed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionFailed {
    pub created: ::serde_json::Number,
    pub data: EventSessionCompactionFailedData,
    pub durable: EventSessionCompactionFailedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionCompactionFailedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionFailedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionFailedData {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cost: ::std::option::Option<super::MoneyUsd>,
    pub error: super::SessionStructuredError,
    #[serde(rename = "inputID", skip_serializing_if = "::std::option::Option::is_none")]
    pub input_id: ::std::option::Option<super::SessionMessageId>,
    pub reason: EventSessionCompactionFailedDataReason,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokens: ::std::option::Option<super::TokenUsageInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionFailedDataReason`
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
pub enum EventSessionCompactionFailedDataReason {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}
impl ::std::fmt::Display for EventSessionCompactionFailedDataReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Manual => f.write_str("manual"),
        }
    }
}
impl ::std::str::FromStr for EventSessionCompactionFailedDataReason {
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
impl ::std::convert::TryFrom<&str> for EventSessionCompactionFailedDataReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionCompactionFailedDataReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionCompactionFailedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionFailedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionFailedType`
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
pub enum EventSessionCompactionFailedType {
    #[serde(rename = "session.compaction.failed")]
    SessionCompactionFailed,
}
impl ::std::fmt::Display for EventSessionCompactionFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCompactionFailed => f.write_str("session.compaction.failed"),
        }
    }
}
impl ::std::str::FromStr for EventSessionCompactionFailedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.compaction.failed" => Ok(Self::SessionCompactionFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionCompactionFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionCompactionFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionCompactionStarted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionStarted {
    pub created: ::serde_json::Number,
    pub data: EventSessionCompactionStartedData,
    pub durable: EventSessionCompactionStartedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionCompactionStartedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionStartedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionStartedData {
    #[serde(rename = "inputID", skip_serializing_if = "::std::option::Option::is_none")]
    pub input_id: ::std::option::Option<super::SessionMessageId>,
    pub reason: EventSessionCompactionStartedDataReason,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionStartedDataReason`
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
pub enum EventSessionCompactionStartedDataReason {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}
impl ::std::fmt::Display for EventSessionCompactionStartedDataReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Manual => f.write_str("manual"),
        }
    }
}
impl ::std::str::FromStr for EventSessionCompactionStartedDataReason {
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
impl ::std::convert::TryFrom<&str> for EventSessionCompactionStartedDataReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionCompactionStartedDataReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionCompactionStartedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCompactionStartedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCompactionStartedType`
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
pub enum EventSessionCompactionStartedType {
    #[serde(rename = "session.compaction.started")]
    SessionCompactionStarted,
}
impl ::std::fmt::Display for EventSessionCompactionStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCompactionStarted => f.write_str("session.compaction.started"),
        }
    }
}
impl ::std::str::FromStr for EventSessionCompactionStartedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.compaction.started" => Ok(Self::SessionCompactionStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionCompactionStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionCompactionStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionCreated`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCreated {
    pub created: ::serde_json::Number,
    pub data: EventSessionCreatedData,
    pub durable: EventSessionCreatedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionCreatedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCreatedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCreatedData {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub agent: ::std::option::Option<super::AgentId>,
    pub location: LocationRef,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<super::SessionMetadata>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub model: ::std::option::Option<super::ModelRef>,
    #[serde(rename = "parentID", skip_serializing_if = "::std::option::Option::is_none")]
    pub parent_id: ::std::option::Option<super::SessionId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub permissions: ::std::option::Option<super::PermissionRuleset>,
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub slug: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subpath: ::std::option::Option<super::RelativePath>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    pub version: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCreatedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionCreatedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionCreatedType`
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
pub enum EventSessionCreatedType {
    #[serde(rename = "session.created")]
    SessionCreated,
}
impl ::std::fmt::Display for EventSessionCreatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionCreated => f.write_str("session.created"),
        }
    }
}
impl ::std::str::FromStr for EventSessionCreatedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.created" => Ok(Self::SessionCreated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionCreatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionDeleted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionDeleted {
    pub created: ::serde_json::Number,
    pub data: EventSessionDeletedData,
    pub durable: EventSessionDeletedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionDeletedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionDeletedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionDeletedData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionDeletedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionDeletedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC3ea984aceae,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionDeletedType`
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
pub enum EventSessionDeletedType {
    #[serde(rename = "session.deleted")]
    SessionDeleted,
}
impl ::std::fmt::Display for EventSessionDeletedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionDeleted => f.write_str("session.deleted"),
        }
    }
}
impl ::std::str::FromStr for EventSessionDeletedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.deleted" => Ok(Self::SessionDeleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionDeletedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionDeletedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionExecutionFailed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionFailed {
    pub created: ::serde_json::Number,
    pub data: EventSessionExecutionFailedData,
    pub durable: EventSessionExecutionFailedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionExecutionFailedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionFailedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionFailedData {
    pub error: super::SessionStructuredError,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionFailedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionFailedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionFailedType`
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
pub enum EventSessionExecutionFailedType {
    #[serde(rename = "session.execution.failed")]
    SessionExecutionFailed,
}
impl ::std::fmt::Display for EventSessionExecutionFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionExecutionFailed => f.write_str("session.execution.failed"),
        }
    }
}
impl ::std::str::FromStr for EventSessionExecutionFailedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.execution.failed" => Ok(Self::SessionExecutionFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionExecutionFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionExecutionFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionExecutionInterrupted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionInterrupted {
    pub created: ::serde_json::Number,
    pub data: EventSessionExecutionInterruptedData,
    pub durable: EventSessionExecutionInterruptedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionExecutionInterruptedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionInterruptedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionInterruptedData {
    pub reason: EventSessionExecutionInterruptedDataReason,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionInterruptedDataReason`
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
pub enum EventSessionExecutionInterruptedDataReason {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "superseded")]
    Superseded,
    #[serde(rename = "inactivity")]
    Inactivity,
}
impl ::std::fmt::Display for EventSessionExecutionInterruptedDataReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => f.write_str("user"),
            Self::Shutdown => f.write_str("shutdown"),
            Self::Superseded => f.write_str("superseded"),
            Self::Inactivity => f.write_str("inactivity"),
        }
    }
}
impl ::std::str::FromStr for EventSessionExecutionInterruptedDataReason {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "user" => Ok(Self::User),
            "shutdown" => Ok(Self::Shutdown),
            "superseded" => Ok(Self::Superseded),
            "inactivity" => Ok(Self::Inactivity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionExecutionInterruptedDataReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionExecutionInterruptedDataReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionExecutionInterruptedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionInterruptedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionInterruptedType`
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
pub enum EventSessionExecutionInterruptedType {
    #[serde(rename = "session.execution.interrupted")]
    SessionExecutionInterrupted,
}
impl ::std::fmt::Display for EventSessionExecutionInterruptedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionExecutionInterrupted => {
                f.write_str("session.execution.interrupted")
            }
        }
    }
}
impl ::std::str::FromStr for EventSessionExecutionInterruptedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.execution.interrupted" => Ok(Self::SessionExecutionInterrupted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionExecutionInterruptedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionExecutionInterruptedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionExecutionStarted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionStarted {
    pub created: ::serde_json::Number,
    pub data: EventSessionExecutionStartedData,
    pub durable: EventSessionExecutionStartedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionExecutionStartedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionStartedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionStartedData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionStartedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionStartedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionStartedType`
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
pub enum EventSessionExecutionStartedType {
    #[serde(rename = "session.execution.started")]
    SessionExecutionStarted,
}
impl ::std::fmt::Display for EventSessionExecutionStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionExecutionStarted => f.write_str("session.execution.started"),
        }
    }
}
impl ::std::str::FromStr for EventSessionExecutionStartedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.execution.started" => Ok(Self::SessionExecutionStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionExecutionStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionExecutionStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionExecutionSucceeded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionSucceeded {
    pub created: ::serde_json::Number,
    pub data: EventSessionExecutionSucceededData,
    pub durable: EventSessionExecutionSucceededDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionExecutionSucceededType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionSucceededData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionSucceededData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionSucceededDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionExecutionSucceededDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionExecutionSucceededType`
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
pub enum EventSessionExecutionSucceededType {
    #[serde(rename = "session.execution.succeeded")]
    SessionExecutionSucceeded,
}
impl ::std::fmt::Display for EventSessionExecutionSucceededType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionExecutionSucceeded => f.write_str("session.execution.succeeded"),
        }
    }
}
impl ::std::str::FromStr for EventSessionExecutionSucceededType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.execution.succeeded" => Ok(Self::SessionExecutionSucceeded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionExecutionSucceededType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionExecutionSucceededType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionForked`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionForked {
    pub created: ::serde_json::Number,
    pub data: EventSessionForkedData,
    pub durable: EventSessionForkedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionForkedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionForkedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionForkedData {
    pub boundary: super::SessionForkBoundary,
    #[serde(
        rename = "instructionEntries",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub instruction_entries: ::std::option::Option<InstructionEntrySnapshot>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<EventSessionForkedDataInstructions>,
    #[serde(rename = "parentID")]
    pub parent_id: super::SessionId,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionForkedDataInstructions`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionForkedDataInstructions {
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionForkedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionForkedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC3ea984aceae,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionForkedType`
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
pub enum EventSessionForkedType {
    #[serde(rename = "session.forked")]
    SessionForked,
}
impl ::std::fmt::Display for EventSessionForkedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionForked => f.write_str("session.forked"),
        }
    }
}
impl ::std::str::FromStr for EventSessionForkedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.forked" => Ok(Self::SessionForked),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionForkedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionForkedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionIdle`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionIdle {
    pub created: ::serde_json::Number,
    pub data: EventSessionIdleData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionIdleType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionIdleData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionIdleData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionIdleType`
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
pub enum EventSessionIdleType {
    #[serde(rename = "session.idle")]
    SessionIdle,
}
impl ::std::fmt::Display for EventSessionIdleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionIdle => f.write_str("session.idle"),
        }
    }
}
impl ::std::str::FromStr for EventSessionIdleType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.idle" => Ok(Self::SessionIdle),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionIdleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionIdleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionInboxCancelled`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxCancelled {
    pub created: ::serde_json::Number,
    pub data: EventSessionInboxCancelledData,
    pub durable: EventSessionInboxCancelledDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionInboxCancelledType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxCancelledData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxCancelledData {
    #[serde(rename = "inboxID")]
    pub inbox_id: super::SessionMessageId,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxCancelledDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxCancelledDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxCancelledType`
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
pub enum EventSessionInboxCancelledType {
    #[serde(rename = "session.inbox.cancelled")]
    SessionInboxCancelled,
}
impl ::std::fmt::Display for EventSessionInboxCancelledType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInboxCancelled => f.write_str("session.inbox.cancelled"),
        }
    }
}
impl ::std::str::FromStr for EventSessionInboxCancelledType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.inbox.cancelled" => Ok(Self::SessionInboxCancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionInboxCancelledType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionInboxCancelledType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionInboxDelivered`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxDelivered {
    pub created: ::serde_json::Number,
    pub data: EventSessionInboxDeliveredData,
    pub durable: EventSessionInboxDeliveredDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionInboxDeliveredType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxDeliveredData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxDeliveredData {
    #[serde(rename = "inboxID")]
    pub inbox_id: super::SessionMessageId,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxDeliveredDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxDeliveredDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxDeliveredType`
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
pub enum EventSessionInboxDeliveredType {
    #[serde(rename = "session.inbox.delivered")]
    SessionInboxDelivered,
}
impl ::std::fmt::Display for EventSessionInboxDeliveredType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInboxDelivered => f.write_str("session.inbox.delivered"),
        }
    }
}
impl ::std::str::FromStr for EventSessionInboxDeliveredType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.inbox.delivered" => Ok(Self::SessionInboxDelivered),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionInboxDeliveredType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionInboxDeliveredType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionInboxDeliveryChanged`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxDeliveryChanged {
    pub created: ::serde_json::Number,
    pub data: EventSessionInboxDeliveryChangedData,
    pub durable: EventSessionInboxDeliveryChangedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionInboxDeliveryChangedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxDeliveryChangedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxDeliveryChangedData {
    pub delivery: super::SessionInboxDelivery,
    #[serde(rename = "inboxID")]
    pub inbox_id: super::SessionMessageId,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxDeliveryChangedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxDeliveryChangedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxDeliveryChangedType`
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
pub enum EventSessionInboxDeliveryChangedType {
    #[serde(rename = "session.inbox.delivery.changed")]
    SessionInboxDeliveryChanged,
}
impl ::std::fmt::Display for EventSessionInboxDeliveryChangedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInboxDeliveryChanged => {
                f.write_str("session.inbox.delivery.changed")
            }
        }
    }
}
impl ::std::str::FromStr for EventSessionInboxDeliveryChangedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.inbox.delivery.changed" => Ok(Self::SessionInboxDeliveryChanged),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionInboxDeliveryChangedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionInboxDeliveryChangedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionInboxEnqueued`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxEnqueued {
    pub created: ::serde_json::Number,
    pub data: EventSessionInboxEnqueuedData,
    pub durable: EventSessionInboxEnqueuedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionInboxEnqueuedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxEnqueuedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxEnqueuedData {
    #[serde(rename = "inboxID")]
    pub inbox_id: super::SessionMessageId,
    pub item: EventInboxItem,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxEnqueuedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionInboxEnqueuedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionInboxEnqueuedType`
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
pub enum EventSessionInboxEnqueuedType {
    #[serde(rename = "session.inbox.enqueued")]
    SessionInboxEnqueued,
}
impl ::std::fmt::Display for EventSessionInboxEnqueuedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionInboxEnqueued => f.write_str("session.inbox.enqueued"),
        }
    }
}
impl ::std::str::FromStr for EventSessionInboxEnqueuedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.inbox.enqueued" => Ok(Self::SessionInboxEnqueued),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionInboxEnqueuedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionInboxEnqueuedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionMetadataUpdated`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionMetadataUpdated {
    pub created: ::serde_json::Number,
    pub data: EventSessionMetadataUpdatedData,
    pub durable: EventSessionMetadataUpdatedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionMetadataUpdatedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionMetadataUpdatedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionMetadataUpdatedData {
    pub metadata: super::SessionMetadata,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionMetadataUpdatedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionMetadataUpdatedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionMetadataUpdatedType`
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
pub enum EventSessionMetadataUpdatedType {
    #[serde(rename = "session.metadata.updated")]
    SessionMetadataUpdated,
}
impl ::std::fmt::Display for EventSessionMetadataUpdatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionMetadataUpdated => f.write_str("session.metadata.updated"),
        }
    }
}
impl ::std::str::FromStr for EventSessionMetadataUpdatedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.metadata.updated" => Ok(Self::SessionMetadataUpdated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionMetadataUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionMetadataUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionModelSelected`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionModelSelected {
    pub created: ::serde_json::Number,
    pub data: EventSessionModelSelectedData,
    pub durable: EventSessionModelSelectedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionModelSelectedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionModelSelectedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionModelSelectedData {
    pub model: super::ModelRef,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub previous: ::std::option::Option<super::ModelRef>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionModelSelectedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionModelSelectedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionModelSelectedType`
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
pub enum EventSessionModelSelectedType {
    #[serde(rename = "session.model.selected")]
    SessionModelSelected,
}
impl ::std::fmt::Display for EventSessionModelSelectedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionModelSelected => f.write_str("session.model.selected"),
        }
    }
}
impl ::std::str::FromStr for EventSessionModelSelectedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.model.selected" => Ok(Self::SessionModelSelected),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionModelSelectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionModelSelectedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionMoved`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionMoved {
    pub created: ::serde_json::Number,
    pub data: EventSessionMovedData,
    pub durable: EventSessionMovedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionMovedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionMovedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionMovedData {
    pub location: LocationRef,
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subpath: ::std::option::Option<super::RelativePath>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionMovedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionMovedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionMovedType`
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
pub enum EventSessionMovedType {
    #[serde(rename = "session.moved")]
    SessionMoved,
}
impl ::std::fmt::Display for EventSessionMovedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionMoved => f.write_str("session.moved"),
        }
    }
}
impl ::std::str::FromStr for EventSessionMovedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.moved" => Ok(Self::SessionMoved),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionMovedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionMovedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionPermissions`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionPermissions {
    pub created: ::serde_json::Number,
    pub data: EventSessionPermissionsData,
    pub durable: EventSessionPermissionsDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionPermissionsType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionPermissionsData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionPermissionsData {
    pub permissions: super::PermissionRuleset,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionPermissionsDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionPermissionsDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionPermissionsType`
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
pub enum EventSessionPermissionsType {
    #[serde(rename = "session.permissions")]
    SessionPermissions,
}
impl ::std::fmt::Display for EventSessionPermissionsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionPermissions => f.write_str("session.permissions"),
        }
    }
}
impl ::std::str::FromStr for EventSessionPermissionsType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.permissions" => Ok(Self::SessionPermissions),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionPermissionsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionPermissionsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionReasoningDelta`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionReasoningDelta {
    pub created: ::serde_json::Number,
    pub data: EventSessionReasoningDeltaData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionReasoningDeltaType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionReasoningDeltaData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionReasoningDeltaData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub delta: ::std::string::String,
    pub ordinal: super::EventInteger<0, 9007199254740991>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionReasoningDeltaType`
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
pub enum EventSessionReasoningDeltaType {
    #[serde(rename = "session.reasoning.delta")]
    SessionReasoningDelta,
}
impl ::std::fmt::Display for EventSessionReasoningDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionReasoningDelta => f.write_str("session.reasoning.delta"),
        }
    }
}
impl ::std::str::FromStr for EventSessionReasoningDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.reasoning.delta" => Ok(Self::SessionReasoningDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionReasoningDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionReasoningDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionReasoningEnded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionReasoningEnded {
    pub created: ::serde_json::Number,
    pub data: EventSessionReasoningEndedData,
    pub durable: EventSessionReasoningEndedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionReasoningEndedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionReasoningEndedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionReasoningEndedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    pub ordinal: super::EventInteger<0, 9007199254740991>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub text: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionReasoningEndedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionReasoningEndedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionReasoningEndedType`
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
pub enum EventSessionReasoningEndedType {
    #[serde(rename = "session.reasoning.ended")]
    SessionReasoningEnded,
}
impl ::std::fmt::Display for EventSessionReasoningEndedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionReasoningEnded => f.write_str("session.reasoning.ended"),
        }
    }
}
impl ::std::str::FromStr for EventSessionReasoningEndedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.reasoning.ended" => Ok(Self::SessionReasoningEnded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionReasoningEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionReasoningEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionReasoningStarted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionReasoningStarted {
    pub created: ::serde_json::Number,
    pub data: EventSessionReasoningStartedData,
    pub durable: EventSessionReasoningStartedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionReasoningStartedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionReasoningStartedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionReasoningStartedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    pub ordinal: super::EventInteger<0, 9007199254740991>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionReasoningStartedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionReasoningStartedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionReasoningStartedType`
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
pub enum EventSessionReasoningStartedType {
    #[serde(rename = "session.reasoning.started")]
    SessionReasoningStarted,
}
impl ::std::fmt::Display for EventSessionReasoningStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionReasoningStarted => f.write_str("session.reasoning.started"),
        }
    }
}
impl ::std::str::FromStr for EventSessionReasoningStartedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.reasoning.started" => Ok(Self::SessionReasoningStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionReasoningStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionReasoningStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionRenamed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionRenamed {
    pub created: ::serde_json::Number,
    pub data: EventSessionRenamedData,
    pub durable: EventSessionRenamedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionRenamedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionRenamedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionRenamedData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub title: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionRenamedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionRenamedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionRenamedType`
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
pub enum EventSessionRenamedType {
    #[serde(rename = "session.renamed")]
    SessionRenamed,
}
impl ::std::fmt::Display for EventSessionRenamedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionRenamed => f.write_str("session.renamed"),
        }
    }
}
impl ::std::str::FromStr for EventSessionRenamedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.renamed" => Ok(Self::SessionRenamed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionRenamedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionRenamedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionRetryScheduled`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionRetryScheduled {
    pub created: ::serde_json::Number,
    pub data: EventSessionRetryScheduledData,
    pub durable: EventSessionRetryScheduledDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionRetryScheduledType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionRetryScheduledData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionRetryScheduledData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub at: super::EventInteger<0, 9007199254740991>,
    pub attempt: super::EventInteger<1, 9007199254740991>,
    pub error: super::SessionStructuredError,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionRetryScheduledDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionRetryScheduledDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionRetryScheduledType`
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
pub enum EventSessionRetryScheduledType {
    #[serde(rename = "session.retry.scheduled")]
    SessionRetryScheduled,
}
impl ::std::fmt::Display for EventSessionRetryScheduledType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionRetryScheduled => f.write_str("session.retry.scheduled"),
        }
    }
}
impl ::std::str::FromStr for EventSessionRetryScheduledType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.retry.scheduled" => Ok(Self::SessionRetryScheduled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionRetryScheduledType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionRetryScheduledType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionStatus`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStatus {
    pub created: ::serde_json::Number,
    pub data: EventSessionStatusData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionStatusType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStatusData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStatusData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub status: EventStatusPayload,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStatusType`
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
pub enum EventSessionStatusType {
    #[serde(rename = "session.status")]
    SessionStatus,
}
impl ::std::fmt::Display for EventSessionStatusType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionStatus => f.write_str("session.status"),
        }
    }
}
impl ::std::str::FromStr for EventSessionStatusType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.status" => Ok(Self::SessionStatus),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionStatusType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionStatusType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionStepEnded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepEnded {
    pub created: ::serde_json::Number,
    pub data: EventSessionStepEndedData,
    pub durable: EventSessionStepEndedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionStepEndedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepEndedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepEndedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub cost: super::MoneyUsd,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub files: ::std::vec::Vec<super::RelativePath>,
    pub finish: EventSessionStepEndedDataFinish,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    #[serde(
        rename = "rawFinish",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub raw_finish: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub snapshot: ::std::option::Option<super::SnapshotId>,
    pub tokens: super::TokenUsageInfo,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepEndedDataFinish`
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
pub enum EventSessionStepEndedDataFinish {
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
impl ::std::fmt::Display for EventSessionStepEndedDataFinish {
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
impl ::std::str::FromStr for EventSessionStepEndedDataFinish {
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
impl ::std::convert::TryFrom<&str> for EventSessionStepEndedDataFinish {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionStepEndedDataFinish {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionStepEndedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepEndedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepEndedType`
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
pub enum EventSessionStepEndedType {
    #[serde(rename = "session.step.ended")]
    SessionStepEnded,
}
impl ::std::fmt::Display for EventSessionStepEndedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionStepEnded => f.write_str("session.step.ended"),
        }
    }
}
impl ::std::str::FromStr for EventSessionStepEndedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.step.ended" => Ok(Self::SessionStepEnded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionStepEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionStepEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionStepFailed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepFailed {
    pub created: ::serde_json::Number,
    pub data: EventSessionStepFailedData,
    pub durable: EventSessionStepFailedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionStepFailedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepFailedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepFailedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cost: ::std::option::Option<super::MoneyUsd>,
    pub error: super::SessionStructuredError,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub files: ::std::vec::Vec<super::RelativePath>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub finish: ::std::option::Option<EventSessionStepFailedDataFinish>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    #[serde(
        rename = "rawFinish",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub raw_finish: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub snapshot: ::std::option::Option<super::SnapshotId>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokens: ::std::option::Option<super::TokenUsageInfo>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepFailedDataFinish`
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
pub enum EventSessionStepFailedDataFinish {
    #[serde(rename = "content-filter")]
    ContentFilter,
}
impl ::std::fmt::Display for EventSessionStepFailedDataFinish {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContentFilter => f.write_str("content-filter"),
        }
    }
}
impl ::std::str::FromStr for EventSessionStepFailedDataFinish {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "content-filter" => Ok(Self::ContentFilter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionStepFailedDataFinish {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionStepFailedDataFinish {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionStepFailedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepFailedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepFailedType`
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
pub enum EventSessionStepFailedType {
    #[serde(rename = "session.step.failed")]
    SessionStepFailed,
}
impl ::std::fmt::Display for EventSessionStepFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionStepFailed => f.write_str("session.step.failed"),
        }
    }
}
impl ::std::str::FromStr for EventSessionStepFailedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.step.failed" => Ok(Self::SessionStepFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionStepFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionStepFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionStepStarted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepStarted {
    pub created: ::serde_json::Number,
    pub data: EventSessionStepStartedData,
    pub durable: EventSessionStepStartedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionStepStartedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepStartedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepStartedData {
    pub agent: super::AgentId,
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub model: super::ModelRef,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub snapshot: ::std::option::Option<super::SnapshotId>,
    pub started: super::EventInteger<0, 9007199254740991>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepStartedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepStartedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepStartedType`
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
pub enum EventSessionStepStartedType {
    #[serde(rename = "session.step.started")]
    SessionStepStarted,
}
impl ::std::fmt::Display for EventSessionStepStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionStepStarted => f.write_str("session.step.started"),
        }
    }
}
impl ::std::str::FromStr for EventSessionStepStartedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.step.started" => Ok(Self::SessionStepStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionStepStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionStepStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionStepStreamed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepStreamed {
    pub created: ::serde_json::Number,
    pub data: EventSessionStepStreamedData,
    pub durable: EventSessionStepStreamedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionStepStreamedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepStreamedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepStreamedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepStreamedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionStepStreamedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionStepStreamedType`
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
pub enum EventSessionStepStreamedType {
    #[serde(rename = "session.step.streamed")]
    SessionStepStreamed,
}
impl ::std::fmt::Display for EventSessionStepStreamedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionStepStreamed => f.write_str("session.step.streamed"),
        }
    }
}
impl ::std::str::FromStr for EventSessionStepStreamedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.step.streamed" => Ok(Self::SessionStepStreamed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionStepStreamedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionStepStreamedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionTextDelta`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionTextDelta {
    pub created: ::serde_json::Number,
    pub data: EventSessionTextDeltaData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionTextDeltaType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionTextDeltaData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionTextDeltaData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub delta: ::std::string::String,
    pub ordinal: super::EventInteger<0, 9007199254740991>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionTextDeltaType`
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
pub enum EventSessionTextDeltaType {
    #[serde(rename = "session.text.delta")]
    SessionTextDelta,
}
impl ::std::fmt::Display for EventSessionTextDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionTextDelta => f.write_str("session.text.delta"),
        }
    }
}
impl ::std::str::FromStr for EventSessionTextDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.text.delta" => Ok(Self::SessionTextDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionTextDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionTextDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionTextEnded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionTextEnded {
    pub created: ::serde_json::Number,
    pub data: EventSessionTextEndedData,
    pub durable: EventSessionTextEndedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionTextEndedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionTextEndedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionTextEndedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    pub ordinal: super::EventInteger<0, 9007199254740991>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub text: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionTextEndedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionTextEndedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionTextEndedType`
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
pub enum EventSessionTextEndedType {
    #[serde(rename = "session.text.ended")]
    SessionTextEnded,
}
impl ::std::fmt::Display for EventSessionTextEndedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionTextEnded => f.write_str("session.text.ended"),
        }
    }
}
impl ::std::str::FromStr for EventSessionTextEndedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.text.ended" => Ok(Self::SessionTextEnded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionTextEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionTextEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionTextStarted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionTextStarted {
    pub created: ::serde_json::Number,
    pub data: EventSessionTextStartedData,
    pub durable: EventSessionTextStartedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionTextStartedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionTextStartedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionTextStartedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub ordinal: super::EventInteger<0, 9007199254740991>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionTextStartedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionTextStartedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionTextStartedType`
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
pub enum EventSessionTextStartedType {
    #[serde(rename = "session.text.started")]
    SessionTextStarted,
}
impl ::std::fmt::Display for EventSessionTextStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionTextStarted => f.write_str("session.text.started"),
        }
    }
}
impl ::std::str::FromStr for EventSessionTextStartedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.text.started" => Ok(Self::SessionTextStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionTextStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionTextStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionToolCalled`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolCalled {
    pub created: ::serde_json::Number,
    pub data: EventSessionToolCalledData,
    pub durable: EventSessionToolCalledDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionToolCalledType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolCalledData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolCalledData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub executed: bool,
    pub id: ::std::string::String,
    pub input: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolCalledDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolCalledDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolCalledType`
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
pub enum EventSessionToolCalledType {
    #[serde(rename = "session.tool.called")]
    SessionToolCalled,
}
impl ::std::fmt::Display for EventSessionToolCalledType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionToolCalled => f.write_str("session.tool.called"),
        }
    }
}
impl ::std::str::FromStr for EventSessionToolCalledType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.tool.called" => Ok(Self::SessionToolCalled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionToolCalledType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionToolCalledType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionToolFailed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolFailed {
    pub created: ::serde_json::Number,
    pub data: EventSessionToolFailedData,
    pub durable: EventSessionToolFailedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionToolFailedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolFailedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolFailedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub content: ::std::option::Option<super::NonEmptyEventItems<super::ToolContent>>,
    pub error: super::SessionStructuredError,
    pub executed: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolFailedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolFailedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC3ea984aceae,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolFailedType`
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
pub enum EventSessionToolFailedType {
    #[serde(rename = "session.tool.failed")]
    SessionToolFailed,
}
impl ::std::fmt::Display for EventSessionToolFailedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionToolFailed => f.write_str("session.tool.failed"),
        }
    }
}
impl ::std::str::FromStr for EventSessionToolFailedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.tool.failed" => Ok(Self::SessionToolFailed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionToolFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionToolFailedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionToolInputDelta`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolInputDelta {
    pub created: ::serde_json::Number,
    pub data: EventSessionToolInputDeltaData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionToolInputDeltaType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolInputDeltaData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolInputDeltaData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub delta: ::std::string::String,
    pub id: ::std::string::String,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolInputDeltaType`
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
pub enum EventSessionToolInputDeltaType {
    #[serde(rename = "session.tool.input.delta")]
    SessionToolInputDelta,
}
impl ::std::fmt::Display for EventSessionToolInputDeltaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionToolInputDelta => f.write_str("session.tool.input.delta"),
        }
    }
}
impl ::std::str::FromStr for EventSessionToolInputDeltaType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.tool.input.delta" => Ok(Self::SessionToolInputDelta),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionToolInputDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionToolInputDeltaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionToolInputEnded`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolInputEnded {
    pub created: ::serde_json::Number,
    pub data: EventSessionToolInputEndedData,
    pub durable: EventSessionToolInputEndedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionToolInputEndedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolInputEndedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolInputEndedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub id: ::std::string::String,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub text: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolInputEndedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolInputEndedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolInputEndedType`
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
pub enum EventSessionToolInputEndedType {
    #[serde(rename = "session.tool.input.ended")]
    SessionToolInputEnded,
}
impl ::std::fmt::Display for EventSessionToolInputEndedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionToolInputEnded => f.write_str("session.tool.input.ended"),
        }
    }
}
impl ::std::str::FromStr for EventSessionToolInputEndedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.tool.input.ended" => Ok(Self::SessionToolInputEnded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionToolInputEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionToolInputEndedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionToolInputStarted`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolInputStarted {
    pub created: ::serde_json::Number,
    pub data: EventSessionToolInputStartedData,
    pub durable: EventSessionToolInputStartedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionToolInputStartedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolInputStartedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolInputStartedData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolInputStartedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolInputStartedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolInputStartedType`
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
pub enum EventSessionToolInputStartedType {
    #[serde(rename = "session.tool.input.started")]
    SessionToolInputStarted,
}
impl ::std::fmt::Display for EventSessionToolInputStartedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionToolInputStarted => f.write_str("session.tool.input.started"),
        }
    }
}
impl ::std::str::FromStr for EventSessionToolInputStartedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.tool.input.started" => Ok(Self::SessionToolInputStarted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionToolInputStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for EventSessionToolInputStartedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionToolProgress`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolProgress {
    pub created: ::serde_json::Number,
    pub data: EventSessionToolProgressData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionToolProgressType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolProgressData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolProgressData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub id: ::std::string::String,
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolProgressType`
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
pub enum EventSessionToolProgressType {
    #[serde(rename = "session.tool.progress")]
    SessionToolProgress,
}
impl ::std::fmt::Display for EventSessionToolProgressType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionToolProgress => f.write_str("session.tool.progress"),
        }
    }
}
impl ::std::str::FromStr for EventSessionToolProgressType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.tool.progress" => Ok(Self::SessionToolProgress),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionToolProgressType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionToolProgressType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionToolSuccess`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolSuccess {
    pub created: ::serde_json::Number,
    pub data: EventSessionToolSuccessData,
    pub durable: EventSessionToolSuccessDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionToolSuccessType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolSuccessData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolSuccessData {
    #[serde(rename = "assistantMessageID")]
    pub assistant_message_id: super::SessionMessageId,
    pub content: super::NonEmptyEventItems<super::ToolContent>,
    pub executed: bool,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub native: ::std::option::Option<super::SessionMessageNative>,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolSuccessDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionToolSuccessDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC3ea984aceae,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionToolSuccessType`
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
pub enum EventSessionToolSuccessType {
    #[serde(rename = "session.tool.success")]
    SessionToolSuccess,
}
impl ::std::fmt::Display for EventSessionToolSuccessType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionToolSuccess => f.write_str("session.tool.success"),
        }
    }
}
impl ::std::str::FromStr for EventSessionToolSuccessType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.tool.success" => Ok(Self::SessionToolSuccess),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionToolSuccessType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionToolSuccessType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionUnarchived`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionUnarchived {
    pub created: ::serde_json::Number,
    pub data: EventSessionUnarchivedData,
    pub durable: EventSessionUnarchivedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionUnarchivedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionUnarchivedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionUnarchivedData {
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionUnarchivedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionUnarchivedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionUnarchivedType`
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
pub enum EventSessionUnarchivedType {
    #[serde(rename = "session.unarchived")]
    SessionUnarchived,
}
impl ::std::fmt::Display for EventSessionUnarchivedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUnarchived => f.write_str("session.unarchived"),
        }
    }
}
impl ::std::str::FromStr for EventSessionUnarchivedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.unarchived" => Ok(Self::SessionUnarchived),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionUnarchivedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionUnarchivedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionUsageUpdated`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionUsageUpdated {
    pub created: ::serde_json::Number,
    pub data: EventSessionUsageUpdatedData,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionUsageUpdatedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionUsageUpdatedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionUsageUpdatedData {
    pub cost: super::MoneyUsd,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    pub tokens: super::TokenUsageInfo,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionUsageUpdatedType`
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
pub enum EventSessionUsageUpdatedType {
    #[serde(rename = "session.usage.updated")]
    SessionUsageUpdated,
}
impl ::std::fmt::Display for EventSessionUsageUpdatedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionUsageUpdated => f.write_str("session.usage.updated"),
        }
    }
}
impl ::std::str::FromStr for EventSessionUsageUpdatedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.usage.updated" => Ok(Self::SessionUsageUpdated),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionUsageUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionUsageUpdatedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventSessionViewed`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionViewed {
    pub created: ::serde_json::Number,
    pub data: EventSessionViewedData,
    pub durable: EventSessionViewedDurable,
    pub id: EventId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<LocationRef>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: EventSessionViewedType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionViewedData`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionViewedData {
    pub idle: ::serde_json::Number,
    #[serde(rename = "sessionID")]
    pub session_id: super::SessionId,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionViewedDurable`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventSessionViewedDurable {
    #[serde(rename = "aggregateID")]
    pub aggregate_id: ::std::string::String,
    pub seq: EventSeq,
    pub version: EventVersionEncodedC77c5abcb68d,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventSessionViewedType`
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
pub enum EventSessionViewedType {
    #[serde(rename = "session.viewed")]
    SessionViewed,
}
impl ::std::fmt::Display for EventSessionViewedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SessionViewed => f.write_str("session.viewed"),
        }
    }
}
impl ::std::str::FromStr for EventSessionViewedType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "session.viewed" => Ok(Self::SessionViewed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventSessionViewedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventSessionViewedType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventStatusPayloadBusy`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventStatusPayloadBusy {
    #[serde(rename = "type")]
    pub type_: EventStatusPayloadBusyType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventStatusPayloadBusyType`
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
pub enum EventStatusPayloadBusyType {
    #[serde(rename = "busy")]
    Busy,
}
impl ::std::fmt::Display for EventStatusPayloadBusyType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Busy => f.write_str("busy"),
        }
    }
}
impl ::std::str::FromStr for EventStatusPayloadBusyType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "busy" => Ok(Self::Busy),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventStatusPayloadBusyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventStatusPayloadBusyType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventStatusPayloadIdle`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventStatusPayloadIdle {
    #[serde(rename = "type")]
    pub type_: EventStatusPayloadIdleType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventStatusPayloadIdleType`
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
pub enum EventStatusPayloadIdleType {
    #[serde(rename = "idle")]
    Idle,
}
impl ::std::fmt::Display for EventStatusPayloadIdleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Idle => f.write_str("idle"),
        }
    }
}
impl ::std::str::FromStr for EventStatusPayloadIdleType {
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
impl ::std::convert::TryFrom<&str> for EventStatusPayloadIdleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventStatusPayloadIdleType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventStatusPayloadRetry`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventStatusPayloadRetry {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<EventStatusPayloadRetryAction>,
    pub attempt: super::EventInteger<0, 9007199254740991>,
    pub message: ::std::string::String,
    pub next: super::EventInteger<0, 9007199254740991>,
    #[serde(rename = "type")]
    pub type_: EventStatusPayloadRetryType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventStatusPayloadRetryAction`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct EventStatusPayloadRetryAction {
    pub label: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub link: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
    pub provider: ::std::string::String,
    pub reason: ::std::string::String,
    pub title: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`EventStatusPayloadRetryType`
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
pub enum EventStatusPayloadRetryType {
    #[serde(rename = "retry")]
    Retry,
}
impl ::std::fmt::Display for EventStatusPayloadRetryType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Retry => f.write_str("retry"),
        }
    }
}
impl ::std::str::FromStr for EventStatusPayloadRetryType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "retry" => Ok(Self::Retry),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventStatusPayloadRetryType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventStatusPayloadRetryType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`EventVersionEncodedC3ea984aceae`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EventVersionEncodedC3ea984aceae(pub super::EventVersion<2>);
impl ::std::ops::Deref for EventVersionEncodedC3ea984aceae {
    type Target = super::EventVersion<2>;
    fn deref(&self) -> &super::EventVersion<2> {
        &self.0
    }
}
impl ::std::convert::From<EventVersionEncodedC3ea984aceae> for super::EventVersion<2> {
    fn from(value: EventVersionEncodedC3ea984aceae) -> Self {
        value.0
    }
}
impl ::std::convert::From<super::EventVersion<2>> for EventVersionEncodedC3ea984aceae {
    fn from(value: super::EventVersion<2>) -> Self {
        Self(value)
    }
}
///`EventVersionEncodedC77c5abcb68d`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EventVersionEncodedC77c5abcb68d(pub super::EventVersion<1>);
impl ::std::ops::Deref for EventVersionEncodedC77c5abcb68d {
    type Target = super::EventVersion<1>;
    fn deref(&self) -> &super::EventVersion<1> {
        &self.0
    }
}
impl ::std::convert::From<EventVersionEncodedC77c5abcb68d> for super::EventVersion<1> {
    fn from(value: EventVersionEncodedC77c5abcb68d) -> Self {
        value.0
    }
}
impl ::std::convert::From<super::EventVersion<1>> for EventVersionEncodedC77c5abcb68d {
    fn from(value: super::EventVersion<1>) -> Self {
        Self(value)
    }
}
///`FormAnswer1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FormAnswer1(
    pub ::std::collections::HashMap<::std::string::String, FormValue1>,
);
impl ::std::ops::Deref for FormAnswer1 {
    type Target = ::std::collections::HashMap<::std::string::String, FormValue1>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, FormValue1> {
        &self.0
    }
}
impl ::std::convert::From<FormAnswer1>
for ::std::collections::HashMap<::std::string::String, FormValue1> {
    fn from(value: FormAnswer1) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, FormValue1>>
for FormAnswer1 {
    fn from(
        value: ::std::collections::HashMap<::std::string::String, FormValue1>,
    ) -> Self {
        Self(value)
    }
}
///`FormBooleanField1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormBooleanField1 {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FormBooleanField1Type,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormBooleanField1Type`
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
pub enum FormBooleanField1Type {
    #[serde(rename = "boolean")]
    Boolean,
}
impl ::std::fmt::Display for FormBooleanField1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Boolean => f.write_str("boolean"),
        }
    }
}
impl ::std::str::FromStr for FormBooleanField1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "boolean" => Ok(Self::Boolean),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormBooleanField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormBooleanField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormExternalField`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormExternalField {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub key: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FormExternalFieldType,
    pub url: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormExternalFieldType`
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
pub enum FormExternalFieldType {
    #[serde(rename = "external")]
    External,
}
impl ::std::fmt::Display for FormExternalFieldType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::External => f.write_str("external"),
        }
    }
}
impl ::std::str::FromStr for FormExternalFieldType {
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
impl ::std::convert::TryFrom<&str> for FormExternalFieldType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormExternalFieldType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormFields1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FormFields1(pub super::NonEmptyEventItems<EventFormField>);
impl ::std::ops::Deref for FormFields1 {
    type Target = super::NonEmptyEventItems<EventFormField>;
    fn deref(&self) -> &super::NonEmptyEventItems<EventFormField> {
        &self.0
    }
}
impl ::std::convert::From<FormFields1> for super::NonEmptyEventItems<EventFormField> {
    fn from(value: FormFields1) -> Self {
        value.0
    }
}
impl ::std::convert::From<super::NonEmptyEventItems<EventFormField>> for FormFields1 {
    fn from(value: super::NonEmptyEventItems<EventFormField>) -> Self {
        Self(value)
    }
}
///`FormId`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FormId(::std::string::String);
impl ::std::ops::Deref for FormId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FormId> for ::std::string::String {
    fn from(value: FormId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FormId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^frm_").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^frm_\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FormId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FormId {
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
///`FormInfo1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormInfo1 {
    pub fields: FormFields1,
    pub id: FormId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<FormMetadata>,
    #[serde(rename = "sessionID")]
    pub session_id: ::std::string::String,
    pub title: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormIntegerField1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormIntegerField1 {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<FormIntegerField1Default>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<FormIntegerField1Maximum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<FormIntegerField1Minimum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FormIntegerField1Type,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormIntegerField1Default`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FormIntegerField1Default {
    Number(::serde_json::Number),
    String(FormIntegerField1DefaultString),
}
impl ::std::convert::From<::serde_json::Number> for FormIntegerField1Default {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<FormIntegerField1DefaultString> for FormIntegerField1Default {
    fn from(value: FormIntegerField1DefaultString) -> Self {
        Self::String(value)
    }
}
///`FormIntegerField1DefaultString`
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
pub enum FormIntegerField1DefaultString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for FormIntegerField1DefaultString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for FormIntegerField1DefaultString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormIntegerField1DefaultString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormIntegerField1DefaultString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormIntegerField1Maximum`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FormIntegerField1Maximum {
    Number(::serde_json::Number),
    String(FormIntegerField1MaximumString),
}
impl ::std::convert::From<::serde_json::Number> for FormIntegerField1Maximum {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<FormIntegerField1MaximumString> for FormIntegerField1Maximum {
    fn from(value: FormIntegerField1MaximumString) -> Self {
        Self::String(value)
    }
}
///`FormIntegerField1MaximumString`
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
pub enum FormIntegerField1MaximumString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for FormIntegerField1MaximumString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for FormIntegerField1MaximumString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormIntegerField1MaximumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormIntegerField1MaximumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormIntegerField1Minimum`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FormIntegerField1Minimum {
    Number(::serde_json::Number),
    String(FormIntegerField1MinimumString),
}
impl ::std::convert::From<::serde_json::Number> for FormIntegerField1Minimum {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<FormIntegerField1MinimumString> for FormIntegerField1Minimum {
    fn from(value: FormIntegerField1MinimumString) -> Self {
        Self::String(value)
    }
}
///`FormIntegerField1MinimumString`
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
pub enum FormIntegerField1MinimumString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for FormIntegerField1MinimumString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for FormIntegerField1MinimumString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormIntegerField1MinimumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormIntegerField1MinimumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormIntegerField1Type`
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
pub enum FormIntegerField1Type {
    #[serde(rename = "integer")]
    Integer,
}
impl ::std::fmt::Display for FormIntegerField1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Integer => f.write_str("integer"),
        }
    }
}
impl ::std::str::FromStr for FormIntegerField1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "integer" => Ok(Self::Integer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormIntegerField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormIntegerField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormMetadata`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FormMetadata(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for FormMetadata {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<FormMetadata>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: FormMetadata) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for FormMetadata {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`FormMultiselectField1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormMultiselectField1 {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub default: ::std::vec::Vec<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(rename = "maxItems", skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub max_items: ::std::option::Option<super::EventInteger<0, 9007199254740991>>,
    #[serde(rename = "minItems", skip_serializing_if = "::std::option::Option::is_none")]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub min_items: ::std::option::Option<super::EventInteger<0, 9007199254740991>>,
    pub options: ::std::vec::Vec<FormOption>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FormMultiselectField1Type,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormMultiselectField1Type`
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
pub enum FormMultiselectField1Type {
    #[serde(rename = "multiselect")]
    Multiselect,
}
impl ::std::fmt::Display for FormMultiselectField1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Multiselect => f.write_str("multiselect"),
        }
    }
}
impl ::std::str::FromStr for FormMultiselectField1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "multiselect" => Ok(Self::Multiselect),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormMultiselectField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormMultiselectField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormNumberField1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormNumberField1 {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<FormNumberField1Default>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<FormNumberField1Maximum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<FormNumberField1Minimum>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FormNumberField1Type,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormNumberField1Default`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FormNumberField1Default {
    Number(::serde_json::Number),
    String(FormNumberField1DefaultString),
}
impl ::std::convert::From<::serde_json::Number> for FormNumberField1Default {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<FormNumberField1DefaultString> for FormNumberField1Default {
    fn from(value: FormNumberField1DefaultString) -> Self {
        Self::String(value)
    }
}
///`FormNumberField1DefaultString`
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
pub enum FormNumberField1DefaultString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for FormNumberField1DefaultString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for FormNumberField1DefaultString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormNumberField1DefaultString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormNumberField1DefaultString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormNumberField1Maximum`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FormNumberField1Maximum {
    Number(::serde_json::Number),
    String(FormNumberField1MaximumString),
}
impl ::std::convert::From<::serde_json::Number> for FormNumberField1Maximum {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<FormNumberField1MaximumString> for FormNumberField1Maximum {
    fn from(value: FormNumberField1MaximumString) -> Self {
        Self::String(value)
    }
}
///`FormNumberField1MaximumString`
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
pub enum FormNumberField1MaximumString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for FormNumberField1MaximumString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for FormNumberField1MaximumString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormNumberField1MaximumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormNumberField1MaximumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormNumberField1Minimum`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FormNumberField1Minimum {
    Number(::serde_json::Number),
    String(FormNumberField1MinimumString),
}
impl ::std::convert::From<::serde_json::Number> for FormNumberField1Minimum {
    fn from(value: ::serde_json::Number) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<FormNumberField1MinimumString> for FormNumberField1Minimum {
    fn from(value: FormNumberField1MinimumString) -> Self {
        Self::String(value)
    }
}
///`FormNumberField1MinimumString`
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
pub enum FormNumberField1MinimumString {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for FormNumberField1MinimumString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for FormNumberField1MinimumString {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormNumberField1MinimumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormNumberField1MinimumString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormNumberField1Type`
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
pub enum FormNumberField1Type {
    #[serde(rename = "number")]
    Number,
}
impl ::std::fmt::Display for FormNumberField1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Number => f.write_str("number"),
        }
    }
}
impl ::std::str::FromStr for FormNumberField1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "number" => Ok(Self::Number),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormNumberField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormNumberField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormOption`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormOption {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub label: ::std::string::String,
    pub value: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormStringField1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormStringField1 {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub custom: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<FormStringField1Format>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<bool>,
    pub key: ::std::string::String,
    #[serde(
        rename = "maxLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub max_length: ::std::option::Option<super::EventInteger<0, 9007199254740991>>,
    #[serde(
        rename = "minLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    #[serde(default, deserialize_with = "super::deserialize_optional_value")]
    pub min_length: ::std::option::Option<super::EventInteger<0, 9007199254740991>>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub options: ::std::vec::Vec<FormOption>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub pattern: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub placeholder: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FormStringField1Type,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub when: ::std::vec::Vec<FormWhen1>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormStringField1Format`
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
pub enum FormStringField1Format {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date-time")]
    DateTime,
}
impl ::std::fmt::Display for FormStringField1Format {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Email => f.write_str("email"),
            Self::Uri => f.write_str("uri"),
            Self::Date => f.write_str("date"),
            Self::DateTime => f.write_str("date-time"),
        }
    }
}
impl ::std::str::FromStr for FormStringField1Format {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "email" => Ok(Self::Email),
            "uri" => Ok(Self::Uri),
            "date" => Ok(Self::Date),
            "date-time" => Ok(Self::DateTime),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormStringField1Format {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormStringField1Format {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormStringField1Type`
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
pub enum FormStringField1Type {
    #[serde(rename = "string")]
    String,
}
impl ::std::fmt::Display for FormStringField1Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::String => f.write_str("string"),
        }
    }
}
impl ::std::str::FromStr for FormStringField1Type {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "string" => Ok(Self::String),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormStringField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormStringField1Type {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormValue1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct FormValue1 {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Number>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<FormValue1Subtype2>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_3: ::std::option::Option<bool>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_4: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
///`FormValue1Subtype2`
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
pub enum FormValue1Subtype2 {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for FormValue1Subtype2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for FormValue1Subtype2 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormValue1Subtype2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormValue1Subtype2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormWhen1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FormWhen1 {
    pub key: ::std::string::String,
    pub op: FormWhen1Op,
    pub value: FormWhen1Value,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FormWhen1Op`
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
pub enum FormWhen1Op {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "neq")]
    Neq,
}
impl ::std::fmt::Display for FormWhen1Op {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Eq => f.write_str("eq"),
            Self::Neq => f.write_str("neq"),
        }
    }
}
impl ::std::str::FromStr for FormWhen1Op {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "eq" => Ok(Self::Eq),
            "neq" => Ok(Self::Neq),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormWhen1Op {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormWhen1Op {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FormWhen1Value`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct FormWhen1Value {
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Number>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<FormWhen1ValueSubtype2>,
    #[serde(flatten, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_3: ::std::option::Option<bool>,
}
///`FormWhen1ValueSubtype2`
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
pub enum FormWhen1ValueSubtype2 {
    NaN,
    Infinity,
    #[serde(rename = "-Infinity")]
    XInfinity,
}
impl ::std::fmt::Display for FormWhen1ValueSubtype2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NaN => f.write_str("NaN"),
            Self::Infinity => f.write_str("Infinity"),
            Self::XInfinity => f.write_str("-Infinity"),
        }
    }
}
impl ::std::str::FromStr for FormWhen1ValueSubtype2 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            "-Infinity" => Ok(Self::XInfinity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FormWhen1ValueSubtype2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FormWhen1ValueSubtype2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Instruction entry key (lowercase alphanumerics plus . _ -)
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InstructionEntryKey(::std::string::String);
impl ::std::ops::Deref for InstructionEntryKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstructionEntryKey> for ::std::string::String {
    fn from(value: InstructionEntryKey) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for InstructionEntryKey {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z0-9][a-z0-9._-]*$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z0-9][a-z0-9._-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InstructionEntryKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstructionEntryKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InstructionEntryKey {
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
///`InstructionEntrySnapshot`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct InstructionEntrySnapshot(pub ::std::vec::Vec<InstructionEntrySnapshotItem>);
impl ::std::ops::Deref for InstructionEntrySnapshot {
    type Target = ::std::vec::Vec<InstructionEntrySnapshotItem>;
    fn deref(&self) -> &::std::vec::Vec<InstructionEntrySnapshotItem> {
        &self.0
    }
}
impl ::std::convert::From<InstructionEntrySnapshot>
for ::std::vec::Vec<InstructionEntrySnapshotItem> {
    fn from(value: InstructionEntrySnapshot) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<InstructionEntrySnapshotItem>>
for InstructionEntrySnapshot {
    fn from(value: ::std::vec::Vec<InstructionEntrySnapshotItem>) -> Self {
        Self(value)
    }
}
///`InstructionEntrySnapshotItem`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct InstructionEntrySnapshotItem {
    pub key: InstructionEntryKey,
    pub removed: bool,
    ///JSON value attached to the session's instructions
    pub value: ::serde_json::Value,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`InstructionHash`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct InstructionHash(::std::string::String);
impl ::std::ops::Deref for InstructionHash {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<InstructionHash> for ::std::string::String {
    fn from(value: InstructionHash) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for InstructionHash {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-f0-9]{64}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-f0-9]{64}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for InstructionHash {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for InstructionHash {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for InstructionHash {
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
///`LocationRef`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct LocationRef {
    pub directory: super::AbsolutePath,
    #[serde(
        rename = "workspaceID",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub workspace_id: ::std::option::Option<WorkspaceId>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
///`PermissionId`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PermissionId(::std::string::String);
impl ::std::ops::Deref for PermissionId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PermissionId> for ::std::string::String {
    fn from(value: PermissionId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PermissionId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^per").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^per\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for PermissionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PermissionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PermissionId {
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
///`PermissionReply`
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
pub enum PermissionReply {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "reject")]
    Reject,
}
impl ::std::fmt::Display for PermissionReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Once => f.write_str("once"),
            Self::Always => f.write_str("always"),
            Self::Reject => f.write_str("reject"),
        }
    }
}
impl ::std::str::FromStr for PermissionReply {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "once" => Ok(Self::Once),
            "always" => Ok(Self::Always),
            "reject" => Ok(Self::Reject),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PermissionReply {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PermissionReply {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PermissionSource`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct PermissionSource {
    pub id: ::std::string::String,
    #[serde(rename = "messageID")]
    pub message_id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: PermissionSourceType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`PermissionSourceType`
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
pub enum PermissionSourceType {
    #[serde(rename = "tool")]
    Tool,
}
impl ::std::fmt::Display for PermissionSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tool => f.write_str("tool"),
        }
    }
}
impl ::std::str::FromStr for PermissionSourceType {
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
impl ::std::convert::TryFrom<&str> for PermissionSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PermissionSourceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SessionInboxCompactionPayload`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SessionInboxCompactionPayload(pub super::NonNullValue);
impl ::std::ops::Deref for SessionInboxCompactionPayload {
    type Target = super::NonNullValue;
    fn deref(&self) -> &super::NonNullValue {
        &self.0
    }
}
impl ::std::convert::From<SessionInboxCompactionPayload> for super::NonNullValue {
    fn from(value: SessionInboxCompactionPayload) -> Self {
        value.0
    }
}
impl ::std::convert::From<super::NonNullValue> for SessionInboxCompactionPayload {
    fn from(value: super::NonNullValue) -> Self {
        Self(value)
    }
}
///`SessionInboxMovePayload1`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SessionInboxMovePayload1 {
    pub location: LocationRef,
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub subpath: ::std::option::Option<super::RelativePath>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
///`WorkspaceId`
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WorkspaceId(::std::string::String);
impl ::std::ops::Deref for WorkspaceId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WorkspaceId> for ::std::string::String {
    fn from(value: WorkspaceId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WorkspaceId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^wrk").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^wrk\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WorkspaceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WorkspaceId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WorkspaceId {
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
#[derive(Clone, Debug, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventInboxItem {
    /// Wire kind `user`.
    User(Box<EventInboxItemUser>),
    /// Wire kind `synthetic`.
    Synthetic(Box<EventInboxItemSynthetic>),
    /// Wire kind `compaction`.
    Compaction(Box<EventInboxItemCompaction>),
    /// Wire kind `move`.
    Move(Box<EventInboxItemMove>),
}
impl<'de> ::serde::Deserialize<'de> for EventInboxItem {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("user") => ::serde_json::from_value(raw).map(Self::User),
            Some("synthetic") => ::serde_json::from_value(raw).map(Self::Synthetic),
            Some("compaction") => ::serde_json::from_value(raw).map(Self::Compaction),
            Some("move") => ::serde_json::from_value(raw).map(Self::Move),
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
#[derive(Clone, Debug, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventStatusPayload {
    /// Wire kind `idle`.
    Idle(Box<EventStatusPayloadIdle>),
    /// Wire kind `retry`.
    Retry(Box<EventStatusPayloadRetry>),
    /// Wire kind `busy`.
    Busy(Box<EventStatusPayloadBusy>),
}
impl<'de> ::serde::Deserialize<'de> for EventStatusPayload {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("idle") => ::serde_json::from_value(raw).map(Self::Idle),
            Some("retry") => ::serde_json::from_value(raw).map(Self::Retry),
            Some("busy") => ::serde_json::from_value(raw).map(Self::Busy),
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
#[derive(Clone, Debug, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventFormField {
    /// Wire kind `string`.
    String(Box<EventFormFieldString>),
    /// Wire kind `number`.
    Number(Box<EventFormFieldNumber>),
    /// Wire kind `integer`.
    Integer(Box<EventFormFieldInteger>),
    /// Wire kind `boolean`.
    Boolean(Box<EventFormFieldBoolean>),
    /// Wire kind `multiselect`.
    Multiselect(Box<EventFormFieldMultiselect>),
    /// Wire kind `external`.
    External(Box<EventFormFieldExternal>),
}
impl<'de> ::serde::Deserialize<'de> for EventFormField {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("string") => ::serde_json::from_value(raw).map(Self::String),
            Some("number") => ::serde_json::from_value(raw).map(Self::Number),
            Some("integer") => ::serde_json::from_value(raw).map(Self::Integer),
            Some("boolean") => ::serde_json::from_value(raw).map(Self::Boolean),
            Some("multiselect") => ::serde_json::from_value(raw).map(Self::Multiselect),
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
#[derive(Clone, Debug, ::serde::Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Event {
    /// Wire kind `session.status`.
    SessionStatus(Box<EventSessionStatus>),
    /// Wire kind `rpc.*`.
    Rpc(Box<EventRpc>),
    /// Wire kind `server.connected`.
    ServerConnected(Box<EventServerConnected>),
    /// Wire kind `form.cancelled`.
    FormCancelled(Box<EventFormCancelled>),
    /// Wire kind `form.created`.
    FormCreated(Box<EventFormCreated>),
    /// Wire kind `form.replied`.
    FormReplied(Box<EventFormReplied>),
    /// Wire kind `permission.asked`.
    PermissionAsked(Box<EventPermissionAsked>),
    /// Wire kind `permission.replied`.
    PermissionReplied(Box<EventPermissionReplied>),
    /// Wire kind `session.agent.selected`.
    SessionAgentSelected(Box<EventSessionAgentSelected>),
    /// Wire kind `session.archived`.
    SessionArchived(Box<EventSessionArchived>),
    /// Wire kind `session.compaction.delta`.
    SessionCompactionDelta(Box<EventSessionCompactionDelta>),
    /// Wire kind `session.compaction.ended`.
    SessionCompactionEnded(Box<EventSessionCompactionEnded>),
    /// Wire kind `session.compaction.failed`.
    SessionCompactionFailed(Box<EventSessionCompactionFailed>),
    /// Wire kind `session.compaction.started`.
    SessionCompactionStarted(Box<EventSessionCompactionStarted>),
    /// Wire kind `session.created`.
    SessionCreated(Box<EventSessionCreated>),
    /// Wire kind `session.deleted`.
    SessionDeleted(Box<EventSessionDeleted>),
    /// Wire kind `session.execution.failed`.
    SessionExecutionFailed(Box<EventSessionExecutionFailed>),
    /// Wire kind `session.execution.interrupted`.
    SessionExecutionInterrupted(Box<EventSessionExecutionInterrupted>),
    /// Wire kind `session.execution.started`.
    SessionExecutionStarted(Box<EventSessionExecutionStarted>),
    /// Wire kind `session.execution.succeeded`.
    SessionExecutionSucceeded(Box<EventSessionExecutionSucceeded>),
    /// Wire kind `session.forked`.
    SessionForked(Box<EventSessionForked>),
    /// Wire kind `session.idle`.
    SessionIdle(Box<EventSessionIdle>),
    /// Wire kind `session.inbox.cancelled`.
    SessionInboxCancelled(Box<EventSessionInboxCancelled>),
    /// Wire kind `session.inbox.delivered`.
    SessionInboxDelivered(Box<EventSessionInboxDelivered>),
    /// Wire kind `session.inbox.delivery.changed`.
    SessionInboxDeliveryChanged(Box<EventSessionInboxDeliveryChanged>),
    /// Wire kind `session.inbox.enqueued`.
    SessionInboxEnqueued(Box<EventSessionInboxEnqueued>),
    /// Wire kind `session.metadata.updated`.
    SessionMetadataUpdated(Box<EventSessionMetadataUpdated>),
    /// Wire kind `session.model.selected`.
    SessionModelSelected(Box<EventSessionModelSelected>),
    /// Wire kind `session.moved`.
    SessionMoved(Box<EventSessionMoved>),
    /// Wire kind `session.permissions`.
    SessionPermissions(Box<EventSessionPermissions>),
    /// Wire kind `session.reasoning.delta`.
    SessionReasoningDelta(Box<EventSessionReasoningDelta>),
    /// Wire kind `session.reasoning.ended`.
    SessionReasoningEnded(Box<EventSessionReasoningEnded>),
    /// Wire kind `session.reasoning.started`.
    SessionReasoningStarted(Box<EventSessionReasoningStarted>),
    /// Wire kind `session.renamed`.
    SessionRenamed(Box<EventSessionRenamed>),
    /// Wire kind `session.retry.scheduled`.
    SessionRetryScheduled(Box<EventSessionRetryScheduled>),
    /// Wire kind `session.step.ended`.
    SessionStepEnded(Box<EventSessionStepEnded>),
    /// Wire kind `session.step.failed`.
    SessionStepFailed(Box<EventSessionStepFailed>),
    /// Wire kind `session.step.started`.
    SessionStepStarted(Box<EventSessionStepStarted>),
    /// Wire kind `session.step.streamed`.
    SessionStepStreamed(Box<EventSessionStepStreamed>),
    /// Wire kind `session.text.delta`.
    SessionTextDelta(Box<EventSessionTextDelta>),
    /// Wire kind `session.text.ended`.
    SessionTextEnded(Box<EventSessionTextEnded>),
    /// Wire kind `session.text.started`.
    SessionTextStarted(Box<EventSessionTextStarted>),
    /// Wire kind `session.tool.called`.
    SessionToolCalled(Box<EventSessionToolCalled>),
    /// Wire kind `session.tool.failed`.
    SessionToolFailed(Box<EventSessionToolFailed>),
    /// Wire kind `session.tool.input.delta`.
    SessionToolInputDelta(Box<EventSessionToolInputDelta>),
    /// Wire kind `session.tool.input.ended`.
    SessionToolInputEnded(Box<EventSessionToolInputEnded>),
    /// Wire kind `session.tool.input.started`.
    SessionToolInputStarted(Box<EventSessionToolInputStarted>),
    /// Wire kind `session.tool.progress`.
    SessionToolProgress(Box<EventSessionToolProgress>),
    /// Wire kind `session.tool.success`.
    SessionToolSuccess(Box<EventSessionToolSuccess>),
    /// Wire kind `session.unarchived`.
    SessionUnarchived(Box<EventSessionUnarchived>),
    /// Wire kind `session.usage.updated`.
    SessionUsageUpdated(Box<EventSessionUsageUpdated>),
    /// Wire kind `session.viewed`.
    SessionViewed(Box<EventSessionViewed>),
    /// Unimplemented or future kind, retaining the complete JSON object.
    Unknown(::serde_json::Value),
}
impl<'de> ::serde::Deserialize<'de> for Event {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = <::serde_json::Value as ::serde::Deserialize>::deserialize(d)?;
        match raw.get("type").and_then(::serde_json::Value::as_str) {
            Some("session.status") => {
                ::serde_json::from_value(raw).map(Self::SessionStatus)
            }
            Some(tag) if tag.starts_with("rpc.") => {
                ::serde_json::from_value(raw).map(Self::Rpc)
            }
            Some("server.connected") => {
                ::serde_json::from_value(raw).map(Self::ServerConnected)
            }
            Some("form.cancelled") => {
                ::serde_json::from_value(raw).map(Self::FormCancelled)
            }
            Some("form.created") => ::serde_json::from_value(raw).map(Self::FormCreated),
            Some("form.replied") => ::serde_json::from_value(raw).map(Self::FormReplied),
            Some("permission.asked") => {
                ::serde_json::from_value(raw).map(Self::PermissionAsked)
            }
            Some("permission.replied") => {
                ::serde_json::from_value(raw).map(Self::PermissionReplied)
            }
            Some("session.agent.selected") => {
                ::serde_json::from_value(raw).map(Self::SessionAgentSelected)
            }
            Some("session.archived") => {
                ::serde_json::from_value(raw).map(Self::SessionArchived)
            }
            Some("session.compaction.delta") => {
                ::serde_json::from_value(raw).map(Self::SessionCompactionDelta)
            }
            Some("session.compaction.ended") => {
                ::serde_json::from_value(raw).map(Self::SessionCompactionEnded)
            }
            Some("session.compaction.failed") => {
                ::serde_json::from_value(raw).map(Self::SessionCompactionFailed)
            }
            Some("session.compaction.started") => {
                ::serde_json::from_value(raw).map(Self::SessionCompactionStarted)
            }
            Some("session.created") => {
                ::serde_json::from_value(raw).map(Self::SessionCreated)
            }
            Some("session.deleted") => {
                ::serde_json::from_value(raw).map(Self::SessionDeleted)
            }
            Some("session.execution.failed") => {
                ::serde_json::from_value(raw).map(Self::SessionExecutionFailed)
            }
            Some("session.execution.interrupted") => {
                ::serde_json::from_value(raw).map(Self::SessionExecutionInterrupted)
            }
            Some("session.execution.started") => {
                ::serde_json::from_value(raw).map(Self::SessionExecutionStarted)
            }
            Some("session.execution.succeeded") => {
                ::serde_json::from_value(raw).map(Self::SessionExecutionSucceeded)
            }
            Some("session.forked") => {
                ::serde_json::from_value(raw).map(Self::SessionForked)
            }
            Some("session.idle") => ::serde_json::from_value(raw).map(Self::SessionIdle),
            Some("session.inbox.cancelled") => {
                ::serde_json::from_value(raw).map(Self::SessionInboxCancelled)
            }
            Some("session.inbox.delivered") => {
                ::serde_json::from_value(raw).map(Self::SessionInboxDelivered)
            }
            Some("session.inbox.delivery.changed") => {
                ::serde_json::from_value(raw).map(Self::SessionInboxDeliveryChanged)
            }
            Some("session.inbox.enqueued") => {
                ::serde_json::from_value(raw).map(Self::SessionInboxEnqueued)
            }
            Some("session.metadata.updated") => {
                ::serde_json::from_value(raw).map(Self::SessionMetadataUpdated)
            }
            Some("session.model.selected") => {
                ::serde_json::from_value(raw).map(Self::SessionModelSelected)
            }
            Some("session.moved") => {
                ::serde_json::from_value(raw).map(Self::SessionMoved)
            }
            Some("session.permissions") => {
                ::serde_json::from_value(raw).map(Self::SessionPermissions)
            }
            Some("session.reasoning.delta") => {
                ::serde_json::from_value(raw).map(Self::SessionReasoningDelta)
            }
            Some("session.reasoning.ended") => {
                ::serde_json::from_value(raw).map(Self::SessionReasoningEnded)
            }
            Some("session.reasoning.started") => {
                ::serde_json::from_value(raw).map(Self::SessionReasoningStarted)
            }
            Some("session.renamed") => {
                ::serde_json::from_value(raw).map(Self::SessionRenamed)
            }
            Some("session.retry.scheduled") => {
                ::serde_json::from_value(raw).map(Self::SessionRetryScheduled)
            }
            Some("session.step.ended") => {
                ::serde_json::from_value(raw).map(Self::SessionStepEnded)
            }
            Some("session.step.failed") => {
                ::serde_json::from_value(raw).map(Self::SessionStepFailed)
            }
            Some("session.step.started") => {
                ::serde_json::from_value(raw).map(Self::SessionStepStarted)
            }
            Some("session.step.streamed") => {
                ::serde_json::from_value(raw).map(Self::SessionStepStreamed)
            }
            Some("session.text.delta") => {
                ::serde_json::from_value(raw).map(Self::SessionTextDelta)
            }
            Some("session.text.ended") => {
                ::serde_json::from_value(raw).map(Self::SessionTextEnded)
            }
            Some("session.text.started") => {
                ::serde_json::from_value(raw).map(Self::SessionTextStarted)
            }
            Some("session.tool.called") => {
                ::serde_json::from_value(raw).map(Self::SessionToolCalled)
            }
            Some("session.tool.failed") => {
                ::serde_json::from_value(raw).map(Self::SessionToolFailed)
            }
            Some("session.tool.input.delta") => {
                ::serde_json::from_value(raw).map(Self::SessionToolInputDelta)
            }
            Some("session.tool.input.ended") => {
                ::serde_json::from_value(raw).map(Self::SessionToolInputEnded)
            }
            Some("session.tool.input.started") => {
                ::serde_json::from_value(raw).map(Self::SessionToolInputStarted)
            }
            Some("session.tool.progress") => {
                ::serde_json::from_value(raw).map(Self::SessionToolProgress)
            }
            Some("session.tool.success") => {
                ::serde_json::from_value(raw).map(Self::SessionToolSuccess)
            }
            Some("session.unarchived") => {
                ::serde_json::from_value(raw).map(Self::SessionUnarchived)
            }
            Some("session.usage.updated") => {
                ::serde_json::from_value(raw).map(Self::SessionUsageUpdated)
            }
            Some("session.viewed") => {
                ::serde_json::from_value(raw).map(Self::SessionViewed)
            }
            Some(_) => Ok(Self::Unknown(raw)),
            _ => {
                return Err(
                    ::serde::de::Error::custom("unknown or missing event/payload type"),
                );
            }
        }
            .map_err(::serde::de::Error::custom)
    }
}
impl Event {
    /// Wire discriminator, independent of the SSE event name.
    pub fn kind(&self) -> &str {
        match self {
            Self::SessionStatus(_) => "session.status",
            Self::Rpc(v) => v.type_.as_str(),
            Self::ServerConnected(_) => "server.connected",
            Self::FormCancelled(_) => "form.cancelled",
            Self::FormCreated(_) => "form.created",
            Self::FormReplied(_) => "form.replied",
            Self::PermissionAsked(_) => "permission.asked",
            Self::PermissionReplied(_) => "permission.replied",
            Self::SessionAgentSelected(_) => "session.agent.selected",
            Self::SessionArchived(_) => "session.archived",
            Self::SessionCompactionDelta(_) => "session.compaction.delta",
            Self::SessionCompactionEnded(_) => "session.compaction.ended",
            Self::SessionCompactionFailed(_) => "session.compaction.failed",
            Self::SessionCompactionStarted(_) => "session.compaction.started",
            Self::SessionCreated(_) => "session.created",
            Self::SessionDeleted(_) => "session.deleted",
            Self::SessionExecutionFailed(_) => "session.execution.failed",
            Self::SessionExecutionInterrupted(_) => "session.execution.interrupted",
            Self::SessionExecutionStarted(_) => "session.execution.started",
            Self::SessionExecutionSucceeded(_) => "session.execution.succeeded",
            Self::SessionForked(_) => "session.forked",
            Self::SessionIdle(_) => "session.idle",
            Self::SessionInboxCancelled(_) => "session.inbox.cancelled",
            Self::SessionInboxDelivered(_) => "session.inbox.delivered",
            Self::SessionInboxDeliveryChanged(_) => "session.inbox.delivery.changed",
            Self::SessionInboxEnqueued(_) => "session.inbox.enqueued",
            Self::SessionMetadataUpdated(_) => "session.metadata.updated",
            Self::SessionModelSelected(_) => "session.model.selected",
            Self::SessionMoved(_) => "session.moved",
            Self::SessionPermissions(_) => "session.permissions",
            Self::SessionReasoningDelta(_) => "session.reasoning.delta",
            Self::SessionReasoningEnded(_) => "session.reasoning.ended",
            Self::SessionReasoningStarted(_) => "session.reasoning.started",
            Self::SessionRenamed(_) => "session.renamed",
            Self::SessionRetryScheduled(_) => "session.retry.scheduled",
            Self::SessionStepEnded(_) => "session.step.ended",
            Self::SessionStepFailed(_) => "session.step.failed",
            Self::SessionStepStarted(_) => "session.step.started",
            Self::SessionStepStreamed(_) => "session.step.streamed",
            Self::SessionTextDelta(_) => "session.text.delta",
            Self::SessionTextEnded(_) => "session.text.ended",
            Self::SessionTextStarted(_) => "session.text.started",
            Self::SessionToolCalled(_) => "session.tool.called",
            Self::SessionToolFailed(_) => "session.tool.failed",
            Self::SessionToolInputDelta(_) => "session.tool.input.delta",
            Self::SessionToolInputEnded(_) => "session.tool.input.ended",
            Self::SessionToolInputStarted(_) => "session.tool.input.started",
            Self::SessionToolProgress(_) => "session.tool.progress",
            Self::SessionToolSuccess(_) => "session.tool.success",
            Self::SessionUnarchived(_) => "session.unarchived",
            Self::SessionUsageUpdated(_) => "session.usage.updated",
            Self::SessionViewed(_) => "session.viewed",
            Self::Unknown(v) => {
                v.get("type").and_then(::serde_json::Value::as_str).unwrap_or("")
            }
        }
    }
}
