use opencoders::{Client, Error, Location, RequestField, models::*};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path},
};

#[test]
fn synthetic_inputs_preserve_omission_null_false_and_empty_values() {
    assert_eq!(
        serde_json::to_value(SessionSyntheticRequest::new("")).unwrap(),
        json!({"text":""})
    );
    let request: SessionSyntheticRequest = serde_json::from_value(
        json!({"text":"", "id":null, "description":null, "delivery":null, "resume":null}),
    )
    .unwrap();
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({"text":"", "id":null, "description":null, "delivery":null, "resume":null})
    );
    let request = SessionSyntheticRequest::new("")
        .id("msg_a".parse().unwrap())
        .description("")
        .metadata(Default::default())
        .delivery(SessionInboxDelivery::Queue)
        .resume(false);
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({"text":"", "id":"msg_a", "description":"", "metadata":{}, "delivery":"queue", "resume":false})
    );
    for value in [
        json!({}),
        json!({"text":null}),
        json!({"text":"", "metadata":null}),
        json!({"text":"", "unknown":true}),
        json!({"text":"", "id":"bad"}),
        json!({"text":"", "resume":"false"}),
    ] {
        assert!(serde_json::from_value::<SessionSyntheticRequest>(value).is_err());
    }
}

#[test]
fn view_keeps_exact_numbers_and_requires_a_strict_body() {
    for idle in [json!(9_007_199_254_740_993_u64), json!(-0.5), json!(0)] {
        let request = SessionViewRequest::new(idle.as_number().unwrap().clone());
        assert_eq!(serde_json::to_value(request).unwrap(), json!({"idle":idle}));
    }
    for value in [
        json!({}),
        json!({"idle":null}),
        json!({"idle":"1"}),
        json!({"idle":1,"unknown":true}),
    ] {
        assert!(serde_json::from_value::<SessionViewRequest>(value).is_err());
    }
}

#[test]
fn active_validates_session_keys_and_states_but_preserves_additive_fields() {
    let response: SessionActiveResponse = serde_json::from_value(
        json!({"data":{"ses_a":{"type":"running","future":1}},"future":true}),
    )
    .unwrap();
    let id: SessionId = "ses_a".parse().unwrap();
    assert_eq!(response.data[&id].type_, SessionActiveType::Running);
    assert_eq!(response.data[&id].extra["future"], 1);
    assert_eq!(response.extra["future"], true);
    assert!(
        serde_json::from_value::<SessionActiveResponse>(json!({"data":{}}))
            .unwrap()
            .data
            .is_empty()
    );
    for data in [
        json!({"bad":{"type":"running"}}),
        json!({"ses_a":{"type":"unknown"}}),
        json!({"ses_a":{}}),
        json!([]),
        Value::Null,
    ] {
        assert!(serde_json::from_value::<SessionActiveResponse>(json!({"data":data})).is_err());
    }
}

#[tokio::test]
async fn activity_uses_exact_paths_bodies_statuses_and_shared_inbox_records() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/session/active"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{}})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/proxy/api/session/ses_a/view"))
        .and(body_json(json!({"idle":9_007_199_254_740_993_u64})))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST")).and(path("/proxy/api/session/ses_a/synthetic")).and(body_json(json!({"text":"", "resume":false,"description":null}))).respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"id":"msg_a","sessionID":"ses_a","type":"synthetic","time":{"created":1},"payload":{"text":"","future":2},"delivery":"steer","future":3},"future":4}))).expect(1).mount(&server).await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let id = "ses_a".parse().unwrap();
    assert!(client.sessions().active().await.unwrap().data.is_empty());
    client
        .sessions()
        .view(
            &id,
            &SessionViewRequest::new(9_007_199_254_740_993_u64.into()),
        )
        .await
        .unwrap();
    let mut request = SessionSyntheticRequest::new("").resume(false);
    request.description = RequestField::Null;
    let response = client.sessions().synthetic(&id, &request).await.unwrap();
    let shared: &SessionInboxSynthetic = &response.data;
    assert_eq!(shared.payload.extra["future"], 2);
    assert_eq!(shared.extra["future"], 3);
    assert_eq!(response.extra["future"], 4);
    for request in server.received_requests().await.unwrap() {
        assert!(request.url.query().is_none());
    }
}

#[tokio::test]
async fn activity_rejects_other_success_statuses_and_preserves_errors_without_retry() {
    for status in [202, 400, 401, 404, 409, 500] {
        let server = MockServer::start().await;
        let body = json!({"_tag":"ConflictError","message":"private detail"});
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(status)
                    .insert_header("x-request-id", "fixture")
                    .set_body_json(body.clone()),
            )
            .expect(2)
            .mount(&server)
            .await;
        let client = Client::new(server.uri()).unwrap();
        let id = "ses_a".parse().unwrap();
        for result in [
            client
                .sessions()
                .view(&id, &SessionViewRequest::new(1.into()))
                .await,
            client
                .sessions()
                .synthetic(&id, &SessionSyntheticRequest::new("private text"))
                .await
                .map(|_| ()),
        ] {
            let Error::Http(error) = result.unwrap_err() else {
                panic!("HTTP error")
            };
            assert_eq!(error.status().as_u16(), status);
            assert_eq!(serde_json::from_slice::<Value>(error.body()).unwrap(), body);
            assert!(matches!(
                error.operation(),
                "session.view" | "session.synthetic"
            ));
            assert!(matches!(
                error.api_error(),
                Some(opencoders::ApiError::Conflict { .. })
            ));
            assert!(!format!("{error:?}").contains("private"));
        }
    }
}
