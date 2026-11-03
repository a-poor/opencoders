#[path = "../examples/support/session_observer.rs"]
mod observer;

use observer::{Connection, PREVIEW_CHARS, Phase, State, StopReason};
use opencoders::{Client, models::SessionId};
use serde_json::{Value, json};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{Semaphore, mpsc, oneshot},
    task::{JoinHandle, JoinSet},
    time::timeout,
};

const ID: &str = "ses_observe";
const DEADLINE: Duration = Duration::from_secs(5);
fn id() -> SessionId {
    ID.parse().unwrap()
}
fn delta(session: &str, text: &str) -> Value {
    json!({"id":"evt_delta", "created":1, "type":"session.text.delta",
        "data":{"sessionID":session,"assistantMessageID":"msg_reply","ordinal":0,"delta":text}})
}

struct Feed {
    frames: mpsc::Sender<String>,
    closed: oneshot::Receiver<()>,
}
impl Feed {
    async fn send(&self, value: Value) {
        self.frames
            .send(format!("data: {value}\n\n"))
            .await
            .unwrap();
    }
    async fn closed(self) {
        timeout(DEADLINE, self.closed).await.unwrap().unwrap();
    }
    async fn eof(self) {
        drop(self.frames);
        timeout(DEADLINE, self.closed).await.unwrap().unwrap();
    }
}

#[derive(Default)]
struct Data {
    revision: usize,
    requests: Vec<String>,
    fail_forms: bool,
    session_gate: Option<Arc<Semaphore>>,
}
struct Fixture {
    client: Client,
    data: Arc<Mutex<Data>>,
    feeds: mpsc::Receiver<Feed>,
    task: JoinHandle<()>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Fixture {
    async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = Client::new(format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let data = Arc::new(Mutex::new(Data::default()));
        let state = data.clone();
        let (tx, feeds) = mpsc::channel(8);
        let task = tokio::spawn(async move {
            let mut tasks = JoinSet::new();
            loop {
                tokio::select! {
                    accepted = listener.accept() => {
                        let (socket, _) = accepted.unwrap();
                        tasks.spawn(serve(socket, state.clone(), tx.clone()));
                    }
                    Some(result) = tasks.join_next() => { result.unwrap(); }
                }
            }
        });
        Self {
            client,
            data,
            feeds,
            task,
        }
    }
    async fn open(&mut self, capacity: usize) -> (Connection, Feed) {
        let connection = Connection::open(&self.client, &id(), capacity)
            .await
            .unwrap();
        let feed = timeout(DEADLINE, self.feeds.recv()).await.unwrap().unwrap();
        (connection, feed)
    }
    fn advance(&self) {
        self.data.lock().unwrap().revision += 1;
    }
}
async fn serve(mut socket: TcpStream, state: Arc<Mutex<Data>>, feeds: mpsc::Sender<Feed>) {
    let mut request = Vec::new();
    while !request.ends_with(b"\r\n\r\n") {
        let Ok(byte) = socket.read_u8().await else {
            return;
        };
        request.push(byte);
        assert!(request.len() < 8192);
    }
    let request = String::from_utf8(request).unwrap();
    let target = request.split_whitespace().nth(1).unwrap();
    let (body, status, gate) = {
        let mut data = state.lock().unwrap();
        data.requests.push(request.clone());
        let route = target.split('?').next().unwrap();
        let revision = data.revision;
        let mut session: Value =
            serde_json::from_str(include_str!("fixtures/session.json")).unwrap();
        session["id"] = json!(ID);
        session["title"] = json!(format!("revision {revision}"));
        let populated = revision > 0;
        let body = match route {
            "/api/event" => Value::Null,
            "/api/session/ses_observe" => json!({"data":session}),
            "/api/session/ses_observe/message" => {
                json!({"data":[{"id":"msg_snapshot","type":"user","time":{"created":1},"text":format!("history {revision}")}],"cursor":{"next":"opaque-more"}})
            }
            "/api/session/ses_observe/inbox" => {
                json!({"data": if populated {vec![json!({"id":"msg_pending","sessionID":ID,"time":{"created":1},"type":"user","delivery":"queue","payload":{"text":"pending"}})]}else{vec![]}})
            }
            "/api/session/ses_observe/permission" => {
                json!({"data": if populated {vec![json!({"id":"per_pending","sessionID":ID,"action":"read","resources":[]})]}else{vec![]}})
            }
            "/api/session/ses_observe/form" => {
                json!({"data": if populated {vec![json!({"id":"frm__pending","sessionID":ID,"title":"Pending","fields":[{"key":"ok","type":"boolean"}]})]}else{vec![]}})
            }
            _ => panic!("unexpected request: {target}"),
        };
        let status = if data.fail_forms && route.ends_with("/form") {
            500
        } else {
            200
        };
        let gate = if route == "/api/session/ses_observe" {
            data.session_gate.clone()
        } else {
            None
        };
        (body, status, gate)
    };
    if target == "/api/event" {
        socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n").await.unwrap();
        let (tx, mut frames) = mpsc::channel::<String>(8);
        let (closed, rx) = oneshot::channel();
        feeds
            .send(Feed {
                frames: tx,
                closed: rx,
            })
            .await
            .unwrap();
        loop {
            tokio::select! {
                frame = frames.recv() => {
                    let Some(frame) = frame else {
                        let _ = socket.write_all(b"0\r\n\r\n").await;
                        break;
                    };
                    if socket.write_all(format!("{:x}\r\n{frame}\r\n",frame.len()).as_bytes()).await.is_err() {break;}
                }
                _ = socket.read_u8() => break,
            }
        }
        let _ = closed.send(());
    } else {
        if let Some(gate) = gate {
            let _permit = gate.acquire().await.unwrap();
        }
        let body = body.to_string();
        let response = format!(
            "HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = socket.write_all(response.as_bytes()).await;
    }
}
async fn buffered(connection: &Connection, count: usize) {
    timeout(DEADLINE, async {
        while connection.buffered_events() < count {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn subscribes_before_loading_bounded_history_and_all_pending_snapshots() {
    let mut fixture = Fixture::start().await;
    fixture.advance();
    let (mut connection, feed) = fixture.open(8).await;
    let mut state = State::default();
    timeout(DEADLINE, connection.refresh(&mut state))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.phase, Phase::Observing);
    assert!(!state.needs_refresh);
    let snapshot = state.snapshot.as_ref().unwrap();
    assert_eq!(snapshot.session.id, id());
    assert_eq!(snapshot.history.cursor.next.as_deref(), Some("opaque-more"));
    assert_eq!(snapshot.inbox.data.len(), 1);
    assert_eq!(snapshot.permissions.data.len(), 1);
    assert_eq!(snapshot.forms.data.len(), 1);
    let requests = fixture.data.lock().unwrap().requests.clone();
    assert!(requests[0].starts_with("GET /api/event "));
    assert_eq!(requests.len(), 6);
    let history = requests.iter().find(|r| r.contains("/message?")).unwrap();
    assert!(history.contains("limit=50") && history.contains("order=desc"));
    assert!(requests.iter().all(|r| r.starts_with("GET ")));
    connection.close().await;
    state.stopped();
    assert_eq!(state.phase, Phase::Stopped);
    feed.closed().await;
}

#[tokio::test]
async fn explicit_reconnect_refreshes_missed_changes_without_replay_or_mutations() {
    let mut fixture = Fixture::start().await;
    let (mut connection, feed) = fixture.open(8).await;
    let mut state = State::default();
    connection.refresh(&mut state).await.unwrap();
    feed.eof().await;
    let end = timeout(DEADLINE, connection.next(&mut state))
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(end.reason, StopReason::Eof);
    assert_eq!(state.phase, Phase::Stale(StopReason::Eof));
    assert_eq!(
        state.snapshot.as_ref().unwrap().session.title.as_deref(),
        Some("revision 0")
    );
    fixture.advance(); // A change happens with no observer and no retained event.
    state.connecting();
    let (mut reopened, feed) = fixture.open(8).await;
    reopened.refresh(&mut state).await.unwrap();
    let snapshot = state.snapshot.as_ref().unwrap();
    assert_eq!(snapshot.session.title.as_deref(), Some("revision 1"));
    assert_eq!(
        serde_json::to_value(&snapshot.history.data[0]).unwrap()["text"],
        "history 1"
    );
    assert_eq!(snapshot.inbox.data.len(), 1);
    assert_eq!(snapshot.permissions.data.len(), 1);
    assert_eq!(snapshot.forms.data.len(), 1);
    let requests = fixture.data.lock().unwrap().requests.clone();
    assert_eq!(
        requests
            .iter()
            .filter(|r| r.starts_with("GET /api/event "))
            .count(),
        2
    );
    assert!(
        requests
            .iter()
            .all(|r| r.starts_with("GET ") && !r.to_lowercase().contains("last-event-id"))
    );
    reopened.close().await;
    feed.closed().await;
}

#[tokio::test]
async fn snapshot_overlap_discards_old_previews_and_requires_another_refresh() {
    let mut fixture = Fixture::start().await;
    let (mut connection, feed) = fixture.open(8).await;
    let mut state = State::default();
    connection.refresh(&mut state).await.unwrap();
    feed.send(delta(ID, "already included by the newer snapshot"))
        .await;
    buffered(&connection, 1).await;
    fixture.advance();
    connection.refresh(&mut state).await.unwrap();
    assert!(state.needs_refresh); // Buffered invalidations are not lost.
    assert!(state.preview.is_none()); // Never append overlapping text to history.
    assert_eq!(
        state.snapshot.as_ref().unwrap().session.title.as_deref(),
        Some("revision 1")
    );
    connection.refresh(&mut state).await.unwrap();
    assert!(!state.needs_refresh);
    connection.close().await;
    feed.closed().await;
}

#[tokio::test]
async fn text_previews_are_bounded_scoped_and_separate_from_snapshot_history() {
    let mut fixture = Fixture::start().await;
    let (mut connection, feed) = fixture.open(8).await;
    let mut state = State::default();
    connection.refresh(&mut state).await.unwrap();
    feed.send(delta("ses_other", "unrelated")).await;
    feed.send(delta(ID, &"雪".repeat(PREVIEW_CHARS + 20))).await;
    timeout(DEADLINE, connection.next(&mut state))
        .await
        .unwrap()
        .unwrap();
    let preview = state.preview.as_ref().unwrap();
    assert_eq!(preview.message_id.as_str(), "msg_reply");
    assert_eq!(preview.text.chars().count(), PREVIEW_CHARS);
    assert!(preview.truncated);
    assert!(state.needs_refresh);
    feed.send(delta(ID, "next fragment")).await;
    connection.next(&mut state).await.unwrap();
    assert_eq!(state.preview.as_ref().unwrap().text, "next fragment");
    assert_eq!(state.snapshot.as_ref().unwrap().history.data.len(), 1);
    feed.send(json!({"type":"future.private","secret":"must not be retained as state"}))
        .await;
    connection.next(&mut state).await.unwrap();
    assert_eq!(state.unknown_events, 1);
    assert!(state.needs_refresh);
    connection.close().await;
    feed.closed().await;
}

#[tokio::test]
async fn full_queue_reports_lag_outside_the_queue_and_discards_buffered_deltas() {
    let mut fixture = Fixture::start().await;
    let (mut connection, feed) = fixture.open(1).await;
    let mut state = State::default();
    connection.refresh(&mut state).await.unwrap();
    feed.send(delta(ID, "first")).await;
    buffered(&connection, 1).await;
    feed.send(delta(ID, "cannot fit")).await;
    feed.closed().await; // Reader exits instead of waiting on the full queue.
    let failure = connection.next(&mut state).await.unwrap_err();
    assert_eq!(failure.reason, StopReason::Lagged);
    assert_eq!(state.phase, Phase::Stale(StopReason::Lagged));
    assert!(state.needs_refresh && state.preview.is_none());
    assert_eq!(connection.buffered_events(), 0);
}

#[tokio::test]
async fn stream_and_snapshot_failures_leave_last_successful_snapshot_stale() {
    let mut fixture = Fixture::start().await;
    let (mut connection, feed) = fixture.open(8).await;
    let mut state = State::default();
    connection.refresh(&mut state).await.unwrap();
    feed.send(json!({"type":"session.text.delta","private":"secret"}))
        .await;
    let failure = timeout(DEADLINE, connection.next(&mut state))
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(failure.reason, StopReason::Stream);
    assert!(failure.error.is_some());
    assert!(!format!("{failure}").contains("secret"));
    feed.closed().await;
    fixture.advance();
    fixture.data.lock().unwrap().fail_forms = true;
    let (mut connection, feed) = fixture.open(8).await;
    let failure = connection.refresh(&mut state).await.unwrap_err();
    assert_eq!(failure.reason, StopReason::Snapshot);
    assert_eq!(state.phase, Phase::Stale(StopReason::Snapshot));
    assert_eq!(
        state.snapshot.as_ref().unwrap().session.title.as_deref(),
        Some("revision 0")
    );
    feed.closed().await;
}

#[tokio::test]
async fn cancelled_reads_preserve_observation_and_shutdown_is_independent() {
    let mut fixture = Fixture::start().await;
    let (mut first, feed1) = fixture.open(8).await;
    let (mut second, feed2) = fixture.open(8).await;
    let mut state = State::default();
    second.refresh(&mut state).await.unwrap();
    assert!(
        timeout(Duration::from_millis(20), second.next(&mut state))
            .await
            .is_err()
    );
    first.close().await;
    first.close().await; // Joining/closing is idempotent.
    feed1.closed().await;
    feed2.send(delta(ID, "still observing")).await;
    timeout(DEADLINE, second.next(&mut state))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.preview.as_ref().unwrap().text, "still observing");
    drop(second); // Fallback abort must not detach a reader task.
    feed2.closed().await;
}

#[tokio::test]
async fn cancelling_a_refresh_keeps_old_state_and_exposes_incomplete_refresh() {
    let mut fixture = Fixture::start().await;
    let (mut connection, feed) = fixture.open(8).await;
    let mut state = State::default();
    connection.refresh(&mut state).await.unwrap();
    fixture.advance();
    let gate = Arc::new(Semaphore::new(0));
    fixture.data.lock().unwrap().session_gate = Some(gate.clone());
    assert!(
        timeout(Duration::from_millis(30), connection.refresh(&mut state))
            .await
            .is_err()
    );
    assert_eq!(state.phase, Phase::Refreshing);
    assert!(state.needs_refresh);
    assert_eq!(
        state.snapshot.as_ref().unwrap().session.title.as_deref(),
        Some("revision 0")
    );
    connection.close().await;
    state.stopped();
    feed.closed().await;
    gate.add_permits(1);
}

#[tokio::test]
async fn lag_interrupts_an_in_flight_refresh_without_publishing_partial_state() {
    let mut fixture = Fixture::start().await;
    let (mut connection, feed) = fixture.open(1).await;
    let mut state = State::default();
    connection.refresh(&mut state).await.unwrap();
    fixture.advance();
    let gate = Arc::new(Semaphore::new(0));
    fixture.data.lock().unwrap().session_gate = Some(gate.clone());
    let failure = {
        let refreshing = connection.refresh(&mut state);
        tokio::pin!(refreshing);
        // Wait for the second session GET to block on the gate, rather than
        // guessing how long snapshot loading or event parsing should take.
        let started = async {
            loop {
                let count = fixture
                    .data
                    .lock()
                    .unwrap()
                    .requests
                    .iter()
                    .filter(|r| r.starts_with("GET /api/session/ses_observe HTTP"))
                    .count();
                if count == 2 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        };
        tokio::select! {
            result = &mut refreshing => panic!("refresh completed before gate: {result:?}"),
            result = timeout(DEADLINE, started) => { result.unwrap(); }
        }
        feed.send(delta(ID, "first")).await;
        feed.send(delta(ID, "overflow")).await;
        timeout(DEADLINE, &mut refreshing)
            .await
            .unwrap()
            .unwrap_err()
    };
    assert_eq!(failure.reason, StopReason::Lagged);
    assert_eq!(state.phase, Phase::Stale(StopReason::Lagged));
    assert_eq!(
        state.snapshot.as_ref().unwrap().session.title.as_deref(),
        Some("revision 0")
    );
    assert!(state.preview.is_none() && state.needs_refresh);
    feed.closed().await;
    gate.add_permits(1);
}
