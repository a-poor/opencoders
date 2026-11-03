use opencoders::{Client, Location, SessionFormCancelOptions, models::*};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path},
};
const OPS: [&str; 6] = [
    "form.list",
    "session.form.list",
    "session.form.get",
    "session.form.create",
    "session.form.reply",
    "session.form.cancel",
];
pub async fn call(client: &Client, op: usize) -> opencoders::Result<()> {
    let session = "ses_test".parse().unwrap();
    let id = "frm_test".parse().unwrap();
    let forms = client.sessions().forms(&session);
    match op {
        0 => client.forms().list().await.map(|_| ()),
        1 => forms.list().await.map(|_| ()),
        2 => forms.get(&id).await.map(|_| ()),
        3 => forms
            .create(&SessionFormCreateRequest::new(
                "test",
                FormFieldsInput::new(FormBooleanFieldInput::new("ok")),
            ))
            .await
            .map(|_| ()),
        4 => {
            forms
                .reply(
                    &id,
                    &SessionFormReplyRequest::new(
                        serde_json::from_value(json!({"ok":false})).unwrap(),
                    ),
                )
                .await
        }
        _ => {
            forms
                .cancel(&id, &SessionFormCancelOptions::default())
                .await
        }
    }
}
fn verb(op: usize) -> &'static str {
    match op {
        3 | 4 => "POST",
        5 => "DELETE",
        _ => "GET",
    }
}

#[tokio::test]
async fn form_errors_are_typed_redacted_and_preserve_malformed_known_shapes() {
    use opencoders::{ApiError, Error};
    for (op, operation) in OPS.iter().enumerate() {
        for (tag, status) in [
            ("FormNotFoundError", 404),
            ("FormAlreadySettledError", 409),
            ("FormInvalidAnswerError", 400),
            ("ConflictError", 409),
            ("SessionNotFoundError", 404),
            ("LocationNotFoundError", 404),
            ("InvalidRequestError", 400),
            ("UnauthorizedError", 401),
            ("FutureError", 502),
        ] {
            for malformed in [false, true] {
                let server = MockServer::start().await;
                let mut wire = json!({"_tag":tag,"id":"unprefixed","sessionID":"unprefixed","location":{"directory":"/missing"},"message":"private"});
                if malformed {
                    wire["message"] = json!(null);
                }
                Mock::given(method(verb(op)))
                    .respond_with(ResponseTemplate::new(status).set_body_json(&wire))
                    .expect(1)
                    .mount(&server)
                    .await;
                let error = call(&Client::new(server.uri()).unwrap(), op)
                    .await
                    .unwrap_err();
                assert_eq!(error.operation(), *operation);
                assert_eq!(error.status().unwrap().as_u16(), status);
                assert!(!format!("{error:?} {error}").contains("private"));
                let Error::Http(error) = error else {
                    panic!("HTTP error")
                };
                assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), wire);
                match (tag, malformed, error.api_error()) {
                    ("FormNotFoundError", false, Some(ApiError::FormNotFound { id, .. }))
                    | (
                        "FormAlreadySettledError",
                        false,
                        Some(ApiError::FormAlreadySettled { id, .. }),
                    )
                    | (
                        "FormInvalidAnswerError",
                        false,
                        Some(ApiError::FormInvalidAnswer { id, .. }),
                    ) => assert_eq!(id, "unprefixed"),
                    ("ConflictError", false, Some(ApiError::Conflict { .. }))
                    | ("SessionNotFoundError", false, Some(ApiError::SessionNotFound { .. }))
                    | ("LocationNotFoundError", false, Some(ApiError::LocationNotFound { .. }))
                    | ("InvalidRequestError", false, Some(ApiError::InvalidRequest { .. }))
                    | ("UnauthorizedError", false, Some(ApiError::Unauthorized { .. })) => {}
                    (_, true, Some(ApiError::Unrecognized(_)))
                    | ("FutureError", _, Some(ApiError::Unrecognized(_))) => {}
                    _ => panic!("unexpected API error: {:?}", error.api_error()),
                }
            }
        }
    }
}

#[tokio::test]
async fn methods_enforce_success_statuses_bounds_and_no_retry_or_redirect() {
    use opencoders::Error;
    for (op, operation) in OPS.iter().enumerate() {
        for status in [if op >= 4 { 200 } else { 204 }, 202, 307, 503] {
            let server = MockServer::start().await;
            Mock::given(method(verb(op)))
                .respond_with(
                    ResponseTemplate::new(status)
                        .set_body_string("private")
                        .insert_header("location", format!("{}/redirect", server.uri())),
                )
                .expect(1)
                .mount(&server)
                .await;
            let client = Client::builder(server.uri())
                .max_error_body_bytes(4)
                .build()
                .unwrap();
            let error = call(&client, op).await.unwrap_err();
            assert_eq!(error.operation(), *operation);
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
        if op >= 4 {
            continue;
        }
        for limit in [4, 1000] {
            let server = MockServer::start().await;
            Mock::given(method(verb(op)))
                .respond_with(ResponseTemplate::new(200).set_body_string("private malformed"))
                .expect(1)
                .mount(&server)
                .await;
            let error = call(
                &Client::builder(server.uri())
                    .max_response_bytes(limit)
                    .max_error_body_bytes(4)
                    .build()
                    .unwrap(),
                op,
            )
            .await
            .unwrap_err();
            assert_eq!(error.operation(), *operation);
            assert!(!format!("{error:?}").contains("private"));
            if limit == 4 {
                assert!(matches!(error, Error::ResponseTooLarge(_)));
            } else {
                let Error::Decode(e) = error else {
                    panic!("decode")
                };
                assert_eq!(e.body(), b"priv");
            }
        }
    }
}

#[tokio::test]
async fn disconnected_form_mutations_are_not_retried_and_deadlines_cover_body_reads() {
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
        time::{Duration, timeout},
    };
    for (op, operation) in OPS.iter().enumerate().skip(3) {
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
            assert!(head.starts_with(verb(op)));
            let len = head
                .lines()
                .find_map(|l| {
                    l.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .map(str::to_owned)
                })
                .map(|s| s.trim().parse::<usize>().unwrap())
                .unwrap_or(0);
            let mut body = vec![0; len];
            socket.read_exact(&mut body).await.unwrap();
            if op == 5 {
                assert!(body.is_empty());
            } else {
                let v: Value = serde_json::from_slice(&body).unwrap();
                assert!(v.get(if op == 3 { "fields" } else { "answer" }).is_some());
            }
            drop(socket);
            assert!(
                timeout(Duration::from_millis(150), listener.accept())
                    .await
                    .is_err()
            );
        });
        let error = call(&client, op).await.unwrap_err();
        assert!(matches!(error, opencoders::Error::Transport(_)));
        assert_eq!(error.operation(), *operation);
        server.await.unwrap();
    }
    for status in [200, 404] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = Client::builder(format!("http://{}", listener.local_addr().unwrap()))
            .request_timeout(Duration::from_millis(150))
            .build()
            .unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut bytes = vec![];
            while !bytes.ends_with(b"\r\n\r\n") {
                bytes.push(socket.read_u8().await.unwrap());
            }
            socket
                .write_all(
                    format!("HTTP/1.1 {status} Test\r\nContent-Length: 100\r\n\r\n{{").as_bytes(),
                )
                .await
                .unwrap();
            assert_eq!(socket.read(&mut [0; 1]).await.unwrap(), 0);
        });
        let error = call(&client, 2).await.unwrap_err();
        assert!(error.is_timeout());
        assert_eq!(error.status().unwrap(), status);
        assert_eq!(error.operation(), OPS[2]);
        timeout(Duration::from_secs(2), server)
            .await
            .unwrap()
            .unwrap();
    }
}
fn info() -> Value {
    json!({"id":"frm_x","sessionID":"ses_x","title":"Question","fields":[{"key":"ok","type":"boolean"}],"future":1})
}

#[tokio::test]
async fn forms_use_location_for_discovery_and_session_paths_for_lifecycle() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/form"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"location":{"directory":"/resolved","extra":1},"data":[info()],"extra":true}),
        ))
        .expect(3)
        .mount(&server)
        .await;
    let root = "/proxy/api/session/ses%2F%3F%E9%9B%AA/form";
    Mock::given(method("GET"))
        .and(path(root))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[info()],"extra":1})))
        .expect(1)
        .mount(&server)
        .await;
    let form_path = format!("{root}/frm_%2F%3F%E9%9B%AA");
    let mut detail = info();
    detail["state"] = json!({"status":"pending"});
    Mock::given(method("GET"))
        .and(path(&form_path))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":detail})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(root))
        .and(body_json(
            json!({"title":"Question","fields":[{"type":"boolean","key":"ok"}]}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":info()})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(format!("{form_path}/reply")))
        .and(body_json(json!({"answer":{"ok":false}})))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(&form_path))
        .respond_with(ResponseTemplate::new(204))
        .expect(3)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .bearer_auth("secret")
        .default_location(Location::directory("/default + 雪"))
        .build()
        .unwrap();
    let list = client.forms().list().await.unwrap();
    assert_eq!(list.location.directory.as_str(), "/resolved");
    assert_eq!(list.extra["extra"], true);
    client
        .forms()
        .list_at(Location::directory(""))
        .await
        .unwrap();
    client.forms().list_at(Location::Unspecified).await.unwrap();
    let session = "ses/?雪".parse().unwrap();
    let id = "frm_/?雪".parse().unwrap();
    let forms = client.sessions().forms(&session);
    assert_eq!(forms.list().await.unwrap().extra["extra"], 1);
    assert!(matches!(
        forms.get(&id).await.unwrap().data.state,
        FormState::Pending(_)
    ));
    forms
        .create(&SessionFormCreateRequest::new(
            "Question",
            FormFieldsInput::new(FormBooleanFieldInput::new("ok")),
        ))
        .await
        .unwrap();
    forms
        .reply(
            &id,
            &SessionFormReplyRequest::new(serde_json::from_value(json!({"ok":false})).unwrap()),
        )
        .await
        .unwrap();
    forms
        .cancel(&id, &SessionFormCancelOptions::default())
        .await
        .unwrap();
    forms
        .cancel(&id, &SessionFormCancelOptions::default().message(""))
        .await
        .unwrap();
    forms
        .cancel(
            &id,
            &SessionFormCancelOptions::default().message("done + 雪 /?&"),
        )
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    let query = |i: usize| {
        requests[i]
            .url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        query(0),
        [("location[directory]".into(), "/default + 雪".into())]
    );
    assert_eq!(query(1), [("location[directory]".into(), "".into())]);
    for request in &requests[2..8] {
        assert!(request.url.query().is_none());
    }
    assert_eq!(query(8), [("message".into(), "".into())]);
    assert_eq!(query(9), [("message".into(), "done + 雪 /?&".into())]);
    for r in &requests {
        assert_eq!(r.headers["authorization"], "Bearer secret");
        if r.method.as_str() == "POST" {
            assert_eq!(r.headers["content-type"], "application/json");
        } else {
            assert!(r.body.is_empty());
        }
    }
}
