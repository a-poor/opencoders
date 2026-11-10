use opencoders::{
    ApiError, Client, Error, Location, RequestField, SessionShellOptions, ShellOutputOptions,
    models::*,
};
use serde_json::{Value, json};
use std::time::Duration;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path, query_param},
};

fn info() -> Value {
    json!({"location":{"directory":"/resolved"},"data":{"id":"sh_test","status":"exited","command":"private-value","cwd":"/work","shell":"/bin/sh","file":"/output","pid":9007199254740991_u64,"exit":-1,"metadata":{"token":"private-value"},"time":{"started":9007199254740991_u64,"completed":3.5},"future":"private-value"},"future":"private-value"})
}

#[test]
fn shell_inputs_preserve_omission_null_empty_and_reject_invalid_values() {
    let mut request = ShellCreateRequest::new("");
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"command":""})
    );
    request = request.cwd("").timeout_ms(0).metadata(Default::default());
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"command":"","cwd":"","timeout":0,"metadata":{}})
    );
    for value in [
        json!({}),
        json!({"command":"x","cwd":null}),
        json!({"command":"x","timeout":null}),
        json!({"command":"x","metadata":null}),
        json!({"command":"x","timeout":-1}),
        json!({"command":"x","timeout":1.5}),
        json!({"command":"x","timeout":9007199254740992_u64}),
        json!({"command":"x","env":{}}),
    ] {
        assert!(serde_json::from_value::<ShellCreateRequest>(value).is_err());
    }
    assert!(serde_json::to_value(ShellCreateRequest::new("x").timeout_ms(u64::MAX)).is_err());
    let mut session = SessionShellRequest::new("private-value");
    assert_eq!(
        serde_json::to_value(&session).unwrap(),
        json!({"command":"private-value"})
    );
    session.id = RequestField::Null;
    assert_eq!(serde_json::to_value(&session).unwrap()["id"], Value::Null);
    session = session.id("msg_shell".parse().unwrap());
    assert_eq!(serde_json::to_value(&session).unwrap()["id"], "msg_shell");
    assert!(!format!("{session:?}").contains("private-value"));
    for value in [
        json!({}),
        json!({"command":null}),
        json!({"command":"x","id":"bad"}),
        json!({"command":"x","extra":true}),
    ] {
        assert!(serde_json::from_value::<SessionShellRequest>(value).is_err());
    }
    for value in [
        json!({}),
        json!({"variables":null}),
        json!({"variables":{"A":1}}),
        json!({"variables":{},"extra":1}),
    ] {
        assert!(serde_json::from_value::<SessionEnvironmentRequest>(value).is_err());
    }
    let env = SessionEnvironmentRequest::new([("TOKEN".into(), "private-value".into())].into());
    assert!(!format!("{env:?}").contains("private-value"));
    assert!(!format!("{:?}", ShellCreateRequest::new("private-value")).contains("private-value"));
}

#[test]
fn shell_responses_share_events_preserve_numbers_extensions_and_redact_debug() {
    let response: ShellResponse = serde_json::from_value(info()).unwrap();
    let shared: &events::ShellInfo = &response.data;
    assert_eq!(shared.pid.unwrap().get(), 9007199254740991);
    assert_eq!(shared.time.started.as_u64(), Some(9007199254740991));
    assert_eq!(shared.exit.as_ref().unwrap().as_i64(), Some(-1));
    assert_eq!(shared.extra["future"], "private-value");
    assert!(!format!("{response:?} {shared:?}").contains("private-value"));
    assert_eq!(serde_json::to_value(&response).unwrap(), info());
    let page: ShellOutputResponse = serde_json::from_value(json!({"location":{"directory":"/resolved"},"data":{"output":"private-value","cursor":9007199254740991_u64,"size":9007199254740991_u64,"truncated":false,"extra":true}})).unwrap();
    let shared: &events::EventSessionShellEndedDataOutput = &page.data;
    assert_eq!(shared.cursor.get(), 9007199254740991);
    assert!(!format!("{page:?} {shared:?}").contains("private-value"));
    for value in [
        json!(-1),
        json!(1.5),
        json!(9007199254740992_u64),
        Value::Null,
    ] {
        let mut malformed = serde_json::to_value(&page).unwrap();
        malformed["data"]["cursor"] = value;
        assert!(serde_json::from_value::<ShellOutputResponse>(malformed).is_err());
    }
    for (key, value) in [
        ("pid", json!(-1)),
        ("pid", json!(9007199254740992_u64)),
        ("status", json!("future")),
        ("metadata", Value::Null),
    ] {
        let mut v = info();
        v["data"][key] = value;
        assert!(serde_json::from_value::<ShellResponse>(v).is_err());
    }
}

#[tokio::test]
async fn shell_routes_preserve_prefix_locations_bodies_and_output_cursors() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/proxy/api/shell"))
        .and(query_param("location[directory]", "/work ? 雪"))
        .and(body_json(
            json!({"command":"","cwd":"","metadata":{},"timeout":0}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(info()))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/shell"))
        .and(query_param("location[directory]", "/work ? 雪"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(
                json!({"location":{"directory":"/resolved"},"data":[info()["data"]]}),
            ),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/shell/sh_test"))
        .and(query_param("location[directory]", "/override"))
        .respond_with(ResponseTemplate::new(200).set_body_json(info()))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET")).and(path("/proxy/api/shell/sh_test/output")).and(query_param("location[directory]","/override")).and(query_param("cursor","9007199254740991")).and(query_param("limit","0")).respond_with(ResponseTemplate::new(200).set_body_json(json!({"location":{"directory":"/override"},"data":{"output":"雪","cursor":7,"size":12,"truncated":true}}))).expect(1).mount(&server).await;
    Mock::given(method("DELETE"))
        .and(path("/proxy/api/shell/sh_test"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/work ? 雪"))
        .build()
        .unwrap();
    let shells = client.shells();
    let id = "sh_test".parse().unwrap();
    assert_eq!(
        shells
            .create(
                &ShellCreateRequest::new("")
                    .cwd("")
                    .timeout_ms(0)
                    .metadata(Default::default())
            )
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        "/resolved"
    );
    assert_eq!(shells.list().await.unwrap().data.len(), 1);
    assert_eq!(
        shells
            .get_at(&id, Location::directory("/override"))
            .await
            .unwrap()
            .data
            .id,
        id
    );
    let page = shells
        .output_at(
            &id,
            &ShellOutputOptions::default()
                .cursor(9007199254740991)
                .limit(0),
            Location::directory("/override"),
        )
        .await
        .unwrap();
    assert_eq!(page.data.output, "雪");
    assert!(page.data.truncated);
    shells.remove(&id).await.unwrap();
}

#[tokio::test]
async fn session_shell_and_environment_have_strict_204_and_independent_deadlines() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/session/ses_test/shell"))
        .and(body_json(json!({"command":"","id":null})))
        .respond_with(ResponseTemplate::new(204).set_delay(Duration::from_millis(80)))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/api/session/ses_test/environment"))
        .and(body_json(json!({"variables":{}})))
        .respond_with(ResponseTemplate::new(200).set_body_json(true))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .default_location(Location::directory("/must-not-be-sent"))
        .request_timeout(Duration::from_millis(20))
        .build()
        .unwrap();
    let id = "ses_test".parse().unwrap();
    let mut r = SessionShellRequest::new("");
    r.id = RequestField::Null;
    client
        .sessions()
        .shell(
            &id,
            &r,
            &SessionShellOptions::default().timeout(Duration::from_secs(2)),
        )
        .await
        .unwrap();
    let error = client
        .sessions()
        .environment(&id, &SessionEnvironmentRequest::new(Default::default()))
        .await
        .unwrap_err();
    assert_eq!(error.status().unwrap(), 200);
    assert_eq!(error.operation(), "session.environment");
    for request in server.received_requests().await.unwrap() {
        assert!(request.url.query().is_none());
    }
}

#[tokio::test]
async fn failed_shell_mutations_are_not_retried_and_missing_shells_are_typed() {
    let server = MockServer::start().await;
    for (verb, route) in [
        ("POST", "/api/shell"),
        ("DELETE", "/api/shell/sh_test"),
        ("POST", "/api/session/ses_test/shell"),
        ("PUT", "/api/session/ses_test/environment"),
    ] {
        Mock::given(method(verb))
            .and(path(route))
            .respond_with(ResponseTemplate::new(503))
            .expect(1)
            .mount(&server)
            .await;
    }
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(
                json!({"_tag":"ShellNotFoundError","id":"sh_test","message":"gone"}),
            ),
        )
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let id = "sh_test".parse().unwrap();
    let sid = "ses_test".parse().unwrap();
    for result in [
        client
            .shells()
            .create(&ShellCreateRequest::new("x"))
            .await
            .map(|_| ()),
        client.shells().remove(&id).await,
        client
            .sessions()
            .shell(
                &sid,
                &SessionShellRequest::new("x"),
                &SessionShellOptions::default(),
            )
            .await,
        client
            .sessions()
            .environment(&sid, &SessionEnvironmentRequest::new(Default::default()))
            .await,
    ] {
        assert_eq!(result.unwrap_err().status().unwrap(), 503);
    }
    let Error::Http(e) = client.shells().get(&id).await.unwrap_err() else {
        panic!("HTTP error")
    };
    assert!(matches!(e.api_error(),Some(ApiError::ShellNotFound{id,..}) if id=="sh_test"));
    assert_eq!(e.operation(), "shell.get");
}

#[tokio::test]
async fn shell_output_omits_default_queries_and_rejects_invalid_offsets_before_sending() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/shell/sh_%2Fa%20%3F/output")).respond_with(ResponseTemplate::new(200).set_body_json(json!({"location":{"directory":"/work"},"data":{"output":"","cursor":0,"size":0,"truncated":false}}))).expect(1).mount(&server).await;
    let client = Client::new(server.uri()).unwrap();
    let id = "sh_/a ?".parse().unwrap();
    client
        .shells()
        .output(&id, &ShellOutputOptions::default())
        .await
        .unwrap();
    assert!(
        server.received_requests().await.unwrap()[0]
            .url
            .query()
            .is_none()
    );
    for options in [
        ShellOutputOptions::default().cursor(u64::MAX),
        ShellOutputOptions::default().limit(9007199254740992),
    ] {
        assert!(matches!(
            client.shells().output(&id, &options).await,
            Err(Error::Encode(_))
        ));
    }
    assert!(matches!(
        client
            .shells()
            .create(&ShellCreateRequest::new("x").timeout_ms(u64::MAX))
            .await,
        Err(Error::Encode(_))
    ));
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn session_shell_default_has_no_sdk_deadline_and_explicit_timeout_is_not_retried() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(204).set_delay(Duration::from_millis(100)))
        .expect(2)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .request_timeout(Duration::from_millis(10))
        .build()
        .unwrap();
    let id = "ses_test".parse().unwrap();
    let request = SessionShellRequest::new("x");
    client
        .sessions()
        .shell(&id, &request, &SessionShellOptions::default())
        .await
        .unwrap();
    let error = client
        .sessions()
        .shell(
            &id,
            &request,
            &SessionShellOptions::default().timeout(Duration::from_millis(20)),
        )
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    assert_eq!(error.operation(), "session.shell");
}

#[tokio::test]
async fn shell_reads_reject_wrong_success_statuses_malformed_envelopes_and_oversized_output() {
    let server = MockServer::start().await;
    Mock::given(path("/api/shell"))
        .respond_with(ResponseTemplate::new(201).set_body_json(info()))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/api/shell/sh_test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":info()["data"]})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/api/shell/sh_test/output"))
        .respond_with(ResponseTemplate::new(200).set_body_string("x".repeat(2000)))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .max_response_bytes(1024)
        .build()
        .unwrap();
    let id = "sh_test".parse().unwrap();
    assert_eq!(
        client.shells().list().await.unwrap_err().status().unwrap(),
        201
    );
    assert!(matches!(
        client.shells().get(&id).await,
        Err(Error::Decode(_))
    ));
    assert!(matches!(
        client
            .shells()
            .output(&id, &ShellOutputOptions::default())
            .await,
        Err(Error::ResponseTooLarge(_))
    ));
}
