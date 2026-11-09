//! Typed API input and response models derived from the pinned OpenAPI schema.
//!
//! Response objects preserve additional fields in flattened `extra` maps.
//! Generated optional fields generally accept absent or null as `None`; the
//! `steps`, `more`, model integer limits, and constrained message numeric/content
//! adapters reject explicit null. Default-model `data` is required but nullable.
//! Request objects use strict
//! input types and [`crate::RequestField`] for nullable optional properties.
//! Unknown closed enum values are decoding errors. Model types and reasoning-field
//! names are open strings, as specified by the schema.

#[rustfmt::skip]
mod workspace;
pub use workspace::{
    FileListResponse, FileSystemEntry, FileSystemEntryType, VcsBase, VcsBaseResponse,
    VcsBaseSource, VcsBranch, VcsBranchList, VcsBranchesResponse, VcsDiffResponse, VcsFileStatus,
    VcsFileStatusStatus, VcsInfo, VcsMode, VcsResponse, VcsStatusResponse, VcsUntracked,
    WorktreeCreateRequest, WorktreeDirectory, WorktreeInfo, WorktreeList, WorktreeRefreshRequest,
    WorktreeRemoveRequest,
};
#[rustfmt::skip]
mod ptys;
pub use ptys::{
    Pty as PtyInfo, PtyConnectTokenResponse, PtyCreateRequest, PtyId, PtyListResponse, PtyResponse,
    PtyStatus, PtyTicketConnectToken, PtyUpdateRequest, PtyUpdateRequestSize as PtySize,
};
mod workspace_adapters;
use workspace_adapters::{
    deserialize_optional_safe_integer, deserialize_positive_integer, deserialize_required_nullable,
};

/// Generated live-event envelopes and their payloads.
#[rustfmt::skip]
pub mod events;
pub use events::{
    Event, EventLogSynced, PermissionId, PermissionReply, PermissionSource, PermissionSourceType,
};
pub use events::{
    EventFormField as FormField, FormAnswer1 as FormAnswer, FormFields1 as FormFields, FormId,
    FormInfo1 as FormInfo, FormMetadata, FormValue1 as FormValue,
    FormWhen1Value as FormConditionValue,
};
#[rustfmt::skip]
mod forms;
pub use forms::{
    FormBooleanFieldInput, FormBooleanFieldInputType, FormDetail, FormExternalFieldInput,
    FormExternalFieldInputType, FormFieldInput, FormIntegerFieldInput, FormIntegerFieldInputType,
    FormListResponse, FormMultiselectFieldInput, FormMultiselectFieldInputType, FormNumber,
    FormNumberFieldInput, FormNumberFieldInputType, FormNumberString, FormOptionInput, FormState,
    FormStateAnswered, FormStateAnsweredStatus, FormStateCancelled, FormStateCancelledStatus,
    FormStatePending, FormStatePendingStatus, FormStringFieldInput, FormStringFieldInputFormat,
    FormStringFieldInputType, FormWhenInput, FormWhenInputOp, SessionFormCreateRequest,
    SessionFormCreateResponse, SessionFormListResponse, SessionFormReplyRequest,
    SessionFormResponse,
};
mod form_inputs;
pub use form_inputs::{FormFieldsInput, FormInputError};
#[rustfmt::skip]
#[allow(dead_code)] // Typify emits a conversion-error helper unused by these models.
mod permissions;
pub use permissions::{
    PermissionRequest, PermissionRequestListResponse, PermissionSavedId, PermissionSavedInfo,
    PermissionSavedInfoTime, PermissionSavedListResponse, PermissionSavedResponse,
    PermissionSavedUpdateRequest, PermissionSourceInput, SessionPermissionCreateRequest,
    SessionPermissionCreateResponse, SessionPermissionCreateResponseData,
    SessionPermissionListResponse, SessionPermissionReplyRequest, SessionPermissionResponse,
};
mod permission_inputs;
mod session_log_adapters;
pub use session_log_adapters::SessionLogItem;
mod event_adapters;
pub use event_adapters::{EventInteger, EventIntegerError, EventVersion, NonEmptyEventItems};

use crate::RequestField;
#[rustfmt::skip]
mod inbox;
pub use inbox::error::ConversionError as InboxModelConversionError;
pub use inbox::{
    PromptInputAgentAttachment, PromptInputFileAttachment, PromptInputFileSource,
    PromptInputMention, PromptInputSkillAttachment, SessionInboxCompaction,
    SessionInboxCompactionTime, SessionInboxCompactionType, SessionInboxDelivery, SessionInboxMove,
    SessionInboxMovePayload, SessionInboxMoveTime, SessionInboxMoveType, SessionInboxResponse,
    SessionInboxSynthetic, SessionInboxSyntheticPayload, SessionInboxSyntheticTime,
    SessionInboxSyntheticType, SessionInboxUpdateRequest, SessionInboxUser,
    SessionInboxUserPayload, SessionInboxUserTime, SessionInboxUserType, SessionPromptRequest,
    SessionPromptResponse,
};
mod inbox_adapters;
pub use inbox_adapters::{NonNullValue, SessionInboxInfo};
mod catalog_adapters;
mod prompt_inputs;
mod session_inputs;
use catalog_adapters::{
    deserialize_model_integer, deserialize_nullable_model, deserialize_optional_model_integer,
};

#[rustfmt::skip]
mod messages;
pub use messages::error::ConversionError as MessageModelConversionError;
pub use messages::{
    PromptAgentAttachment, PromptBase64, PromptFileAttachment, PromptFileSource,
    PromptFileSourceVariant0Type, PromptFileSourceVariant1Type, PromptMention,
    PromptSkillAttachment, SessionMessageAgentSelected, SessionMessageAgentSelectedTime,
    SessionMessageAgentSelectedType, SessionMessageAssistant, SessionMessageAssistantContentItem,
    SessionMessageAssistantFinish, SessionMessageAssistantReasoning,
    SessionMessageAssistantReasoningTime, SessionMessageAssistantReasoningType,
    SessionMessageAssistantRetry, SessionMessageAssistantSnapshot, SessionMessageAssistantText,
    SessionMessageAssistantTextType, SessionMessageAssistantTime, SessionMessageAssistantTool,
    SessionMessageAssistantToolState, SessionMessageAssistantToolTime,
    SessionMessageAssistantToolType, SessionMessageAssistantType, SessionMessageCompaction,
    SessionMessageCompactionCompleted, SessionMessageCompactionCompletedReason,
    SessionMessageCompactionCompletedStatus, SessionMessageCompactionCompletedTime,
    SessionMessageCompactionCompletedType, SessionMessageCompactionFailed,
    SessionMessageCompactionFailedReason, SessionMessageCompactionFailedStatus,
    SessionMessageCompactionFailedTime, SessionMessageCompactionFailedType,
    SessionMessageCompactionRunning, SessionMessageCompactionRunningReason,
    SessionMessageCompactionRunningStatus, SessionMessageCompactionRunningTime,
    SessionMessageCompactionRunningType, SessionMessageIdle, SessionMessageIdleOutcome,
    SessionMessageIdleTime, SessionMessageIdleType, SessionMessageLocationSwitched,
    SessionMessageLocationSwitchedPrevious, SessionMessageLocationSwitchedTime,
    SessionMessageLocationSwitchedType, SessionMessageModelSelected,
    SessionMessageModelSelectedTime, SessionMessageModelSelectedType, SessionMessageNative,
    SessionMessageResponse, SessionMessageShell, SessionMessageShellExit,
    SessionMessageShellExitString, SessionMessageShellOutput, SessionMessageShellStatus,
    SessionMessageShellTime, SessionMessageShellType, SessionMessageSkill, SessionMessageSkillTime,
    SessionMessageSkillType, SessionMessageSynthetic, SessionMessageSyntheticTime,
    SessionMessageSyntheticType, SessionMessageSystem, SessionMessageSystemTime,
    SessionMessageSystemType, SessionMessageToolStateCompleted,
    SessionMessageToolStateCompletedStatus, SessionMessageToolStateError,
    SessionMessageToolStateErrorStatus, SessionMessageToolStateRunning,
    SessionMessageToolStateRunningStatus, SessionMessageToolStateStreaming,
    SessionMessageToolStateStreamingStatus, SessionMessageType, SessionMessageUser,
    SessionMessageUserTime, SessionMessageUserType, SessionMessagesResponse,
    SessionMessagesResponseCursor, SessionStructuredError, SessionStructuredErrorResponse, ShellId,
    SkillId, SkillName, ToolContent, ToolFileContent, ToolFileContentType, ToolTextContent,
    ToolTextContentType,
};
mod message_adapters;
pub use message_adapters::SessionMessageInfo;
use message_adapters::{
    deserialize_message_attempt, deserialize_message_retry_delay, deserialize_message_status,
    deserialize_tool_content,
};


#[rustfmt::skip]
#[allow(dead_code)] // Typify emits conversion helpers unused by these selected models.
mod generated;
pub use generated::{ServerInfo, ServerInfoCapabilities, ServerInfoPaths};

#[rustfmt::skip]
mod sessions;
pub use sessions::error::ConversionError as SessionModelConversionError;
pub use sessions::{
    FileDiffInfo, FileDiffInfoStatus, MoneyUsd, PermissionRuleInput, PermissionRulesetInput,
    RelativePath, SessionCreateRequest, SessionForkBoundary, SessionForkBoundaryVariant0Type,
    SessionForkBoundaryVariant1Type, SessionForkRequest, SessionId, SessionInfo, SessionInfoFork,
    SessionInfoOutcome, SessionInfoTime, SessionInterruptResponse, SessionLocationInput,
    SessionMessageId, SessionMetadata, SessionModelInput, SessionResponse, SessionRevert,
    SessionSwitchAgentRequest, SessionSwitchModelRequest, SessionUpdateRequest, SessionsCursor,
    SessionsResponse, SessionsResponseCursor, SnapshotId, TokenUsageInfo, TokenUsageInfoCache,
};

fn deserialize_optional_value<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[rustfmt::skip]
mod discovery;
pub use discovery::error::ConversionError as ModelConversionError;
pub use discovery::{
    AbsolutePath, AgentColor, AgentId, AgentInfo, AgentInfoMode, AgentListResponse, AgentResponse,
    LocationPublicInfo, LocationPublicInfoProject, LocationPublicRef, ModelCacheFormat,
    ModelCapabilities, ModelCompatibility, ModelCost, ModelCostCache, ModelCostTier,
    ModelCostTierType, ModelDefaultResponse, ModelInfo, ModelInfoLimit, ModelInfoStatus,
    ModelInfoTime, ModelListResponse, ModelMaxTokensField, ModelMediaInput, ModelReasoningField,
    ModelRef, ModelSettings, ModelType, ModelVariant, ModelVariantSettings,
    MoneyUsdPerMillionTokens, PermissionEffect, PermissionRule, PermissionRuleset, ProjectId,
    ProviderCache, ProviderCacheTtl, ProviderCacheWarming, ProviderCompaction,
    ProviderCompactionVariant0Type, ProviderCompactionVariant1Type, ProviderEncoding, ProviderId,
    ProviderRequest, ProviderSettings, ProviderTransport,
};

/// Provider timeout value: a JSON number, or `false` to disable the timeout.
/// The wire contract permits no other boolean value.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ProviderTimeout {
    /// Numeric timeout in the upstream provider's units.
    Number(serde_json::Number),
    /// Disable this timeout, encoded as `false`.
    Disabled,
}

impl<'de> serde::Deserialize<'de> for ProviderTimeout {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum Wire {
            Number(serde_json::Number),
            Boolean(bool),
        }
        match Wire::deserialize(deserializer)? {
            Wire::Number(value) => Ok(Self::Number(value)),
            Wire::Boolean(false) => Ok(Self::Disabled),
            Wire::Boolean(true) => Err(serde::de::Error::custom(
                "provider timeout must be a number or false",
            )),
        }
    }
}

impl serde::Serialize for ProviderTimeout {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Number(value) => serde::Serialize::serialize(value, serializer),
            Self::Disabled => serializer.serialize_bool(false),
        }
    }
}

impl std::fmt::Debug for ProviderRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Provider headers, body, settings, and extensions can all carry credentials.
        formatter
            .debug_struct("ProviderRequest")
            .finish_non_exhaustive()
    }
}

fn deserialize_steps<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u64>, D::Error> {
    let steps = <u64 as serde::Deserialize>::deserialize(deserializer)?;
    if !(1..=9_007_199_254_740_991).contains(&steps) {
        return Err(serde::de::Error::custom(
            "steps must be a positive safe integer",
        ));
    }
    Ok(Some(steps))
}

fn deserialize_diff_count<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<u64, D::Error> {
    let count = <u64 as serde::Deserialize>::deserialize(deserializer)?;
    if count > 9_007_199_254_740_991 {
        return Err(serde::de::Error::custom(
            "file diff count exceeds the safe-integer maximum",
        ));
    }
    Ok(count)
}

fn deserialize_more<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<bool>, D::Error> {
    if !<bool as serde::Deserialize>::deserialize(deserializer)? {
        return Err(serde::de::Error::custom(
            "file diff more must be true when present",
        ));
    }
    Ok(Some(true))
}

fn deserialize_pid<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    let pid = <u64 as serde::Deserialize>::deserialize(deserializer)?;
    if pid > 9_007_199_254_740_991 {
        return Err(serde::de::Error::custom(
            "pid exceeds the schema's safe-integer maximum",
        ));
    }
    Ok(pid)
}

#[rustfmt::skip]
#[allow(dead_code)] // Typify conversion helper is unused for these external-leaf models.
mod workflows;
pub use workflows::{
    CommandInfo, CommandListResponse, SessionCommandRequest, SessionCompactRequest,
    SessionCompactResponse, SessionContextResponse, SessionDiffResponse, SessionRevertResponse,
    SessionRevertStageRequest, SkillInfo, SkillListResponse,
};
mod workflow_inputs;
