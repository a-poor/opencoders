use opencoders::{
    Client, Error,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde_json::json;
use std::time::Duration;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, path},
};

fn response() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({
        "version": "2.test", "pid": 1, "urls": [], "paths": {"tmp": "/tmp"}
    }))
}

#[test]
fn base_url_validation_does_not_expose_credentials() {
    for url in [
        "relative/path",
        "ftp://localhost/",
        "file:///tmp/server",
        "http://user:secret-token@localhost",
        "http://localhost?token=secret-token",
        "http://localhost#secret-token",
    ] {
        let builder = Client::builder(url);
        assert!(!format!("{builder:?}").contains("secret-token"));
        let error = builder.build().unwrap_err();
        assert!(!format!("{error:?} {error}").contains("secret-token"));
    }
}

#[test]
fn invalid_bearer_header_is_a_build_error_without_its_value() {
    let error = Client::builder("http://localhost")
        .bearer_auth("secret-token\r\nInjected: bad")
        .build()
        .unwrap_err();
    assert!(!format!("{error:?} {error}").contains("secret-token"));
}

#[test]
fn explicit_connect_timeout_cannot_be_silently_ignored_for_a_supplied_client() {
    let http = reqwest::Client::new();
    assert!(
        Client::builder("http://localhost")
            .http_client(http)
            .connect_timeout(Duration::from_secs(1))
            .build()
            .is_err()
    );
}

#[tokio::test]
async fn bearer_and_application_headers_are_sent_and_debug_is_redacted() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .and(header("authorization", "Bearer secret-token"))
        .and(header("x-app-key", "private-app-key"))
        .respond_with(response())
        .expect(1)
        .mount(&server)
        .await;
    let mut headers = HeaderMap::new();
    headers.insert("x-app-key", HeaderValue::from_static("private-app-key"));
    let builder = Client::builder(server.uri())
        .default_headers(headers)
        .bearer_auth("secret-token");
    assert!(!format!("{builder:?}").contains("secret-token"));
    assert!(!format!("{builder:?}").contains("private-app-key"));
    let client = builder.build().unwrap();
    assert!(!format!("{client:?}").contains("secret-token"));
    assert!(!format!("{client:?}").contains("private-app-key"));
    client.server().info().await.unwrap();
}

#[tokio::test]
async fn injected_client_configuration_and_sdk_header_precedence_are_preserved() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .and(header("authorization", "Bearer sdk-token"))
        .and(header("x-injected", "present"))
        .and(header("user-agent", "custom-client"))
        .respond_with(response())
        .expect(1)
        .mount(&server)
        .await;
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Bearer transport-token"),
    );
    headers.insert("x-injected", HeaderValue::from_static("present"));
    let http = reqwest::Client::builder()
        .no_proxy()
        .default_headers(headers)
        .user_agent("custom-client")
        .resolve("opencode.invalid", *server.address())
        .build()
        .unwrap();
    let client = Client::builder(format!(
        "http://opencode.invalid:{}",
        server.address().port()
    ))
    .http_client(http)
    .bearer_auth("sdk-token")
    .build()
    .unwrap();
    client.server().info().await.unwrap();
}

#[tokio::test]
async fn clones_can_be_used_in_independent_tasks() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Client>();
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(response())
        .expect(2)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let cloned = client.clone();
    let task = tokio::spawn(async move { cloned.server().info().await });
    client.server().info().await.unwrap();
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn request_timeout_is_a_distinguishable_transport_failure() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(response().set_delay(Duration::from_millis(250)))
        .mount(&server)
        .await;
    let error = Client::builder(server.uri())
        .request_timeout(Duration::from_millis(30))
        .build()
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    assert!(matches!(error, Error::Transport(_)));
}

#[tokio::test]
async fn sdk_request_timeout_overrides_injected_client_default() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(response().set_delay(Duration::from_millis(100)))
        .mount(&server)
        .await;
    let http = reqwest::Client::builder()
        .timeout(Duration::from_millis(10))
        .build()
        .unwrap();
    let client = Client::builder(server.uri())
        .http_client(http)
        .request_timeout(Duration::from_secs(2))
        .build()
        .unwrap();
    client.server().info().await.unwrap();
}

#[tokio::test]
async fn default_transport_does_not_follow_redirects() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", "/other"))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/other"))
        .respond_with(response())
        .expect(0)
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
    assert_eq!(error.status().as_u16(), 302);
}

#[tokio::test]
async fn error_diagnostics_are_bounded_and_truncation_is_explicit() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(503).set_body_string("x".repeat(100)))
        .expect(1)
        .mount(&server)
        .await;
    let Error::Http(error) = Client::builder(server.uri())
        .max_error_body_bytes(12)
        .build()
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error");
    };
    assert_eq!(error.body(), b"xxxxxxxxxxxx");
    assert!(error.is_body_truncated());
    assert!(error.api_error().is_none());
}

#[tokio::test]
async fn success_body_limit_is_enforced() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(response())
        .mount(&server)
        .await;
    let error = Client::builder(server.uri())
        .max_response_bytes(8)
        .build()
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err();
    assert!(matches!(error, Error::ResponseTooLarge(_)));
}

#[tokio::test]
async fn removing_sdk_deadline_preserves_supplied_transport_deadline() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(response().set_delay(Duration::from_millis(250)))
        .mount(&server)
        .await;
    let http = reqwest::Client::builder()
        .timeout(Duration::from_millis(30))
        .build()
        .unwrap();
    let error = Client::builder(server.uri())
        .http_client(http)
        .without_request_timeout()
        .build()
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err();
    assert!(error.is_timeout());
}

#[tokio::test]
async fn percent_encoded_proxy_prefix_is_not_double_encoded() {
    let server = MockServer::start().await;
    Mock::given(path("/tenant%20space/api/info"))
        .respond_with(response())
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(format!("{}/tenant%20space", server.uri())).unwrap();
    client.server().info().await.unwrap();
}
