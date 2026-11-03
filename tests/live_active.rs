//! Opt-in only: `python3 tools/live/active.py` owns both servers and all data.

use opencoders::{
    Client, EventOptions, EventStream, Location, MessageListOptions, MessageOrder,
    SessionFormCancelOptions, SessionInterruptOptions, SessionListOptions, SessionWaitOptions,
    models::*,
};
use serde_json::{Value, json};
use std::{env, path::PathBuf, time::Duration};

fn required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} missing; use python3 tools/live/active.py"))
}

struct LocalProvider {
    http: reqwest::Client,
    origin: String,
    token: String,
}

impl LocalProvider {
    async fn state(&self) -> Value {
        let bytes = self
            .http
            .get(format!("{}/state", self.origin))
            .bearer_auth(&self.token)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .bytes()
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    async fn release_output(&self) {
        self.http
            .post(format!("{}/release/output", self.origin))
            .bearer_auth(&self.token)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
    }
}

async fn until(events: &mut EventStream, kind: &str) -> Vec<Event> {
    tokio::time::timeout(Duration::from_secs(15), async {
        let mut seen = Vec::new();
        loop {
            let event = events
                .next()
                .await
                .expect("event stream closed")
                .unwrap()
                .event;
            let done = event.kind() == kind;
            // A terminal failure must not be hidden by a later timeout.
            assert!(
                !matches!(event, Event::SessionExecutionFailed(_)) || kind == event.kind(),
                "unexpected execution failure: {event:?}"
            );
            seen.push(event);
            assert!(seen.len() <= 100, "unbounded event traffic");
            if done {
                return seen;
            }
        }
    })
    .await
    .unwrap_or_else(|_| panic!("timed out awaiting {kind}"))
}

async fn history(client: &Client, id: &SessionId) -> Vec<SessionMessageInfo> {
    let messages = client
        .sessions()
        .messages(id)
        .list(
            &MessageListOptions::default()
                .order(MessageOrder::Asc)
                .limit(50),
        )
        .await
        .unwrap();
    assert!(messages.data.len() < 50, "fixture exceeded history bound");
    messages.data
}

fn assistant_text(messages: &[SessionMessageInfo]) -> String {
    messages
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
        .collect()
}

async fn still_running(client: &Client, id: &SessionId) {
    let error = client
        .sessions()
        .experimental_wait(
            id,
            &SessionWaitOptions::default().timeout(Duration::from_millis(150)),
        )
        .await
        .expect_err("execution must still be waiting on the gated provider/tool");
    assert!(error.is_timeout(), "expected wait timeout, got {error:?}");
}

#[tokio::test]
#[ignore = "requires isolated pinned server and local provider; run python3 tools/live/active.py"]
async fn pinned_server_active_execution_and_waiting_tools() {
    let root = PathBuf::from(required("OPENCODERS_LIVE_ROOT"))
        .canonicalize()
        .unwrap();
    let workspace = root.join("workspace");
    let url = required("OPENCODERS_LIVE_URL");
    // The feed has no whole-connection timeout; every awaited phase below is bounded.
    let http = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let client = Client::builder(&url)
        .http_client(http.clone())
        .basic_auth("opencode", required("OPENCODERS_LIVE_PASSWORD"))
        .default_location(Location::directory(workspace.to_str().unwrap()))
        .build()
        .unwrap();
    let provider = LocalProvider {
        http: reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap(),
        origin: required("OPENCODERS_FIXTURE_URL"),
        token: required("OPENCODERS_FIXTURE_TOKEN"),
    };
    for origin in [
        client.base_url().clone(),
        provider.origin.parse::<reqwest::Url>().unwrap(),
    ] {
        assert_eq!(origin.scheme(), "http");
        assert_eq!(origin.host_str(), Some("127.0.0.1"));
        assert!(origin.port().is_some());
        assert!(origin.username().is_empty() && origin.password().is_none());
    }
    // Ownership checks precede writes. Do not manually aim the test at an existing server.
    let info = client.server().info().await.unwrap();
    assert_eq!(info.version, required("OPENCODERS_LIVE_VERSION"));
    assert_eq!(
        info.pid,
        required("OPENCODERS_LIVE_PID").parse::<u64>().unwrap()
    );
    assert!(PathBuf::from(info.paths.tmp).starts_with(&root));
    assert!(
        client
            .sessions()
            .list(&SessionListOptions::default())
            .await
            .unwrap()
            .data
            .is_empty()
    );
    assert_eq!(
        provider.state().await,
        json!({"requests": {}, "aborted": [], "errors": []})
    );

    for scenario in [
        "output",
        "interrupt",
        "failure",
        "permission",
        "question",
        "cancel",
    ] {
        println!("active scenario: {scenario}");
        let session = client
            .sessions()
            .create(
                &SessionCreateRequest::default()
                    .title(format!("active fixture {scenario}"))
                    .location(SessionLocationInput::new(workspace.to_str().unwrap()))
                    .agent("fixture".parse().unwrap())
                    .model(SessionModelInput::new("fixture".parse().unwrap(), scenario)),
            )
            .await
            .unwrap()
            .data;
        let id = &session.id;
        let mut events = client
            .events()
            .subscribe(&EventOptions::default().types([
                "session.text.delta",
                "session.execution.succeeded",
                "session.execution.failed",
                "session.execution.interrupted",
                "session.tool.success",
                "session.tool.failed",
                "permission.asked",
                "permission.replied",
                "form.created",
                "form.replied",
                "form.cancelled",
            ]))
            .await
            .unwrap();
        until(&mut events, "server.connected").await;
        let prompt_id: SessionMessageId = format!("msg_active_{scenario}").parse().unwrap();
        let admitted = client
            .sessions()
            .prompt(
                id,
                &SessionPromptRequest::new(format!("Run local fixture {scenario}."))
                    .id(prompt_id.clone())
                    .resume(true),
            )
            .await
            .unwrap();
        assert_eq!(admitted.data.id, prompt_id);
        let mut observed = Vec::new();

        if scenario == "output" || scenario == "interrupt" {
            observed.extend(until(&mut events, "session.text.delta").await);
            let Event::SessionTextDelta(first) = observed.last().unwrap() else {
                unreachable!()
            };
            assert_eq!(first.data.session_id, *id);
            assert_eq!(first.data.delta, "first ");
            // v2.2.5 projects text on text.ended, not on transient text.delta.
            let partial = client
                .sessions()
                .messages(id)
                .get(&first.data.assistant_message_id)
                .await
                .unwrap()
                .data;
            let SessionMessageInfo::Assistant(partial) = partial else {
                panic!("assistant snapshot")
            };
            assert!(partial.time.completed.is_none());
            assert_eq!(
                assistant_text(&[SessionMessageInfo::Assistant(partial)]),
                ""
            );
            still_running(&client, id).await;
            assert_eq!(provider.state().await["requests"][scenario], 1);
            if scenario == "output" {
                provider.release_output().await;
            } else {
                assert!(
                    client
                        .sessions()
                        .interrupt(id, &SessionInterruptOptions::default().resume(false))
                        .await
                        .unwrap()
                        .interrupted
                );
            }
        } else if scenario == "permission" {
            observed.extend(until(&mut events, "permission.asked").await);
            let pending = client.sessions().permissions(id).list().await.unwrap().data;
            assert_eq!(pending.len(), 1);
            let request = &pending[0];
            assert_eq!(request.action, "read");
            assert_eq!(request.resources, ["fixture.txt"]);
            assert_eq!(request.source.as_ref().unwrap().id, "call_permission");
            assert_eq!(
                client.permissions().list().await.unwrap().data[0].id,
                request.id
            );
            assert_running_tool(&history(&client, id).await, "call_permission");
            still_running(&client, id).await;
            assert_eq!(provider.state().await["requests"][scenario], 1);
            client
                .sessions()
                .permissions(id)
                .reply(
                    &request.id,
                    &SessionPermissionReplyRequest::new(PermissionReply::Once),
                )
                .await
                .unwrap();
        } else if scenario == "question" || scenario == "cancel" {
            observed.extend(until(&mut events, "form.created").await);
            let pending = client.sessions().forms(id).list().await.unwrap().data;
            assert_eq!(pending.len(), 1);
            let form = &pending[0];
            assert_eq!(form.metadata.as_ref().unwrap()["kind"], "question");
            assert_eq!(
                form.metadata.as_ref().unwrap()["tool"]["id"],
                format!("call_{scenario}")
            );
            assert_eq!(client.forms().list().await.unwrap().data[0].id, form.id);
            assert_running_tool(&history(&client, id).await, &format!("call_{scenario}"));
            still_running(&client, id).await;
            assert_eq!(provider.state().await["requests"][scenario], 1);
            if scenario == "question" {
                client
                    .sessions()
                    .forms(id)
                    .reply(
                        &form.id,
                        &SessionFormReplyRequest::new(
                            serde_json::from_value(json!({"q0": "fixture-answer"})).unwrap(),
                        ),
                    )
                    .await
                    .unwrap();
                assert!(matches!(
                    client
                        .sessions()
                        .forms(id)
                        .get(&form.id)
                        .await
                        .unwrap()
                        .data
                        .state,
                    FormState::Answered(_)
                ));
            } else {
                client
                    .sessions()
                    .forms(id)
                    .cancel(
                        &form.id,
                        &SessionFormCancelOptions::default().message("fixture-feedback"),
                    )
                    .await
                    .unwrap();
                assert!(matches!(
                    client
                        .sessions()
                        .forms(id)
                        .get(&form.id)
                        .await
                        .unwrap()
                        .data
                        .state,
                    FormState::Cancelled(_)
                ));
            }
        }

        let outcome = match scenario {
            "interrupt" => "interrupted",
            "failure" => "failed",
            _ => "succeeded",
        };
        observed.extend(until(&mut events, &format!("session.execution.{outcome}")).await);
        client
            .sessions()
            .experimental_wait(
                id,
                &SessionWaitOptions::default().timeout(Duration::from_secs(10)),
            )
            .await
            .unwrap();
        let settled = client.sessions().get(id).await.unwrap().data;
        assert_eq!(settled.outcome.unwrap().to_string(), outcome);
        let messages = history(&client, id).await;
        assert!(messages.iter().any(
            |message| matches!(message, SessionMessageInfo::User(user) if user.id == prompt_id)
        ));
        let Some(SessionMessageInfo::Idle(idle)) = messages.last() else {
            panic!("missing terminal idle message")
        };
        assert_eq!(idle.outcome.to_string(), outcome);
        assert!(
            client
                .sessions()
                .inbox(id)
                .list()
                .await
                .unwrap()
                .data
                .is_empty()
        );
        assert!(
            client
                .sessions()
                .permissions(id)
                .list()
                .await
                .unwrap()
                .data
                .is_empty()
        );
        assert!(
            client
                .sessions()
                .forms(id)
                .list()
                .await
                .unwrap()
                .data
                .is_empty()
        );
        match scenario {
            "output" => {
                let deltas: String = observed
                    .iter()
                    .filter_map(|event| match event {
                        Event::SessionTextDelta(e) => Some(e.data.delta.as_str()),
                        _ => None,
                    })
                    .collect();
                assert_eq!(deltas, "first second");
                assert_eq!(assistant_text(&messages), deltas);
            }
            "interrupt" => {
                assert_eq!(assistant_text(&messages), "first ");
                assert!(observed.iter().any(|event| matches!(event, Event::SessionExecutionInterrupted(e) if e.data.reason.to_string() == "user")));
                tokio::time::timeout(Duration::from_secs(5), async {
                    while provider.state().await["aborted"] != json!(["interrupt"]) {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                })
                .await
                .expect("provider connection should be cancelled");
                assert!(
                    !client
                        .sessions()
                        .interrupt(id, &SessionInterruptOptions::default().resume(false))
                        .await
                        .unwrap()
                        .interrupted
                );
            }
            "failure" => {
                assert!(idle.error.is_some());
                assert!(assistant_text(&messages).is_empty());
            }
            _ => {
                assert_eq!(assistant_text(&messages), format!("{scenario} resumed"));
                let tool = find_tool(&messages, &format!("call_{scenario}"));
                if scenario == "cancel" {
                    assert!(matches!(
                        tool.state,
                        SessionMessageAssistantToolState::Error(_)
                    ));
                    assert!(
                        observed
                            .iter()
                            .any(|event| matches!(event, Event::FormCancelled(_)))
                    );
                    assert!(
                        observed
                            .iter()
                            .any(|event| matches!(event, Event::SessionToolFailed(_)))
                    );
                } else {
                    assert!(matches!(
                        tool.state,
                        SessionMessageAssistantToolState::Completed(_)
                    ));
                    assert!(
                        observed
                            .iter()
                            .any(|event| matches!(event, Event::SessionToolSuccess(_)))
                    );
                    let reply = if scenario == "permission" {
                        "permission.replied"
                    } else {
                        "form.replied"
                    };
                    assert!(observed.iter().any(|event| event.kind() == reply));
                }
            }
        }
        drop(events);
        client.sessions().remove(id).await.unwrap();
    }
    assert_eq!(
        provider.state().await,
        json!({
            "requests": {"output": 1, "interrupt": 1, "failure": 1, "permission": 2, "question": 2, "cancel": 2},
            "aborted": ["interrupt"], "errors": [],
        })
    );
    assert!(
        client
            .sessions()
            .list(&SessionListOptions::default())
            .await
            .unwrap()
            .data
            .is_empty()
    );
}

fn find_tool<'a>(
    messages: &'a [SessionMessageInfo],
    call: &str,
) -> &'a SessionMessageAssistantTool {
    messages
        .iter()
        .filter_map(|message| match message {
            SessionMessageInfo::Assistant(assistant) => Some(assistant.content.iter()),
            _ => None,
        })
        .flatten()
        .find_map(|part| match part {
            SessionMessageAssistantContentItem::Tool(tool) if tool.id == call => {
                Some(tool.as_ref())
            }
            _ => None,
        })
        .expect("tool in projected history")
}

fn assert_running_tool(messages: &[SessionMessageInfo], call: &str) {
    assert!(matches!(
        find_tool(messages, call).state,
        SessionMessageAssistantToolState::Running(_)
    ));
}
