// Generated from openapi/opencode-v2.json by model-spike --server-info.
// Do not edit manually; adaptations live in tools/model-spike/src/server_info.rs.
///`ServerInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ServerInfo {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub capabilities: ::std::option::Option<ServerInfoCapabilities>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub hostname: ::std::option::Option<::std::string::String>,
    pub paths: ServerInfoPaths,
    #[serde(deserialize_with = "super::deserialize_pid")]
    pub pid: u64,
    pub urls: ::std::vec::Vec<::std::string::String>,
    pub version: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ServerInfoCapabilities`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ServerInfoCapabilities {
    #[serde(
        rename = "persistentPty",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub persistent_pty: ::std::option::Option<bool>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`ServerInfoPaths`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct ServerInfoPaths {
    pub tmp: ::std::string::String,
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
