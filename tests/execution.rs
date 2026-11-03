use opencoders::{
    ApiError, Client, Error, Location, SessionInterruptOptions, SessionWaitOptions,
    models::SessionInterruptResponse,
};
use serde_json::{Value, json};
use std::time::Duration;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

#[test]
fn interrupt_response_requires_a_boolean_and_preserves_extensions() {
    for interrupted in [false, true] {
        let wire = json!({"interrupted":interrupted,"future":{"value":1}});
        let response: SessionInterruptResponse = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(response.interrupted, interrupted);
        assert_eq!(response.extra["future"], json!({"value":1}));
        assert_eq!(serde_json::to_value(response).unwrap(), wire);
    }
    for invalid in [
        json!({}),
        json!({"interrupted":null}),
        json!({"interrupted":"false"}),
    ] {
        assert!(serde_json::from_value::<SessionInterruptResponse>(invalid).is_err());
    }
}

#[tokio::test]
async fn interrupt_preserves_resume_omission_and_false_and_encodes_the_session_id() {
    for resume in [None, Some(false), Some(true)] {
        let server = MockServer::start().await;
        let route = "/proxy%20prefix/api/session/ses%2F..%2F%3F%23%25%E9%9B%AA/interrupt";
        Mock::given(method("POST"))
            .and(path(route))
            .and(header("authorization", "Bearer private"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({"interrupted":true,"extra":42})),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::builder(format!("{}/proxy%20prefix", server.uri()))
            .default_location(Location::directory("/unused"))
            .bearer_auth("private")
            .build()
            .unwrap();
        let mut options = SessionInterruptOptions::default();
        if let Some(value) = resume {
            options = options.resume(value);
        }
        let response = client
            .sessions()
            .interrupt(&"ses/../?#%雪".parse().unwrap(), &options)
            .await
            .unwrap();
        assert!(response.interrupted);
        assert_eq!(response.extra["extra"], 42);
        let requests = server.received_requests().await.unwrap();
        let request = &requests[0];
        assert_eq!(
            request.url.query(),
            resume.map(|v| format!("resume={v}")).as_deref()
        );
        assert!(request.body.is_empty());
        assert!(!request.headers.contains_key("content-type"));
    }
}

#[tokio::test]
async fn wait_posts_without_a_body_query_or_location_and_accepts_only_204() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/proxy/api/experimental/session/ses%2F%3F%23%25/wait"))
        .and(header("authorization", "Bearer private"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/unused"))
        .bearer_auth("private")
        .max_response_bytes(0)
        .build()
        .unwrap();
    client
        .sessions()
        .experimental_wait(&"ses/?#%".parse().unwrap(), &SessionWaitOptions::default())
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].url.query().is_none());
    assert!(requests[0].body.is_empty());
    assert!(!requests[0].headers.contains_key("content-type"));
}

async fn call(client: &Client, wait: bool) -> opencoders::Result<()> {
    let id = "ses_test".parse().unwrap();
    if wait {
        client
            .sessions()
            .experimental_wait(&id, &SessionWaitOptions::default())
            .await
    } else {
        client
            .sessions()
            .interrupt(&id, &SessionInterruptOptions::default())
            .await
            .map(|_| ())
    }
}
fn operation(wait: bool) -> &'static str {
    if wait {
        "experimental.session.wait"
    } else {
        "session.interrupt"
    }
}

#[tokio::test]
async fn execution_errors_keep_status_typed_details_and_unknown_bodies_without_retry() {
    for wait in [false, true] {
        for (status, wire) in [
            (
                400,
                json!({"_tag":"InvalidRequestError","message":"private"}),
            ),
            (401, json!({"_tag":"UnauthorizedError","message":"private"})),
            (
                404,
                json!({"_tag":"SessionNotFoundError","sessionID":"ses_test","message":"private"}),
            ),
            (
                503,
                json!({"_tag":"ServiceUnavailableError","service":"session","message":"private"}),
            ),
            (409, json!({"_tag":"FutureConflict","message":"private"})),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("POST"))
                .respond_with(ResponseTemplate::new(status).set_body_json(wire.clone()))
                .expect(1)
                .mount(&server)
                .await;
            let error = call(&Client::new(server.uri()).unwrap(), wait)
                .await
                .unwrap_err();
            assert_eq!(error.operation(), operation(wait));
            assert_eq!(error.status().unwrap().as_u16(), status);
            assert!(!format!("{error:?} {error}").contains("private"));
            let Error::Http(error) = error else {
                panic!("expected HTTP error")
            };
            assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), wire);
            match status {
                400 => assert!(matches!(
                    error.api_error(),
                    Some(ApiError::InvalidRequest { .. })
                )),
                401 => assert!(matches!(
                    error.api_error(),
                    Some(ApiError::Unauthorized { .. })
                )),
                404 => assert!(
                    matches!(error.api_error(),Some(ApiError::SessionNotFound { session_id, .. }) if session_id == "ses_test")
                ),
                503 => assert!(matches!(
                    error.api_error(),
                    Some(ApiError::ServiceUnavailable { .. })
                )),
                _ => assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(_)))),
            }
            assert_eq!(server.received_requests().await.unwrap().len(), 1);
        }
    }
}

#[tokio::test]
async fn execution_rejects_unexpected_successes_and_redirects_and_bounds_error_bodies() {
    for wait in [false, true] {
        for status in [if wait { 200 } else { 204 }, 202, 307, 500] {
            let server = MockServer::start().await;
            Mock::given(method("POST"))
                .respond_with(
                    ResponseTemplate::new(status)
                        .insert_header("location", format!("{}/redirect", server.uri()))
                        .set_body_string("private-body"),
                )
                .expect(1)
                .mount(&server)
                .await;
            let error = call(
                &Client::builder(server.uri())
                    .max_error_body_bytes(4)
                    .build()
                    .unwrap(),
                wait,
            )
            .await
            .unwrap_err();
            assert_eq!(error.operation(), operation(wait));
            let Error::Http(error) = error else {
                panic!("expected HTTP error")
            };
            assert_eq!(error.status().as_u16(), status);
            if status != 204 {
                assert_eq!(error.body(), b"priv");
                assert!(error.is_body_truncated());
            }
            assert_eq!(server.received_requests().await.unwrap().len(), 1);
        }
    }
}

#[tokio::test]
async fn interrupt_bounds_success_and_decode_diagnostics() {
    for body in [
        "{}",
        "{\"interrupted\":null}",
        "{\"interrupted\":\"private\"}",
        "private malformed",
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .expect(1)
            .mount(&server)
            .await;
        let error = call(
            &Client::builder(server.uri())
                .max_error_body_bytes(4)
                .build()
                .unwrap(),
            false,
        )
        .await
        .unwrap_err();
        assert_eq!(error.operation(), "session.interrupt");
        assert_eq!(error.status().unwrap(), 200);
        assert!(!format!("{error:?} {error}").contains("private"));
        let Error::Decode(error) = error else {
            panic!("expected decode error")
        };
        assert_eq!(error.body(), &body.as_bytes()[..body.len().min(4)]);
    }
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"interrupted":false})))
        .expect(1)
        .mount(&server)
        .await;
    let error = call(
        &Client::builder(server.uri())
            .max_response_bytes(4)
            .build()
            .unwrap(),
        false,
    )
    .await
    .unwrap_err();
    assert!(matches!(error, Error::ResponseTooLarge(_)));
}

#[tokio::test]
async fn wait_deadline_is_independent_and_does_not_change_ordinary_requests() {
    let server = MockServer::start().await;
    Mock::given(path("/api/experimental/session/ses_test/wait"))
        .respond_with(ResponseTemplate::new(204).set_delay(Duration::from_millis(150)))
        .expect(3)
        .mount(&server)
        .await;
    Mock::given(path("/api/session/ses_test/interrupt"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"interrupted":false}))
                .set_delay(Duration::from_millis(150)),
        )
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .request_timeout(Duration::from_millis(30))
        .build()
        .unwrap();
    let id = "ses_test".parse().unwrap();
    assert!(SessionWaitOptions::default().timeout.is_none());
    for options in [
        SessionWaitOptions::default(),
        SessionWaitOptions::default().timeout(Duration::from_secs(2)),
    ] {
        client
            .sessions()
            .experimental_wait(&id, &options)
            .await
            .unwrap();
    }
    let error = client
        .sessions()
        .experimental_wait(
            &id,
            &SessionWaitOptions::default().timeout(Duration::from_millis(30)),
        )
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    assert_eq!(error.operation(), "experimental.session.wait");
    assert!(error.status().is_none());
    assert!(call(&client, false).await.unwrap_err().is_timeout());
}

#[tokio::test]
async fn wait_respects_injected_deadlines_and_explicit_total_timeout_override() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(204).set_delay(Duration::from_millis(150)))
        .expect(2)
        .mount(&server)
        .await;
    let http = reqwest::Client::builder()
        .timeout(Duration::from_millis(30))
        .build()
        .unwrap();
    let client = Client::builder(server.uri())
        .http_client(http)
        .build()
        .unwrap();
    let id = "ses_test".parse().unwrap();
    let options = SessionWaitOptions::default().timeout(Duration::from_secs(2));
    client
        .sessions()
        .experimental_wait(&id, &options)
        .await
        .unwrap();
    let error = client
        .sessions()
        .experimental_wait(&id, &options.without_timeout())
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    assert_eq!(error.operation(), "experimental.session.wait");
}
