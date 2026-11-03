use opencoders::{
    ApiError, Client, Error, Location, RequestField,
    models::{SessionForkRequest, SessionId, SessionMessageId, SessionResponse},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

fn envelope() -> Value {
    json!({"data":serde_json::from_str::<Value>(include_str!("fixtures/session.json")).unwrap(),"future":true})
}

#[test]
fn fork_input_preserves_omitted_null_and_typed_boundary_without_extra_fields() {
    let mut request = SessionForkRequest::default();
    assert_eq!(serde_json::to_value(&request).unwrap(), json!({}));
    request.before = RequestField::Null;
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"before":null})
    );
    let id: SessionMessageId = "msg_/../?雪".parse().unwrap();
    request = request.before(id);
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"before":"msg_/../?雪"})
    );
    for wire in [
        json!({}),
        json!({"before":null}),
        json!({"before":"msg_test"}),
    ] {
        let v: SessionForkRequest = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(v).unwrap(), wire);
    }
    for wire in [
        json!({"before":"wrong"}),
        json!({"before":false}),
        json!({"before":{}}),
        json!({"parentID":"ses_x"}),
        json!({"before":"msg_x","title":"extra"}),
    ] {
        assert!(serde_json::from_value::<SessionForkRequest>(wire).is_err());
    }
}

#[tokio::test]
async fn fork_posts_required_json_with_encoded_id_and_preserves_the_envelope() {
    let server = MockServer::start().await;
    let client = Client::builder(format!("{}/proxy%20prefix", server.uri()))
        .bearer_auth("secret")
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let id: SessionId = "ses/../?x=#%2F雪".parse().unwrap();
    for before in [
        RequestField::Omitted,
        RequestField::Null,
        RequestField::Value("msg_/../?雪".parse().unwrap()),
    ] {
        let mut request = SessionForkRequest::default();
        request.before = before;
        let wire = serde_json::to_value(&request).unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/proxy%20prefix/api/session/ses%2F..%2F%3Fx=%23%252F%E9%9B%AA/fork",
            ))
            .and(header("content-type", "application/json"))
            .and(header("accept", "application/json"))
            .and(header("authorization", "Bearer secret"))
            .and(body_json(wire.clone()))
            .respond_with(ResponseTemplate::new(200).set_body_json(envelope()))
            .expect(1)
            .mount(&server)
            .await;
        let response = client.sessions().fork(&id, &request).await.unwrap();
        assert_eq!(serde_json::to_value(response).unwrap(), envelope());
        assert_eq!(serde_json::to_value(request).unwrap(), wire);
    }
    assert!(
        server
            .received_requests()
            .await
            .unwrap()
            .iter()
            .all(|r| r.url.query().is_none())
    );
}

#[test]
fn fork_provenance_preserves_both_boundaries_and_rejects_malformed_known_shapes() {
    for kind in ["before", "through"] {
        let mut raw = envelope();
        raw["data"]["fork"]["boundary"]["type"] = json!(kind);
        let v: SessionResponse = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(v).unwrap(), raw);
        for key in ["type", "messageID"] {
            let mut bad = raw.clone();
            bad["data"]["fork"]["boundary"]
                .as_object_mut()
                .unwrap()
                .remove(key);
            assert!(serde_json::from_value::<SessionResponse>(bad).is_err());
        }
    }
    let mut bad = envelope();
    bad["data"]["fork"]["boundary"]["type"] = json!("future");
    assert!(serde_json::from_value::<SessionResponse>(bad).is_err());
}

#[tokio::test]
async fn fork_errors_retain_kind_ids_raw_details_and_redact_diagnostics() {
    for (status, wire) in [
        (
            400,
            json!({"_tag":"InvalidRequestError","kind":"empty_session","message":"private"}),
        ),
        (401, json!({"_tag":"UnauthorizedError","message":"private"})),
        (
            404,
            json!({"_tag":"SessionNotFoundError","sessionID":"unprefixed","message":"private"}),
        ),
        (
            404,
            json!({"_tag":"MessageNotFoundError","sessionID":"unprefixed","messageID":"raw-id","message":"private"}),
        ),
        (
            404,
            json!({"_tag":"MessageNotFoundError","message":"private"}),
        ),
        (500, json!({"_tag":"FutureError","details":["private"]})),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(status).set_body_json(&wire))
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::new(server.uri())
            .unwrap()
            .sessions()
            .fork(&"ses_test".parse().unwrap(), &SessionForkRequest::default())
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "session.fork");
        assert_eq!(error.status().unwrap().as_u16(), status);
        assert!(!format!("{error:?} {error}").contains("private"));
        let Error::Http(error) = error else {
            panic!("HTTP error")
        };
        assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), wire);
        match wire["_tag"].as_str().unwrap() {
            "InvalidRequestError" => assert!(
                matches!(error.api_error(),Some(ApiError::InvalidRequest{kind:Some(k),..}) if k=="empty_session")
            ),
            "UnauthorizedError" => assert!(matches!(
                error.api_error(),
                Some(ApiError::Unauthorized { .. })
            )),
            "SessionNotFoundError" => assert!(
                matches!(error.api_error(),Some(ApiError::SessionNotFound{session_id,..}) if session_id=="unprefixed")
            ),
            "MessageNotFoundError" if wire.get("messageID").is_some() => assert!(
                matches!(error.api_error(),Some(ApiError::MessageNotFound{session_id,message_id,..}) if session_id=="unprefixed"&&message_id=="raw-id")
            ),
            _ => assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(_)))),
        }
    }
}

#[tokio::test]
async fn fork_enforces_statuses_and_response_limits_without_retry_or_redirect() {
    for status in [201, 202, 204, 307, 503] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(status)
                    .set_body_string("private")
                    .insert_header("location", format!("{}/redirect", server.uri())),
            )
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::builder(server.uri())
            .max_error_body_bytes(4)
            .build()
            .unwrap()
            .sessions()
            .fork(&"ses_test".parse().unwrap(), &SessionForkRequest::default())
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "session.fork");
        let Error::Http(error) = error else {
            panic!("HTTP error")
        };
        assert_eq!(error.status().as_u16(), status);
        if status != 204 {
            assert_eq!(error.body(), b"priv");
            assert!(error.is_body_truncated());
        }
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }
    for (limit, body) in [
        (4, "private malformed"),
        (1000, "private malformed"),
        (1000, "{}"),
        (1000, "{\"data\":null}"),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::builder(server.uri())
            .max_response_bytes(limit)
            .max_error_body_bytes(4)
            .build()
            .unwrap()
            .sessions()
            .fork(&"ses_test".parse().unwrap(), &SessionForkRequest::default())
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "session.fork");
        assert_eq!(error.status().unwrap(), 200);
        assert!(!format!("{error:?} {error}").contains("private"));
        if limit == 4 {
            assert!(matches!(error, Error::ResponseTooLarge(_)));
        } else {
            assert!(matches!(error, Error::Decode(_)));
        }
    }
}

#[tokio::test]
async fn fork_disconnect_after_accepting_the_body_does_not_retry() {
    use tokio::{
        io::AsyncReadExt,
        net::TcpListener,
        time::{Duration, timeout},
    };
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = Client::new(format!("http://{}", listener.local_addr().unwrap())).unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut head = vec![];
        while !head.ends_with(b"\r\n\r\n") {
            head.push(socket.read_u8().await.unwrap());
            assert!(head.len() < 8192);
        }
        let head = String::from_utf8(head).unwrap();
        assert!(head.starts_with("POST /api/session/ses_test/fork "));
        let length: usize = head
            .lines()
            .find_map(|l| {
                l.to_ascii_lowercase()
                    .strip_prefix("content-length: ")
                    .map(str::to_owned)
            })
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let mut body = vec![0; length];
        socket.read_exact(&mut body).await.unwrap();
        assert_eq!(body, b"{}");
        drop(socket);
        assert!(
            timeout(Duration::from_millis(150), listener.accept())
                .await
                .is_err()
        );
    });
    let error = client
        .sessions()
        .fork(&"ses_test".parse().unwrap(), &SessionForkRequest::default())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Transport(_)));
    assert_eq!(error.operation(), "session.fork");
    assert!(error.status().is_none());
    server.await.unwrap();
}

#[tokio::test]
async fn fork_deadline_covers_success_and_error_body_reads() {
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
        time::{Duration, timeout},
    };
    for status in [200, 400] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = Client::builder(format!("http://{}", listener.local_addr().unwrap()))
            .request_timeout(Duration::from_millis(150))
            .build()
            .unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut head = vec![];
            while !head.ends_with(b"\r\n\r\n") {
                head.push(socket.read_u8().await.unwrap());
            }
            // This request's explicit empty JSON object is still a two-byte body.
            let mut body = [0; 2];
            socket.read_exact(&mut body).await.unwrap();
            assert_eq!(&body, b"{}");
            socket
                .write_all(
                    format!("HTTP/1.1 {status} Test\r\nContent-Length: 100\r\n\r\n{{").as_bytes(),
                )
                .await
                .unwrap();
            assert_eq!(socket.read(&mut [0; 1]).await.unwrap(), 0);
        });
        let error = client
            .sessions()
            .fork(&"ses_test".parse().unwrap(), &SessionForkRequest::default())
            .await
            .unwrap_err();
        assert!(error.is_timeout());
        assert_eq!(error.status().unwrap(), status);
        assert_eq!(error.operation(), "session.fork");
        timeout(Duration::from_secs(2), server)
            .await
            .unwrap()
            .unwrap();
    }
}
