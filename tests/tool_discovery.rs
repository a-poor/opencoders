use opencoders::{ApiError, Client, Error, Location, models::*};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn integration() -> Value {
    json!({"id":"fixture","name":"private-value","metadata":{"extra":9007199254740991_u64},"methods":[
        {"type":"oauth","id":"oauth","label":"OAuth","form":[{"type":"string","key":"token","title":"Token","default":"private-value"}]},
        {"type":"command","id":"command","label":"Command","command":["private-value"]},
        {"type":"key","label":"Key"},{"type":"external","id":"external","label":"External"},{"type":"env","names":["FIXTURE_KEY"]}],
        "connections":[{"type":"credential","id":"credential","label":"Stored","method":"oauth","status":{"status":"needs_auth","message":"private-value","url":"private-value"}},{"type":"env","name":"FIXTURE_KEY","extra":true}],"future":"private-value"})
}
fn envelope(data: Value) -> Value {
    json!({"location":{"directory":"/resolved","projectID":"project"},"data":data,"future":"private-value"})
}

#[test]
fn integration_methods_connections_and_forms_are_typed_and_redacted() {
    let value: IntegrationInfo = serde_json::from_value(integration()).unwrap();
    assert_eq!(value.methods.len(), 5);
    assert_eq!(value.connections.len(), 2);
    let IntegrationMethod::Oauth(oauth) = &value.methods[0] else {
        panic!("oauth")
    };
    let form: &FormFields = oauth.form.as_ref().unwrap();
    assert_eq!(form.0.as_slice().len(), 1);
    let shared: &setup::IntegrationId = &value.id;
    assert_eq!(shared.as_str(), "fixture");
    let encoded = serde_json::to_value(&value).unwrap();
    assert_eq!(encoded["metadata"]["extra"], 9007199254740991_u64);
    assert_eq!(encoded["connections"][1]["extra"], true);
    for debug in [
        format!("{value:?}"),
        format!("{:?}", value.methods),
        format!("{oauth:?}"),
        format!("{:?}", value.connections),
    ] {
        assert!(!debug.contains("private-value"));
    }
    for invalid in [
        json!({"type":"oauth"}),
        json!({"type":"external","id":"x"}),
        json!({"type":"command","id":"x","label":"X"}),
        json!({"type":"env"}),
        json!({"type":"future"}),
        json!({"type":"key","form":[]}),
        json!({"type":"key","form":[{"type":"integer","key":"n","minimum":"invalid"}]}),
    ] {
        assert!(serde_json::from_value::<IntegrationMethod>(invalid).is_err());
    }
    let numeric:IntegrationMethod=serde_json::from_value(json!({"type":"key","form":[{"type":"integer","key":"n","minimum":-1,"maximum":9007199254740991_u64}]})).unwrap();
    assert_eq!(
        serde_json::to_value(numeric).unwrap()["form"][0]["maximum"],
        9007199254740991_u64
    );
    for invalid in [
        json!({"type":"credential","id":"x","method":"key"}),
        json!({"type":"env"}),
        json!({"type":"future"}),
        json!({"type":"env","name":"x","status":{"status":"future","message":"x"}}),
    ] {
        assert!(serde_json::from_value::<ConnectionInfo>(invalid).is_err());
    }
}

#[test]
fn mcp_statuses_and_catalogs_require_known_fields_and_preserve_extensions() {
    for status in [
        json!({"status":"connected"}),
        json!({"status":"pending"}),
        json!({"status":"disabled"}),
        json!({"status":"failed","error":"private-value"}),
        json!({"status":"needs_auth","error":"private-value"}),
    ] {
        let server:McpServer=serde_json::from_value(json!({"name":"fixture","status":status,"integrationID":"fixture","future":"private-value"})).unwrap();
        assert_eq!(serde_json::to_value(&server).unwrap()["status"], status);
        assert!(!format!("{server:?} {:?}", server.status).contains("private-value"));
    }
    for invalid in [
        json!({"status":"failed"}),
        json!({"status":"needs_auth"}),
        json!({"status":"future"}),
        json!({}),
    ] {
        assert!(serde_json::from_value::<McpStatus>(invalid).is_err());
    }
    let catalog:McpResourceCatalog=serde_json::from_value(json!({"resources":[{"server":"f","name":"n","uri":"fixture://x","mimeType":"text/plain","future":9007199254740991_u64}],"templates":[{"server":"f","name":"n","uriTemplate":"fixture://{name}","description":"private-value"}],"future":true})).unwrap();
    assert_eq!(catalog.resources[0].extra["future"], 9007199254740991_u64);
    assert_eq!(catalog.templates[0].uri_template, "fixture://{name}");
    assert!(!format!("{catalog:?} {:?}", catalog.templates).contains("private-value"));
    for invalid in [
        json!({"resources":[]}),
        json!({"resources":[],"templates":[{"server":"f","name":"x"}]}),
    ] {
        assert!(serde_json::from_value::<McpResourceCatalog>(invalid).is_err());
    }
}

#[test]
fn plugin_sources_states_and_true_only_flags_are_validated() {
    for source in [
        json!({"type":"builtin"}),
        json!({"type":"sdk"}),
        json!({"type":"local","path":"private-value"}),
        json!({"type":"package","target":"private-value","version":"1","outdated":true,"updating":true}),
    ] {
        let plugin:PluginInfo=serde_json::from_value(json!({"source":source,"features":{"server":true,"gui":true,"rpc":true,"tui":true},"state":{"status":"failed","error":"private-value","ref":"private-value"},"future":true})).unwrap();
        assert!(plugin.id.is_none());
        assert!(plugin.features.server.unwrap().get());
        assert_eq!(serde_json::to_value(&plugin).unwrap()["source"], source);
        assert!(
            !format!("{plugin:?} {:?} {:?}", plugin.source, plugin.state).contains("private-value")
        );
    }
    for source in [
        json!({"type":"package"}),
        json!({"type":"local"}),
        json!({"type":"future"}),
        json!({"type":"package","target":"x","outdated":false}),
        json!({"type":"package","target":"x","updating":null}),
    ] {
        assert!(serde_json::from_value::<PluginSource>(source).is_err());
    }
    for feature in [false, true] {
        let result = serde_json::from_value::<PluginInfo>(
            json!({"source":{"type":"sdk"},"features":{"server":feature},"state":{"status":"active"}}),
        );
        assert_eq!(result.is_ok(), feature);
    }
    assert!(serde_json::from_value::<PluginState>(json!({"status":"failed"})).is_err());
    assert!(serde_json::from_value::<PluginState>(json!({"status":"future"})).is_err());
}

#[tokio::test]
async fn tool_discovery_preserves_locations_envelopes_order_and_encoded_ids() {
    let server = MockServer::start().await;
    for (route, data) in [
        ("integration", json!([integration()])),
        ("mcp", json!([{"name":"f","status":{"status":"connected"}}])),
        ("mcp/resource", json!({"resources":[],"templates":[]})),
        (
            "plugin",
            json!([{"source":{"type":"builtin"},"features":{},"state":{"status":"active"}}]),
        ),
        (
            "websearch/provider",
            json!([{"id":"z","name":"Z"},{"id":"a","name":"A"}]),
        ),
    ] {
        Mock::given(method("GET"))
            .and(path(format!("/proxy/api/{route}")))
            .and(query_param("location[directory]", "/work ? 雪"))
            .respond_with(ResponseTemplate::new(200).set_body_json(envelope(data)))
            .expect(1)
            .mount(&server)
            .await;
    }
    Mock::given(method("GET"))
        .and(path("/proxy/api/integration/a%2Fb%20%3F%20%E9%9B%AA"))
        .and(query_param("location[directory]", "/override"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(integration())))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/work ? 雪"))
        .build()
        .unwrap();
    let integrations = client.integrations().list().await.unwrap();
    assert_eq!(integrations.location.directory.as_str(), "/resolved");
    assert_eq!(integrations.extra["future"], "private-value");
    assert!(!format!("{integrations:?}").contains("private-value"));
    assert_eq!(
        client
            .integrations()
            .get_at(
                &"a/b ? 雪".parse().unwrap(),
                Location::directory("/override")
            )
            .await
            .unwrap()
            .data
            .methods
            .len(),
        5
    );
    assert_eq!(client.mcp().list().await.unwrap().data.len(), 1);
    assert!(
        client
            .mcp()
            .resource_catalog()
            .await
            .unwrap()
            .data
            .resources
            .is_empty()
    );
    assert_eq!(client.plugins().list().await.unwrap().data.len(), 1);
    assert_eq!(
        client
            .websearch()
            .providers()
            .await
            .unwrap()
            .data
            .iter()
            .map(|p| p.id.as_str())
            .collect::<Vec<_>>(),
        ["z", "a"]
    );
}

#[tokio::test]
async fn discovery_overrides_preserve_empty_and_unspecified_location_queries() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503).set_body_json(
            json!({"_tag":"ServiceUnavailableError","service":"fixture","message":"offline"}),
        ))
        .expect(12)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let id = "fixture".parse().unwrap();
    for location in [Location::Unspecified, Location::directory("")] {
        let errors = [
            client
                .integrations()
                .list_at(location.clone())
                .await
                .unwrap_err(),
            client
                .integrations()
                .get_at(&id, location.clone())
                .await
                .unwrap_err(),
            client.mcp().list_at(location.clone()).await.unwrap_err(),
            client
                .mcp()
                .resource_catalog_at(location.clone())
                .await
                .unwrap_err(),
            client
                .plugins()
                .list_at(location.clone())
                .await
                .unwrap_err(),
            client.websearch().providers_at(location).await.unwrap_err(),
        ];
        for error in errors {
            let Error::Http(e) = error else {
                panic!("HTTP error")
            };
            assert!(matches!(
                e.api_error(),
                Some(ApiError::ServiceUnavailable { .. })
            ));
        }
    }
    let requests = server.received_requests().await.unwrap();
    for r in &requests[..6] {
        assert!(r.url.query().is_none());
    }
    for r in &requests[6..] {
        assert_eq!(
            r.url.query_pairs().collect::<Vec<_>>(),
            [("location[directory]".into(), "".into())]
        );
    }
}

#[tokio::test]
async fn integration_errors_are_typed_and_unrepresentable_ids_never_send() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).respond_with(ResponseTemplate::new(404).set_body_json(json!({"_tag":"IntegrationNotFoundError","integrationID":"missing","message":"private-value"}))).expect(1).mount(&server).await;
    let client = Client::new(server.uri()).unwrap();
    for id in ["", ".", ".."] {
        assert!(matches!(
            client.integrations().get(&id.parse().unwrap()).await,
            Err(Error::Encode(_))
        ));
    }
    let Error::Http(e) = client
        .integrations()
        .get(&"missing".parse().unwrap())
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(e.operation(), "integration.get");
    assert_eq!(e.status(), 404);
    assert!(
        matches!(e.api_error(),Some(ApiError::IntegrationNotFound{integration_id,..}) if integration_id=="missing")
    );
    assert!(!format!("{e:?}").contains("private-value"));
}

#[tokio::test]
async fn discovery_rejects_wrong_status_missing_envelopes_and_oversized_catalogs() {
    let server = MockServer::start().await;
    Mock::given(path("/api/integration"))
        .respond_with(ResponseTemplate::new(201).set_body_json(envelope(json!([]))))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/api/plugin"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/api/mcp/resource"))
        .respond_with(ResponseTemplate::new(200).set_body_string("x".repeat(2048)))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .max_response_bytes(1024)
        .build()
        .unwrap();
    assert_eq!(
        client
            .integrations()
            .list()
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        201
    );
    assert!(matches!(
        client.plugins().list().await,
        Err(Error::Decode(_))
    ));
    assert!(matches!(
        client.mcp().resource_catalog().await,
        Err(Error::ResponseTooLarge(_))
    ));
}
