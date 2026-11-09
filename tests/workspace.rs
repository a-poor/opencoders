use opencoders::{
    Client, FileFindOptions, FileListOptions, Location, VcsBranchOptions, VcsDiffOptions,
    models::{FileSystemEntryType, VcsMode, VcsUntracked},
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

#[tokio::test]
async fn filesystem_discovery_preserves_paths_location_overrides_and_server_order() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/proxy/api/fs/list"))
        .and(query_param("location[directory]","/default space?"))
        .and(query_param("path","../other +/"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"location":{"directory":"/resolved"},"data":[{"path":"z/","type":"directory"},{"path":"a","type":"file"}],"extra":1}))).expect(1).mount(&server).await;
    Mock::given(method("GET"))
        .and(path("/proxy/api/fs/find"))
        .and(query_param("location[directory]", "/override"))
        .and(query_param("query", "a+b &/?"))
        .and(query_param("type", "file"))
        .and(query_param("limit", "2"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"location":{"directory":"/override"},"data":[]})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/default space?"))
        .build()
        .unwrap();
    let result = client
        .files()
        .list(&FileListOptions::default().path("../other +/"))
        .await
        .unwrap();
    assert_eq!(result.data[0].path.as_str(), "z/");
    assert_eq!(result.extra["extra"], 1);
    client
        .files()
        .find_at(
            &FileFindOptions::new("a+b &/?")
                .kind(FileSystemEntryType::File)
                .limit(2),
            Location::directory("/override"),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn vcs_queries_preserve_empty_base_zero_context_and_required_mode() {
    let server = MockServer::start().await;
    for (route, body) in [
        ("/api/vcs", json!({"branch":{},"provider":"git"})),
        ("/api/vcs/base", json!(null)),
        ("/api/vcs/status", json!([])),
        ("/api/vcs/branch", json!(["z", "a"])),
        ("/api/vcs/diff", json!([])),
    ] {
        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"location":{"directory":"/repo"},"data":body})),
            )
            .expect(1)
            .mount(&server)
            .await;
    }
    let client = Client::builder(server.uri())
        .default_location(Location::directory("/repo"))
        .build()
        .unwrap();
    client.vcs().get().await.unwrap();
    assert!(client.vcs().base().await.unwrap().data.is_none());
    client.vcs().status().await.unwrap();
    let branches = client
        .vcs()
        .branches(&VcsBranchOptions::default().search("").limit(7))
        .await
        .unwrap();
    assert_eq!(branches.data.0, vec!["z", "a"]);
    client
        .vcs()
        .diff_at(
            &VcsDiffOptions::new(VcsMode::Committed)
                .base("")
                .context(0)
                .untracked(VcsUntracked::Folders),
            Location::Unspecified,
        )
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    let diff = requests.last().unwrap();
    let pairs = diff
        .url
        .query_pairs()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(pairs["mode"], "committed");
    assert_eq!(pairs["base"], "");
    assert_eq!(pairs["context"], "0");
    assert_eq!(pairs["untracked"], "folders");
    assert!(!pairs.contains_key("location[directory]"));
    assert_eq!(
        requests[3]
            .url
            .query_pairs()
            .find(|(key, _)| key == "search")
            .unwrap()
            .1,
        ""
    );
}

#[tokio::test]
async fn worktree_inventory_uses_project_id_and_ignores_default_location() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/worktree"))
        .and(query_param("projectID", "project /?&"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!([{"directory":"/repo","strategy":"git","future":true}])),
        )
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .default_location(Location::directory("/unused"))
        .build()
        .unwrap();
    let result = client
        .worktrees()
        .list(&"project /?&".parse().unwrap())
        .await
        .unwrap();
    assert_eq!(result.0[0].extra["future"], true);
    assert_eq!(
        server.received_requests().await.unwrap()[0]
            .url
            .query_pairs()
            .count(),
        1
    );
}

#[tokio::test]
async fn workspace_errors_keep_typed_missing_resources_and_bounded_diagnostics() {
    use opencoders::{ApiError, Error};
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/fs/list"))
        .respond_with(ResponseTemplate::new(404).set_body_json(
            json!({"_tag":"FileNotFoundError","path":"private-path","message":"private-message"}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let err = client
        .files()
        .list(&FileListOptions::default())
        .await
        .unwrap_err();
    assert!(!format!("{err:?}").contains("private-"));
    let Error::Http(error) = err else {
        panic!("expected HTTP error")
    };
    assert!(
        matches!(error.api_error(),Some(ApiError::FileNotFound {path,..}) if path=="private-path")
    );
}
