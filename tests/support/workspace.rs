//! Only called after the parent fixture verifies disposable server ownership.
use opencoders::{
    Client, FileFindOptions, FileListOptions, FileReadOptions, Location, VcsBranchOptions,
    VcsDiffOptions, models::*,
};
use std::{path::Path, process::Command, time::Duration};

pub async fn verify(client: &Client, root: &Path, workspace: &str, other: &str) {
    let path = "binary + 雪 ?#%.dat";
    let bytes = b"\0\xffhello\nworld";
    std::fs::write(Path::new(workspace).join(path), bytes).unwrap();
    let entries = client
        .files()
        .list(&FileListOptions::default())
        .await
        .unwrap();
    assert!(entries.data.iter().any(|entry| entry.path.as_str() == path));
    let found = client
        .files()
        .find(&FileFindOptions::new("binary").limit(10))
        .await
        .unwrap();
    assert!(found.data.iter().any(|entry| entry.path.as_str() == path));
    let response = client
        .files()
        .read(path, &FileReadOptions::default())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let etag = response.headers()["etag"].to_str().unwrap().to_owned();
    let modified = response.headers()["last-modified"]
        .to_str()
        .unwrap()
        .to_owned();
    assert_eq!(collect(response).await, bytes);
    let response = client
        .files()
        .read(path, &FileReadOptions::default().range("bytes=2-6"))
        .await
        .unwrap();
    assert_eq!(response.status(), 206);
    assert_eq!(
        response.headers()["content-range"],
        format!("bytes 2-6/{}", bytes.len())
    );
    assert_eq!(collect(response).await, b"hello");
    let response = client
        .files()
        .read(path, &FileReadOptions::default().if_none_match(etag))
        .await
        .unwrap();
    assert_eq!(response.status(), 304);
    assert!(collect(response).await.is_empty());
    let response = client
        .files()
        .read(
            path,
            &FileReadOptions::default().if_modified_since(modified),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 304);
    let response = client
        .files()
        .read(path, &FileReadOptions::default().range("bytes=99999-"))
        .await
        .unwrap();
    assert_eq!(response.status(), 416);
    assert!(collect(response).await.is_empty());
    assert_eq!(
        client
            .files()
            .read("missing.dat", &FileReadOptions::default())
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        404
    );

    // Create a separate Git fixture before its first location resolution. No
    // personal Git config, hooks, repository or network operation participates.
    let repo = root.join("vcs-workspace");
    std::fs::create_dir(&repo).unwrap();
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .args([
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
            ])
            .args(args)
            .current_dir(&repo)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "fixture git failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    git(&["init", "--initial-branch=main"]);
    std::fs::write(repo.join("tracked.txt"), "before\n").unwrap();
    git(&["add", "tracked.txt"]);
    git(&["commit", "-m", "Fixture"]);
    std::fs::write(repo.join("tracked.txt"), "after\n").unwrap();
    std::fs::write(repo.join("untracked.txt"), "new\n").unwrap();
    let location = Location::directory(repo.to_str().unwrap());
    // The first VCS read can precede asynchronous provider activation.
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let info = client.vcs().get_at(location.clone()).await.unwrap();
            if info.data.branch.current.as_deref() == Some("main") {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("Git provider did not become ready");
    client.vcs().base_at(location.clone()).await.unwrap();
    let status = client.vcs().status_at(location.clone()).await.unwrap();
    assert!(status.data.iter().any(|entry| entry.file == "tracked.txt"));
    let branches = client
        .vcs()
        .branches_at(
            &VcsBranchOptions::default().search("mai").limit(5),
            location.clone(),
        )
        .await
        .unwrap();
    assert!(branches.data.0.contains(&"main".to_owned()));
    let diff = client
        .vcs()
        .diff_at(
            &VcsDiffOptions::new(VcsMode::Working)
                .context(1)
                .untracked(VcsUntracked::Files),
            location.clone(),
        )
        .await
        .unwrap();
    assert!(!diff.data.is_empty());
    let diff = client
        .vcs()
        .diff_at(
            &VcsDiffOptions::new(VcsMode::Committed).base("HEAD"),
            location.clone(),
        )
        .await
        .unwrap();
    assert!(diff.data.is_empty());
    let project = client.location().get_at(location).await.unwrap().project.id;
    client.worktrees().list(&project).await.unwrap();

    let pty = client
        .ptys()
        .create(
            &PtyCreateRequest::new()
                .command("/bin/cat")
                .args(Vec::<String>::new())
                .title("fixture"),
        )
        .await
        .unwrap();
    let id = &pty.data.id;
    assert_eq!(pty.data.status, PtyStatus::Running);
    assert!(
        client
            .ptys()
            .list()
            .await
            .unwrap()
            .data
            .iter()
            .any(|item| &item.id == id)
    );
    assert_eq!(
        client.ptys().get(id).await.unwrap().data.command,
        "/bin/cat"
    );
    assert_eq!(
        client
            .ptys()
            .get_at(id, Location::directory(other))
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        404
    );
    let updated = client
        .ptys()
        .update(id, &PtyUpdateRequest::new().title("renamed").size(31, 99))
        .await
        .unwrap();
    assert_eq!(updated.data.title, "renamed");
    let ticket = client.ptys().connect_token(id).await.unwrap();
    assert_eq!(ticket.data.expires_in, 60);
    #[cfg(feature = "websocket")]
    verify_socket(client, id, ticket).await;
    client.ptys().remove(id).await.unwrap();
    assert_eq!(
        client.ptys().get(id).await.unwrap_err().status().unwrap(),
        404
    );

    let exited = client
        .ptys()
        .create(&PtyCreateRequest::new().command("/usr/bin/false"))
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let info = client.ptys().get(&exited.data.id).await.unwrap();
            if info.data.status == PtyStatus::Exited {
                assert_eq!(info.data.exit_code, Some(1));
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    #[cfg(feature = "websocket")]
    {
        let mut connection = client
            .ptys()
            .connect(&exited.data.id, &opencoders::PtyConnectOptions::default())
            .await
            .unwrap();
        assert!(matches!(
            connection.next().await.unwrap().unwrap(),
            opencoders::PtyOutput::Closed {
                code: Some(4404),
                ..
            }
        ));
    }
    client.ptys().remove(&exited.data.id).await.unwrap();
}

async fn collect(response: opencoders::FileResponse) -> Vec<u8> {
    let mut body = response.into_body();
    let mut output = Vec::new();
    while let Some(bytes) = body.next().await {
        output.extend_from_slice(&bytes.unwrap());
    }
    output
}

#[cfg(feature = "websocket")]
async fn verify_socket(client: &Client, id: &PtyId, ticket: PtyConnectTokenResponse) {
    use opencoders::{PtyConnectOptions, PtyOutput};
    let location = Location::directory(ticket.location.directory.as_str());
    let options = PtyConnectOptions::default()
        .ticket(ticket.data.ticket)
        .idle_timeout(Duration::from_secs(5));
    let mut socket = client
        .ptys()
        .connect_at(id, &options, location.clone())
        .await
        .unwrap();
    let mut cursor = loop {
        if let PtyOutput::Cursor(cursor) = socket.next().await.unwrap().unwrap() {
            break cursor;
        }
    };
    assert_eq!(
        client
            .ptys()
            .connect_at(id, &options, location)
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        403
    );
    socket.send("fixture 🌻\n".to_owned()).await.unwrap();
    let mut output = String::new();
    while !output.contains("fixture 🌻") {
        if let PtyOutput::Data(text) = socket.next().await.unwrap().unwrap() {
            cursor += text.encode_utf16().count() as u64;
            output.push_str(&text);
        }
    }
    socket.close().await.unwrap();
    assert_eq!(
        client.ptys().get(id).await.unwrap().data.status,
        PtyStatus::Running
    );
    let mut resumed = client
        .ptys()
        .connect(
            id,
            &PtyConnectOptions::default()
                .cursor(cursor as i64)
                .idle_timeout(Duration::from_secs(5)),
        )
        .await
        .unwrap();
    loop {
        if let PtyOutput::Cursor(next) = resumed.next().await.unwrap().unwrap() {
            assert!(next >= cursor);
            break;
        }
    }
    resumed.disconnect();
}
