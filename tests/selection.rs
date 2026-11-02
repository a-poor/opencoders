use opencoders::{
    ApiError, Client, Error, Location, ModelListOptions,
    models::{SessionModelInput, SessionSwitchAgentRequest, SessionSwitchModelRequest},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

fn model() -> Value {
    serde_json::from_str(include_str!("fixtures/model.json")).unwrap()
}
fn agent() -> Value {
    json!({"id":"build","name":"Build","mode":"primary","hidden":false,"request":{"settings":{},"headers":{},"body":{}},"permissions":[]})
}

#[tokio::test]
async fn discovery_preserves_envelopes_defaults_overrides_and_repeated_types() {
    let server = MockServer::start().await;
    let models = json!({"location":{"directory":"/resolved"},"data":[model()],"extra":1});
    let default = json!({"location":{"directory":"/resolved"},"data":null,"extra":2});
    let single = json!({"location":{"directory":"/resolved"},"data":agent(),"extra":3});
    for (route, body) in [
        ("model", &models),
        ("model/default", &default),
        ("agent/build%2F..%2F%3F%23%25%E9%9B%AA", &single),
    ] {
        Mock::given(method("GET"))
            .and(path(format!("/proxy/api/{route}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
    }
    let mut headers = opencoders::header::HeaderMap::new();
    headers.insert("x-opencode-directory", "/header".parse().unwrap());
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_headers(headers)
        .default_location(Location::directory("/default +&雪"))
        .bearer_auth("token")
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(
            client
                .models()
                .list(&ModelListOptions::default())
                .await
                .unwrap()
        )
        .unwrap(),
        models
    );
    assert_eq!(
        serde_json::to_value(client.models().default().await.unwrap()).unwrap(),
        default
    );
    let id = "build/../?#%雪".to_owned().into();
    assert_eq!(
        serde_json::to_value(client.agents().get(&id).await.unwrap()).unwrap(),
        single
    );
    client
        .models()
        .list_at(
            &ModelListOptions::default().types(["chat", "vendor+&雪", "chat", ""]),
            Location::directory("/other?%雪"),
        )
        .await
        .unwrap();
    client
        .models()
        .default_at(Location::Unspecified)
        .await
        .unwrap();
    client
        .agents()
        .get_at(&id, Location::Unspecified)
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    for r in &requests[..3] {
        assert_eq!(
            r.url.query_pairs().collect::<Vec<_>>(),
            [("location[directory]".into(), "/default +&雪".into())]
        );
    }
    assert_eq!(
        requests[3].url.query_pairs().collect::<Vec<_>>(),
        [
            ("location[directory]".into(), "/other?%雪".into()),
            ("types".into(), "chat".into()),
            ("types".into(), "vendor+&雪".into()),
            ("types".into(), "chat".into()),
            ("types".into(), "".into())
        ]
    );
    for r in &requests[4..] {
        assert!(r.url.query().is_none());
    }
    for r in &requests {
        assert_eq!(r.headers["authorization"], "Bearer token");
        assert_eq!(r.headers["x-opencode-directory"], "/header");
    }
    assert_eq!(
        client.default_location(),
        &Location::directory("/default +&雪")
    );
}

#[tokio::test]
async fn switches_send_strict_json_accept_204_and_ignore_discovery_location() {
    let server = MockServer::start().await;
    let root = "/proxy/api/session/ses%2F..%2F%3F%23%25%E9%9B%AA";
    for (route, body) in [
        ("agent", json!({"agent":"build +雪"})),
        (
            "model",
            json!({"model":{"providerID":"provider","id":"public-name","variant":"fast"}}),
        ),
    ] {
        Mock::given(method("POST"))
            .and(path(format!("{root}/{route}")))
            .and(body_json(body))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&server)
            .await;
    }
    let mut headers = opencoders::header::HeaderMap::new();
    headers.insert(
        opencoders::header::CONTENT_TYPE,
        "text/plain".parse().unwrap(),
    );
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_headers(headers)
        .default_location(Location::directory("/ignored"))
        .max_response_bytes(0)
        .build()
        .unwrap();
    let id = "ses/../?#%雪".parse().unwrap();
    client
        .sessions()
        .switch_agent(
            &id,
            &SessionSwitchAgentRequest::new("build +雪".to_owned().into()),
        )
        .await
        .unwrap();
    client
        .sessions()
        .switch_model(
            &id,
            &SessionSwitchModelRequest::new(
                SessionModelInput::new("provider".to_owned().into(), "public-name").variant("fast"),
            ),
        )
        .await
        .unwrap();
    for r in server.received_requests().await.unwrap() {
        assert!(r.url.query().is_none());
    }
}

#[tokio::test]
async fn missing_agent_errors_are_typed_inspectable_and_redacted() {
    let server = MockServer::start().await;
    let raw = json!({"_tag":"AgentNotFoundError","agentID":"missing","message":"private-error","extra":true});
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&raw))
        .mount(&server)
        .await;
    let error = Client::new(server.uri())
        .unwrap()
        .agents()
        .get(&"missing".to_owned().into())
        .await
        .unwrap_err();
    assert_eq!(error.operation(), "agent.get");
    assert_eq!(error.status().unwrap(), 404);
    assert!(!format!("{error:?} {error}").contains("private-"));
    let Error::Http(error) = error else {
        panic!("expected HTTP error")
    };
    assert!(
        matches!(error.api_error(),Some(ApiError::AgentNotFound{agent_id,..}) if agent_id=="missing")
    );
    assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), raw);
}

#[tokio::test]
async fn agent_ids_that_cannot_form_a_path_segment_fail_before_http() {
    let server = MockServer::start().await;
    let client = Client::new(server.uri()).unwrap();
    for id in ["", ".", ".."] {
        let error = client
            .agents()
            .get(&id.to_owned().into())
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "agent.get");
        assert!(matches!(error, Error::Encode(_)));
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}

async fn request(client: &Client, operation: &str) -> opencoders::Result<()> {
    match operation {
        "model.list" => client
            .models()
            .list(&ModelListOptions::default())
            .await
            .map(|_| ()),
        "model.default" => client.models().default().await.map(|_| ()),
        "agent.get" => client
            .agents()
            .get(&"build".to_owned().into())
            .await
            .map(|_| ()),
        "session.switchAgent" => {
            client
                .sessions()
                .switch_agent(
                    &"ses_test".parse().unwrap(),
                    &SessionSwitchAgentRequest::new("build".to_owned().into()),
                )
                .await
        }
        "session.switchModel" => {
            client
                .sessions()
                .switch_model(
                    &"ses_test".parse().unwrap(),
                    &SessionSwitchModelRequest::new(SessionModelInput::new(
                        "provider".to_owned().into(),
                        "public-name",
                    )),
                )
                .await
        }
        _ => unreachable!(),
    }
}

#[tokio::test]
async fn discovery_rejects_missing_envelopes_and_malformed_known_values() {
    for (operation, data) in [
        ("model.list", json!([])),
        ("model.default", Value::Null),
        ("agent.get", agent()),
    ] {
        for body in [
            json!({"data":data}),
            json!({"location":{"directory":"/workspace"}}),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .respond_with(ResponseTemplate::new(200).set_body_json(body))
                .expect(1)
                .mount(&server)
                .await;
            let error = request(&Client::new(server.uri()).unwrap(), operation)
                .await
                .unwrap_err();
            assert_eq!(error.operation(), operation);
            assert_eq!(error.status().unwrap(), 200);
            assert!(matches!(error, Error::Decode(_)));
        }
    }
    for (operation, mut data) in [
        ("model.list", model()),
        ("model.default", model()),
        ("agent.get", agent()),
    ] {
        if operation == "agent.get" {
            data["mode"] = json!("future-mode");
        } else {
            data["status"] = json!("future-status");
        }
        let data = if operation == "model.list" {
            json!([data])
        } else {
            data
        };
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"location":{"directory":"/workspace"},"data":data})),
            )
            .expect(1)
            .mount(&server)
            .await;
        assert!(matches!(
            request(&Client::new(server.uri()).unwrap(), operation)
                .await
                .unwrap_err(),
            Error::Decode(_)
        ));
    }
}

#[tokio::test]
async fn selection_preserves_typed_http_errors_and_unknown_details() {
    for (operation, status, body) in [
        (
            "model.list",
            404,
            json!({"_tag":"LocationNotFoundError","location":{"directory":"/private-workspace"},"message":"private-error"}),
        ),
        (
            "model.default",
            503,
            json!({"_tag":"ServiceUnavailableError","message":"private-error"}),
        ),
        (
            "agent.get",
            404,
            json!({"_tag":"AgentNotFoundError","agentID":false,"message":"private-error"}),
        ),
        (
            "session.switchAgent",
            404,
            json!({"_tag":"SessionNotFoundError","sessionID":"ses_test","message":"private-error"}),
        ),
        (
            "session.switchModel",
            404,
            json!({"_tag":"SessionNotFoundError","sessionID":"ses_test","message":"private-error"}),
        ),
    ] {
        let server = MockServer::start().await;
        Mock::given(wiremock::matchers::any())
            .respond_with(ResponseTemplate::new(status).set_body_json(&body))
            .expect(1)
            .mount(&server)
            .await;
        let error = request(&Client::new(server.uri()).unwrap(), operation)
            .await
            .unwrap_err();
        assert_eq!(error.operation(), operation);
        assert_eq!(error.status().unwrap(), status);
        assert!(!format!("{error} {error:?}").contains("private-"));
        let Error::Http(error) = error else {
            panic!("expected HTTP error")
        };
        match operation {
            "model.list" => assert!(matches!(
                error.api_error(),
                Some(ApiError::LocationNotFound { .. })
            )),
            "model.default" => assert!(matches!(
                error.api_error(),
                Some(ApiError::ServiceUnavailable { .. })
            )),
            "agent.get" => assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(_)))),
            _ => assert!(matches!(
                error.api_error(),
                Some(ApiError::SessionNotFound { .. })
            )),
        }
        assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), body);
    }
}

#[tokio::test]
async fn switches_reject_other_successes_redirects_and_failures_without_retry() {
    for operation in ["session.switchAgent", "session.switchModel"] {
        for status in [200, 202, 307, 503] {
            let server = MockServer::start().await;
            Mock::given(method("POST"))
                .respond_with(
                    ResponseTemplate::new(status)
                        .insert_header("location", "/redirect")
                        .set_body_string("private-response"),
                )
                .expect(1)
                .mount(&server)
                .await;
            let error = request(&Client::new(server.uri()).unwrap(), operation)
                .await
                .unwrap_err();
            assert_eq!(error.operation(), operation);
            assert_eq!(error.status().unwrap(), status);
            assert!(matches!(error, Error::Http(_)));
            assert_eq!(server.received_requests().await.unwrap().len(), 1);
        }
    }
}
