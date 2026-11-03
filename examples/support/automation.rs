//! Example-owned automation policy, not a public SDK orchestration API.
use opencoders::models::{
    SessionFormListResponse, SessionInboxResponse, SessionInfo, SessionMessageId,
    SessionMessageInfo, SessionPermissionListResponse,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitState {
    Idle,
    TimedOut,
    Failed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Succeeded,
    Failed,
    Interrupted,
    AwaitingInput,
    Pending,
    Inconclusive,
}
#[derive(Clone)]
pub struct Inspection {
    pub session: SessionInfo,
    pub history: Vec<SessionMessageInfo>,
    pub history_complete: bool,
    pub inbox: SessionInboxResponse,
    pub permissions: SessionPermissionListResponse,
    pub forms: SessionFormListResponse,
}
/// Conservative policy for a fresh, otherwise exclusively used session. The API
/// does not supply a per-prompt completion receipt or an atomic snapshot boundary.
pub fn assess(view: &Inspection, prompt: &SessionMessageId, wait: WaitState) -> Outcome {
    use opencoders::models::{SessionInfoOutcome as Session, SessionMessageIdleOutcome as Idle};
    if !view.permissions.data.is_empty() || !view.forms.data.is_empty() {
        return Outcome::AwaitingInput;
    }
    if view.inbox.data.iter().any(|item| item.id() == prompt) {
        return Outcome::Pending;
    }
    if wait != WaitState::Idle || !view.history_complete || !view.inbox.data.is_empty() {
        return Outcome::Inconclusive;
    }
    let mut ids = std::collections::BTreeSet::new();
    let mut input = false;
    let mut terminal = None;
    let mut assistants = 0;
    let mut incomplete_output = false;
    for message in &view.history {
        if !ids.insert(message.id().as_str()) {
            return Outcome::Inconclusive;
        }
        match message {
            SessionMessageInfo::User(user) if user.id == *prompt && !input => input = true,
            SessionMessageInfo::AgentSelected(_) | SessionMessageInfo::ModelSelected(_)
                if !input => {}
            // The pinned message updater projects instruction changes as system
            // notices. They can precede or follow delivery of the one user input.
            SessionMessageInfo::System(system)
                if terminal.is_none()
                    && system
                        .metadata
                        .get("notice")
                        .and_then(serde_json::Value::as_str)
                        == Some("instructions") => {}
            SessionMessageInfo::Assistant(assistant) if input && terminal.is_none() => {
                assistants += 1;
                incomplete_output |=
                    assistant.time.completed.is_none() || assistant.error.is_some();
            }
            SessionMessageInfo::Idle(idle) if input && terminal.is_none() => {
                terminal = Some(idle.outcome);
                incomplete_output |= idle.error.is_some();
            }
            _ => return Outcome::Inconclusive,
        }
    }
    match (view.session.outcome, terminal) {
        (Some(Session::Succeeded), Some(Idle::Succeeded))
            if assistants > 0 && !incomplete_output =>
        {
            Outcome::Succeeded
        }
        (Some(Session::Failed), Some(Idle::Failed)) => Outcome::Failed,
        (Some(Session::Interrupted), Some(Idle::Interrupted)) => Outcome::Interrupted,
        _ => Outcome::Inconclusive,
    }
}

use opencoders::{
    Client, EventOptions, EventStream, MessageListOptions, MessageOrder, SessionWaitOptions,
    models::{
        AgentId, Event, SessionCreateRequest, SessionId, SessionInboxDelivery,
        SessionLocationInput, SessionMessageAssistantContentItem, SessionModelInput,
        SessionPromptRequest,
    },
};
use std::{collections::BTreeSet, fmt, future::Future, time::Duration};
use tokio::time::{Instant, timeout_at};

pub struct Job {
    pub session_id: SessionId,
    pub prompt_id: SessionMessageId,
    pub directory: String,
    pub agent: AgentId,
    pub model: SessionModelInput,
    pub text: String,
}
#[derive(Clone, Copy)]
pub struct Limits {
    /// Includes creation, subscription, admission and waiting; not reset per step.
    pub work: Duration,
    /// A separate diagnostic window, including when the work deadline expired.
    pub inspection: Duration,
    pub max_pages: usize,
    pub max_events: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            work: Duration::from_secs(30),
            inspection: Duration::from_secs(10),
            max_pages: 3,
            max_events: 1024,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Create,
    Subscribe,
    Admission,
}
#[derive(Debug)]
pub enum Problem {
    Deadline,
    Api(opencoders::Error),
    UnexpectedIdentity,
}
impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deadline => f.write_str("local deadline expired; server work may continue"),
            Self::Api(error) => fmt::Display::fmt(error, f),
            Self::UnexpectedIdentity => {
                f.write_str("response identity did not match the requested session/prompt")
            }
        }
    }
}
#[derive(Debug)]
pub struct Failure {
    pub stage: Stage,
    pub problem: Problem,
}
impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.stage, self.problem)
    }
}
impl std::error::Error for Failure {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notice {
    Created,
    Admitted,
    Waiting,
    TextFragment(usize),
    Execution(Outcome),
    UnknownEvent,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedState {
    Open,
    Stopped,
    Ended,
    Failed,
    BudgetReached,
}
pub struct Trace {
    pub feed: FeedState,
    pub events_seen: usize,
    pub text_fragments: usize,
    pub unknown_events: usize,
    pub stream_error: Option<opencoders::Error>,
}
pub struct Report {
    pub outcome: Outcome,
    pub wait: WaitState,
    pub wait_error: Option<Problem>,
    pub inspection: Option<Inspection>,
    pub inspection_error: Option<Problem>,
    pub trace: Trace,
}

async fn bounded<T>(
    deadline: Instant,
    work: impl Future<Output = opencoders::Result<T>>,
) -> Result<T, Problem> {
    if Instant::now() >= deadline {
        return Err(Problem::Deadline);
    }
    timeout_at(deadline, work)
        .await
        .map_err(|_| Problem::Deadline)?
        .map_err(Problem::Api)
}
struct Observer<'a, F> {
    session: &'a SessionId,
    stream: Option<EventStream>,
    trace: Trace,
    budget: usize,
    notice: F,
}
impl<F: FnMut(Notice)> Observer<'_, F> {
    /// Drive progress alongside the request, including while admission is in
    /// flight. No background task or unbounded queue is needed by this example.
    async fn during<T>(
        &mut self,
        deadline: Instant,
        work: impl Future<Output = opencoders::Result<T>>,
    ) -> Result<T, Problem> {
        if Instant::now() >= deadline {
            return Err(Problem::Deadline);
        }
        tokio::pin!(work);
        loop {
            if self.trace.feed == FeedState::Open && self.trace.events_seen >= self.budget {
                self.trace.feed = FeedState::BudgetReached;
                self.stream = None;
            }
            tokio::select! {
                biased; // Neither a busy global feed nor ready work can starve the deadline.
                _ = tokio::time::sleep_until(deadline) => return Err(Problem::Deadline),
                result = &mut work => return result.map_err(Problem::Api),
                item = async {
                    match self.stream.as_mut() {
                        Some(stream) => stream.next().await,
                        None => std::future::pending().await,
                    }
                } => match item {
                    Some(Ok(message)) => {
                        self.trace.events_seen += 1; // All traffic counts toward the budget.
                        match message.event {
                            Event::SessionTextDelta(e) if e.data.session_id == *self.session => {
                                self.trace.text_fragments += 1;
                                (self.notice)(Notice::TextFragment(e.data.delta.chars().count()));
                            }
                            Event::SessionExecutionSucceeded(e) if e.data.session_id == *self.session => (self.notice)(Notice::Execution(Outcome::Succeeded)),
                            Event::SessionExecutionFailed(e) if e.data.session_id == *self.session => (self.notice)(Notice::Execution(Outcome::Failed)),
                            Event::SessionExecutionInterrupted(e) if e.data.session_id == *self.session => (self.notice)(Notice::Execution(Outcome::Interrupted)),
                            Event::Unknown(_) => {
                                self.trace.unknown_events += 1;
                                (self.notice)(Notice::UnknownEvent);
                            }
                            _ => {}
                        }
                    }
                    Some(Err(error)) => {
                        self.trace.feed = FeedState::Failed;
                        self.trace.stream_error = Some(error);
                        self.stream = None;
                    }
                    None => { self.trace.feed = FeedState::Ended; self.stream = None; }
                }
            }
        }
    }
}

impl Inspection {
    async fn load(client: &Client, id: &SessionId, max_pages: usize) -> opencoders::Result<Self> {
        let sessions = client.sessions();
        let inbox = sessions.inbox(id);
        let permissions = sessions.permissions(id);
        let forms = sessions.forms(id);
        let (session, (history, history_complete), inbox, permissions, forms) = tokio::try_join!(
            sessions.get(id),
            read_history(client, id, max_pages),
            inbox.list(),
            permissions.list(),
            forms.list(),
        )?;
        Ok(Self {
            session: session.data,
            history,
            history_complete,
            inbox,
            permissions,
            forms,
        })
    }
    /// Call only after assessing success. These are stored text parts, not live
    /// fragments, and do not establish semantic correctness of an answer.
    pub fn answer_text(&self) -> impl Iterator<Item = &str> {
        self.history
            .iter()
            .filter_map(|message| match message {
                SessionMessageInfo::Assistant(assistant) => Some(assistant.content.iter()),
                _ => None,
            })
            .flatten()
            .filter_map(|part| match part {
                SessionMessageAssistantContentItem::Text(text) => Some(text.text.as_str()),
                _ => None,
            })
    }
}
async fn read_history(
    client: &Client,
    id: &SessionId,
    max_pages: usize,
) -> opencoders::Result<(Vec<SessionMessageInfo>, bool)> {
    let mut options = MessageListOptions::default()
        .limit(50)
        .order(MessageOrder::Asc);
    let mut history = Vec::new();
    let mut cursors = BTreeSet::new();
    for _ in 0..max_pages {
        let page = client.sessions().messages(id).list(&options).await?;
        if page.data.is_empty() {
            return Ok((history, true));
        }
        let oversized = page.data.len() > 50;
        history.extend(page.data.into_iter().take(50));
        if oversized {
            return Ok((history, false));
        }
        let Some(next) = page.cursor.next else {
            return Ok((history, true));
        };
        if !cursors.insert(next.clone()) {
            return Ok((history, false));
        }
        options.cursor = Some(next);
        options.order = None; // Order is cursor-bound; do not resend it with a cursor.
    }
    Ok((history, false)) // A cursor/short page does not prove the end of history.
}

/// One fresh-session attempt. No retries, reconnection, approval policy, interrupt
/// or deletion. Caller-chosen IDs remain available even if a response is lost.
/// All created state is retained; local deadlines do not undo mutations.
pub async fn run(
    client: &Client,
    job: &Job,
    limits: Limits,
    mut notice: impl FnMut(Notice),
) -> Result<Report, Failure> {
    let deadline = Instant::now() + limits.work;
    let create = SessionCreateRequest::default()
        .id(job.session_id.clone())
        .title("Bounded automation example")
        .location(SessionLocationInput::new(job.directory.clone()))
        .agent(job.agent.clone())
        .model(job.model.clone());
    let created = bounded(deadline, client.sessions().create(&create))
        .await
        .map_err(|problem| Failure {
            stage: Stage::Create,
            problem,
        })?;
    if created.data.id != job.session_id {
        return Err(Failure {
            stage: Stage::Create,
            problem: Problem::UnexpectedIdentity,
        });
    }
    notice(Notice::Created);
    let stream = bounded(
        deadline,
        client
            .events()
            .subscribe(&EventOptions::default().max_event_bytes(64 * 1024)),
    )
    .await
    .map_err(|problem| Failure {
        stage: Stage::Subscribe,
        problem,
    })?;
    let mut observer = Observer {
        session: &job.session_id,
        stream: Some(stream),
        budget: limits.max_events,
        notice,
        trace: Trace {
            feed: FeedState::Open,
            events_seen: 0,
            text_fragments: 0,
            unknown_events: 0,
            stream_error: None,
        },
    };
    let prompt = SessionPromptRequest::new(job.text.clone())
        .id(job.prompt_id.clone())
        .delivery(SessionInboxDelivery::Steer)
        .resume(true);
    let admitted = observer
        .during(deadline, client.sessions().prompt(&job.session_id, &prompt))
        .await
        .map_err(|problem| Failure {
            stage: Stage::Admission,
            problem,
        })?;
    if admitted.data.id != job.prompt_id || admitted.data.session_id != job.session_id {
        return Err(Failure {
            stage: Stage::Admission,
            problem: Problem::UnexpectedIdentity,
        });
    }
    (observer.notice)(Notice::Admitted);
    (observer.notice)(Notice::Waiting);
    let wait_options =
        SessionWaitOptions::default().timeout(deadline.saturating_duration_since(Instant::now()));
    let wait_result = observer
        .during(
            deadline,
            client
                .sessions()
                .experimental_wait(&job.session_id, &wait_options),
        )
        .await;
    let (wait, wait_error) = match wait_result {
        Ok(()) => (WaitState::Idle, None),
        Err(Problem::Deadline) => (WaitState::TimedOut, Some(Problem::Deadline)),
        Err(Problem::Api(error)) if error.is_timeout() => {
            (WaitState::TimedOut, Some(Problem::Deadline))
        }
        Err(error) => (WaitState::Failed, Some(error)),
    };
    // Inspect once even if waiting failed/timed out. Separate requests are not
    // atomic; a successful fetch is evidence, not a permanent-idle guarantee.
    let inspected = observer
        .during(
            Instant::now() + limits.inspection,
            Inspection::load(client, &job.session_id, limits.max_pages),
        )
        .await;
    let (inspection, inspection_error) = match inspected {
        Ok(view) if view.session.id == job.session_id => (Some(view), None),
        Ok(_) => (None, Some(Problem::UnexpectedIdentity)),
        Err(error) => (None, Some(error)),
    };
    let outcome = inspection.as_ref().map_or(Outcome::Inconclusive, |view| {
        assess(view, &job.prompt_id, wait)
    });
    // Close this observation explicitly; "stopped" is not a continuity claim.
    if let Some(mut stream) = observer.stream.take() {
        stream.close();
        observer.trace.feed = FeedState::Stopped;
    }
    Ok(Report {
        outcome,
        wait,
        wait_error,
        inspection,
        inspection_error,
        trace: observer.trace,
    })
}
