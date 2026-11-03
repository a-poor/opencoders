//! Real-server recovery evidence using the example's application-owned observer.
#[path = "../../examples/support/session_observer.rs"]
mod observer;
#[path = "recovery_proxy.rs"]
mod proxy;

use super::{LocalProvider, assistant_text, required, still_running, until};
use observer::{Connection, Phase, State, StopReason};
use opencoders::{Client, EventOptions, EventStream, SessionWaitOptions, models::*};
use proxy::Relay;
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;

const DEADLINE: Duration = Duration::from_secs(10);

fn client(relay: &Relay) -> Client {
    Client::builder(relay.url())
        .basic_auth("opencode", required("OPENCODERS_LIVE_PASSWORD"))
        .http_client(
            reqwest::Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .retry(reqwest::retry::never())
                .connect_timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        )
        .request_timeout(DEADLINE)
        .max_response_bytes(1024 * 1024)
        .build()
        .unwrap()
}

async fn create(client: &Client, workspace: &str, scenario: &str) -> SessionId {
    client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .title(format!("active fixture {scenario}"))
                .location(SessionLocationInput::new(workspace))
                .agent("fixture".parse().unwrap())
                .model(SessionModelInput::new("fixture".parse().unwrap(), scenario)),
        )
        .await
        .unwrap()
        .data
        .id
}

async fn witness(client: &Client) -> EventStream {
    let mut stream = client
        .events()
        .subscribe(&EventOptions::default().types([
            "session.text.delta",
            "session.execution.succeeded",
            "session.execution.failed",
            "form.created",
        ]))
        .await
        .unwrap();
    until(&mut stream, "server.connected").await;
    stream
}

async fn prompt(client: &Client, id: &SessionId, scenario: &str) -> SessionMessageId {
    let prompt_id: SessionMessageId = format!("msg_active_{scenario}").parse().unwrap();
    client
        .sessions()
        .prompt(
            id,
            &SessionPromptRequest::new(format!("Local {scenario} fixture"))
                .id(prompt_id.clone())
                .resume(true),
        )
        .await
        .unwrap();
    prompt_id
}

async fn refresh(connection: &mut Connection, state: &mut State) {
    timeout(DEADLINE, connection.refresh(state))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.phase, Phase::Observing);
    assert!(state.preview.is_none());
}

async fn cut(relay: &Relay, connection: &mut Connection, state: &mut State) {
    let before = serde_json::to_value(&state.snapshot.as_ref().unwrap().history).unwrap();
    assert_eq!(relay.cut_events().await, 1);
    let failure = timeout(DEADLINE, async {
        loop {
            if let Err(failure) = connection.next(state).await {
                break failure;
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(failure.reason, StopReason::Stream);
    assert!(failure.error.is_some());
    assert_eq!(state.phase, Phase::Stale(StopReason::Stream));
    assert!(state.needs_refresh && state.preview.is_none());
    assert_eq!(connection.buffered_events(), 0);
    assert_eq!(
        serde_json::to_value(&state.snapshot.as_ref().unwrap().history).unwrap(),
        before
    );
}

async fn reopen(client: &Client, id: &SessionId, state: &mut State) -> Connection {
    state.connecting();
    let mut connection = Connection::open(client, id, 32).await.unwrap();
    refresh(&mut connection, state).await;
    connection
}

async fn quiet(connection: &mut Connection, state: &mut State) {
    // With the provider held and no other writers, reconnection supplies no old
    // transient deltas or form/terminal events in this observation window.
    assert!(
        timeout(Duration::from_millis(150), connection.next(state))
            .await
            .is_err()
    );
    assert!(state.preview.is_none());
}

fn read_only_requests(relay: &Relay, opens: usize) {
    let requests = relay.requests();
    assert!(
        requests
            .iter()
            .all(|request| request.method == "GET" && !request.last_event_id)
    );
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.target == "/api/event")
            .count(),
        opens
    );
    assert!(requests.iter().any(
        |request| request.target.contains("limit=50") && request.target.contains("order=desc")
    ));
}

async fn settled(client: &Client, id: &SessionId) {
    client
        .sessions()
        .experimental_wait(id, &SessionWaitOptions::default().timeout(DEADLINE))
        .await
        .unwrap();
}

async fn converge(
    connection: &mut Connection,
    state: &mut State,
    id: &SessionId,
    prompt: &SessionMessageId,
    text: &str,
) {
    // Bounded application policy: separate GETs can overlap events. After the
    // controlled execution has settled, repeat a dirty refresh at most five times.
    for _ in 0..5 {
        refresh(connection, state).await;
        if !state.needs_refresh {
            break;
        }
    }
    assert!(!state.needs_refresh);
    let snapshot = state.snapshot.as_ref().unwrap();
    assert_eq!(snapshot.session.id, *id);
    assert_eq!(
        snapshot.session.outcome,
        Some(SessionInfoOutcome::Succeeded)
    );
    assert_eq!(assistant_text(&snapshot.history.data), text);
    assert!(
        snapshot
            .history
            .data
            .iter()
            .any(|m| matches!(m, SessionMessageInfo::User(user) if user.id == *prompt))
    );
    let Some(SessionMessageInfo::Idle(idle)) = snapshot.history.data.first() else {
        panic!("newest history is idle")
    };
    assert_eq!(idle.outcome, SessionMessageIdleOutcome::Succeeded);
    assert!(snapshot.history.data.len() < 50);
    assert!(
        snapshot.inbox.data.is_empty()
            && snapshot.permissions.data.is_empty()
            && snapshot.forms.data.is_empty()
    );
}

pub async fn verify(client: &Client, provider: &LocalProvider, workspace: &str) {
    streaming(client, provider, workspace).await;
    question(client, workspace).await;
}

async fn streaming(direct: &Client, provider: &LocalProvider, workspace: &str) {
    println!("active scenario: recovery (transport cut and overlapping snapshot)");
    let id = create(direct, workspace, "recovery").await;
    let relay = Relay::start(direct.base_url().as_str()).await;
    let observer_client = client(&relay);
    let mut state = State::default();
    let mut connection = reopen(&observer_client, &id, &mut state).await;
    let mut feed = witness(direct).await;
    let prompt_id = prompt(direct, &id, "recovery").await;
    until(&mut feed, "session.text.delta").await;
    timeout(DEADLINE, async {
        while state
            .preview
            .as_ref()
            .is_none_or(|preview| preview.text != "before ")
        {
            connection.next(&mut state).await.unwrap_or_else(|failure| {
                if let Some(opencoders::Error::Decode(error)) = &failure.error {
                    let body: serde_json::Value = serde_json::from_slice(error.body()).unwrap();
                    panic!(
                        "fixture event {} failed to decode: {}",
                        body["type"],
                        std::error::Error::source(error).unwrap()
                    );
                }
                panic!("{failure:?}");
            });
        }
    })
    .await
    .unwrap();
    let message_id = state.preview.as_ref().unwrap().message_id.clone();
    assert!(!state.preview.as_ref().unwrap().truncated);
    cut(&relay, &mut connection, &mut state).await;
    read_only_requests(&relay, 1); // The observer did not reconnect on its own.
    still_running(direct, &id).await;

    provider.release("recovery/missed").await;
    let missed = until(&mut feed, "session.text.delta").await;
    let Event::SessionTextDelta(delta) = missed.last().unwrap() else {
        unreachable!()
    };
    assert_eq!(delta.data.delta, "missed ");
    assert_eq!(delta.data.assistant_message_id, message_id);
    // Execution continues while the UI has no feed. The direct witness is only a
    // test barrier; its event data never enters the recovering observer's state.
    let mut connection = reopen(&observer_client, &id, &mut state).await;
    let snapshot = state.snapshot.as_ref().unwrap();
    assert!(snapshot.history.data.iter().any(|m| m.id() == &message_id));
    assert_eq!(assistant_text(&snapshot.history.data), "");
    assert!(snapshot.session.outcome.is_none());
    quiet(&mut connection, &mut state).await;
    read_only_requests(&relay, 2);

    provider.release("recovery/after").await;
    let after = until(&mut feed, "session.text.delta").await;
    let Event::SessionTextDelta(delta) = after.last().unwrap() else {
        unreachable!()
    };
    assert_eq!(delta.data.delta, "after");
    // Leave the newly delivered delta queued so refresh must invalidate without
    // patching an overlapping (and in this case still text-empty) history response.
    timeout(DEADLINE, async {
        while connection.buffered_events() == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    let mut hold = relay.hold_history();
    {
        let refreshing = connection.refresh(&mut state);
        tokio::pin!(refreshing);
        tokio::select! {
            result = &mut refreshing => panic!("snapshot escaped gate: {result:?}"),
            _ = hold.captured() => {},
        }
        provider.release("recovery/end").await;
        until(&mut feed, "session.execution.succeeded").await;
        settled(direct, &id).await;
        hold.release();
        timeout(DEADLINE, &mut refreshing).await.unwrap().unwrap();
    }
    assert!(state.needs_refresh && state.preview.is_none());
    assert_eq!(
        assistant_text(&state.snapshot.as_ref().unwrap().history.data),
        ""
    );
    converge(
        &mut connection,
        &mut state,
        &id,
        &prompt_id,
        "before missed after",
    )
    .await;
    assert_eq!(provider.state().await["requests"]["recovery"], 1);
    read_only_requests(&relay, 2);
    connection.close().await;
    state.stopped();
    assert_eq!(state.phase, Phase::Stopped);
    relay.close().await;
    drop(feed);
    direct.sessions().remove(&id).await.unwrap();
}

async fn question(direct: &Client, workspace: &str) {
    println!("active scenario: recovery_question (missed creation and settlement)");
    let id = create(direct, workspace, "recovery_question").await;
    let relay = Relay::start(direct.base_url().as_str()).await;
    let observer_client = client(&relay);
    let mut state = State::default();
    let mut connection = reopen(&observer_client, &id, &mut state).await;
    let mut feed = witness(direct).await;
    cut(&relay, &mut connection, &mut state).await;
    let prompt_id = prompt(direct, &id, "recovery_question").await;
    until(&mut feed, "form.created").await;
    assert!(state.snapshot.as_ref().unwrap().forms.data.is_empty());
    let mut connection = reopen(&observer_client, &id, &mut state).await;
    let snapshot = state.snapshot.as_ref().unwrap();
    assert_eq!(snapshot.forms.data.len(), 1);
    let form = snapshot.forms.data[0].clone();
    assert_eq!(
        form.metadata.as_ref().unwrap()["tool"]["id"],
        "call_recovery_question"
    );
    super::assert_running_tool(&snapshot.history.data, "call_recovery_question");
    still_running(direct, &id).await;
    quiet(&mut connection, &mut state).await;
    cut(&relay, &mut connection, &mut state).await;
    read_only_requests(&relay, 2);
    direct
        .sessions()
        .forms(&id)
        .reply(
            &form.id,
            &SessionFormReplyRequest::new(
                serde_json::from_value(json!({"q0":"fixture-answer"})).unwrap(),
            ),
        )
        .await
        .unwrap();
    until(&mut feed, "session.execution.succeeded").await;
    settled(direct, &id).await;
    // This stale form was already answered elsewhere; pending snapshots must be
    // replaced on recovery even if the corresponding reply event was never seen.
    assert_eq!(state.snapshot.as_ref().unwrap().forms.data[0].id, form.id);
    let mut connection = reopen(&observer_client, &id, &mut state).await;
    converge(
        &mut connection,
        &mut state,
        &id,
        &prompt_id,
        "recovery_question resumed",
    )
    .await;
    quiet(&mut connection, &mut state).await;
    read_only_requests(&relay, 3);
    connection.close().await;
    state.stopped();
    relay.close().await;
    drop(feed);
    direct.sessions().remove(&id).await.unwrap();
}
