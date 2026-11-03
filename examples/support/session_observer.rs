//! Application-owned observation policy shared by the example and its tests.
//! This is intentionally not a public SDK state-store or reconnect abstraction.
use opencoders::{
    Client, EventOptions, EventStream, MessageListOptions, MessageOrder,
    models::{
        Event, SessionFormListResponse, SessionId, SessionInboxResponse, SessionInfo,
        SessionMessageId, SessionMessagesResponse, SessionPermissionListResponse,
    },
};
use std::{fmt, time::Duration};
use tokio::{sync::mpsc, task::JoinHandle};

pub const PREVIEW_CHARS: usize = 256;

/// One recent history page, not a complete transcript. Each resource is fetched
/// independently: neither the bundle nor its relationship to events is atomic.
pub struct Snapshot {
    pub session: SessionInfo,
    pub history: SessionMessagesResponse,
    pub inbox: SessionInboxResponse,
    pub permissions: SessionPermissionListResponse,
    pub forms: SessionFormListResponse,
}
impl Snapshot {
    async fn load(client: &Client, id: &SessionId) -> opencoders::Result<Self> {
        let sessions = client.sessions();
        let messages = sessions.messages(id);
        let inbox = sessions.inbox(id);
        let permissions = sessions.permissions(id);
        let forms = sessions.forms(id);
        let history_options = MessageListOptions::default()
            .limit(50)
            .order(MessageOrder::Desc);
        let (session, history, inbox, permissions, forms) = tokio::try_join!(
            sessions.get(id),
            messages.list(&history_options),
            inbox.list(),
            permissions.list(),
            forms.list(),
        )?;
        Ok(Self {
            session: session.data,
            history,
            inbox,
            permissions,
            forms,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopReason {
    Eof,
    Lagged,
    Stream,
    Snapshot,
    ReaderStopped,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Connecting,
    Refreshing,
    Observing,
    Stale(StopReason),
    Stopped,
}

/// Only the latest observed fragment, never appended to snapshot history. It may
/// overlap stored text, be duplicated, or miss fragments; it is not a transcript.
pub struct Preview {
    pub message_id: SessionMessageId,
    pub text: String,
    pub truncated: bool,
}

pub struct State {
    pub phase: Phase,
    pub snapshot: Option<Snapshot>,
    pub preview: Option<Preview>,
    pub needs_refresh: bool,
    pub unknown_events: u64,
}
impl Default for State {
    fn default() -> Self {
        Self {
            phase: Phase::Connecting,
            snapshot: None,
            preview: None,
            needs_refresh: true,
            unknown_events: 0,
        }
    }
}
impl State {
    pub fn connecting(&mut self) {
        self.phase = Phase::Connecting;
        self.needs_refresh = true;
        self.preview = None;
    }
    pub fn stopped(&mut self) {
        self.phase = Phase::Stopped;
        self.needs_refresh = true;
        self.preview = None;
    }
    fn invalidate(&mut self, event: Event, allow_preview: bool) {
        self.needs_refresh = true;
        match event {
            Event::SessionTextDelta(event) if allow_preview => {
                self.preview = Some(Preview {
                    message_id: event.data.assistant_message_id,
                    truncated: event.data.delta.chars().count() > PREVIEW_CHARS,
                    text: event.data.delta.chars().take(PREVIEW_CHARS).collect(),
                });
            }
            Event::Unknown(_) => self.unknown_events = self.unknown_events.saturating_add(1),
            _ => {} // Known but unhandled events also invalidate the snapshots.
        }
    }
}

#[derive(Debug)]
pub struct Failure {
    pub reason: StopReason,
    pub error: Option<opencoders::Error>,
}
impl Failure {
    fn new(reason: StopReason) -> Self {
        Self {
            reason,
            error: None,
        }
    }
    fn with_error(reason: StopReason, error: opencoders::Error) -> Self {
        Self {
            reason,
            error: Some(error),
        }
    }
}
impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.reason)?;
        if let Some(error) = &self.error {
            write!(f, ": {error}")?;
        }
        Ok(())
    }
}
impl std::error::Error for Failure {}

/// Explicitly scoped kinds avoid filling the queue with other sessions' traffic.
/// Unknown/unhandled transient kinds conservatively invalidate the selected
/// snapshots: their schema cannot safely be assumed to contain a session ID.
fn relevant(event: &Event, id: &SessionId) -> bool {
    let session = match event {
        Event::ServerConnected(_) => return false,
        Event::SessionTextDelta(e) => Some(e.data.session_id.as_str()),
        Event::SessionStatus(e) => Some(e.data.session_id.as_str()),
        Event::PermissionAsked(e) => Some(e.data.session_id.as_str()),
        Event::PermissionReplied(e) => Some(e.data.session_id.as_str()),
        Event::FormCreated(e) => Some(e.data.form.session_id.as_str()),
        Event::FormReplied(e) => Some(e.data.session_id.as_str()),
        Event::FormCancelled(e) => Some(e.data.session_id.as_str()),
        _ => event.durable_aggregate_id(),
    };
    session.is_none_or(|session| session == id.as_str())
}

async fn read(mut stream: EventStream, id: SessionId, tx: mpsc::Sender<Event>) -> Failure {
    while let Some(message) = stream.next().await {
        let event = match message {
            Ok(message) => message.event, // SSE IDs/retry hints are not recovery cursors.
            Err(error) => return Failure::with_error(StopReason::Stream, error),
        };
        if !relevant(&event, &id) {
            continue;
        }
        match tx.try_send(event) {
            Ok(()) => {}
            Err(mpsc::error::TrySendError::Full(_)) => return Failure::new(StopReason::Lagged),
            Err(mpsc::error::TrySendError::Closed(_)) => {
                return Failure::new(StopReason::ReaderStopped);
            }
        }
    }
    Failure::new(StopReason::Eof)
}

async fn reader_end(reader: &mut Option<JoinHandle<Failure>>) -> Failure {
    let Some(task) = reader.as_mut() else {
        return Failure::new(StopReason::ReaderStopped);
    };
    let result = task.await;
    reader.take(); // Never poll a completed JoinHandle a second time.
    result.unwrap_or_else(|_| Failure::new(StopReason::ReaderStopped))
}

/// One explicitly opened subscription and its bounded background reader. End
/// reasons travel via the JoinHandle, so a full event queue cannot hide lag/EOF.
/// The owning application decides when to open a new connection.
pub struct Connection {
    client: Client,
    id: SessionId,
    capacity: usize,
    events: mpsc::Receiver<Event>,
    reader: Option<JoinHandle<Failure>>,
}
impl Connection {
    /// `capacity` must be nonzero. The example uses 32 frames of at most 64 KiB.
    /// These bounds do not include HTTP chunk allocation or Rust object overhead.
    pub async fn open(
        client: &Client,
        id: &SessionId,
        capacity: usize,
    ) -> opencoders::Result<Self> {
        assert!(capacity > 0);
        let options = EventOptions::default()
            .max_event_bytes(64 * 1024)
            .handshake_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(45));
        let stream = client.events().subscribe(&options).await?;
        let (tx, events) = mpsc::channel(capacity);
        let reader = tokio::spawn(read(stream, id.clone(), tx));
        Ok(Self {
            client: client.clone(),
            id: id.clone(),
            capacity,
            events,
            reader: Some(reader),
        })
    }
    pub fn buffered_events(&self) -> usize {
        self.events.len()
    }

    pub async fn refresh(&mut self, state: &mut State) -> Result<(), Failure> {
        state.phase = Phase::Refreshing;
        state.needs_refresh = true;
        state.preview = None;
        let result = tokio::select! {
            biased;
            end = reader_end(&mut self.reader) => Err(end),
            snapshot = Snapshot::load(&self.client, &self.id) =>
                snapshot.map_err(|error| Failure::with_error(StopReason::Snapshot, error)),
        };
        let snapshot = match result {
            Ok(snapshot) => snapshot,
            Err(end) => return self.fail(state, end).await,
        };
        // Publish the bundle only after every GET succeeds. Preserve the previous
        // bundle on failure/cancellation; individual GETs still are not atomic.
        state.snapshot = Some(snapshot);
        state.needs_refresh = false;
        // Events queued before/during the reads are invalidations, not patches.
        // Discard their previews and schedule another refresh without spinning
        // indefinitely under continuous traffic. The caller also refreshes on a timer.
        for _ in 0..self.capacity {
            match self.events.try_recv() {
                Ok(event) => state.invalidate(event, false),
                Err(mpsc::error::TryRecvError::Empty) => break,
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    let end = reader_end(&mut self.reader).await;
                    return self.fail(state, end).await;
                }
            }
        }
        if self
            .reader
            .as_ref()
            .is_some_and(|reader| reader.is_finished())
        {
            let end = reader_end(&mut self.reader).await;
            return self.fail(state, end).await;
        }
        state.phase = Phase::Observing; // Best effort, not an assertion of consistency.
        Ok(())
    }

    pub async fn next(&mut self, state: &mut State) -> Result<(), Failure> {
        let result = tokio::select! {
            biased;
            end = reader_end(&mut self.reader) => Err(end),
            event = self.events.recv() => match event {
                Some(event) => Ok(event),
                None => Err(reader_end(&mut self.reader).await),
            },
        };
        match result {
            Ok(event) => {
                state.invalidate(event, true);
                Ok(())
            }
            Err(end) => self.fail(state, end).await,
        }
    }

    async fn fail(&mut self, state: &mut State, end: Failure) -> Result<(), Failure> {
        state.phase = Phase::Stale(end.reason);
        state.needs_refresh = true;
        state.preview = None;
        self.close().await;
        Err(end)
    }

    /// Cancel and join the reader; drop queued events from this connection.
    /// This never interrupts server work. Call even after a cancelled refresh.
    pub async fn close(&mut self) {
        self.events.close();
        if let Some(reader) = self.reader.take() {
            reader.abort();
            let _ = reader.await;
        }
        while self.events.try_recv().is_ok() {}
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        // Dropping a Tokio JoinHandle alone would detach it. Explicit close joins;
        // cancellation/unwinding still aborts the reader through this fallback.
        if let Some(reader) = &self.reader {
            reader.abort();
        }
    }
}
