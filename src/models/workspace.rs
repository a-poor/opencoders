// Generated from openapi/opencode-v2.json by model-spike --workspace.
// Do not edit manually; adaptations live in tools/model-spike/src/workspace.rs.
///`FileListResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FileListResponse {
    pub data: ::std::vec::Vec<FileSystemEntry>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FileSystemEntry`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct FileSystemEntry {
    pub path: super::RelativePath,
    #[serde(rename = "type")]
    pub type_: FileSystemEntryType,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`FileSystemEntryType`
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
pub enum FileSystemEntryType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "directory")]
    Directory,
}
impl ::std::fmt::Display for FileSystemEntryType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => f.write_str("file"),
            Self::Directory => f.write_str("directory"),
        }
    }
}
impl ::std::str::FromStr for FileSystemEntryType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            "directory" => Ok(Self::Directory),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FileSystemEntryType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FileSystemEntryType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`VcsBase`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsBase {
    pub name: ::std::string::String,
    #[serde(rename = "ref")]
    pub ref_: ::std::string::String,
    pub source: VcsBaseSource,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsBaseResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsBaseResponse {
    #[serde(deserialize_with = "super::deserialize_required_nullable")]
    pub data: ::std::option::Option<VcsBase>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsBaseSource`
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
pub enum VcsBaseSource {
    #[serde(rename = "reflog")]
    Reflog,
    #[serde(rename = "default")]
    Default,
}
impl ::std::fmt::Display for VcsBaseSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Reflog => f.write_str("reflog"),
            Self::Default => f.write_str("default"),
        }
    }
}
impl ::std::str::FromStr for VcsBaseSource {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "reflog" => Ok(Self::Reflog),
            "default" => Ok(Self::Default),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VcsBaseSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VcsBaseSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`VcsBranch`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsBranch {
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub current: ::std::option::Option<::std::string::String>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsBranchList`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct VcsBranchList(pub ::std::vec::Vec<::std::string::String>);
impl ::std::ops::Deref for VcsBranchList {
    type Target = ::std::vec::Vec<::std::string::String>;
    fn deref(&self) -> &::std::vec::Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<VcsBranchList> for ::std::vec::Vec<::std::string::String> {
    fn from(value: VcsBranchList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for VcsBranchList {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
///`VcsBranchesResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsBranchesResponse {
    pub data: VcsBranchList,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsDiffResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsDiffResponse {
    pub data: ::std::vec::Vec<super::FileDiffInfo>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsFileStatus`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsFileStatus {
    #[serde(deserialize_with = "super::deserialize_pid")]
    pub additions: u64,
    #[serde(deserialize_with = "super::deserialize_pid")]
    pub deletions: u64,
    pub file: ::std::string::String,
    pub status: VcsFileStatusStatus,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsFileStatusStatus`
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
pub enum VcsFileStatusStatus {
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "modified")]
    Modified,
}
impl ::std::fmt::Display for VcsFileStatusStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Added => f.write_str("added"),
            Self::Deleted => f.write_str("deleted"),
            Self::Modified => f.write_str("modified"),
        }
    }
}
impl ::std::str::FromStr for VcsFileStatusStatus {
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
impl ::std::convert::TryFrom<&str> for VcsFileStatusStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VcsFileStatusStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`VcsInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsInfo {
    pub branch: VcsBranch,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub provider: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsMode`
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
pub enum VcsMode {
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "committed")]
    Committed,
}
impl ::std::fmt::Display for VcsMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Working => f.write_str("working"),
            Self::Branch => f.write_str("branch"),
            Self::Committed => f.write_str("committed"),
        }
    }
}
impl ::std::str::FromStr for VcsMode {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "working" => Ok(Self::Working),
            "branch" => Ok(Self::Branch),
            "committed" => Ok(Self::Committed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VcsMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VcsMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`VcsResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsResponse {
    pub data: VcsInfo,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsStatusResponse`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct VcsStatusResponse {
    pub data: ::std::vec::Vec<VcsFileStatus>,
    pub location: super::LocationPublicRef,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`VcsUntracked`
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
pub enum VcsUntracked {
    #[serde(rename = "files")]
    Files,
    #[serde(rename = "folders")]
    Folders,
}
impl ::std::fmt::Display for VcsUntracked {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Files => f.write_str("files"),
            Self::Folders => f.write_str("folders"),
        }
    }
}
impl ::std::str::FromStr for VcsUntracked {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "files" => Ok(Self::Files),
            "folders" => Ok(Self::Folders),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VcsUntracked {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VcsUntracked {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`WorktreeCreateRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct WorktreeCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub branch: Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub directory: Option<super::AbsolutePath>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub from: Option<super::AbsolutePath>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "super::deserialize_optional_value"
    )]
    pub name: Option<::std::string::String>,
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
}
///`WorktreeDirectory`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct WorktreeDirectory {
    pub directory: super::AbsolutePath,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub strategy: ::std::option::Option<::std::string::String>,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`WorktreeInfo`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct WorktreeInfo {
    pub directory: super::AbsolutePath,
    #[serde(flatten)]
    pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
///`WorktreeList`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct WorktreeList(pub ::std::vec::Vec<WorktreeDirectory>);
impl ::std::ops::Deref for WorktreeList {
    type Target = ::std::vec::Vec<WorktreeDirectory>;
    fn deref(&self) -> &::std::vec::Vec<WorktreeDirectory> {
        &self.0
    }
}
impl ::std::convert::From<WorktreeList> for ::std::vec::Vec<WorktreeDirectory> {
    fn from(value: WorktreeList) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<WorktreeDirectory>> for WorktreeList {
    fn from(value: ::std::vec::Vec<WorktreeDirectory>) -> Self {
        Self(value)
    }
}
///`WorktreeRefreshRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct WorktreeRefreshRequest {
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
}
///`WorktreeRemoveRequest`
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct WorktreeRemoveRequest {
    pub directory: super::AbsolutePath,
    pub force: bool,
    #[serde(rename = "projectID")]
    pub project_id: super::ProjectId,
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
