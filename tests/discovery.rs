use opencoders::{ApiError, Client, Error, Location};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

fn location_info() -> Value {
    json!({"directory": "/resolved/project", "project": {
        "id": "project_1", "directory": "/resolved", "canonical": "/canonical",
        "futureProject": true
    }, "futureLocation": "retained"})
}

fn agents() -> Value {
    json!({"location": {"directory": "/resolved/project", "futureRef": 1}, "data": [{
        "id": "build", "name": "Build", "mode": "primary", "hidden": false,
        "request": {"settings": {"timeout": false, "futureSetting": {"enabled": true}},
            "headers": {"x-provider-key": "private-key"}, "body": {"temperature": 0.5},
            "futureRequest": 2},
        "permissions": [{"action": "read", "resource": "*", "effect": "allow", "futureRule": 3}],
        "model": {"id": "model", "providerID": "provider", "futureModel": 4},
        "steps": 5, "futureAgent": 6
    }], "futureEnvelope": 7})
}

async fn mount(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/proxy/api/location"))
        .respond_with(ResponseTemplate::new(200).set_body_json(location_info()))
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/agent"))
        .respond_with(ResponseTemplate::new(200).set_body_json(agents()))
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/info"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "version": "test", "pid": 1, "urls": [], "paths": {"tmp": "/tmp"}
        })))
        .mount(server)
        .await;
}

#[tokio::test]
async fn default_location_applies_only_to_location_aware_routes() {
    let server = MockServer::start().await;
    mount(&server).await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/server/work"))
        .bearer_auth("test-token")
        .build()
        .unwrap();
    client.location().get().await.unwrap();
    client.agents().list().await.unwrap();
    client.server().info().await.unwrap();
    let requests = server.received_requests().await.unwrap();
    for request in &requests[..2] {
        assert_eq!(
            request.url.query_pairs().collect::<Vec<_>>(),
            [("location[directory]".into(), "/server/work".into())]
        );
        assert_eq!(request.headers["authorization"], "Bearer test-token");
    }
    assert!(requests[2].url.query().is_none());
}

#[tokio::test]
async fn overrides_and_omission_do_not_mutate_shared_defaults() {
    let server = MockServer::start().await;
    mount(&server).await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/default"))
        .build()
        .unwrap();
    client
        .location()
        .get_at(Location::directory("/override"))
        .await
        .unwrap();
    client
        .agents()
        .list_at(Location::Unspecified)
        .await
        .unwrap();
    let clone = client.clone();
    tokio::spawn(async move { clone.agents().list_at(Location::directory("/task")).await })
        .await
        .unwrap()
        .unwrap();
    client.location().get().await.unwrap();
    let requests = server.received_requests().await.unwrap();
    let directories: Vec<_> = requests
        .iter()
        .map(|r| {
            r.url
                .query_pairs()
                .find(|(k, _)| k == "location[directory]")
                .map(|(_, v)| v.into_owned())
        })
        .collect();
    assert_eq!(
        directories,
        [
            Some("/override".into()),
            None,
            Some("/task".into()),
            Some("/default".into())
        ]
    );
}

#[tokio::test]
async fn directory_strings_are_encoded_once_without_local_path_interpretation() {
    let server = MockServer::start().await;
    mount(&server).await;
    let client = Client::new(format!("{}/proxy", server.uri())).unwrap();
    for directory in [
        "/remote/a b/+&?#%=雪",
        r"C:\remote\project",
        "../remote-only",
        "null",
        "",
    ] {
        client
            .agents()
            .list_at(Location::directory(directory))
            .await
            .unwrap();
        client
            .location()
            .get_at(Location::directory(directory))
            .await
            .unwrap();
    }
    for (request, expected) in server.received_requests().await.unwrap().iter().zip(
        [
            "/remote/a b/+&?#%=雪",
            r"C:\remote\project",
            "../remote-only",
            "null",
            "",
        ]
        .into_iter()
        .flat_map(|d| [d, d]),
    ) {
        assert_eq!(
            request.url.query_pairs().collect::<Vec<_>>(),
            [("location[directory]".into(), expected.into())]
        );
        assert!(request.url.fragment().is_none());
    }
}

#[tokio::test]
async fn unspecified_location_omits_the_query_and_keeps_application_headers() {
    let server = MockServer::start().await;
    mount(&server).await;
    let mut headers = opencoders::header::HeaderMap::new();
    headers.insert("x-opencode-directory", "/header-location".parse().unwrap());
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_headers(headers)
        .build()
        .unwrap();
    client.location().get().await.unwrap();
    client.agents().list().await.unwrap();
    for request in server.received_requests().await.unwrap() {
        assert!(request.url.query().is_none());
        assert_eq!(request.headers["x-opencode-directory"], "/header-location");
    }
}

#[tokio::test]
async fn discovery_preserves_resolved_location_and_additive_response_fields() {
    let server = MockServer::start().await;
    mount(&server).await;
    let client = Client::new(format!("{}/proxy", server.uri())).unwrap();
    let location = client.location().get().await.unwrap();
    assert_eq!(location.directory.as_str(), "/resolved/project");
    assert_eq!(serde_json::to_value(location).unwrap(), location_info());
    let response = client.agents().list().await.unwrap();
    assert_eq!(response.location.directory.as_str(), "/resolved/project");
    assert_eq!(response.data[0].name, "Build");
    assert_eq!(response.data[0].steps, Some(5));
    assert_eq!(serde_json::to_value(response).unwrap(), agents());
}

#[tokio::test]
async fn location_and_service_errors_are_typed_and_redacted() {
    for (route, status, body) in [
        (
            "location",
            404,
            json!({"_tag": "LocationNotFoundError", "message": "private-text", "location": {"directory": "/private-path", "future": true}}),
        ),
        (
            "agent",
            503,
            json!({"_tag": "ServiceUnavailableError", "message": "private-text", "service": "private-service"}),
        ),
    ] {
        let server = MockServer::start().await;
        Mock::given(path(format!("/api/{route}")))
            .respond_with(ResponseTemplate::new(status).set_body_json(&body))
            .mount(&server)
            .await;
        let client = Client::new(server.uri()).unwrap();
        let error = if route == "location" {
            client.location().get().await.unwrap_err()
        } else {
            client.agents().list().await.unwrap_err()
        };
        assert_eq!(
            error.operation(),
            if route == "location" {
                "location.get"
            } else {
                "agent.list"
            }
        );
        assert_eq!(error.status().unwrap().as_u16(), status);
        assert!(!format!("{error:?} {error}").contains("private-"));
        let Error::Http(error) = error else {
            panic!("expected HTTP error")
        };
        assert!(!format!("{:?}", error.api_error()).contains("private-"));
        assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), body);
        match error.api_error().unwrap() {
            ApiError::LocationNotFound { location, message } => {
                assert_eq!(location.directory.as_str(), "/private-path");
                assert_eq!(message, "private-text");
            }
            ApiError::ServiceUnavailable { service, message } => {
                assert_eq!(service.as_deref(), Some("private-service"));
                assert_eq!(message, "private-text");
            }
            _ => panic!("expected typed discovery error"),
        }
    }
}

#[tokio::test]
async fn malformed_or_unrecognized_discovery_errors_remain_inspectable() {
    for body in [
        json!({"_tag": "LocationNotFoundError", "message": "missing location"}),
        json!({"_tag": "ServiceUnavailableError", "message": "wrong service", "service": 3}),
        json!({"_tag": "NewError", "context": true}),
    ] {
        let server = MockServer::start().await;
        Mock::given(path("/api/agent"))
            .respond_with(ResponseTemplate::new(503).set_body_json(&body))
            .mount(&server)
            .await;
        let Error::Http(error) = Client::new(server.uri())
            .unwrap()
            .agents()
            .list()
            .await
            .unwrap_err()
        else {
            panic!("expected HTTP error")
        };
        assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(raw)) if raw == &body));
    }
}

#[tokio::test]
async fn malformed_success_envelopes_are_decode_errors() {
    for body in [
        json!([]),
        json!({"data": []}),
        json!({"location": {"directory": "/remote"}}),
        json!({"location": {"directory": "/remote"}, "data": [{"name": "missing required fields"}]}),
    ] {
        let server = MockServer::start().await;
        Mock::given(path("/api/agent"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let error = Client::new(server.uri())
            .unwrap()
            .agents()
            .list()
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "agent.list");
        assert!(matches!(error, Error::Decode(_)));
    }
}
