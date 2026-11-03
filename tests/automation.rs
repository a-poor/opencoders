#[path = "../examples/support/automation.rs"]
mod automation;
use automation::{Inspection, Outcome, WaitState, assess};
use opencoders::models::SessionMessageId;
use serde_json::{Value, json};

const SESSION: &str = "ses_automation";
const PROMPT: &str = "msg_automation";
fn prompt_id() -> SessionMessageId {
    PROMPT.parse().unwrap()
}
fn history(outcome: &str) -> Vec<Value> {
    vec![
        json!({"id":PROMPT,"type":"user","text":"question","time":{"created":1}}),
        json!({"id":"msg_answer","type":"assistant","agent":"build","model":{"providerID":"fixture","id":"fixture"},"content":[{"type":"text","text":"answer"}],"time":{"created":2,"completed":3}}),
        json!({"id":"msg_idle","type":"idle","outcome":outcome,"time":{"created":4}}),
    ]
}
fn session(outcome: &str) -> Value {
    let mut value: Value = serde_json::from_str(include_str!("fixtures/session.json")).unwrap();
    value["id"] = json!(SESSION);
    value["outcome"] = json!(outcome);
    value
}
fn view(outcome: &str) -> Inspection {
    Inspection {
        session: serde_json::from_value(session(outcome)).unwrap(),
        history: serde_json::from_value(json!(history(outcome))).unwrap(),
        history_complete: true,
        inbox: serde_json::from_value(json!({"data":[]})).unwrap(),
        permissions: serde_json::from_value(json!({"data":[]})).unwrap(),
        forms: serde_json::from_value(json!({"data":[]})).unwrap(),
    }
}
#[test]
fn idle_requires_matching_prompt_history_and_consistent_terminal_outcomes() {
    for (wire, expected) in [
        ("succeeded", Outcome::Succeeded),
        ("failed", Outcome::Failed),
        ("interrupted", Outcome::Interrupted),
    ] {
        assert_eq!(assess(&view(wire), &prompt_id(), WaitState::Idle), expected);
    }
    let mut empty = view("succeeded");
    empty.history.clear();
    assert_eq!(
        assess(&empty, &prompt_id(), WaitState::Idle),
        Outcome::Inconclusive
    );
    assert_eq!(
        assess(
            &view("succeeded"),
            &"msg_other".parse().unwrap(),
            WaitState::Idle
        ),
        Outcome::Inconclusive
    );
}
#[test]
fn pending_input_and_human_requests_override_old_success_even_after_wait_timeout() {
    let mut pending = view("succeeded");
    pending.inbox = serde_json::from_value(json!({"data":[{"id":PROMPT,"sessionID":SESSION,"type":"user","delivery":"queue","payload":{"text":"question"},"time":{"created":1}}]})).unwrap();
    assert_eq!(
        assess(&pending, &prompt_id(), WaitState::Idle),
        Outcome::Pending
    );
    let mut permission = view("succeeded");
    permission.permissions = serde_json::from_value(
        json!({"data":[{"id":"per_pending","sessionID":SESSION,"action":"read","resources":[]}]}),
    )
    .unwrap();
    assert_eq!(
        assess(&permission, &prompt_id(), WaitState::TimedOut),
        Outcome::AwaitingInput
    );
    let mut form = view("succeeded");
    form.forms = serde_json::from_value(json!({"data":[{"id":"frm__pending","sessionID":SESSION,"title":"Input","fields":[{"key":"ok","type":"boolean"}]}]})).unwrap();
    assert_eq!(
        assess(&form, &prompt_id(), WaitState::Idle),
        Outcome::AwaitingInput
    );
}
#[test]
fn ambiguous_waits_history_bounds_and_foreign_work_never_become_success() {
    for wait in [WaitState::TimedOut, WaitState::Failed] {
        assert_eq!(
            assess(&view("succeeded"), &prompt_id(), wait),
            Outcome::Inconclusive
        );
    }
    let mut truncated = view("succeeded");
    truncated.history_complete = false;
    assert_eq!(
        assess(&truncated, &prompt_id(), WaitState::Idle),
        Outcome::Inconclusive
    );
    for extra in [
        json!({"id":"msg_other","type":"user","text":"someone else","time":{"created":2}}),
        json!({"id":"msg_other","type":"system","text":"unhandled work","time":{"created":2}}),
        history("succeeded")[0].clone(),
    ] {
        let mut candidate = view("succeeded");
        candidate
            .history
            .insert(1, serde_json::from_value(extra).unwrap());
        assert_eq!(
            assess(&candidate, &prompt_id(), WaitState::Idle),
            Outcome::Inconclusive
        );
    }
}
#[test]
fn partial_conflicting_or_unfinished_output_stays_inconclusive() {
    let mut candidates = Vec::new();
    let mut no_marker = history("succeeded");
    no_marker.pop();
    candidates.push(no_marker);
    let mut no_answer = history("succeeded");
    no_answer.remove(1);
    candidates.push(no_answer);
    let mut unfinished = history("succeeded");
    unfinished[1]["time"]
        .as_object_mut()
        .unwrap()
        .remove("completed");
    candidates.push(unfinished);
    let mut error = history("succeeded");
    error[1]["error"] = json!({"type":"fixture","message":"model failure"});
    candidates.push(error);
    candidates.push(history("failed"));
    let mut after_idle = history("succeeded");
    after_idle.swap(1, 2);
    candidates.push(after_idle);
    let mut two_markers = history("succeeded");
    two_markers
        .push(json!({"id":"msg_later","type":"idle","outcome":"succeeded","time":{"created":5}}));
    candidates.push(two_markers);
    for history in candidates {
        let mut candidate = view("succeeded");
        candidate.history = serde_json::from_value(json!(history)).unwrap();
        assert_eq!(
            assess(&candidate, &prompt_id(), WaitState::Idle),
            Outcome::Inconclusive
        );
    }
}

use automation::{FeedState, Job, Limits, Notice, Problem, Stage, run};
use opencoders::{Client, models::SessionModelInput};
use std::time::Duration;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};
fn job() -> Job {
    Job {
        session_id: SESSION.parse().unwrap(),
        prompt_id: prompt_id(),
        directory: "/remote/project".into(),
        agent: "build".to_owned().into(),
        model: SessionModelInput::new("fixture".to_owned().into(), "fixture"),
        text: "question".into(),
    }
}
fn limits() -> Limits {
    Limits {
        work: Duration::from_secs(3),
        inspection: Duration::from_secs(2),
        ..Limits::default()
    }
}
fn admitted() -> Value {
    json!({"data":{"id":PROMPT,"sessionID":SESSION,"type":"user","delivery":"steer","payload":{"text":"question"},"time":{"created":1}}})
}
fn event(kind: &str) -> Value {
    let fixtures: Vec<Value> = serde_json::from_str(include_str!("fixtures/events.json")).unwrap();
    let mut event = fixtures.into_iter().find(|v| v["type"] == kind).unwrap();
    event["data"]["sessionID"] = json!(SESSION);
    if event.get("durable").is_some() {
        event["durable"]["aggregateID"] = json!(SESSION);
    }
    event
}
async fn override_route(server: &MockServer, verb: &str, route: &str, response: ResponseTemplate) {
    Mock::given(method(verb))
        .and(path(route))
        .respond_with(response)
        .with_priority(2)
        .mount(server)
        .await;
}
async fn server() -> MockServer {
    let server = MockServer::start().await;
    let mut foreign = event("session.text.delta");
    foreign["data"]["sessionID"] = json!("ses_foreign");
    let events = [
        foreign,
        event("session.text.delta"),
        event("session.execution.succeeded"),
        json!({"type":"future.example","private":"do not retain"}),
    ]
    .iter()
    .map(|v| format!("data: {v}\n\n"))
    .collect::<String>();
    for (verb, route, response) in [
        (
            "POST",
            "/api/session",
            ResponseTemplate::new(200).set_body_json(json!({"data":session("succeeded")})),
        ),
        (
            "GET",
            "/api/event",
            ResponseTemplate::new(200).set_body_raw(events, "text/event-stream"),
        ),
        (
            "POST",
            "/api/session/ses_automation/prompt",
            ResponseTemplate::new(200)
                .set_body_json(admitted())
                .set_delay(Duration::from_millis(30)),
        ),
        (
            "POST",
            "/api/experimental/session/ses_automation/wait",
            ResponseTemplate::new(204).set_delay(Duration::from_millis(30)),
        ),
        (
            "GET",
            "/api/session/ses_automation",
            ResponseTemplate::new(200).set_body_json(json!({"data":session("succeeded")})),
        ),
        (
            "GET",
            "/api/session/ses_automation/message",
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":history("succeeded"),"cursor":{}})),
        ),
        (
            "GET",
            "/api/session/ses_automation/inbox",
            ResponseTemplate::new(200).set_body_json(json!({"data":[]})),
        ),
        (
            "GET",
            "/api/session/ses_automation/permission",
            ResponseTemplate::new(200).set_body_json(json!({"data":[]})),
        ),
        (
            "GET",
            "/api/session/ses_automation/form",
            ResponseTemplate::new(200).set_body_json(json!({"data":[]})),
        ),
    ] {
        Mock::given(method(verb))
            .and(path(route))
            .respond_with(response)
            .mount(&server)
            .await;
    }
    server
}
#[tokio::test]
async fn automation_subscribes_before_admission_observes_progress_and_inspects_outcomes() {
    let server = server().await;
    let mut notices = Vec::new();
    let result = run(
        &Client::new(server.uri()).unwrap(),
        &job(),
        limits(),
        |notice| notices.push(notice),
    )
    .await
    .unwrap();
    assert_eq!(result.outcome, Outcome::Succeeded);
    assert_eq!(result.wait, WaitState::Idle);
    assert!(result.wait_error.is_none() && result.inspection_error.is_none());
    assert_eq!(result.trace.text_fragments, 1);
    assert_eq!(result.trace.unknown_events, 1);
    assert_eq!(result.trace.feed, FeedState::Ended);
    assert!(notices.contains(&Notice::Admitted) && notices.contains(&Notice::Waiting));
    assert_eq!(
        result.inspection.unwrap().answer_text().collect::<Vec<_>>(),
        ["answer"]
    );
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests[0].url.path(), "/api/session");
    assert_eq!(requests[1].url.path(), "/api/event");
    assert_eq!(requests[2].url.path(), "/api/session/ses_automation/prompt");
    let create: Value = requests[0].body_json().unwrap();
    assert_eq!(create["id"], SESSION);
    assert_eq!(create["agent"], "build");
    assert_eq!(create["model"]["providerID"], "fixture");
    assert_eq!(create["location"]["directory"], "/remote/project");
    let prompt: Value = requests[2].body_json().unwrap();
    assert_eq!(
        prompt,
        json!({"id":PROMPT,"text":"question","resume":true,"delivery":"steer"})
    );
    let writes = requests
        .iter()
        .filter(|r| r.method == "POST")
        .map(|r| r.url.path())
        .collect::<Vec<_>>();
    assert_eq!(
        writes,
        [
            "/api/session",
            "/api/session/ses_automation/prompt",
            "/api/experimental/session/ses_automation/wait"
        ]
    );
    assert!(
        requests
            .iter()
            .all(|r| r.method != "DELETE" && !r.headers.contains_key("last-event-id"))
    );
}
#[tokio::test]
async fn deadline_stops_local_waiting_but_still_inspects_pending_requests() {
    let server = server().await;
    override_route(
        &server,
        "POST",
        "/api/session/ses_automation/prompt",
        ResponseTemplate::new(200).set_body_json(admitted()),
    )
    .await;
    override_route(
        &server,
        "POST",
        "/api/experimental/session/ses_automation/wait",
        ResponseTemplate::new(204).set_delay(Duration::from_secs(2)),
    )
    .await;
    override_route(&server,"GET","/api/session/ses_automation/form",ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"frm__pending","sessionID":SESSION,"title":"Input","fields":[{"key":"ok","type":"boolean"}]}]}))).await;
    let limits = Limits {
        work: Duration::from_secs(1),
        ..limits()
    };
    let result = run(&Client::new(server.uri()).unwrap(), &job(), limits, |_| {})
        .await
        .unwrap();
    assert_eq!(result.wait, WaitState::TimedOut);
    assert!(matches!(result.wait_error, Some(Problem::Deadline)));
    assert_eq!(result.outcome, Outcome::AwaitingInput);
    assert_eq!(result.inspection.unwrap().forms.data.len(), 1);
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests
            .iter()
            .filter(|r| r.url.path().ends_with("/wait"))
            .count(),
        1
    );
    assert!(
        !requests
            .iter()
            .any(|r| r.url.path().ends_with("/interrupt") || r.method == "DELETE")
    );
}
#[tokio::test]
async fn ambiguous_admission_is_never_resubmitted_or_cleaned_up_automatically() {
    for response in [
        ResponseTemplate::new(500),
        ResponseTemplate::new(200)
            .set_body_json(admitted())
            .set_delay(Duration::from_secs(2)),
    ] {
        let server = server().await;
        override_route(
            &server,
            "POST",
            "/api/session/ses_automation/prompt",
            response,
        )
        .await;
        let failure = run(
            &Client::new(server.uri()).unwrap(),
            &job(),
            Limits {
                work: Duration::from_secs(1),
                ..limits()
            },
            |_| {},
        )
        .await
        .err()
        .unwrap();
        assert_eq!(failure.stage, Stage::Admission);
        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests
                .iter()
                .filter(|r| r.url.path().ends_with("/prompt"))
                .count(),
            1
        );
        assert!(
            !requests
                .iter()
                .any(|r| r.url.path().ends_with("/wait") || r.method == "DELETE")
        );
    }
}
#[tokio::test]
async fn setup_failures_and_identity_mismatches_stop_before_further_mutations() {
    let mut other = session("succeeded");
    other["id"] = json!("ses_other");
    for (verb, route, response, stage) in [
        (
            "POST",
            "/api/session",
            ResponseTemplate::new(500),
            Stage::Create,
        ),
        (
            "GET",
            "/api/event",
            ResponseTemplate::new(401),
            Stage::Subscribe,
        ),
        (
            "POST",
            "/api/session",
            ResponseTemplate::new(200).set_body_json(json!({"data":other})),
            Stage::Create,
        ),
    ] {
        let server = server().await;
        override_route(&server, verb, route, response).await;
        let failure = run(
            &Client::new(server.uri()).unwrap(),
            &job(),
            limits(),
            |_| {},
        )
        .await
        .err()
        .unwrap();
        assert_eq!(failure.stage, stage);
        assert!(
            !server
                .received_requests()
                .await
                .unwrap()
                .iter()
                .any(|r| r.url.path().ends_with("/prompt"))
        );
    }
}
#[tokio::test]
async fn history_pagination_is_explicit_bounded_and_requires_a_complete_traversal() {
    let server = server().await;
    override_route(
        &server,
        "GET",
        "/api/session/ses_automation/message",
        ResponseTemplate::new(200)
            .set_body_json(json!({"data":history("succeeded"),"cursor":{"next":"opaque/+&雪"}})),
    )
    .await;
    Mock::given(method("GET"))
        .and(path("/api/session/ses_automation/message"))
        .and(query_param("cursor", "opaque/+&雪"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[],"cursor":{}})))
        .with_priority(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let complete = run(&client, &job(), limits(), |_| {}).await.unwrap();
    assert_eq!(complete.outcome, Outcome::Succeeded);
    let requests = server.received_requests().await.unwrap();
    let pages = requests
        .iter()
        .filter(|r| r.url.path().ends_with("/message"))
        .collect::<Vec<_>>();
    assert_eq!(pages.len(), 2);
    assert!(
        pages[0]
            .url
            .query_pairs()
            .any(|(k, v)| k == "order" && v == "asc")
    );
    assert!(!pages[1].url.query_pairs().any(|(k, _)| k == "order"));
    assert!(
        pages[1]
            .url
            .query_pairs()
            .any(|(k, v)| k == "cursor" && v == "opaque/+&雪")
    );
    let bounded = run(
        &client,
        &job(),
        Limits {
            max_pages: 1,
            ..limits()
        },
        |_| {},
    )
    .await
    .unwrap();
    assert_eq!(bounded.outcome, Outcome::Inconclusive);
    assert!(!bounded.inspection.unwrap().history_complete);
}
#[tokio::test]
async fn snapshot_failures_are_inconclusive_and_do_not_publish_partial_views() {
    for response in [
        ResponseTemplate::new(500),
        ResponseTemplate::new(200)
            .set_body_json(json!({"data":[]}))
            .set_delay(Duration::from_secs(2)),
    ] {
        let server = server().await;
        override_route(&server, "GET", "/api/session/ses_automation/form", response).await;
        let result = run(
            &Client::new(server.uri()).unwrap(),
            &job(),
            Limits {
                inspection: Duration::from_millis(100),
                ..limits()
            },
            |_| {},
        )
        .await
        .unwrap();
        assert_eq!(result.outcome, Outcome::Inconclusive);
        assert!(result.inspection.is_none() && result.inspection_error.is_some());
    }
}
#[tokio::test]
async fn feed_budget_and_decode_failure_do_not_masquerade_as_execution_failure() {
    for (body, budget, feed) in [
        (
            format!("data: {}\n\n", json!({"type":"future.event"})).repeat(20),
            2,
            FeedState::BudgetReached,
        ),
        (
            "data: {\"type\":\"session.text.delta\"}\n\n".into(),
            20,
            FeedState::Failed,
        ),
    ] {
        let server = server().await;
        override_route(
            &server,
            "GET",
            "/api/event",
            ResponseTemplate::new(200).set_body_raw(body, "text/event-stream"),
        )
        .await;
        let result = run(
            &Client::new(server.uri()).unwrap(),
            &job(),
            Limits {
                max_events: budget,
                ..limits()
            },
            |_| {},
        )
        .await
        .unwrap();
        assert_eq!(result.trace.feed, feed);
        assert!(result.trace.events_seen <= budget);
        assert_eq!(result.outcome, Outcome::Succeeded); // Supported by snapshots, not feed health.
    }
}

#[tokio::test]
async fn a_wait_error_is_observation_failure_not_failed_execution() {
    let server = server().await;
    override_route(
        &server,
        "POST",
        "/api/experimental/session/ses_automation/wait",
        ResponseTemplate::new(500),
    )
    .await;
    let result = run(
        &Client::new(server.uri()).unwrap(),
        &job(),
        limits(),
        |_| {},
    )
    .await
    .unwrap();
    assert_eq!(result.wait, WaitState::Failed);
    assert!(matches!(result.wait_error, Some(Problem::Api(_))));
    assert!(result.inspection.is_some());
    assert_eq!(result.outcome, Outcome::Inconclusive);
}
#[tokio::test]
async fn mismatched_admission_identity_stops_without_waiting_or_resubmitting() {
    let server = server().await;
    let mut wrong = admitted();
    wrong["data"]["id"] = json!("msg_other");
    override_route(
        &server,
        "POST",
        "/api/session/ses_automation/prompt",
        ResponseTemplate::new(200).set_body_json(wrong),
    )
    .await;
    let failure = run(
        &Client::new(server.uri()).unwrap(),
        &job(),
        limits(),
        |_| {},
    )
    .await
    .err()
    .unwrap();
    assert_eq!(failure.stage, Stage::Admission);
    assert!(matches!(failure.problem, Problem::UnexpectedIdentity));
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests
            .iter()
            .filter(|r| r.url.path().ends_with("/prompt"))
            .count(),
        1
    );
    assert!(!requests.iter().any(|r| r.url.path().ends_with("/wait")));
}

#[test]
fn recognized_instruction_updates_can_surround_the_prompt_without_becoming_a_second_input() {
    for index in [0, 1] {
        let mut candidate = view("succeeded");
        candidate.history.insert(index,serde_json::from_value(json!({"id":"msg_instructions","type":"system","text":"server instructions","metadata":{"notice":"instructions"},"time":{"created":1}})).unwrap());
        assert_eq!(
            assess(&candidate, &prompt_id(), WaitState::Idle),
            Outcome::Succeeded
        );
    }
}
