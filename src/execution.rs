use crate::{
    Result, Sessions,
    models::{SessionId, SessionInterruptResponse},
};
use std::time::Duration;

/// Query options for [`Sessions::interrupt`]. Defaults omit `resume`.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SessionInterruptOptions {
    /// Resume eligible steering/control work after interruption. Omission is false
    /// on v2.2.5. `true` can start eligible work even when `interrupted` is false;
    /// queued next-turn prompts remain parked. It does not cancel inbox entries.
    pub resume: Option<bool>,
}
impl SessionInterruptOptions {
    /// Explicitly choose whether eligible work should resume.
    pub fn resume(mut self, resume: bool) -> Self {
        self.resume = Some(resume);
        self
    }
}

/// Per-call deadline for [`Sessions::experimental_wait`].
///
/// Defaults to no SDK deadline, independently of `ClientBuilder::request_timeout`.
/// Injected reqwest connection/read/total deadlines still apply. A configured
/// `timeout` overrides its total-request deadline, including error-body reads.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct SessionWaitOptions {
    /// Total request deadline. `None` adds no SDK deadline; it does not disable a
    /// supplied HTTP client's deadlines. Timeout does not interrupt execution.
    pub timeout: Option<Duration>,
}
impl SessionWaitOptions {
    /// Set a deadline for this wait without affecting other client requests.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    /// Remove this wait's SDK deadline; injected HTTP-client deadlines remain.
    pub fn without_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }
}

impl Sessions<'_> {
    /// Interrupt execution owned by this server process (`session.interrupt`).
    ///
    /// Returns the complete HTTP 200 response. `interrupted: true` acknowledges
    /// interruption before asynchronous cleanup finishes; use `experimental_wait`
    /// to observe idle. `false` can mean idle, already stopping, or settling, and
    /// does not establish idleness. On v2.2.5 omitted/false `resume` does not request
    /// resumption; `true` may wake eligible work even when nothing was interrupted.
    /// New concurrent admissions can also start work after interruption.
    ///
    /// Sends a bodyless POST and preserves omitted/false/true query choices. Uses
    /// the ordinary request deadline and ignores the discovery location default.
    /// Dropping/timing out this request does not undo an accepted interruption.
    /// Failures can follow an applied action; the default transport does not retry.
    /// Injected HTTP-client policies remain in effect.
    pub async fn interrupt(
        &self,
        id: &SessionId,
        options: &SessionInterruptOptions,
    ) -> Result<SessionInterruptResponse> {
        let query: Vec<_> = options
            .resume
            .into_iter()
            .map(|v| ("resume", v.to_string()))
            .collect();
        self.client
            .post_query_json(
                "api/session",
                &[id.as_str(), "interrupt"],
                "session.interrupt",
                &query,
            )
            .await
    }

    /// Wait for process-local execution to become idle (`experimental.session.wait`).
    ///
    /// Sends a bodyless POST; returns `()` only for HTTP 204. On v2.2.5 this never
    /// starts work and returns immediately if already idle. It waits through
    /// cleanup and successor executions, but does not report their outcome. Idle
    /// can follow success, failure, or interruption and can coexist with queued
    /// input. New work can start immediately afterward. Inspect session/history
    /// snapshots and execution events separately; this is not a per-prompt receipt.
    ///
    /// Uses this call's deadline instead of the ordinary client request deadline,
    /// with the normal bounded response diagnostics. Injected HTTP policies remain
    /// effective. Dropping the future or timing out cancels only local waiting;
    /// it neither interrupts execution nor removes pending input. No automatic
    /// retry or polling is performed. The discovery location default is ignored.
    pub async fn experimental_wait(
        &self,
        id: &SessionId,
        options: &SessionWaitOptions,
    ) -> Result<()> {
        self.client
            .post_empty_with_timeout(
                "api/experimental/session",
                &[id.as_str(), "wait"],
                "experimental.session.wait",
                options.timeout,
            )
            .await
    }
}
