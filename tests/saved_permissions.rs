use opencoders::{
    ApiError, Client, Error, Location, RequestField, SavedPermissionListOptions, models::*,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path},
};
fn saved() -> Value {
    json!({"id":"saved /?雪","projectID":"project +?雪","action":"read","resource":"src/*","time":{"created":9007199254740993u64,"updated":2,"future":1},"future":true})
}
#[test]
fn saved_rule_updates_preserve_omitted_null_empty_and_numeric_metadata() {
    assert_eq!(
        serde_json::to_value(PermissionSavedUpdateRequest::default()).unwrap(),
        json!({})
    );
    let mut request = PermissionSavedUpdateRequest::default().resource("");
    request.action = RequestField::Null;
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"action":null,"resource":""})
    );
    let value: PermissionSavedUpdateRequest =
        serde_json::from_value(json!({"action":null,"resource":""})).unwrap();
    assert_eq!(
        serde_json::to_value(value).unwrap(),
        serde_json::to_value(request).unwrap()
    );
    for value in [
        json!({"effect":"deny"}),
        json!({"projectID":"p"}),
        json!({"action":false}),
    ] {
        assert!(serde_json::from_value::<PermissionSavedUpdateRequest>(value).is_err());
    }
    let value: PermissionSavedInfo = serde_json::from_value(saved()).unwrap();
    assert_eq!(value.time.created.as_u64(), Some(9007199254740993));
    assert_eq!(value.time.extra["future"], 1);
    assert_eq!(value.extra["future"], true);
}
#[tokio::test]
async fn saved_rule_lists_use_only_explicit_project_filters_and_retain_envelopes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/permission/saved"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":[saved()],"future":42})),
        )
        .expect(3)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let resource = client.permissions().saved();
    let response = resource
        .list(&SavedPermissionListOptions::default())
        .await
        .unwrap();
    assert_eq!(response.extra["future"], 42);
    assert_eq!(response.data[0].project_id.as_str(), "project +?雪");
    resource
        .list(&SavedPermissionListOptions::default().project("project +?雪".to_owned().into()))
        .await
        .unwrap();
    resource
        .list(&SavedPermissionListOptions::default().project("".to_owned().into()))
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].url.query().is_none());
    assert_eq!(
        requests[1].url.query_pairs().collect::<Vec<_>>(),
        vec![("projectID".into(), "project +?雪".into())]
    );
    assert_eq!(requests[2].url.query(), Some("projectID="));
}
#[tokio::test]
async fn saved_rule_mutations_encode_ids_and_use_patch_json_or_empty_delete() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/proxy/api/permission/saved/saved%20%2F%3F%E9%9B%AA"))
        .and(body_json(json!({"action":"write","resource":null})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":saved(),"future":7})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let resource = client.permissions().saved();
    let id: PermissionSavedId = "saved /?雪".to_owned().into();
    let mut request = PermissionSavedUpdateRequest::default().action("write");
    request.resource = RequestField::Null;
    assert_eq!(
        resource.update(&id, &request).await.unwrap().extra["future"],
        7
    );
    resource.remove(&id).await.unwrap();
    let requests = server.received_requests().await.unwrap();
    for r in &requests {
        assert!(r.url.query().is_none());
    }
    assert!(requests[1].body.is_empty());
    assert!(!requests[1].headers.contains_key("content-type"));
    assert_eq!(requests[0].url, requests[1].url);
    for id in ["", ".", ".."] {
        let id = id.to_owned().into();
        assert!(matches!(
            resource
                .update(&id, &PermissionSavedUpdateRequest::default())
                .await,
            Err(Error::Encode(_))
        ));
        assert!(matches!(resource.remove(&id).await, Err(Error::Encode(_))));
    }
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}
#[tokio::test]
async fn saved_rule_failures_preserve_missing_conflict_and_unexpected_success() {
    let server = MockServer::start().await;
    for (id, status, body) in [
        (
            "missing",
            404,
            json!({"_tag":"PermissionSavedNotFoundError","id":"private-rule","message":"private-message"}),
        ),
        (
            "conflict",
            409,
            json!({"_tag":"ConflictError","resource":"private-rule","message":"private-message"}),
        ),
        ("unexpected", 204, json!(null)),
        (
            "malformed",
            404,
            json!({"_tag":"PermissionSavedNotFoundError","id":false,"message":"private-message"}),
        ),
    ] {
        Mock::given(method("PATCH"))
            .and(path(format!("/api/permission/saved/{id}")))
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .expect(1)
            .mount(&server)
            .await;
    }
    let client = Client::new(server.uri()).unwrap();
    let resource = client.permissions().saved();
    for id in ["missing", "conflict", "unexpected", "malformed"] {
        let Error::Http(error) = resource
            .update(
                &id.to_owned().into(),
                &PermissionSavedUpdateRequest::default(),
            )
            .await
            .unwrap_err()
        else {
            panic!("HTTP error")
        };
        match id {
            "missing" => assert!(
                matches!(error.api_error(),Some(ApiError::PermissionSavedNotFound{id,..}) if id=="private-rule")
            ),
            "conflict" => assert!(
                matches!(error.api_error(),Some(ApiError::Conflict{resource:Some(r),..}) if r=="private-rule")
            ),
            "unexpected" => assert_eq!(error.status(), 204),
            _ => assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(_)))),
        }
        assert!(!format!("{error:?} {:?}", error.api_error()).contains("private-"));
    }
}

#[tokio::test]
async fn lost_saved_rule_responses_do_not_retry_update_or_removal() {
    use std::time::Duration;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };
    for operation in ["update", "remove"] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut headers = Vec::new();
            while !headers.ends_with(b"\r\n\r\n") {
                assert!(headers.len() < 8192);
                headers.push(socket.read_u8().await.unwrap());
            }
            let headers = String::from_utf8(headers).unwrap();
            let length: usize = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse().unwrap())
                })
                .unwrap_or(0);
            let mut body = vec![0; length];
            socket.read_exact(&mut body).await.unwrap();
            if operation == "remove" {
                assert!(body.is_empty());
            } else {
                assert_eq!(
                    serde_json::from_slice::<Value>(&body).unwrap(),
                    json!({"resource":"private-pattern"})
                );
            }
            socket.shutdown().await.unwrap();
            drop(socket);
            assert!(
                tokio::time::timeout(Duration::from_millis(150), listener.accept())
                    .await
                    .is_err(),
                "unexpected retry of {operation}"
            );
        });
        let client = Client::new(format!("http://{addr}")).unwrap();
        let id: PermissionSavedId = "private-id".to_owned().into();
        let resource = client.permissions().saved();
        let result = if operation == "update" {
            resource
                .update(
                    &id,
                    &PermissionSavedUpdateRequest::default().resource("private-pattern"),
                )
                .await
                .map(|_| ())
        } else {
            resource.remove(&id).await
        };
        let error = result.unwrap_err();
        assert!(matches!(error, opencoders::Error::Transport(_)));
        assert!(error.status().is_none());
        assert!(!format!("{error:?}").contains("private-pattern"));
        server.await.unwrap();
    }
}
