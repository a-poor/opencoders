use opencoders::{ApiError, Client, Error};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

fn info() -> Value {
    json!({"version": "2.test", "pid": 42, "urls": ["http://localhost:4096"],
           "paths": {"tmp": "/server/tmp"}})
}

#[tokio::test]
async fn server_info_uses_get_and_decodes_the_direct_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/info"))
        .and(header("accept", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(info()))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let response = client.server().info().await.unwrap();
    assert_eq!(response.version, "2.test");
    assert_eq!(response.pid, 42);
    assert_eq!(response.paths.tmp, "/server/tmp");
    assert!(response.hostname.is_none());
    assert!(response.capabilities.is_none());
}

#[tokio::test]
async fn base_url_prefix_is_preserved_with_or_without_a_trailing_slash() {
    let server = MockServer::start().await;
    Mock::given(path("/proxy/opencode/api/info"))
        .respond_with(ResponseTemplate::new(200).set_body_json(info()))
        .expect(2)
        .mount(&server)
        .await;
    for suffix in ["/proxy/opencode", "/proxy/opencode/"] {
        let client = Client::new(format!("{}{suffix}", server.uri())).unwrap();
        client.server().info().await.unwrap();
    }
}

#[tokio::test]
async fn response_accepts_and_preserves_additive_fields_at_every_level() {
    let server = MockServer::start().await;
    let mut value = info();
    value["newField"] = json!({"nested": true});
    value["paths"]["cache"] = json!("/server/cache");
    value["capabilities"] = json!({"persistentPty": true, "futureCapability": [1, 2]});
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&value))
        .mount(&server)
        .await;
    let response = Client::new(server.uri())
        .unwrap()
        .server()
        .info()
        .await
        .unwrap();
    assert_eq!(
        response.capabilities.as_ref().unwrap().persistent_pty,
        Some(true)
    );
    assert_eq!(serde_json::to_value(response).unwrap(), value);
}

#[tokio::test]
async fn known_api_errors_keep_status_operation_and_structured_details() {
    let server = MockServer::start().await;
    let value = json!({"_tag": "InvalidRequestError", "message": "bad input",
                       "kind": "query", "field": "location", "future": true});
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&value))
        .expect(1)
        .mount(&server)
        .await;
    let Error::Http(error) = Client::new(server.uri())
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error");
    };
    assert_eq!(error.status().as_u16(), 400);
    assert_eq!(error.operation(), "server.info");
    let Some(ApiError::InvalidRequest {
        message,
        kind,
        field,
    }) = error.api_error()
    else {
        panic!("expected invalid-request details");
    };
    assert_eq!(message, "bad input");
    assert_eq!(kind.as_deref(), Some("query"));
    assert_eq!(field.as_deref(), Some("location"));
    assert_eq!(
        serde_json::from_slice::<Value>(error.body()).unwrap(),
        value
    );
    assert!(!error.is_body_truncated());
}

#[tokio::test]
async fn authentication_errors_are_typed() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "_tag": "UnauthorizedError", "message": "authentication required"
        })))
        .mount(&server)
        .await;
    let Error::Http(error) = Client::new(server.uri())
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error");
    };
    assert!(matches!(
        error.api_error(),
        Some(ApiError::Unauthorized { .. })
    ));
}

#[tokio::test]
async fn unknown_or_malformed_api_errors_retain_their_json() {
    for value in [
        json!({"_tag": "FutureError", "context": [1, 2]}),
        json!({"_tag": "UnauthorizedError", "message": 42}),
    ] {
        let server = MockServer::start().await;
        Mock::given(path("/api/info"))
            .respond_with(ResponseTemplate::new(500).set_body_json(&value))
            .expect(1)
            .mount(&server)
            .await;
        let Error::Http(error) = Client::new(server.uri())
            .unwrap()
            .server()
            .info()
            .await
            .unwrap_err()
        else {
            panic!("expected HTTP error");
        };
        let Some(ApiError::Unrecognized(raw)) = error.api_error() else {
            panic!("expected retained JSON");
        };
        assert_eq!(raw, &value);
    }
}

#[tokio::test]
async fn non_json_http_errors_remain_inspectable_without_entering_logs() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(502).set_body_string("secret-token upstream failure"))
        .mount(&server)
        .await;
    let error = Client::new(server.uri())
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("secret-token"));
    let Error::Http(error) = error else {
        panic!("expected HTTP error");
    };
    assert_eq!(error.body(), b"secret-token upstream failure");
    assert!(error.api_error().is_none());
}

#[tokio::test]
async fn invalid_success_bodies_are_decode_errors() {
    for body in [
        "",
        "not JSON",
        r#"{"version":"2.test"}"#,
        r#"{"version":"secret-token","pid":"secret-token","urls":[],"paths":{"tmp":"/tmp"}}"#,
    ] {
        let server = MockServer::start().await;
        Mock::given(path("/api/info"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;
        let error = Client::new(server.uri())
            .unwrap()
            .server()
            .info()
            .await
            .unwrap_err();
        assert!(!format!("{error:?} {error}").contains("secret-token"));
        assert!(matches!(error, Error::Decode(_)));
    }
}
