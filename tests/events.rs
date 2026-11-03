use opencoders::{Client, Error, EventOptions, StreamErrorKind, models::Event};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

fn connected() -> String {
    json!({"id":"evt_connected","type":"server.connected","data":{}}).to_string()
}
fn frame(data: &str) -> String {
    format!("data: {data}\n\n")
}

#[tokio::test]
async fn subscription_uses_explicit_filters_headers_and_preserves_sse_metadata() {
    let server = MockServer::start().await;
    let body = format!(
        "\u{feff}: heartbeat\r\nid: cursor\r\nretry: 1500\r\nevent: notification\r\n{}",
        frame(&connected()).replace('\n', "\r\n")
    );
    Mock::given(method("GET"))
        .and(path("/proxy/api/event"))
        .and(header("accept", "text/event-stream"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(body, "text/event-stream; charset=utf-8"),
        )
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .bearer_auth("private-token")
        .default_location(opencoders::Location::directory("/ignored"))
        .build()
        .unwrap();
    let mut stream = client
        .events()
        .subscribe(&EventOptions::default().types([
            "session.created",
            "rpc.+&雪",
            "session.created",
        ]))
        .await
        .unwrap();
    let message = stream.next().await.unwrap().unwrap();
    assert!(matches!(message.event, Event::ServerConnected(_)));
    assert_eq!(message.id.as_deref(), Some("cursor"));
    assert_eq!(message.event_name, "notification");
    assert_eq!(message.retry, Some(std::time::Duration::from_millis(1500)));
    assert!(stream.next().await.is_none());
    assert!(stream.next().await.is_none());
    let r = server.received_requests().await.unwrap();
    assert_eq!(r[0].headers["authorization"], "Bearer private-token");
    assert_eq!(
        r[0].url.query_pairs().collect::<Vec<_>>(),
        [
            ("types".into(), "session.created".into()),
            ("types".into(), "rpc.+&雪".into()),
            ("types".into(), "session.created".into())
        ]
    );
    assert!(!r[0].headers.contains_key("last-event-id"));
}

#[tokio::test]
async fn unknown_events_are_retained_but_malformed_known_events_terminate() {
    let server = MockServer::start().await;
    let raw = json!({"type":"future.event","arbitrary":{"private":"payload"}});
    let body = format!(
        "{}{}{}",
        frame(&raw.to_string()),
        frame(r#"{"type":"session.text.delta"}"#),
        frame(&connected())
    );
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body, "text/event-stream"))
        .mount(&server)
        .await;
    let mut stream = Client::new(server.uri())
        .unwrap()
        .events()
        .subscribe(&EventOptions::default())
        .await
        .unwrap();
    let Event::Unknown(value) = stream.next().await.unwrap().unwrap().event else {
        panic!("unknown")
    };
    assert_eq!(value, raw);
    let error = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(error, Error::Decode(_)));
    assert_eq!(error.operation(), "event.subscribe");
    assert_eq!(error.status().unwrap(), 200);
    assert!(stream.next().await.is_none());
}

#[tokio::test]
async fn failure_frames_and_size_limits_are_terminal_and_redacted() {
    for (body,limit,kind) in [
        ("event: effect/http-api/stream/failure\ndata: [{\"_tag\":\"Die\",\"defect\":\"private-failure\"}]\n\n".to_owned(),1024,StreamErrorKind::ServerFailure),
        (format!("data: {}\n\n","x".repeat(100)),16,StreamErrorKind::FrameTooLarge),
    ] {
        let server=MockServer::start().await;
        Mock::given(method("GET")).respond_with(ResponseTemplate::new(200).set_body_raw(format!("{body}{}",frame(&connected())),"text/event-stream")).mount(&server).await;
        let mut stream=Client::builder(server.uri()).max_error_body_bytes(12).build().unwrap().events().subscribe(&EventOptions::default().max_event_bytes(limit)).await.unwrap();
        let error=stream.next().await.unwrap().unwrap_err();assert!(!format!("{error} {error:?}").contains("private-"));
        let Error::Stream(error)=error else{panic!("stream error")};assert_eq!(error.kind(),kind);
        if kind==StreamErrorKind::ServerFailure{assert_eq!(error.body().len(),12);assert!(error.is_body_truncated());}
        assert!(stream.next().await.is_none());
    }
}

#[tokio::test]
async fn handshake_validates_status_content_type_and_bounded_api_errors() {
    for limit in [0, 12] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(json!({"_tag":"UnauthorizedError","message":"private-error"})),
            )
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::builder(server.uri())
            .max_error_body_bytes(limit)
            .build()
            .unwrap()
            .events()
            .subscribe(&EventOptions::default())
            .await
            .unwrap_err();
        assert!(!format!("{error} {error:?}").contains("private-error"));
        let Error::Http(error) = error else {
            panic!("HTTP error")
        };
        assert_eq!(error.body().len(), limit);
        assert!(error.is_body_truncated());
        assert!(error.api_error().is_none());
        assert!(
            server.received_requests().await.unwrap()[0]
                .url
                .query()
                .is_none()
        );
    }
    for status in [204, 307, 401, 503] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(status)
                    .insert_header("location", "/redirect")
                    .set_body_json(json!({"_tag":"UnauthorizedError","message":"private-error"})),
            )
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::new(server.uri())
            .unwrap()
            .events()
            .subscribe(&EventOptions::default())
            .await
            .unwrap_err();
        assert_eq!(error.status().unwrap(), status);
        assert!(matches!(error, Error::Http(_)));
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }
    for content_type in [None, Some("application/json"), Some("text/event-streaming")] {
        let server = MockServer::start().await;
        let mut response = ResponseTemplate::new(200);
        if let Some(value) = content_type {
            response = response.insert_header("content-type", value);
        }
        Mock::given(method("GET"))
            .respond_with(response)
            .mount(&server)
            .await;
        let Error::Stream(error) = Client::new(server.uri())
            .unwrap()
            .events()
            .subscribe(&EventOptions::default())
            .await
            .unwrap_err()
        else {
            panic!("content type error")
        };
        assert_eq!(error.kind(), StreamErrorKind::InvalidContentType);
    }
}
