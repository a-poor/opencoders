use opencoders::{
    ApiError, Client, Error, Location, RequestField,
    models::{SessionCreateRequest, SessionId, SessionLocationInput},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

fn envelope() -> Value {
    json!({"data": serde_json::from_str::<Value>(include_str!("fixtures/session.json")).unwrap(), "future": {"kept": true}})
}

#[tokio::test]
async fn create_sends_required_empty_json_body_and_preserves_the_response_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/proxy%20prefix/api/session"))
        .and(header("content-type", "application/json"))
        .and(header("accept", "application/json"))
        .and(header("authorization", "Bearer test-token"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope()))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy%20prefix", server.uri()))
        .default_location(Location::directory("/must-not-be-injected"))
        .bearer_auth("test-token")
        .build()
        .unwrap();
    let result = client
        .sessions()
        .create(&SessionCreateRequest::default())
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(result).unwrap(), envelope());
    assert!(
        server.received_requests().await.unwrap()[0]
            .url
            .query()
            .is_none()
    );
}

#[tokio::test]
async fn create_uses_json_content_type_even_with_a_non_json_default_header() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope()))
        .expect(1)
        .mount(&server)
        .await;
    let mut headers = opencoders::header::HeaderMap::new();
    headers.insert(
        opencoders::header::CONTENT_TYPE,
        "text/plain".parse().unwrap(),
    );
    Client::builder(server.uri())
        .default_headers(headers)
        .build()
        .unwrap()
        .sessions()
        .create(&SessionCreateRequest::default())
        .await
        .unwrap();
}

#[tokio::test]
async fn create_sends_explicit_nulls_and_nested_location_without_modifying_the_request() {
    let server = MockServer::start().await;
    let mut request = SessionCreateRequest::default()
        .title("")
        .location(SessionLocationInput::new("/remote/a +雪"));
    request.parent_id = RequestField::Null;
    request.permissions = RequestField::Value(vec![].into());
    let wire = json!({"title": "", "parentID": null, "location": {"directory": "/remote/a +雪"}, "permissions": []});
    Mock::given(method("POST"))
        .and(path("/api/session"))
        .and(body_json(wire.clone()))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope()))
        .expect(1)
        .mount(&server)
        .await;
    Client::new(server.uri())
        .unwrap()
        .sessions()
        .create(&request)
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(request).unwrap(), wire);
}

#[tokio::test]
async fn get_encodes_a_session_id_as_one_segment_without_changing_proxy_prefix_or_query() {
    let server = MockServer::start().await;
    let id: SessionId = "ses/../?x=#%2F\\雪".parse().unwrap();
    Mock::given(method("GET"))
        .and(path(
            "/proxy%20prefix/api/session/ses%2F..%2F%3Fx=%23%252F%5C%E9%9B%AA",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope()))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy%20prefix/", server.uri()))
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let result = client.sessions().get(&id).await.unwrap();
    assert_eq!(serde_json::to_value(result).unwrap(), envelope());
    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].url.query().is_none());
    assert!(requests[0].body.is_empty());
}

#[tokio::test]
async fn create_and_get_decode_missing_session_errors_without_requiring_an_id_prefix() {
    let server = MockServer::start().await;
    let wire = json!({"_tag": "SessionNotFoundError", "sessionID": "server-reported-id", "message": "private reason", "future": true});
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(404).set_body_json(wire.clone()))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(404).set_body_json(wire.clone()))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    for (operation, result) in [
        (
            "session.create",
            client
                .sessions()
                .create(&SessionCreateRequest::default())
                .await,
        ),
        (
            "session.get",
            client.sessions().get(&"ses_missing".parse().unwrap()).await,
        ),
    ] {
        let Error::Http(error) = result.unwrap_err() else {
            panic!("expected HTTP error")
        };
        assert_eq!(error.status(), 404);
        assert_eq!(error.operation(), operation);
        let Some(ApiError::SessionNotFound {
            session_id,
            message,
        }) = error.api_error()
        else {
            panic!("expected missing session")
        };
        assert_eq!(session_id, "server-reported-id");
        assert_eq!(message, "private reason");
        assert!(!format!("{error:?} {error}").contains("private reason"));
        assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), wire);
    }
}

#[tokio::test]
async fn malformed_known_session_errors_remain_inspectable() {
    let server = MockServer::start().await;
    let wire = json!({"_tag": "SessionNotFoundError", "sessionID": 7, "message": "missing"});
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(404).set_body_json(wire.clone()))
        .mount(&server)
        .await;
    let Error::Http(error) = Client::new(server.uri())
        .unwrap()
        .sessions()
        .get(&"ses_missing".parse().unwrap())
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error")
    };
    assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(value)) if value == &wire));
}

#[tokio::test]
async fn create_does_not_follow_redirects_retry_server_errors_or_accept_undocumented_successes() {
    for status in [307, 503, 201, 204] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(status)
                    .insert_header("location", format!("{}/redirected", server.uri()))
                    .set_body_json(envelope()),
            )
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::new(server.uri())
            .unwrap()
            .sessions()
            .create(&SessionCreateRequest::default())
            .await
            .unwrap_err();
        assert_eq!(error.status().unwrap().as_u16(), status);
        assert_eq!(error.operation(), "session.create");
        assert!(matches!(error, Error::Http(_)));
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }
}

#[tokio::test]
async fn create_response_limits_and_decode_errors_keep_operation_context() {
    for (limit, body) in [(4, envelope().to_string()), (1024, "{}".to_owned())] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::builder(server.uri())
            .max_response_bytes(limit)
            .build()
            .unwrap()
            .sessions()
            .create(&SessionCreateRequest::default())
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "session.create");
        assert_eq!(error.status().unwrap(), 200);
        if limit == 4 {
            assert!(matches!(error, Error::ResponseTooLarge(_)));
        } else {
            assert!(matches!(error, Error::Decode(_)));
        }
    }
}
