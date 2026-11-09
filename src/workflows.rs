//! Stable session workflows and command/skill discovery.
use crate::{Client, Location, Result, Sessions, StatusCode, models::*};
use reqwest::Method;

/// Registered command discovery, obtained from [`Client::commands`].
#[derive(Clone, Copy, Debug)]
pub struct Commands<'a> {
    pub(crate) client: &'a Client,
}
impl Commands<'_> {
    /// List commands at the client default location, preserving the resolved location.
    /// Registration depends on server plugins; the catalog may be empty.
    pub async fn list(&self) -> Result<CommandListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// Override the default location; `Unspecified` omits the location query.
    /// Only HTTP 200 is accepted; ordinary transport, HTTP and decode errors apply.
    pub async fn list_at(&self, location: Location) -> Result<CommandListResponse> {
        self.client
            .get_json_at("api/command", "command.list", StatusCode::OK, &location)
            .await
    }
}
/// Registered skill discovery, obtained from [`Client::skills`].
#[derive(Clone, Copy, Debug)]
pub struct Skills<'a> {
    pub(crate) client: &'a Client,
}
impl Skills<'_> {
    /// List skills, including their content, at the client's default location.
    pub async fn list(&self) -> Result<SkillListResponse> {
        self.list_at(self.client.default_location().clone()).await
    }
    /// Override the default location; `Unspecified` omits the location query.
    /// Only HTTP 200 is accepted; ordinary transport, HTTP and decode errors apply.
    pub async fn list_at(&self, location: Location) -> Result<SkillListResponse> {
        self.client
            .get_json_at("api/skill", "skill.list", StatusCode::OK, &location)
            .await
    }
}
/// A session turn range. Omission selects the latest user turn and full-file patches.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SessionDiffOptions {
    /// User message whose turn starts the range.
    pub from: Option<SessionMessageId>,
    /// User message whose turn ends the range, inclusive.
    pub to: Option<SessionMessageId>,
    /// Patch context lines; zero is distinct from omission. Server bounds apply.
    pub context: Option<u64>,
}
impl SessionDiffOptions {
    /// Select the starting user message's turn.
    pub fn from(mut self, message: SessionMessageId) -> Self {
        self.from = Some(message);
        self
    }
    /// Extend through the ending user message's turn.
    pub fn to(mut self, message: SessionMessageId) -> Self {
        self.to = Some(message);
        self
    }
    /// Set patch context lines explicitly.
    pub fn context(mut self, lines: u64) -> Self {
        self.context = Some(lines);
        self
    }
}
impl<'a> Sessions<'a> {
    /// Execute a registered slash-command callback immediately (`session.command`).
    /// HTTP 204 means the callback returned, not that any work it starts completed.
    /// Callbacks can mutate state, files, or enqueue execution. Uses the ordinary
    /// request deadline; a timeout/lost response leaves the outcome uncertain.
    /// The SDK never retries. Session ownership determines location.
    pub async fn command(&self, id: &SessionId, request: &SessionCommandRequest) -> Result<()> {
        self.client
            .post_json_empty(
                "api/session",
                &[id.as_str(), "command"],
                "session.command",
                request,
            )
            .await
    }
    /// Admit durable compaction input, returning its inbox record (HTTP 200).
    /// Commits any staged revert first, defaults to steering and wakes execution
    /// on v2.2.5. Observe events/history
    /// for completion; admission can succeed even if the response is lost.
    /// An explicit ID permits server reconciliation, not automatic SDK retries.
    pub async fn compact(
        &self,
        id: &SessionId,
        request: &SessionCompactRequest,
    ) -> Result<SessionCompactResponse> {
        self.client
            .post_json_segments(
                "api/session",
                &[id.as_str(), "compact"],
                "session.compact",
                request,
            )
            .await
    }
    /// Read active context after the latest completed compaction (HTTP 200).
    /// Reuses projected message types; this is neither full history nor a rendered
    /// provider prompt. Session ownership determines location.
    pub async fn context(&self, id: &SessionId) -> Result<SessionContextResponse> {
        self.client
            .get_json_segments(
                "api/session",
                &[id.as_str(), "context"],
                "session.context",
                StatusCode::OK,
                &[],
            )
            .await
    }
    /// Read snapshot diffs for a turn/range (HTTP 200). Prompts steered before the
    /// next idle marker belong to the same turn. Cross-location ranges are rejected;
    /// an actively running step can compare against the current working copy.
    pub async fn diff(
        &self,
        id: &SessionId,
        options: &SessionDiffOptions,
    ) -> Result<SessionDiffResponse> {
        let mut query = Vec::new();
        if let Some(from) = &options.from {
            query.push(("from", from.to_string()));
        }
        if let Some(to) = &options.to {
            query.push(("to", to.to_string()));
        }
        if let Some(context) = options.context {
            query.push(("context", context.to_string()));
        }
        self.client
            .get_json_segments(
                "api/session",
                &[id.as_str(), "diff"],
                "session.diff",
                StatusCode::OK,
                &query,
            )
            .await
    }
    /// Manage a staged revert. Operations require an idle session on v2.2.5.
    pub fn revert(&self, id: &SessionId) -> SessionReverts<'a> {
        SessionReverts {
            client: self.client,
            id: id.clone(),
        }
    }
}
/// Revert lifecycle for one session, obtained from [`Sessions::revert`].
/// No operation retries; session ownership determines location.
#[derive(Clone, Debug)]
pub struct SessionReverts<'a> {
    client: &'a Client,
    id: SessionId,
}
impl SessionReverts<'_> {
    /// Stage/move an inclusive history boundary and optionally restore files.
    /// Omitted/null `files` enables restoration on v2.2.5; set false for history
    /// only. Moving a prior file revert first restores its original snapshot.
    /// HTTP 200 returns the staged metadata; history is retained until commit.
    pub async fn stage(
        &self,
        request: &SessionRevertStageRequest,
    ) -> Result<SessionRevertResponse> {
        self.client
            .post_json_segments(
                "api/session",
                &[self.id.as_str(), "revert", "stage"],
                "session.revert.stage",
                request,
            )
            .await
    }
    /// Clear a staged boundary and restore files changed by staging (HTTP 204).
    /// Restoration can overwrite edits made to those paths after staging. Does
    /// not restore files when none is staged, but still wakes queued execution
    /// on v2.2.5. Busy sessions fail before clearing.
    pub async fn clear(&self) -> Result<()> {
        self.client
            .delete_empty(
                "api/session",
                &[self.id.as_str(), "revert"],
                "session.revert.clear",
            )
            .await
    }
    /// Permanently remove history at/after the staged message and corresponding
    /// queued inputs, retaining staged file changes (HTTP 204). Cannot be undone
    /// through `clear`. No staged boundary is a no-op on v2.2.5.
    pub async fn commit(&self) -> Result<()> {
        self.client
            .send_empty(
                self.client.request(
                    Method::POST,
                    self.client
                        .segments_url("api/session", &[self.id.as_str(), "revert", "commit"]),
                ),
                "session.revert.commit",
            )
            .await
    }
}
