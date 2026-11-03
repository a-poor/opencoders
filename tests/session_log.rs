use opencoders::{
    Client, Error, SessionLogOptions, StreamErrorKind,
    models::{Event, SessionLogItem},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

fn event(seq: u64) -> Value {
    json!({"id":format!("evt_{seq}"),"created":1,"type":"session.renamed","durable":{"aggregateID":"ses_log","seq":seq,"version":1},"data":{"sessionID":"ses_log","title":format!("title {seq}")}})
}
fn frame(id: &str, data: Value) -> String {
    format!("id: {id}\nevent: message\ndata: {data}\n\n")
}

#[tokio::test]
async fn log_encodes_explicit_queries_and_segments_and_retains_markers_and_sse_ids() {
    let server = MockServer::start().await;
    let body = format!(
        "{}{}{}",
        frame("opaque-2", event(2)),
        frame("opaque-5", event(5)),
        frame(
            "watermark",
            json!({"type":"log.synced","aggregateID":"ses_log","seq":9,"future":true})
        )
    );
    Mock::given(method("GET"))
        .and(path(
            "/proxy/api/experimental/session/ses_a%2Fb%3F%23%25/log",
        ))
        .and(header("accept", "text/event-stream"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body, "text/event-stream"))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .bearer_auth("private-token")
        .default_location(opencoders::Location::directory("/ignored"))
        .build()
        .unwrap();
    let mut stream = client
        .sessions()
        .experimental_log(
            &"ses_a/b?#%".parse().unwrap(),
            &SessionLogOptions::default().after(0).follow(false),
        )
        .await
        .unwrap();
    for (seq, id) in [(2, "opaque-2"), (5, "opaque-5")] {
        let message = stream.next().await.unwrap().unwrap();
        assert_eq!(message.id.as_deref(), Some(id));
        assert_eq!(message.item.sequence(), Some(seq));
        assert_eq!(message.item.aggregate_id(), Some("ses_log"));
        assert!(
            matches!(message.item,SessionLogItem::Event(e) if matches!(*e,Event::SessionRenamed(_)))
        );
    }
    let message = stream.next().await.unwrap().unwrap();
    assert_eq!(message.id.as_deref(), Some("watermark"));
    assert_eq!(message.item.sequence(), Some(9));
    let SessionLogItem::Synced(marker) = message.item else {
        panic!("marker")
    };
    assert_eq!(marker.extra["future"], true);
    assert!(stream.next().await.is_none());
    assert!(stream.next().await.is_none());
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests[0].url.query(), Some("after=0&follow=false"));
    assert_eq!(requests[0].headers["authorization"], "Bearer private-token");
    assert!(!requests[0].headers.contains_key("last-event-id"));
}

#[tokio::test]
async fn defaults_omit_queries_and_unknown_items_survive_without_an_inferred_cursor() {
    let server = MockServer::start().await;
    let raw = json!({"type":"future.log-item","durable":{"seq":90},"private":"payload"});
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            format!(
                "{}{}",
                frame("untrusted-cursor", raw.clone()),
                frame("", json!({"type":"log.synced","aggregateID":"ses_log"}))
            ),
            "text/event-stream",
        ))
        .mount(&server)
        .await;
    let mut stream = Client::new(server.uri())
        .unwrap()
        .sessions()
        .experimental_log(&"ses_log".parse().unwrap(), &SessionLogOptions::default())
        .await
        .unwrap();
    let first = stream.next().await.unwrap().unwrap();
    assert!(matches!(first.item,SessionLogItem::Unknown(ref v) if v==&raw));
    assert_eq!(first.item.sequence(), None);
    let marker = stream.next().await.unwrap().unwrap();
    assert_eq!(marker.item.sequence(), None);
    assert!(matches!(marker.item, SessionLogItem::Synced(_)));
    assert!(stream.next().await.is_none());
    assert!(
        server.received_requests().await.unwrap()[0]
            .url
            .query()
            .is_none()
    );
}

#[tokio::test]
async fn log_failures_keep_operation_context_and_terminate() {
    for (body, limit, expected) in [
        (
            frame(
                "",
                json!({"type":"log.synced","aggregateID":"ses_log","seq":null}),
            ),
            4096,
            "decode",
        ),
        (
            frame("", json!({"type":"session.text.delta"})),
            4096,
            "decode",
        ),
        (
            "event: effect/http-api/stream/failure\ndata: private-failure\n\n".into(),
            4096,
            "failure",
        ),
        (format!("data:{}\n\n", "x".repeat(100)), 10, "limit"),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "text/event-stream"))
            .mount(&server)
            .await;
        let mut stream = Client::builder(server.uri())
            .max_error_body_bytes(5)
            .build()
            .unwrap()
            .sessions()
            .experimental_log(
                &"ses_log".parse().unwrap(),
                &SessionLogOptions::default().max_event_bytes(limit),
            )
            .await
            .unwrap();
        let error = stream.next().await.unwrap().unwrap_err();
        assert_eq!(error.operation(), "experimental.session.log");
        assert_eq!(error.status().unwrap(), 200);
        assert!(!format!("{error} {error:?}").contains("private"));
        match (expected, error) {
            ("decode", Error::Decode(e)) => {
                assert_eq!(e.body().len(), 5);
                assert!(e.is_body_truncated());
            }
            ("failure", Error::Stream(e)) => {
                assert_eq!(e.kind(), StreamErrorKind::ServerFailure);
                assert_eq!(e.body().len(), 5);
                assert!(e.is_body_truncated());
            }
            ("limit", Error::Stream(e)) => assert_eq!(e.kind(), StreamErrorKind::FrameTooLarge),
            _ => panic!("unexpected error"),
        }
        assert!(stream.next().await.is_none());
    }
}

#[tokio::test]
async fn log_status_errors_are_typed_bounded_and_not_retried() {
    for (status, tag) in [
        (400, "InvalidRequestError"),
        (401, "UnauthorizedError"),
        (404, "SessionNotFoundError"),
        (204, "Unexpected"),
        (307, "Unexpected"),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(status)
                    .insert_header("location", "/redirect")
                    .set_body_json(
                        json!({"_tag":tag,"sessionID":"ses_log","message":"private-error"}),
                    ),
            )
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::new(server.uri())
            .unwrap()
            .sessions()
            .experimental_log(
                &"ses_log".parse().unwrap(),
                &SessionLogOptions::default().after(u64::MAX).follow(true),
            )
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "experimental.session.log");
        assert_eq!(error.status().unwrap(), status);
        let Error::Http(e) = error else {
            panic!("HTTP")
        };
        if status == 404 {
            assert!(matches!(
                e.api_error(),
                Some(opencoders::ApiError::SessionNotFound { .. })
            ));
        }
        assert_eq!(
            server.received_requests().await.unwrap()[0].url.query(),
            Some("after=18446744073709551615&follow=true")
        );
    }
}
