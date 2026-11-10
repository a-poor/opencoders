//! Called after the smoke harness verifies disposable server ownership.
use opencoders::{ApiError, Client, Error, Location, ProjectListOptions, models::*};
use std::{path::Path, process::Command};

pub(super) async fn verify(client: &Client, fallback: &Client, root: &Path, other: &str) {
    let entries = client.config().get().await.unwrap();
    let documents: Vec<_> = entries
        .iter()
        .filter_map(|e| match e {
            ConfigEntry::DocumentEncoded(d) => Some(d),
            _ => None,
        })
        .collect();
    let file = documents
        .iter()
        .position(|d| d.info.username.as_deref() == Some("setup-file"))
        .unwrap();
    let content = documents
        .iter()
        .position(|d| d.info.username.as_deref() == Some("setup-content"))
        .unwrap();
    assert!(file < content);
    assert_eq!(
        documents[file].path.as_ref().unwrap().as_str(),
        root.join("config/opencode.json").to_str().unwrap()
    );
    assert!(documents[content].path.is_none());
    assert!(
        entries
            .iter()
            .any(|e| matches!(e, ConfigEntry::DirectoryEncoded(_)))
    );
    assert_eq!(
        serde_json::to_value(
            fallback
                .config()
                .get_at(Location::directory(other))
                .await
                .unwrap()
        )
        .unwrap(),
        serde_json::to_value(&entries).unwrap()
    );
    let shells = client.config().shells().await.unwrap();
    assert!(!shells.is_empty());
    assert!(
        shells
            .iter()
            .all(|s| !s.path.is_empty() && !s.name.is_empty())
    );
    assert!(shells.iter().any(|s| s.acceptable));
    let providers = client.providers().list().await.unwrap(); // waits for plugin registration
    let fixture = providers
        .data
        .iter()
        .find(|p| p.id.as_str() == "setup-provider")
        .unwrap();
    let detail = client.providers().get(&fixture.id).await.unwrap();
    assert_eq!(
        serde_json::to_value(&detail.data).unwrap(),
        serde_json::to_value(fixture).unwrap()
    );
    let other_detail = fallback
        .providers()
        .get_at(&fixture.id, Location::directory(other))
        .await
        .unwrap();
    assert_eq!(other_detail.location.directory.as_str(), other);
    assert_eq!(
        serde_json::to_value(other_detail.data).unwrap(),
        serde_json::to_value(fixture).unwrap()
    );
    assert_eq!(
        client
            .providers()
            .list_at(Location::directory(other))
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        other
    );
    let Error::Http(error) = client
        .providers()
        .get(&"missing-setup-provider".parse().unwrap())
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(error.status(), 404);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::ProviderNotFound { .. })
    ));
    let references = client.references().list().await.unwrap();
    let local = references
        .data
        .iter()
        .find(|r| r.name == "local-fixture")
        .unwrap();
    assert_eq!(local.hidden, Some(true));
    assert_eq!(
        local.path.as_str(),
        root.join("config/reference-fixture").to_str().unwrap()
    );
    assert_eq!(
        client
            .references()
            .list_at(Location::directory(other))
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        other
    );
    assert!(
        Command::new("python3")
            .arg("tools/live/seed_projects.py")
            .status()
            .unwrap()
            .success()
    );
    let project_ids = |projects: Vec<Project>| {
        projects
            .into_iter()
            .filter(|p| p.id.as_str().starts_with("opencoders-setup-"))
            .map(|p| p.id.to_string())
            .collect::<Vec<_>>()
    };
    for options in [
        ProjectListOptions::default(),
        ProjectListOptions::default().archived(false),
    ] {
        assert_eq!(
            project_ids(client.projects().list(&options).await.unwrap()),
            ["opencoders-setup-a", "opencoders-setup-b"]
        );
    }
    assert_eq!(
        project_ids(
            client
                .projects()
                .list(&ProjectListOptions::default().archived(true))
                .await
                .unwrap()
        ),
        [
            "opencoders-setup-c",
            "opencoders-setup-a",
            "opencoders-setup-b"
        ]
    );
    assert!(
        Command::new("python3")
            .args(["tools/live/seed_projects.py", "--cleanup"])
            .status()
            .unwrap()
            .success()
    );
}
