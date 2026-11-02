use opencoders::{
    ApiError, Client, Error, Location, ParentFilter, SessionListOptions, SessionOrder,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use wiremock::{
    Mock, MockServer, Request, ResponseTemplate,
    matchers::{method, path},
};

fn session(id: &str) -> Value {
    let mut value: Value = serde_json::from_str(include_str!("fixtures/session.json")).unwrap();
    value["id"] = json!(id);
    value
}

fn query(request: &Request) -> BTreeMap<String, String> {
    request
        .url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect()
}

#[tokio::test]
async fn list_defaults_send_no_query_or_inherited_location_and_preserve_the_envelope() {
    let server = MockServer::start().await;
    let body = json!({"data": [session("ses_b"), session("ses_a")],
        "cursor": {"next": "opaque+/?=&雪", "futureCursor": 1}, "futureEnvelope": 2});
    Mock::given(method("GET"))
        .and(path("/proxy/api/session"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/unrelated-default"))
        .bearer_auth("test-token")
        .build()
        .unwrap();
    let response = client
        .sessions()
        .list(&SessionListOptions::default())
        .await
        .unwrap();
    assert_eq!(response.data[0].id.as_str(), "ses_b");
    assert_eq!(response.data[1].id.as_str(), "ses_a");
    assert_eq!(
        response.cursor.next.as_ref().unwrap().as_str(),
        "opaque+/?=&雪"
    );
    assert_eq!(serde_json::to_value(response).unwrap(), body);
    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].url.query().is_none());
    assert_eq!(requests[0].headers["authorization"], "Bearer test-token");
}

#[tokio::test]
async fn every_explicit_filter_is_encoded_once_with_its_wire_name() {
    let server = MockServer::start().await;
    Mock::given(path("/api/session"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": [], "cursor": {}})))
        .expect(1)
        .mount(&server)
        .await;
    let options = SessionListOptions::default()
        .limit(25)
        .order(SessionOrder::Asc)
        .search("text +&?#%=雪")
        .parent(ParentFilter::ChildrenOf("ses_parent +&?".parse().unwrap()))
        .archived(false)
        .directory(r"C:\remote\a b&+%")
        .project("project +&?".to_owned().into())
        .subpath("../a +&?%")
        .cursor("opaque+/?=&雪".to_owned().into());
    Client::new(server.uri())
        .unwrap()
        .sessions()
        .list(&options)
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        query(&requests[0]),
        BTreeMap::from([
            ("limit".into(), "25".into()),
            ("order".into(), "asc".into()),
            ("search".into(), "text +&?#%=雪".into()),
            ("parentID".into(), "ses_parent +&?".into()),
            ("archived".into(), "false".into()),
            ("directory".into(), r"C:\remote\a b&+%".into()),
            ("project".into(), "project +&?".into()),
            ("subpath".into(), "../a +&?%".into()),
            ("cursor".into(), "opaque+/?=&雪".into())
        ])
    );
    assert_eq!(requests[0].url.query_pairs().count(), 9);
    assert!(requests[0].url.fragment().is_none());
}

#[tokio::test]
async fn roots_archived_and_empty_values_are_distinct_from_omission() {
    let server = MockServer::start().await;
    Mock::given(path("/api/session"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": [], "cursor": {}})))
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    client
        .sessions()
        .list(&SessionListOptions::default())
        .await
        .unwrap();
    client
        .sessions()
        .list(
            &SessionListOptions::default()
                .parent(ParentFilter::RootOnly)
                .archived(true)
                .limit(0)
                .search("")
                .directory("")
                .subpath("")
                .cursor("".to_owned().into()),
        )
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    assert!(query(&requests[0]).is_empty());
    assert_eq!(
        query(&requests[1]),
        BTreeMap::from([
            ("parentID".into(), "null".into()),
            ("archived".into(), "true".into()),
            ("limit".into(), "0".into()),
            ("search".into(), "".into()),
            ("directory".into(), "".into()),
            ("subpath".into(), "".into()),
            ("cursor".into(), "".into())
        ])
    );
}

#[tokio::test]
async fn next_and_previous_cursors_round_trip_without_automatic_requests_or_reordering() {
    let server = MockServer::start().await;
    let next = "next/+?&=雪";
    let previous = "previous/+?&=雪";
    Mock::given(path("/api/session")).respond_with(move |request: &Request| {
        let second = query(request).get("cursor").is_some_and(|value| value == next);
        ResponseTemplate::new(200).set_body_json(if second {
            json!({"data": [session("ses_2"), session("ses_1")], "cursor": {"previous": previous, "next": null}})
        } else {
            json!({"data": [session("ses_4"), session("ses_3")], "cursor": {"next": next}})
        })
    }).expect(3).mount(&server).await;
    let client = Client::new(server.uri()).unwrap();
    let options = SessionListOptions::default()
        .limit(2)
        .search("same filter")
        .archived(false)
        .order(SessionOrder::Desc);
    let first = client.sessions().list(&options).await.unwrap();
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
    let mut following = options.clone().cursor(first.cursor.next.unwrap());
    following.order = None; // The upstream contract documents order for the first page.
    let second = client.sessions().list(&following).await.unwrap();
    assert!(second.cursor.next.is_none());
    assert_eq!(
        second
            .data
            .iter()
            .map(|s| s.id.as_str())
            .collect::<Vec<_>>(),
        ["ses_2", "ses_1"]
    );
    following.cursor = second.cursor.previous;
    let back = client.sessions().list(&following).await.unwrap();
    assert_eq!(
        back.data.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
        ["ses_4", "ses_3"]
    );
    assert!(options.cursor.is_none());
    let requests = server.received_requests().await.unwrap();
    assert_eq!(query(&requests[0])["order"], "desc");
    assert_eq!(query(&requests[1])["cursor"], next);
    assert_eq!(query(&requests[2])["cursor"], previous);
    for request in &requests[1..] {
        let q = query(request);
        assert_eq!(q["limit"], "2");
        assert_eq!(q["search"], "same filter");
        assert_eq!(q["archived"], "false");
        assert!(!q.contains_key("order"));
    }
}

#[tokio::test]
async fn empty_pages_preserve_cursor_boundaries() {
    for cursor in [
        json!({}),
        json!({"next": null, "previous": null}),
        json!({"next": "", "previous": "prior"}),
    ] {
        let server = MockServer::start().await;
        Mock::given(path("/api/session"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({"data": [], "cursor": cursor})),
            )
            .expect(1)
            .mount(&server)
            .await;
        let response = Client::new(server.uri())
            .unwrap()
            .sessions()
            .list(&SessionListOptions::default())
            .await
            .unwrap();
        assert!(response.data.is_empty());
        assert_eq!(
            response.cursor.next.as_deref().map(|v| v.as_str()),
            cursor["next"].as_str()
        );
        assert_eq!(
            response.cursor.previous.as_deref().map(|v| v.as_str()),
            cursor["previous"].as_str()
        );
    }
}

#[tokio::test]
async fn invalid_cursor_errors_are_typed_inspectable_and_redacted() {
    let server = MockServer::start().await;
    let body =
        json!({"_tag": "InvalidCursorError", "message": "private-cursor-details", "future": true});
    Mock::given(path("/api/session"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;
    let error = Client::new(server.uri())
        .unwrap()
        .sessions()
        .list(&SessionListOptions::default())
        .await
        .unwrap_err();
    assert_eq!(error.operation(), "session.list");
    assert_eq!(error.status().unwrap().as_u16(), 400);
    assert!(!format!("{error:?} {error}").contains("private-"));
    let Error::Http(error) = error else {
        panic!("expected HTTP error")
    };
    assert!(
        matches!(error.api_error(), Some(ApiError::InvalidCursor { message }) if message == "private-cursor-details")
    );
    assert!(!format!("{:?}", error.api_error()).contains("private-"));
    assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), body);
}

#[tokio::test]
async fn malformed_cursor_errors_remain_unrecognized() {
    let server = MockServer::start().await;
    let body = json!({"_tag": "InvalidCursorError", "message": 3});
    Mock::given(path("/api/session"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&body))
        .mount(&server)
        .await;
    let Error::Http(error) = Client::new(server.uri())
        .unwrap()
        .sessions()
        .list(&SessionListOptions::default())
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error")
    };
    assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(raw)) if raw == &body));
}

#[tokio::test]
async fn malformed_session_pages_fail_decoding_with_operation_context() {
    for body in [
        json!([]),
        json!({"data": []}),
        json!({"cursor": {}}),
        json!({"data": [], "cursor": null}),
        json!({"data": [], "cursor": {"next": 3}}),
        json!({"data": [{"id": "ses_incomplete"}], "cursor": {}}),
    ] {
        let server = MockServer::start().await;
        Mock::given(path("/api/session"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
        let error = Client::new(server.uri())
            .unwrap()
            .sessions()
            .list(&SessionListOptions::default())
            .await
            .unwrap_err();
        assert_eq!(error.operation(), "session.list");
        assert!(matches!(error, Error::Decode(_)));
    }
}
