// Generated from openapi/opencode-v2.json by model-spike --shells.
// Do not edit manually; adaptations live in tools/model-spike/src/shells.rs.
///`SessionEnvironmentRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionEnvironmentRequest {
    pub variables: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
}
///`SessionShellRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct SessionShellRequest {
    pub command: ::std::string::String,
    #[serde(default, skip_serializing_if = "crate::RequestField::is_omitted")]
    pub id: crate::RequestField<super::SessionMessageId>,
}
///`ShellCreateRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct ShellCreateRequest {
    pub command: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub cwd: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub metadata: Option<::serde_json::Map<String, ::serde_json::Value>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_safe_integer",
        serialize_with = "super::shell_inputs::serialize_timeout"
    )]
    pub timeout: Option<u64>,
}
///`ShellListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ShellListResponse {
    pub data: ::std::vec::Vec<super::ShellInfo>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ShellOutputResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ShellOutputResponse {
    pub data: super::ShellOutput,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ShellResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]
#[non_exhaustive]
pub struct ShellResponse {
    pub data: super::ShellInfo,
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
impl ::std::fmt::Debug for SessionEnvironmentRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("SessionEnvironmentRequest").finish_non_exhaustive()
    }
}
impl ::std::fmt::Debug for SessionShellRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("SessionShellRequest").finish_non_exhaustive()
    }
}
impl ::std::fmt::Debug for ShellCreateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("ShellCreateRequest").finish_non_exhaustive()
    }
}
impl ::std::fmt::Debug for ShellListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("ShellListResponse").finish_non_exhaustive()
    }
}
impl ::std::fmt::Debug for ShellOutputResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("ShellOutputResponse").finish_non_exhaustive()
    }
}
impl ::std::fmt::Debug for ShellResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("ShellResponse").finish_non_exhaustive()
    }
}
