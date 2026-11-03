use opencoders::{
    ApiError, Client, Error, Location, RequestField,
    models::{PermissionReply, SessionPermissionCreateRequest, SessionPermissionReplyRequest},
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

fn pending() -> Value {
    json!({"id":"per_test","sessionID":"ses_test","action":"read","resources":["a"],"future":true})
}

#[tokio::test]
async fn location_lists_preserve_metadata_order_and_per_call_overrides() {
    let server = MockServer::start().await;
    let mut second = pending();
    second["id"] = json!("per_second");
    Mock::given(method("GET")).and(path("/proxy/api/permission/request")).and(header("authorization","Bearer private"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"location":{"directory":"/resolved","future":1},"data":[second,pending()],"extra":true})))
        .expect(3).mount(&server).await;
    let mut headers = opencoders::header::HeaderMap::new();
    headers.insert("x-opencode-directory", "/header".parse().unwrap());
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .bearer_auth("private")
        .default_headers(headers)
        .default_location(Location::directory("/default + 雪"))
        .build()
        .unwrap();
    let result = client.permissions().list().await.unwrap();
    assert_eq!(result.location.directory.as_str(), "/resolved");
    assert_eq!(result.extra["extra"], true);
    assert_eq!(
        result
            .data
            .iter()
            .map(|r| r.id.as_str())
            .collect::<Vec<_>>(),
        ["per_second", "per_test"]
    );
    client
        .permissions()
        .list_at(Location::directory(""))
        .await
        .unwrap();
    client
        .permissions()
        .list_at(Location::Unspecified)
        .await
        .unwrap();
    let received = server.received_requests().await.unwrap();
    assert_eq!(
        received[0].url.query_pairs().collect::<Vec<_>>(),
        [("location[directory]".into(), "/default + 雪".into())]
    );
    assert_eq!(
        received[1].url.query_pairs().collect::<Vec<_>>(),
        [("location[directory]".into(), "".into())]
    );
    assert!(received[2].url.query().is_none());
    assert!(
        received
            .iter()
            .all(|r| r.headers["x-opencode-directory"] == "/header")
    );
}

#[tokio::test]
async fn session_permissions_encode_ids_ignore_location_and_preserve_wire_bodies() {
    let server = MockServer::start().await;
    let root = "/proxy/api/session/ses%2F..%2F%3F%23%25%E9%9B%AA/permission";
    for (verb, route, body, response) in [
        (
            "GET",
            root.to_owned(),
            None,
            ResponseTemplate::new(200).set_body_json(json!({"data":[pending()],"extra":1})),
        ),
        (
            "GET",
            format!("{root}/per%2F..%2F%3F%23%25%E9%9B%AA"),
            None,
            ResponseTemplate::new(200).set_body_json(json!({"data":pending(),"extra":1})),
        ),
        (
            "POST",
            root.to_owned(),
            Some(json!({"action":"read","resources":[],"agent":null,"save":[],"metadata":{}})),
            ResponseTemplate::new(200)
                .set_body_json(json!({"data":{"id":"per_test","effect":"ask"}})),
        ),
        (
            "POST",
            format!("{root}/per%2F..%2F%3F%23%25%E9%9B%AA/reply"),
            Some(json!({"decision":"reject","message":null})),
            ResponseTemplate::new(204),
        ),
    ] {
        let mut mock = Mock::given(method(verb))
            .and(path(route))
            .and(header("authorization", "Bearer private"));
        if let Some(body) = body {
            mock = mock
                .and(body_json(body))
                .and(header("content-type", "application/json"));
        }
        mock.respond_with(response).expect(1).mount(&server).await;
    }
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .bearer_auth("private")
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let id = "ses/../?#%雪".parse().unwrap();
    let request_id = "per/../?#%雪".parse().unwrap();
    let permissions = client.sessions().permissions(&id);
    assert_eq!(permissions.list().await.unwrap().extra["extra"], 1);
    assert_eq!(
        permissions.get(&request_id).await.unwrap().data.id.as_str(),
        "per_test"
    );
    let mut create = SessionPermissionCreateRequest::new("read", vec![])
        .save(vec![])
        .metadata(serde_json::Map::new());
    create.agent = RequestField::Null;
    assert_eq!(
        permissions.create(&create).await.unwrap().data.id.as_str(),
        "per_test"
    );
    let mut reply = SessionPermissionReplyRequest::new(PermissionReply::Reject);
    reply.message = RequestField::Null;
    permissions.reply(&request_id, &reply).await.unwrap();
    assert!(
        server
            .received_requests()
            .await
            .unwrap()
            .iter()
            .all(|r| r.url.query().is_none())
    );
}

async fn call(client: &Client, op: usize) -> opencoders::Result<()> {
    let id = "ses_test".parse().unwrap();
    let request_id = "per_test".parse().unwrap();
    let resource = client.sessions().permissions(&id);
    match op {
        0 => client.permissions().list().await.map(|_| ()),
        1 => resource.list().await.map(|_| ()),
        2 => resource.get(&request_id).await.map(|_| ()),
        3 => resource
            .create(&SessionPermissionCreateRequest::new("read", vec![]))
            .await
            .map(|_| ()),
        _ => {
            resource
                .reply(
                    &request_id,
                    &SessionPermissionReplyRequest::new(PermissionReply::Once),
                )
                .await
        }
    }
}
const OPS: [&str; 5] = [
    "permission.request.list",
    "session.permission.list",
    "session.permission.get",
    "session.permission.create",
    "session.permission.reply",
];

#[tokio::test]
async fn permission_errors_are_typed_redacted_and_keep_raw_unknown_details() {
    for (op, &operation) in OPS.iter().enumerate() {
        for (status, wire) in [
            (
                400,
                json!({"_tag":"InvalidRequestError","message":"private"}),
            ),
            (401, json!({"_tag":"UnauthorizedError","message":"private"})),
            (
                404,
                json!({"_tag":"SessionNotFoundError","sessionID":"ses_test","message":"private"}),
            ),
            (
                404,
                json!({"_tag":"LocationNotFoundError","location":{"directory":"/missing"},"message":"private"}),
            ),
            (
                404,
                json!({"_tag":"PermissionNotFoundError","requestID":"unprefixed","message":"private"}),
            ),
            (
                404,
                json!({"_tag":"PermissionNotFoundError","message":"private"}),
            ),
            (409, json!({"_tag":"Future","message":"private"})),
        ] {
            let server = MockServer::start().await;
            Mock::given(method(if op >= 3 { "POST" } else { "GET" }))
                .respond_with(ResponseTemplate::new(status).set_body_json(wire.clone()))
                .expect(1)
                .mount(&server)
                .await;
            let error = call(&Client::new(server.uri()).unwrap(), op)
                .await
                .unwrap_err();
            assert_eq!(error.operation(), operation);
            assert_eq!(error.status().unwrap().as_u16(), status);
            assert!(!format!("{error:?} {error}").contains("private"));
            let Error::Http(error) = error else {
                panic!("HTTP error")
            };
            assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), wire);
            match wire["_tag"].as_str().unwrap() {
                "InvalidRequestError" => assert!(matches!(
                    error.api_error(),
                    Some(ApiError::InvalidRequest { .. })
                )),
                "UnauthorizedError" => assert!(matches!(
                    error.api_error(),
                    Some(ApiError::Unauthorized { .. })
                )),
                "SessionNotFoundError" => assert!(matches!(
                    error.api_error(),
                    Some(ApiError::SessionNotFound { .. })
                )),
                "LocationNotFoundError" => assert!(matches!(
                    error.api_error(),
                    Some(ApiError::LocationNotFound { .. })
                )),
                "PermissionNotFoundError" if wire.get("requestID").is_some() => assert!(
                    matches!(error.api_error(),Some(ApiError::PermissionNotFound{request_id,..}) if request_id=="unprefixed")
                ),
                _ => assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(_)))),
            }
        }
    }
}

#[tokio::test]
async fn permission_methods_reject_unexpected_statuses_without_retries_or_redirects() {
    for (op, &operation) in OPS.iter().enumerate() {
        for status in [if op == 4 { 200 } else { 204 }, 202, 307, 503] {
            let server = MockServer::start().await;
            Mock::given(method(if op >= 3 { "POST" } else { "GET" }))
                .respond_with(
                    ResponseTemplate::new(status)
                        .set_body_string("private-error")
                        .insert_header("location", format!("{}/redirect", server.uri())),
                )
                .expect(1)
                .mount(&server)
                .await;
            let error = call(
                &Client::builder(server.uri())
                    .max_error_body_bytes(4)
                    .build()
                    .unwrap(),
                op,
            )
            .await
            .unwrap_err();
            assert_eq!(error.operation(), operation);
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
    }
}

#[tokio::test]
async fn permission_reads_and_creation_bound_success_and_decode_failures() {
    for (op, &operation) in OPS.iter().enumerate().take(4) {
        for limit in [4, 1000] {
            let server = MockServer::start().await;
            Mock::given(method(if op == 3 { "POST" } else { "GET" }))
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
            assert_eq!(error.operation(), operation);
            assert_eq!(error.status().unwrap(), 200);
            assert!(!format!("{error:?} {error}").contains("private"));
            if limit == 4 {
                assert!(matches!(error, Error::ResponseTooLarge(_)));
            } else {
                let Error::Decode(error) = error else {
                    panic!("decode error")
                };
                assert_eq!(error.body(), b"priv");
            }
        }
    }
}
