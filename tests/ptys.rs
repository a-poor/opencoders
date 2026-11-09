use opencoders::{
    Client, Error, Location,
    models::{PtyCreateRequest, PtyStatus, PtyUpdateRequest},
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path, query_param},
};

fn info() -> serde_json::Value {
    json!({"location":{"directory":"/resolved"},"data":{"id":"pty_test","title":"T","command":"sh","args":[],"cwd":"/work","pid":7,"status":"running","extra":true}})
}

#[tokio::test]
async fn failed_pty_mutations_and_ticket_minting_are_not_retried() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/pty"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/pty/pty_test/connect-token"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let id = "pty_test".parse().unwrap();
    assert_eq!(
        client
            .ptys()
            .create(&PtyCreateRequest::new())
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        503
    );
    assert_eq!(
        client
            .ptys()
            .update(&id, &PtyUpdateRequest::new())
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        503
    );
    assert_eq!(
        client
            .ptys()
            .remove(&id)
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        503
    );
    assert_eq!(
        client
            .ptys()
            .connect_token(&id)
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        503
    );
}

#[tokio::test]
async fn pty_lifecycle_preserves_location_envelopes_and_strict_request_bodies() {
    let server = MockServer::start().await;
    let detail = info();
    Mock::given(method("POST"))
        .and(path("/proxy/api/pty"))
        .and(query_param("location[directory]", "/work"))
        .and(body_json(
            json!({"command":"sh","args":[],"env":{},"title":""}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&detail))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/pty"))
        .and(query_param("location[directory]", "/work"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"location":{"directory":"/work"},"data":[detail["data"]]})),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/pty/pty_test"))
        .and(query_param("location[directory]", "/work"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&detail))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/proxy/api/pty/pty_test"))
        .and(body_json(
            json!({"title":"new","size":{"rows":24,"cols":80}}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(&detail))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/proxy/api/pty/pty_test"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/work"))
        .build()
        .unwrap();
    let created = client
        .ptys()
        .create(
            &PtyCreateRequest::new()
                .command("sh")
                .args(Vec::<String>::new())
                .env(Default::default())
                .title(""),
        )
        .await
        .unwrap();
    assert_eq!(created.data.extra["extra"], true);
    assert_eq!(client.ptys().list().await.unwrap().data.len(), 1);
    assert_eq!(
        client
            .ptys()
            .get(&created.data.id)
            .await
            .unwrap()
            .data
            .status,
        PtyStatus::Running
    );
    client
        .ptys()
        .update_at(
            &created.data.id,
            &PtyUpdateRequest::new().title("new").size(24, 80),
            Location::Unspecified,
        )
        .await
        .unwrap();
    client
        .ptys()
        .remove_at(&created.data.id, Location::Unspecified)
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    for request in &requests[3..] {
        assert!(request.url.query().is_none());
    }
}

#[tokio::test]
async fn ticket_requests_set_required_header_and_encode_ids_without_leaking_secrets() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(header("x-opencode-ticket","1")).and(query_param("location[directory]","/override"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"location":{"directory":"/override"},"data":{"ticket":"private-ticket","expires_in":60}}))).expect(1).mount(&server).await;
    let client = Client::new(server.uri()).unwrap();
    let result = client
        .ptys()
        .connect_token_at(
            &"pty /?#%".parse().unwrap(),
            Location::directory("/override"),
        )
        .await
        .unwrap();
    assert_eq!(result.data.ticket, "private-ticket");
    assert!(!format!("{result:?}").contains("private-ticket"));
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests[0].url.path(),
        "/api/pty/pty%20%2F%3F%23%25/connect-token"
    );
    assert!(requests[0].body.is_empty());
}

#[tokio::test]
async fn invalid_terminal_sizes_fail_before_http_and_not_found_stays_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(404).set_body_json(
            json!({"_tag":"PtyNotFoundError","ptyID":"pty_test","message":"private-message"}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let id = "pty_test".parse().unwrap();
    for (rows, cols) in [(0, 80), (24, 0), (9_007_199_254_740_992, 80)] {
        assert!(matches!(
            client
                .ptys()
                .update(&id, &PtyUpdateRequest::new().size(rows, cols))
                .await,
            Err(Error::Encode(_))
        ));
    }
    let error = client.ptys().get(&id).await.unwrap_err();
    assert!(!format!("{error:?}").contains("private-message"));
    let Error::Http(error) = error else {
        panic!("HTTP error expected")
    };
    assert!(
        matches!(error.api_error(),Some(opencoders::ApiError::PtyNotFound {pty_id,..}) if pty_id=="pty_test")
    );
}
