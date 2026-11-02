use opencoders::{
    ApiError, Client, Error, Location, RequestField,
    models::{SessionInboxDelivery, SessionInboxUpdateRequest, SessionPromptRequest},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

fn items() -> Vec<Value> {
    serde_json::from_str(include_str!("fixtures/inbox.json")).unwrap()
}

#[tokio::test]
async fn prompt_and_inbox_preserve_wire_contracts_and_ignore_location_defaults() {
    let server = MockServer::start().await;
    let root = "/proxy/api/session/ses%2F..%2F%3F%23%25%E9%9B%AA";
    let admitted = json!({"data":items()[0],"future":1});
    let listed = json!({"data":items(),"future":2});
    Mock::given(method("POST"))
        .and(path(format!("{root}/prompt")))
        .and(header("content-type", "application/json"))
        .and(body_json(
            json!({"text":"hello +&雪","resume":false,"delivery":"queue","id":null,"files":[]}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&admitted))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("{root}/inbox")))
        .respond_with(ResponseTemplate::new(200).set_body_json(&listed))
        .expect(1)
        .mount(&server)
        .await;
    for verb in ["PATCH", "DELETE"] {
        Mock::given(method(verb))
            .and(path(format!("{root}/inbox/msg_%2F..%2F%3F%23%25%E9%9B%AA")))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&server)
            .await;
    }
    let mut headers = opencoders::header::HeaderMap::new();
    headers.insert("content-type", "text/plain".parse().unwrap());
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_headers(headers)
        .default_location(Location::directory("/ignored"))
        .bearer_auth("token")
        .build()
        .unwrap();
    let id = "ses/../?#%雪".parse().unwrap();
    let inbox_id = "msg_/../?#%雪".parse().unwrap();
    let mut request = SessionPromptRequest::new("hello +&雪")
        .resume(false)
        .delivery(SessionInboxDelivery::Queue)
        .files(vec![]);
    request.id = RequestField::Null;
    assert_eq!(
        serde_json::to_value(client.sessions().prompt(&id, &request).await.unwrap()).unwrap(),
        admitted
    );
    let inbox = client.sessions().inbox(&id);
    assert_eq!(
        serde_json::to_value(inbox.list().await.unwrap()).unwrap(),
        listed
    );
    inbox
        .update(
            &inbox_id,
            &SessionInboxUpdateRequest::new(SessionInboxDelivery::Steer),
        )
        .await
        .unwrap();
    inbox.cancel(&inbox_id).await.unwrap();
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[2].body).unwrap(),
        json!({"delivery":"steer"})
    );
    assert_eq!(requests[2].headers["content-type"], "application/json");
    assert!(requests[3].body.is_empty());
    for r in requests {
        assert!(r.url.query().is_none());
        assert_eq!(r.headers["authorization"], "Bearer token");
    }
}

async fn call(client: &Client, operation: &str) -> opencoders::Result<()> {
    let id = "ses_test".parse().unwrap();
    let inbox_id = "msg_test".parse().unwrap();
    match operation {
        "session.prompt" => client
            .sessions()
            .prompt(&id, &SessionPromptRequest::new("private-text"))
            .await
            .map(|_| ()),
        "session.inbox.list" => client.sessions().inbox(&id).list().await.map(|_| ()),
        "session.inbox.update" => {
            client
                .sessions()
                .inbox(&id)
                .update(
                    &inbox_id,
                    &SessionInboxUpdateRequest::new(SessionInboxDelivery::Queue),
                )
                .await
        }
        "session.inbox.cancel" => client.sessions().inbox(&id).cancel(&inbox_id).await,
        _ => unreachable!(),
    }
}

#[tokio::test]
async fn prompt_and_inbox_errors_preserve_context_conflicts_and_raw_details() {
    for operation in [
        "session.prompt",
        "session.inbox.list",
        "session.inbox.update",
        "session.inbox.cancel",
    ] {
        for (status, body) in [
            (
                404,
                json!({"_tag":"SessionNotFoundError","sessionID":"ses_test","message":"private-error"}),
            ),
            (
                409,
                json!({"_tag":"ConflictError","resource":"msg_test","message":"private-error","future":1}),
            ),
            (
                409,
                json!({"_tag":"ConflictError","resource":1,"message":"private-error"}),
            ),
        ] {
            let server = MockServer::start().await;
            Mock::given(wiremock::matchers::any())
                .respond_with(ResponseTemplate::new(status).set_body_json(&body))
                .expect(1)
                .mount(&server)
                .await;
            let error = call(&Client::new(server.uri()).unwrap(), operation)
                .await
                .unwrap_err();
            assert_eq!(error.operation(), operation);
            assert_eq!(error.status().unwrap(), status);
            assert!(!format!("{error} {error:?}").contains("private-"));
            let Error::Http(error) = error else {
                panic!("HTTP error expected")
            };
            match (status, body["resource"].as_str()) {
                (404, _) => assert!(matches!(
                    error.api_error(),
                    Some(ApiError::SessionNotFound { .. })
                )),
                (409, Some(_)) => assert!(
                    matches!(error.api_error(),Some(ApiError::Conflict{resource:Some(id),..}) if id=="msg_test")
                ),
                _ => assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(_)))),
            }
            assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), body);
        }
    }
}

#[tokio::test]
async fn prompt_and_inbox_validate_statuses_without_retry_or_redirect() {
    for (operation, statuses) in [
        ("session.prompt", vec![201, 202, 204, 307, 503]),
        ("session.inbox.list", vec![204, 307]),
        ("session.inbox.update", vec![200, 202, 307, 503]),
        ("session.inbox.cancel", vec![200, 202, 307, 503]),
    ] {
        for status in statuses {
            let server = MockServer::start().await;
            Mock::given(wiremock::matchers::any())
                .respond_with(ResponseTemplate::new(status).insert_header("location", "/redirect"))
                .expect(1)
                .mount(&server)
                .await;
            let error = call(&Client::new(server.uri()).unwrap(), operation)
                .await
                .unwrap_err();
            assert_eq!(error.operation(), operation);
            assert_eq!(error.status().unwrap(), status);
            assert!(matches!(error, Error::Http(_)));
            assert_eq!(server.received_requests().await.unwrap().len(), 1);
        }
    }
}

#[tokio::test]
async fn prompt_and_inbox_reject_invalid_envelopes_and_enforce_body_limits() {
    for operation in ["session.prompt", "session.inbox.list"] {
        for body in [
            json!({}),
            json!({"data":null}),
            json!({"data":[{"type":"future"}]}),
        ] {
            let server = MockServer::start().await;
            Mock::given(wiremock::matchers::any())
                .respond_with(ResponseTemplate::new(200).set_body_json(body))
                .mount(&server)
                .await;
            let error = call(&Client::new(server.uri()).unwrap(), operation)
                .await
                .unwrap_err();
            assert_eq!(error.operation(), operation);
            assert!(matches!(error, Error::Decode(_)));
            let error = call(
                &Client::builder(server.uri())
                    .max_response_bytes(0)
                    .build()
                    .unwrap(),
                operation,
            )
            .await
            .unwrap_err();
            assert!(matches!(error, Error::ResponseTooLarge(_)));
        }
    }
    for operation in ["session.inbox.update", "session.inbox.cancel"] {
        let server = MockServer::start().await;
        Mock::given(wiremock::matchers::any())
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        call(
            &Client::builder(server.uri())
                .max_response_bytes(0)
                .build()
                .unwrap(),
            operation,
        )
        .await
        .unwrap();
    }
}
