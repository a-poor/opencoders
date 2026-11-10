use opencoders::{Client, Location, ProjectListOptions, models::*};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

#[test]
fn configuration_preserves_documents_sources_numbers_and_redacts_nested_values() {
    let value = json!({"type":"document","info":{"username":"private-value","providers":{"fixture":{"settings":{"apiKey":"private-value","timeout":false},"headers":{"authorization":"private-value"}}},"mcp":{"servers":{"remote":{"type":"remote","url":"private-value","oauth":{"client_secret":"private-value"}}}},"compaction":{"keep":{"tokens":9_007_199_254_740_991_u64}},"future":{"token":"private-value"}},"future":"private-value"});
    let entry: ConfigEntry = serde_json::from_value(value).unwrap();
    let serialized = serde_json::to_value(&entry).unwrap();
    assert_eq!(
        serialized["info"]["compaction"]["keep"]["tokens"],
        json!(9_007_199_254_740_991_u64)
    );
    assert_eq!(
        serialized["info"]["providers"]["fixture"]["settings"]["apiKey"],
        "private-value"
    );
    assert!(!format!("{entry:?}").contains("private-value"));
    let document: models_setup::ConfigDocumentEncoded = serde_json::from_value(serialized).unwrap();
    assert!(
        !format!("{:?} {:?}", document.info, document.info.providers).contains("private-value")
    );
    for (index,value) in [json!({"type":"unknown","info":{}}),json!({"type":"directory"}),json!({"type":"document"}),json!({"type":"document","info":{"providers":{"x":{"settings":{"timeout":true}}}}}),json!({"type":"document","info":{"websearch":true}}),json!({"type":"document","info":{"compaction":{"keep":{"tokens":9_007_199_254_740_992_u64}}}})].into_iter().enumerate() {
        assert!(serde_json::from_value::<ConfigEntry>(value).is_err(),"case {index}");
    }
    let directory: ConfigEntry =
        serde_json::from_value(json!({"type":"directory","path":"/server/path","future":1}))
            .unwrap();
    assert_eq!(serde_json::to_value(directory).unwrap()["future"], 1);
}

#[test]
fn projects_and_references_validate_known_shapes_and_preserve_extensions() {
    let value = json!({"id":"p","canonical":"/work","time":{"created":0,"updated":1,"active":9_007_199_254_740_991_u64},"sandboxes":["/second","/first"],"vcs":"git","extra":true});
    let project: Project = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(project.id.as_str(), "p");
    assert_eq!(project.extra["extra"], true);
    assert_eq!(
        serde_json::to_value(project).unwrap()["sandboxes"],
        value["sandboxes"]
    );
    for number in [json!(-1), json!(9_007_199_254_740_992_u64), json!(1.5)] {
        let mut v = value.clone();
        v["time"]["active"] = number;
        assert!(serde_json::from_value::<Project>(v).is_err());
    }
    for source in [
        json!({"type":"local","path":"/work","hidden":true}),
        json!({"type":"git","repository":"https://example.invalid/repo","branch":"main"}),
    ] {
        let r: ReferenceInfo = serde_json::from_value(
            json!({"name":"fixture","path":"/work","source":source,"future":true}),
        )
        .unwrap();
        assert_eq!(serde_json::to_value(r).unwrap()["source"], source);
    }
    assert!(
        serde_json::from_value::<ReferenceInfo>(
            json!({"name":"f","path":"/work","source":{"type":"local","repository":"bad"}})
        )
        .is_err()
    );
}

#[tokio::test]
async fn setup_reads_preserve_array_shapes_locations_filters_and_provider_ids() {
    let server = MockServer::start().await;
    for (route, body, scoped) in [
        (
            "config",
            json!([{ "type":"directory","path":"/a"},{"type":"document","info":{}}]),
            true,
        ),
        (
            "config/shell",
            json!([{"name":"sh","path":"/bin/sh","acceptable":false}]),
            false,
        ),
        ("project", json!([]), false),
        (
            "reference",
            json!({"location":{"projectID":"p","directory":"/resolved"},"data":[],"future":true}),
            true,
        ),
        (
            "provider",
            json!({"location":{"projectID":"p","directory":"/resolved"},"data":[{"id":"fixture","name":"f","activation":"auto","package":"fixture","settings":{"apiKey":"private-value"}}],"future":"private-value"}),
            true,
        ),
    ] {
        let mut mock = Mock::given(method("GET")).and(path(format!("/proxy/api/{route}")));
        if scoped {
            mock = mock.and(query_param("location[directory]", "/work ? 雪"));
        }
        if route == "project" {
            mock = mock.and(query_param("archived", "false"));
        }
        mock.respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(1)
            .mount(&server)
            .await;
    }
    Mock::given(method("GET")).and(path("/proxy/api/provider/a%2Fb%20%3F%20%E9%9B%AA")).respond_with(ResponseTemplate::new(200).set_body_json(json!({"location":{"projectID":"p","directory":"/resolved"},"data":{"id":"a/b ? 雪","name":"f","activation":"disabled","package":"fixture"}}))).expect(1).mount(&server).await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/work ? 雪"))
        .build()
        .unwrap();
    assert_eq!(client.config().get().await.unwrap().len(), 2);
    assert!(!client.config().shells().await.unwrap()[0].acceptable);
    assert!(
        client
            .projects()
            .list(&ProjectListOptions::default().archived(false))
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        client.references().list().await.unwrap().extra["future"],
        true
    );
    let providers = client.providers().list().await.unwrap();
    assert!(!format!("{providers:?} {:?}", providers.data[0].settings).contains("private-value"));
    client
        .providers()
        .get_at(&"a/b ? 雪".parse().unwrap(), Location::Unspecified)
        .await
        .unwrap();
    for r in server.received_requests().await.unwrap() {
        if r.url.path().ends_with("/project") {
            assert_eq!(r.url.query(), Some("archived=false"));
        }
        if r.url.path().ends_with("/shell") || r.url.path().contains("/provider/a") {
            assert!(r.url.query().is_none());
        }
        assert!(r.body.is_empty());
    }
}

#[tokio::test]
async fn provider_errors_retain_typed_details_and_reject_unrepresentable_ids() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).respond_with(ResponseTemplate::new(404).set_body_json(json!({"_tag":"ProviderNotFoundError","providerID":"private-provider","message":"private-detail"}))).expect(1).mount(&server).await;
    let client = Client::new(server.uri()).unwrap();
    for id in ["", ".", ".."] {
        assert!(matches!(
            client.providers().get(&id.parse().unwrap()).await,
            Err(opencoders::Error::Encode(_))
        ));
    }
    let opencoders::Error::Http(e) = client
        .providers()
        .get(&"missing".parse().unwrap())
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(e.operation(), "provider.get");
    assert!(
        matches!(e.api_error(),Some(opencoders::ApiError::ProviderNotFound{provider_id,..}) if provider_id=="private-provider")
    );
    assert!(!format!("{e:?} {:?}", e.api_error()).contains("private-"));
    assert_eq!(
        serde_json::from_slice::<Value>(e.body()).unwrap()["message"],
        "private-detail"
    );
}

use opencoders::models::setup as models_setup;

#[tokio::test]
async fn setup_location_overrides_preserve_empty_and_unspecified_queries() {
    for location in [
        Location::directory(""),
        Location::directory("/override ? 雪"),
        Location::Unspecified,
    ] {
        let server = MockServer::start().await;
        for (route, body) in [
            ("config", json!([])),
            (
                "provider",
                json!({"location":{"projectID":"p","directory":"/resolved"},"data":[]}),
            ),
            (
                "reference",
                json!({"location":{"projectID":"p","directory":"/resolved"},"data":[]}),
            ),
        ] {
            Mock::given(method("GET"))
                .and(path(format!("/api/{route}")))
                .respond_with(ResponseTemplate::new(200).set_body_json(body))
                .expect(1)
                .mount(&server)
                .await;
        }
        let client = Client::builder(server.uri())
            .default_location(Location::directory("/default"))
            .build()
            .unwrap();
        client.config().get_at(location.clone()).await.unwrap();
        client.providers().list_at(location.clone()).await.unwrap();
        client.references().list_at(location.clone()).await.unwrap();
        let expected = match &location {
            Location::Unspecified => None,
            _ => Some(if location == Location::directory("") {
                ""
            } else {
                "/override ? 雪"
            }),
        };
        for request in server.received_requests().await.unwrap() {
            let pairs: Vec<_> = request.url.query_pairs().collect();
            if let Some(value) = expected {
                assert_eq!(pairs.len(), 1);
                assert_eq!(pairs[0].0, "location[directory]");
                assert_eq!(pairs[0].1, value);
            } else {
                assert!(pairs.is_empty());
            }
        }
    }
}

#[tokio::test]
async fn setup_reads_reject_wrong_success_statuses_and_malformed_envelopes() {
    for status in [202, 204, 400, 401, 500] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(status).set_body_json(json!({})))
            .expect(6)
            .mount(&server)
            .await;
        let client = Client::new(server.uri()).unwrap();
        let results = [
            client.config().get().await.map(|_| ()),
            client.config().shells().await.map(|_| ()),
            client
                .projects()
                .list(&ProjectListOptions::default())
                .await
                .map(|_| ()),
            client.providers().list().await.map(|_| ()),
            client
                .providers()
                .get(&"x".parse().unwrap())
                .await
                .map(|_| ()),
            client.references().list().await.map(|_| ()),
        ];
        for result in results {
            assert!(matches!(result, Err(opencoders::Error::Http(_))));
        }
    }
    for body in [
        json!([]),
        json!({"data":[]}),
        json!({"location":{"projectID":"p","directory":"/work"},"data":[{"id":"x","name":"x","package":"x","activation":"future"}]}),
    ] {
        assert!(serde_json::from_value::<ProviderListResponse>(body).is_err());
    }
}

#[test]
fn configuration_unions_and_known_scalars_reject_invalid_values_without_losing_extensions() {
    for info in [
        json!({"lsp":{"x":{"disabled":false}}}),
        json!({"mcp":{"timeout":{"startup":0}}}),
        json!({"mcp":{"servers":{"x":{"type":"remote","url":"/x","oauth":{"callback_port":65536}}}}}),
        json!({"model":"missing-slash"}),
    ] {
        assert!(
            serde_json::from_value::<ConfigEntry>(json!({"type":"document","info":info})).is_err()
        );
    }
    let entry:ConfigEntry=serde_json::from_value(json!({"type":"document","info":{"providers":{"x":{"settings":{"future":null,"timeout":9007199254740993_u64}}},"websearch":{"provider":"custom"},"warming":{"interval":"auto"}}})).unwrap();
    let value = serde_json::to_value(entry).unwrap();
    assert_eq!(
        value["info"]["providers"]["x"]["settings"]["timeout"],
        json!(9007199254740993_u64)
    );
    assert_eq!(
        value["info"]["providers"]["x"]["settings"]["future"],
        Value::Null
    );
}
