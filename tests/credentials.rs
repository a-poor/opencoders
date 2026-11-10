use opencoders::{ApiError, Client, Error, Location, RequestField, models::*};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path, query_param},
};

const PRIVATE: &str = "owned-test-value-never-a-real-credential";
fn entry() -> Value {
    json!({"id":"fixture", "integrationID":"owned", "label":PRIVATE, "active":true,
        "value":{"type":"key","key":PRIVATE}, "future":9007199254740991_u64})
}

#[test]
fn credential_values_are_strict_inputs_tolerant_responses_and_redacted() {
    for value in [
        json!({"type":"key","key":PRIVATE}),
        json!({"type":"external","methodID":"external"}),
        json!({"type":"oauth","methodID":"oauth","access":PRIVATE,"refresh":PRIVATE,"expires":9007199254740991_u64}),
    ] {
        let input: CredentialValueInput = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(&input).unwrap(), value);
        assert!(!format!("{input:?}").contains(PRIVATE));
        let mut extended = value.clone();
        extended["future"] = json!(PRIVATE);
        assert!(serde_json::from_value::<CredentialValueInput>(extended.clone()).is_err());
        let output: CredentialValue = serde_json::from_value(extended.clone()).unwrap();
        assert_eq!(serde_json::to_value(&output).unwrap(), extended);
        assert!(!format!("{output:?}").contains(PRIVATE));
        let mut empty = value.clone();
        empty["metadata"] = json!({});
        let decoded: CredentialValueInput = serde_json::from_value(empty.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), empty);
        let mut invalid = value;
        invalid["metadata"] = Value::Null;
        assert!(serde_json::from_value::<CredentialValueInput>(invalid).is_err());
    }
    for value in [
        json!({"type":"key"}),
        json!({"type":"external"}),
        json!({"type":"future","key":PRIVATE}),
        json!({"type":"oauth","methodID":"x","access":"a","refresh":"r","expires":9007199254740992_u64}),
        json!({"type":"oauth","methodID":"x","access":"a","refresh":"r","expires":-1}),
        json!({"type":"oauth","methodID":"x","access":"a","refresh":"r","expires":0.5}),
    ] {
        assert!(serde_json::from_value::<CredentialValueInput>(value.clone()).is_err());
        assert!(serde_json::from_value::<CredentialValue>(value).is_err());
    }
    let value = CredentialValueInput::key(PRIVATE);
    let request = CredentialCreateRequest::new("owned".parse().unwrap(), value)
        .activate(false)
        .label("");
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"integrationID":"owned","value":{"type":"key","key":PRIVATE},"activate":false,"label":""})
    );
    assert!(!format!("{request:?}").contains(PRIVATE));
    for field in ["id", "label", "activate"] {
        let mut invalid = json!({"integrationID":"owned","value":{"type":"key","key":PRIVATE}});
        invalid[field] = Value::Null;
        assert!(serde_json::from_value::<CredentialCreateRequest>(invalid).is_err());
    }
    assert!(
        serde_json::from_value::<CredentialUpdateRequest>(json!({"label":"x","value":{}})).is_err()
    );
    assert!(serde_json::from_value::<CredentialUpdateRequest>(json!({"label":null})).is_err());
    assert!(
        CredentialValueInput::oauth(
            "oauth".parse().unwrap(),
            "refresh",
            "access",
            9007199254740992
        )
        .is_err()
    );
    let response: CredentialListResponse =
        serde_json::from_value(json!({"data":[entry()],"extra":PRIVATE})).unwrap();
    assert_eq!(response.data[0].extra["future"], 9007199254740991_u64);
    let shared: &CredentialId = &response.data[0].id;
    assert_eq!(shared.as_str(), "fixture");
    let CredentialValue::Key(key) = &response.data[0].value else {
        panic!("key")
    };
    assert!(!format!("{response:?} {:?} {key:?}", response.data).contains(PRIVATE));
    let mut malformed = entry();
    malformed["active"] = Value::Null;
    assert!(serde_json::from_value::<CredentialEntry>(malformed).is_err());
}

#[test]
fn connection_inputs_preserve_omitted_null_empty_and_typed_answers() {
    let mut key = IntegrationConnectKeyRequest::new(PRIVATE);
    assert_eq!(serde_json::to_value(&key).unwrap(), json!({"key":PRIVATE}));
    key.answer = RequestField::Null;
    key.label = RequestField::Null;
    assert_eq!(
        serde_json::to_value(&key).unwrap(),
        json!({"key":PRIVATE,"answer":null,"label":null})
    );
    let answer: FormAnswer =
        serde_json::from_value(json!({"n":9007199254740991_u64,"items":["x"],"enabled":true}))
            .unwrap();
    let external = IntegrationConnectExternalRequest::new("external".parse().unwrap())
        .answer(answer)
        .label("");
    assert_eq!(
        serde_json::to_value(&external).unwrap(),
        json!({"methodID":"external","answer":{"n":9007199254740991_u64,"items":["x"],"enabled":true},"label":""})
    );
    assert!(!format!("{key:?}").contains(PRIVATE));
    for invalid in [
        json!({"key":null}),
        json!({"key":PRIVATE,"answer":{"bad":null}}),
        json!({"key":PRIVATE,"extra":true}),
    ] {
        assert!(serde_json::from_value::<IntegrationConnectKeyRequest>(invalid).is_err());
    }
    assert!(
        serde_json::from_value::<IntegrationConnectExternalRequest>(json!({"methodID":null}))
            .is_err()
    );
}

#[tokio::test]
async fn credential_routes_are_global_and_preserve_envelopes_and_encoded_ids() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/prefix/api/credential"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"data":[entry()],"future":true})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let request =
        CredentialCreateRequest::new("owned".parse().unwrap(), CredentialValueInput::key(PRIVATE))
            .id("a/b ? 雪".parse().unwrap())
            .activate(false);
    Mock::given(method("POST"))
        .and(path("/prefix/api/credential"))
        .and(body_json(serde_json::to_value(&request).unwrap()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":entry()})))
        .expect(1)
        .mount(&server)
        .await;
    for (verb, suffix) in [("PATCH", ""), ("POST", "/activate"), ("DELETE", "")] {
        Mock::given(method(verb))
            .and(path(format!(
                "/prefix/api/credential/a%2Fb%20%3F%20%E9%9B%AA{suffix}"
            )))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&server)
            .await;
    }
    let client = Client::builder(format!("{}/prefix/", server.uri()))
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let listed = client.credentials().list().await.unwrap();
    assert_eq!(listed.extra["future"], true);
    assert!(
        client
            .credentials()
            .create(&request)
            .await
            .unwrap()
            .data
            .active
    );
    let id = "a/b ? 雪".parse().unwrap();
    client
        .credentials()
        .update(&id, &CredentialUpdateRequest::new(""))
        .await
        .unwrap();
    client.credentials().activate(&id).await.unwrap();
    client.credentials().remove(&id).await.unwrap();
    let requests = server.received_requests().await.unwrap();
    assert!(requests.iter().all(|r| r.url.query().is_none()));
    assert_eq!(
        requests[2].body_json::<Value>().unwrap(),
        json!({"label":""})
    );
    assert!(requests[3].body.is_empty() && requests[4].body.is_empty());
}

#[tokio::test]
async fn direct_connections_preserve_location_selection_and_null_fields() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/integration/a%2Fb/connect/key"))
        .and(query_param("location[directory]", "/default"))
        .and(body_json(json!({"key":PRIVATE})))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/integration/a%2Fb/connect/external"))
        .and(query_param("location[directory]", "/override"))
        .and(body_json(
            json!({"methodID":"external","answer":null,"label":null}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .default_location(Location::directory("/default"))
        .build()
        .unwrap();
    let id = "a/b".parse().unwrap();
    client
        .integrations()
        .connect_key(&id, &IntegrationConnectKeyRequest::new(PRIVATE))
        .await
        .unwrap();
    let mut external = IntegrationConnectExternalRequest::new("external".parse().unwrap());
    external.answer = RequestField::Null;
    external.label = RequestField::Null;
    client
        .integrations()
        .connect_external_at(&id, &external, Location::directory("/override"))
        .await
        .unwrap();
    server.reset().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(204))
        .expect(4)
        .mount(&server)
        .await;
    for at in [Location::Unspecified, Location::directory("")] {
        client
            .integrations()
            .connect_key_at(&id, &IntegrationConnectKeyRequest::new(""), at.clone())
            .await
            .unwrap();
        client
            .integrations()
            .connect_external_at(&id, &external, at)
            .await
            .unwrap();
    }
    let calls = server.received_requests().await.unwrap();
    assert!(calls[..2].iter().all(|r| r.url.query().is_none()));
    for r in &calls[2..] {
        assert_eq!(
            r.url.query_pairs().collect::<Vec<_>>(),
            [("location[directory]".into(), "".into())]
        );
    }
}

#[tokio::test]
async fn credential_mutations_do_not_retry_and_diagnostics_hide_secret_values() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).respond_with(ResponseTemplate::new(400).set_body_json(json!({"_tag":"InvalidRequestError","message":PRIVATE,"kind":"integration_authorization"}))).expect(4).mount(&server).await;
    for verb in ["PATCH", "DELETE"] {
        Mock::given(method(verb))
            .respond_with(ResponseTemplate::new(503).set_body_json(
                json!({"_tag":"ServiceUnavailableError","service":"fixture","message":PRIVATE}),
            ))
            .expect(1)
            .mount(&server)
            .await;
    }
    let client = Client::new(server.uri()).unwrap();
    let id = "fixture".parse().unwrap();
    let integration: IntegrationId = "owned".parse().unwrap();
    let failures = [
        client
            .credentials()
            .create(&CredentialCreateRequest::new(
                integration.clone(),
                CredentialValueInput::key(PRIVATE),
            ))
            .await
            .unwrap_err(),
        client
            .credentials()
            .update(&id, &CredentialUpdateRequest::new(PRIVATE))
            .await
            .unwrap_err(),
        client.credentials().activate(&id).await.unwrap_err(),
        client.credentials().remove(&id).await.unwrap_err(),
        client
            .integrations()
            .connect_key(&integration, &IntegrationConnectKeyRequest::new(PRIVATE))
            .await
            .unwrap_err(),
        client
            .integrations()
            .connect_external(
                &integration,
                &IntegrationConnectExternalRequest::new("external".parse().unwrap()),
            )
            .await
            .unwrap_err(),
    ];
    for error in failures {
        assert!(!format!("{error} {error:?}").contains(PRIVATE));
        let Error::Http(e) = error else {
            panic!("http")
        };
        assert!(matches!(
            e.api_error(),
            Some(ApiError::InvalidRequest { .. } | ApiError::ServiceUnavailable { .. })
        ));
        assert!(String::from_utf8_lossy(e.body()).contains(PRIVATE));
    }
}

#[tokio::test]
async fn credential_path_ids_fail_locally_without_sending_requests() {
    let server = MockServer::start().await;
    let client = Client::new(server.uri()).unwrap();
    for value in ["", ".", ".."] {
        let id = value.parse().unwrap();
        let integration = value.parse().unwrap();
        let failures = [
            client
                .credentials()
                .update(&id, &CredentialUpdateRequest::new("x"))
                .await
                .unwrap_err(),
            client.credentials().activate(&id).await.unwrap_err(),
            client.credentials().remove(&id).await.unwrap_err(),
            client
                .integrations()
                .connect_key(&integration, &IntegrationConnectKeyRequest::new("x"))
                .await
                .unwrap_err(),
            client
                .integrations()
                .connect_external(
                    &integration,
                    &IntegrationConnectExternalRequest::new("x".parse().unwrap()),
                )
                .await
                .unwrap_err(),
        ];
        assert!(failures.iter().all(|e| matches!(e, Error::Encode(_))));
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn credential_reads_bound_bodies_and_reject_wrong_status_or_shape() {
    let server = MockServer::start().await;
    let client = Client::builder(server.uri())
        .max_response_bytes(1024)
        .max_error_body_bytes(128)
        .build()
        .unwrap();
    for (status, body, kind) in [
        (201, json!({"data":[]}).to_string(), "status"),
        (
            200,
            json!({"data":[{"value":PRIVATE}]}).to_string(),
            "decode",
        ),
        (200, "x".repeat(2048), "large"),
    ] {
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(status).set_body_string(body))
            .expect(1)
            .mount(&server)
            .await;
        let error = client.credentials().list().await.unwrap_err();
        assert!(!format!("{error} {error:?}").contains(PRIVATE));
        assert!(matches!(
            (kind, &error),
            ("status", Error::Http(_))
                | ("decode", Error::Decode(_))
                | ("large", Error::ResponseTooLarge(_))
        ));
        server.reset().await;
    }
}
