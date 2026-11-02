use opencoders::{ApiError, Client, Error, Location, RequestField, models::SessionUpdateRequest};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

#[tokio::test]
async fn update_and_remove_encode_one_segment_and_accept_empty_204_without_json() {
    let server = MockServer::start().await;
    let id = "ses/../?x=#%2F\\雪".parse().unwrap();
    let route = "/proxy%20prefix/api/session/ses%2F..%2F%3Fx=%23%252F%5C%E9%9B%AA";
    Mock::given(method("PATCH"))
        .and(path(route))
        .and(body_json(json!({})))
        .and(header("content-type", "application/json"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(route))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy%20prefix", server.uri()))
        .default_location(Location::directory("/not-in-query"))
        .bearer_auth("test-token")
        .max_response_bytes(0)
        .build()
        .unwrap();
    client
        .sessions()
        .update(&id, &SessionUpdateRequest::default())
        .await
        .unwrap();
    client.sessions().remove(&id).await.unwrap();
    let requests = server.received_requests().await.unwrap();
    assert!(requests.iter().all(|r| r.url.query().is_none()));
    assert!(requests[1].body.is_empty());
    assert!(!requests[1].headers.contains_key("content-type"));
}

#[tokio::test]
async fn update_sends_only_explicit_fields_and_forces_json_content_type() {
    let server = MockServer::start().await;
    let mut request = SessionUpdateRequest::default()
        .title("")
        .archived(false)
        .permissions(vec![]);
    request.metadata = RequestField::Null;
    let wire = json!({"title": "", "archived": false, "permissions": [], "metadata": null});
    Mock::given(method("PATCH"))
        .and(body_json(wire.clone()))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let mut headers = opencoders::header::HeaderMap::new();
    headers.insert(
        opencoders::header::CONTENT_TYPE,
        "text/plain".parse().unwrap(),
    );
    Client::builder(server.uri())
        .default_headers(headers)
        .build()
        .unwrap()
        .sessions()
        .update(&"ses_test".parse().unwrap(), &request)
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(request).unwrap(), wire);
}

#[tokio::test]
async fn mutation_errors_preserve_status_typed_details_and_bounded_diagnostics() {
    for remove in [false, true] {
        let server = MockServer::start().await;
        let wire = json!({"_tag": "SessionNotFoundError", "sessionID": "ses_missing", "message": "private"});
        Mock::given(method(if remove { "DELETE" } else { "PATCH" }))
            .respond_with(ResponseTemplate::new(404).set_body_json(wire.clone()))
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new(server.uri()).unwrap();
        let id = "ses_missing".parse().unwrap();
        let error = if remove {
            client.sessions().remove(&id).await
        } else {
            client
                .sessions()
                .update(&id, &SessionUpdateRequest::default())
                .await
        }
        .unwrap_err();
        assert_eq!(
            error.operation(),
            if remove {
                "session.remove"
            } else {
                "session.update"
            }
        );
        let Error::Http(error) = error else {
            panic!("expected HTTP error")
        };
        assert_eq!(error.status(), 404);
        assert!(
            matches!(error.api_error(), Some(ApiError::SessionNotFound { session_id, .. }) if session_id == "ses_missing")
        );
        assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), wire);
        assert!(!format!("{error:?} {error}").contains("private"));
    }
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .respond_with(ResponseTemplate::new(500).set_body_string("private diagnostics"))
        .mount(&server)
        .await;
    let Error::Http(error) = Client::builder(server.uri())
        .max_error_body_bytes(4)
        .build()
        .unwrap()
        .sessions()
        .update(
            &"ses_test".parse().unwrap(),
            &SessionUpdateRequest::default(),
        )
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error")
    };
    assert_eq!(error.body(), b"priv");
    assert!(error.is_body_truncated());
}

#[tokio::test]
async fn update_exposes_a_missing_location_error() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH")).respond_with(ResponseTemplate::new(404).set_body_json(json!({
        "_tag": "LocationNotFoundError", "location": {"directory": "/missing"}, "message": "missing"
    }))).mount(&server).await;
    let Error::Http(error) = Client::new(server.uri())
        .unwrap()
        .sessions()
        .update(
            &"ses_test".parse().unwrap(),
            &SessionUpdateRequest::default().title("new"),
        )
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error")
    };
    assert!(matches!(
        error.api_error(),
        Some(ApiError::LocationNotFound { .. })
    ));
}

#[tokio::test]
async fn empty_response_methods_reject_other_successes_and_do_not_retry_or_redirect() {
    for remove in [false, true] {
        for status in [200, 202, 307, 503] {
            let server = MockServer::start().await;
            Mock::given(method(if remove { "DELETE" } else { "PATCH" }))
                .respond_with(
                    ResponseTemplate::new(status)
                        .set_body_json(json!(null))
                        .insert_header("location", format!("{}/elsewhere", server.uri())),
                )
                .expect(1)
                .mount(&server)
                .await;
            let client = Client::new(server.uri()).unwrap();
            let id = "ses_test".parse().unwrap();
            let error = if remove {
                client.sessions().remove(&id).await
            } else {
                client
                    .sessions()
                    .update(&id, &SessionUpdateRequest::default())
                    .await
            }
            .unwrap_err();
            assert!(matches!(error, Error::Http(_)));
            assert_eq!(error.status().unwrap().as_u16(), status);
            assert_eq!(server.received_requests().await.unwrap().len(), 1);
        }
    }
}
