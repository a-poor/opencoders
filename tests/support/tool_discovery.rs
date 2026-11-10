//! Populated discovery fixtures after the smoke harness has verified ownership.
use opencoders::{ApiError, Client, Error, Location, models::*};
use std::{path::Path, time::Duration};

async fn settled_mcp(client: &Client, location: Location) -> McpListResponse {
    tokio::time::timeout(Duration::from_secs(15), async {
        loop {
            let servers = client.mcp().list_at(location.clone()).await.unwrap();
            if servers.data.len() == 4
                && servers
                    .data
                    .iter()
                    .all(|s| !matches!(s.status, McpStatus::Pending(_)))
            {
                return servers;
            }
            tokio::time::sleep(Duration::from_millis(30)).await;
        }
    })
    .await
    .expect("owned MCP fixture did not settle")
}
pub(super) async fn verify(client: &Client, fallback: &Client, root: &Path, other: &str) {
    println!("smoke scenario: integration, MCP, plugin and search discovery");
    let workspace = root.join("workspace + unicode 雪");
    let integrations = client.integrations().list().await.unwrap();
    assert_eq!(
        integrations.location.directory.as_str(),
        workspace.to_str().unwrap()
    );
    let id: IntegrationId = "opencoders-discovery".parse().unwrap();
    if !integrations.data.iter().any(|i| i.id == id) {
        for plugin in client.plugins().list().await.unwrap().data {
            if let PluginSource::Local(local) = &plugin.source
                && local.path.contains("discovery-fixtures")
                && let PluginState::Failed(failure) = &plugin.state
            {
                // Only our owned fixture's loader/setup error, never raw catalogs.
                eprintln!("owned discovery plugin failed: {}", failure.error);
            }
        }
    }
    let listed = integrations
        .data
        .iter()
        .find(|i| i.id == id)
        .expect("owned plugin integration missing");
    let detail = client.integrations().get(&id).await.unwrap().data;
    assert!(serde_json::to_value(&detail).unwrap() == serde_json::to_value(listed).unwrap());
    assert_eq!(detail.methods.len(), 5);
    assert_eq!(detail.connections.len(), 1);
    let kinds: Vec<_> = detail
        .methods
        .iter()
        .map(|m| match m {
            IntegrationMethod::Oauth(_) => "oauth",
            IntegrationMethod::Command(_) => "command",
            IntegrationMethod::Key(_) => "key",
            IntegrationMethod::External(_) => "external",
            IntegrationMethod::Env(_) => "env",
            _ => panic!("unexpected method"),
        })
        .collect();
    assert_eq!(kinds, ["oauth", "command", "key", "external", "env"]);
    let IntegrationMethod::Oauth(oauth) = &detail.methods[0] else {
        unreachable!()
    };
    assert_eq!(oauth.form.as_ref().unwrap().0.as_slice().len(), 1);
    let ConnectionInfo::Env(connection) = &detail.connections[0] else {
        panic!("environment connection")
    };
    assert_eq!(connection.name, "OPENCODERS_DISCOVERY_PRESENT");
    assert_eq!(
        connection.status.as_ref().unwrap().message,
        "Owned fixture status"
    );
    assert!(
        !serde_json::to_string(&detail)
            .unwrap()
            .contains("fixture-marker")
    );
    let denied: IntegrationId = "opencoders-discovery-denied".parse().unwrap();
    assert!(!integrations.data.iter().any(|i| i.id == denied));
    assert_eq!(
        client.integrations().get(&denied).await.unwrap().data.id,
        denied
    );
    let Error::Http(error) = client
        .integrations()
        .get(&"missing-discovery-integration".parse().unwrap())
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(error.status(), 404);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::IntegrationNotFound { .. })
    ));

    let servers = settled_mcp(client, Location::directory(workspace.to_str().unwrap())).await;
    assert_eq!(
        servers
            .data
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>(),
        [
            "fixture-catalog",
            "fixture-disabled",
            "fixture-failed",
            "fixture-no-templates"
        ]
    );
    assert!(matches!(servers.data[0].status, McpStatus::Connected(_)));
    assert!(matches!(servers.data[1].status, McpStatus::Disabled(_)));
    assert!(matches!(servers.data[2].status, McpStatus::Failed(_)));
    assert!(matches!(servers.data[3].status, McpStatus::Connected(_)));
    assert_eq!(servers.data[0].title.as_deref(), Some("Owned MCP fixture"));
    assert!(servers.data.iter().all(|s| s.integration_id.is_none()));
    let catalog = client.mcp().resource_catalog().await.unwrap();
    assert_eq!(
        catalog
            .data
            .resources
            .iter()
            .map(|r| (r.server.as_str(), r.name.as_str(), r.uri.as_str()))
            .collect::<Vec<_>>(),
        [
            ("fixture-catalog", "Alpha", "fixture://a"),
            ("fixture-catalog", "Zulu", "fixture://z"),
            ("fixture-no-templates", "Alpha", "fixture://a"),
            ("fixture-no-templates", "Zulu", "fixture://z"),
        ]
    );
    assert_eq!(
        catalog
            .data
            .templates
            .iter()
            .map(|t| (t.server.as_str(), t.name.as_str(), t.uri_template.as_str()))
            .collect::<Vec<_>>(),
        [
            ("fixture-catalog", "Alpha", "fixture://a/{name}"),
            ("fixture-catalog", "Zulu", "fixture://z/{name}"),
        ]
    );
    assert_eq!(
        catalog.data.resources[0].mime_type.as_deref(),
        Some("application/json")
    );
    let plugins = client.plugins().list().await.unwrap();
    let plugin = plugins
        .data
        .iter()
        .find(|p| {
            p.id.as_ref()
                .is_some_and(|id| id.as_str() == "opencoders.discovery.fixture")
        })
        .unwrap();
    assert!(matches!(plugin.state, PluginState::Active(_)));
    assert!(plugin.features.server.unwrap().get());
    let PluginSource::Local(local) = &plugin.source else {
        panic!("local plugin")
    };
    assert!(Path::new(&local.path).starts_with(root));
    let failed = plugins
        .data
        .iter()
        .find(|p| {
            p.id.as_ref()
                .is_some_and(|id| id.as_str() == "opencoders.discovery.failed")
        })
        .unwrap();
    assert!(matches!(failed.state, PluginState::Failed(_)));
    assert!(
        plugins
            .data
            .iter()
            .any(|p| matches!(p.source, PluginSource::Builtin(_)))
    );
    let search = client.websearch().providers().await.unwrap();
    let owned: Vec<_> = search
        .data
        .iter()
        .filter(|p| p.id.as_str().starts_with("fixture-"))
        .map(|p| p.id.as_str())
        .collect();
    assert_eq!(owned, ["fixture-a", "fixture-z"]); // registered even with websearch:false

    let at = Location::directory(other);
    let other_list = fallback.integrations().list_at(at.clone()).await.unwrap();
    assert_eq!(other_list.location.directory.as_str(), other);
    assert_eq!(
        fallback
            .integrations()
            .get_at(&id, at.clone())
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        other
    );
    assert_eq!(
        settled_mcp(fallback, at.clone())
            .await
            .location
            .directory
            .as_str(),
        other
    );
    assert_eq!(
        fallback
            .mcp()
            .resource_catalog_at(at.clone())
            .await
            .unwrap()
            .data
            .resources
            .len(),
        4
    );
    assert_eq!(
        fallback
            .plugins()
            .list_at(at.clone())
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        other
    );
    assert_eq!(
        fallback
            .websearch()
            .providers_at(at)
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        other
    );
    let fixture = root.join("discovery-fixtures");
    assert!(!fixture.join("fixture-disabled.jsonl").exists());
    for name in ["fixture-catalog", "fixture-no-templates"] {
        let contents = std::fs::read_to_string(fixture.join(format!("{name}.jsonl"))).unwrap();
        let calls: Vec<serde_json::Value> = contents
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert!(calls.iter().any(|c| c["method"] == "resources/list"));
        assert!(
            calls
                .iter()
                .any(|c| c["method"] == "resources/templates/list")
        );
        assert!(calls.iter().all(|c| matches!(
            c["method"].as_str(),
            Some(
                "initialize"
                    | "notifications/initialized"
                    | "tools/list"
                    | "prompts/list"
                    | "resources/list"
                    | "resources/templates/list"
                    | "ping"
            )
        )));
    }
}
