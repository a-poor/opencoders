use opencoders::{
    ApiError, Client, Error, Location, MessageListOptions, MessageOrder, models::SessionMessageType,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, Request, ResponseTemplate,
    matchers::{method, path},
};

fn user(id: &str) -> Value {
    json!({"id":id,"type":"user","text":"hello","time":{"created":1}})
}

#[tokio::test]
async fn list_and_get_encode_ids_preserve_envelopes_and_ignore_default_location() {
    let server = MockServer::start().await;
    let session = "ses/../?x=#%2F\\雪".parse().unwrap();
    let message = "msg_/../?x=#%2F\\雪".parse().unwrap();
    let route = "/proxy/api/session/ses%2F..%2F%3Fx=%23%252F%5C%E9%9B%AA/message";
    let page = json!({"data":[user("msg_b"),user("msg_a")],"cursor":{"next":"next","future":1},"future":2});
    let single = json!({"data":user("msg_test"),"future":3});
    Mock::given(method("GET"))
        .and(path(route))
        .respond_with(ResponseTemplate::new(200).set_body_json(&page))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!(
            "{route}/msg_%2F..%2F%3Fx=%23%252F%5C%E9%9B%AA"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_json(&single))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/ignored"))
        .bearer_auth("token")
        .build()
        .unwrap();
    let messages = client.sessions().messages(&session);
    assert_eq!(
        serde_json::to_value(messages.list(&MessageListOptions::default()).await.unwrap()).unwrap(),
        page
    );
    assert_eq!(
        serde_json::to_value(messages.get(&message).await.unwrap()).unwrap(),
        single
    );
    for request in server.received_requests().await.unwrap() {
        assert!(request.url.query().is_none());
        assert!(request.body.is_empty());
        assert_eq!(request.headers["authorization"], "Bearer token");
    }
}

#[tokio::test]
async fn pagination_is_explicit_preserves_order_and_repeats_type_filter() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(|r: &Request| {
            let cursor = r
                .url
                .query_pairs()
                .find(|(k, _)| k == "cursor")
                .map(|(_, v)| v.into_owned());
            ResponseTemplate::new(200).set_body_json(if cursor.as_deref() == Some("next/+?雪") {
                json!({"data":[user("msg_2"),user("msg_1")],"cursor":{"previous":"previous/+?雪"}})
            } else {
                json!({"data":[user("msg_4"),user("msg_3")],"cursor":{"next":"next/+?雪"}})
            })
        })
        .expect(3)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let id = "ses_a".parse().unwrap();
    let messages = client.sessions().messages(&id);
    let original = MessageListOptions::default()
        .limit(2)
        .order(MessageOrder::Desc)
        .message_type(SessionMessageType::User);
    let first = messages.list(&original).await.unwrap();
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
    let mut following = original.clone().cursor(first.cursor.next.unwrap());
    following.order = None;
    let second = messages.list(&following).await.unwrap();
    assert_eq!(
        second
            .data
            .iter()
            .map(|m| m.id().as_str())
            .collect::<Vec<_>>(),
        ["msg_2", "msg_1"]
    );
    following.cursor = second.cursor.previous;
    let back = messages.list(&following).await.unwrap();
    assert_eq!(back.data[0].id().as_str(), "msg_4");
    assert!(original.cursor.is_none());
    let requests = server.received_requests().await.unwrap();
    for (i, r) in requests.iter().enumerate() {
        let q = r
            .url
            .query_pairs()
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_eq!(q["limit"], "2");
        assert_eq!(q["type"], "user");
        if i > 0 {
            assert!(!q.contains_key("order"));
            assert_eq!(
                q["cursor"],
                if i == 1 {
                    "next/+?雪"
                } else {
                    "previous/+?雪"
                }
            );
        }
    }
}

#[tokio::test]
async fn missing_message_errors_are_typed_and_redacted_with_raw_details() {
    let server = MockServer::start().await;
    let wire = json!({"_tag":"MessageNotFoundError","sessionID":"raw-session","messageID":"raw-message","message":"private details","future":true});
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&wire))
        .mount(&server)
        .await;
    let error = Client::new(server.uri())
        .unwrap()
        .sessions()
        .messages(&"ses_a".parse().unwrap())
        .get(&"msg_a".parse().unwrap())
        .await
        .unwrap_err();
    assert_eq!(error.operation(), "session.message.get");
    assert_eq!(error.status().unwrap(), 404);
    assert!(!format!("{error:?} {error}").contains("private"));
    let Error::Http(error) = error else {
        panic!("expected HTTP error")
    };
    assert!(
        matches!(error.api_error(),Some(ApiError::MessageNotFound{session_id,message_id,..}) if session_id=="raw-session" && message_id=="raw-message")
    );
    assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), wire);
}

#[tokio::test]
async fn explicit_zero_empty_cursor_and_order_are_sent_unchanged() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[],"cursor":{}})))
        .expect(2)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let id = "ses_test".parse().unwrap();
    let messages = client.sessions().messages(&id);
    let mut options = MessageListOptions::default()
        .limit(0)
        .cursor("")
        .order(MessageOrder::Asc)
        .message_type(SessionMessageType::AgentSwitched);
    messages.list(&options).await.unwrap();
    options.limit = None;
    options.cursor = None;
    options.order = None;
    options.message_type = None;
    messages.list(&options).await.unwrap();
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests[0]
            .url
            .query_pairs()
            .collect::<std::collections::BTreeMap<_, _>>(),
        std::collections::BTreeMap::from([
            ("limit".into(), "0".into()),
            ("cursor".into(), "".into()),
            ("order".into(), "asc".into()),
            ("type".into(), "agent-switched".into())
        ])
    );
    assert!(requests[1].url.query().is_none());
}

#[tokio::test]
async fn invalid_cursors_and_unknown_or_malformed_errors_remain_inspectable() {
    for wire in [
        json!({"_tag":"InvalidCursorError","message":"invalid"}),
        json!({"_tag":"MessageNotFoundError","messageID":1,"sessionID":"ses_a","message":"bad"}),
        json!({"_tag":"FutureError","detail":1}),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(400).set_body_json(&wire))
            .expect(1)
            .mount(&server)
            .await;
        let error = Client::new(server.uri())
            .unwrap()
            .sessions()
            .messages(&"ses_a".parse().unwrap())
            .list(
                &MessageListOptions::default()
                    .cursor("opaque")
                    .order(MessageOrder::Asc),
            )
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "session.message.list");
        let Error::Http(error) = error else {
            panic!("expected HTTP error")
        };
        if wire["_tag"] == "InvalidCursorError" {
            assert!(matches!(
                error.api_error(),
                Some(ApiError::InvalidCursor { .. })
            ));
        } else {
            assert!(matches!(error.api_error(),Some(ApiError::Unrecognized(raw)) if raw==&wire));
        }
        assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), wire);
    }
}

#[tokio::test]
async fn malformed_message_responses_are_decode_errors_and_other_success_statuses_are_http_errors()
{
    for get in [false, true] {
        for (status, body) in [
            (200, json!({})),
            (200, json!({"data":{"type":"future"},"cursor":{}})),
            (204, json!(null)),
            (202, json!({})),
            (307, json!({})),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .respond_with(
                    ResponseTemplate::new(status)
                        .set_body_json(body)
                        .insert_header("location", format!("{}/elsewhere", server.uri())),
                )
                .expect(1)
                .mount(&server)
                .await;
            let client = Client::new(server.uri()).unwrap();
            let id = "ses_a".parse().unwrap();
            let messages = client.sessions().messages(&id);
            let error = if get {
                messages.get(&"msg_a".parse().unwrap()).await.map(|_| ())
            } else {
                messages
                    .list(&MessageListOptions::default())
                    .await
                    .map(|_| ())
            }
            .unwrap_err();
            assert_eq!(
                error.operation(),
                if get {
                    "session.message.get"
                } else {
                    "session.message.list"
                }
            );
            assert_eq!(error.status().unwrap().as_u16(), status);
            assert!(if status == 200 {
                matches!(error, Error::Decode(_))
            } else {
                matches!(error, Error::Http(_))
            });
            assert_eq!(server.received_requests().await.unwrap().len(), 1);
        }
    }
}
