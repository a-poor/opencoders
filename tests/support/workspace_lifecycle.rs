//! Disposable fixture only: the parent verifies server ownership first.
use opencoders::{ApiError, Client, Error, Location, VcsInitOptions, models::*};
use std::{path::Path, process::Command};

fn git(repo: &Path, args: &[&str]) -> String {
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
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "fixture git failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

pub async fn verify(client: &Client, root: &Path) {
    let repo = root.join("lifecycle-repo");
    std::fs::create_dir(&repo).unwrap();
    let location = Location::directory(repo.to_str().unwrap());
    let error = client
        .vcs()
        .init_at(
            &VcsInitOptions::default().provider("missing-provider"),
            location.clone(),
        )
        .await
        .unwrap_err();
    assert_eq!(error.status().unwrap(), 400);
    assert!(!repo.join(".git").exists());
    client
        .vcs()
        .init_at(&VcsInitOptions::default(), location.clone())
        .await
        .unwrap();
    assert!(repo.join(".git").is_dir());
    git(&repo, &["symbolic-ref", "HEAD", "refs/heads/main"]);
    std::fs::write(repo.join("tracked.txt"), "fixture\n").unwrap();
    git(&repo, &["add", "tracked.txt"]);
    git(&repo, &["commit", "-m", "Fixture"]);
    let project = client
        .location()
        .get_at(location.clone())
        .await
        .unwrap()
        .project
        .id;
    let Error::Http(error) = client
        .vcs()
        .init_at(&VcsInitOptions::default(), location)
        .await
        .unwrap_err()
    else {
        panic!("expected conflict");
    };
    assert_eq!(error.status(), 409);
    assert!(matches!(error.api_error(), Some(ApiError::Conflict { .. })));

    let parent = root.join("managed-checkouts + 雪");
    let request = WorktreeCreateRequest::new(project.clone())
        .from(repo.to_str().unwrap().to_owned().into())
        .branch("HEAD")
        .directory(parent.to_str().unwrap().to_owned().into())
        .name("review");
    let created = client.worktrees().create(&request).await.unwrap();
    let path = Path::new(created.directory.as_str());
    assert!(path.starts_with(&parent));
    assert_eq!(path, parent.join("review"));
    assert_eq!(
        std::fs::read(path.join("tracked.txt")).unwrap(),
        b"fixture\n"
    );
    assert_eq!(
        git(path, &["rev-parse", "--abbrev-ref", "HEAD"]).trim(),
        "HEAD"
    );
    assert!(
        client
            .worktrees()
            .list(&project)
            .await
            .unwrap()
            .0
            .iter()
            .any(|item| item.directory == created.directory
                && item.strategy.as_deref() == Some("git"))
    );

    // The server chooses a distinct path when the requested name exists.
    let second = client.worktrees().create(&request).await.unwrap();
    assert_eq!(
        Path::new(second.directory.as_str()),
        parent.join("review-2")
    );
    client
        .worktrees()
        .remove(&WorktreeRemoveRequest::new(
            project.clone(),
            second.directory.clone(),
            false,
        ))
        .await
        .unwrap();
    assert!(!Path::new(second.directory.as_str()).exists());

    std::fs::write(path.join("untracked.txt"), "owned fixture data\n").unwrap();
    let Error::Http(error) = client
        .worktrees()
        .remove(&WorktreeRemoveRequest::new(
            project.clone(),
            created.directory.clone(),
            false,
        ))
        .await
        .unwrap_err()
    else {
        panic!("expected force-required error");
    };
    assert_eq!(error.status(), 400);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::Worktree {
            force_required: Some(true),
            ..
        })
    ));
    assert!(path.join("untracked.txt").exists());
    // Only the known, freshly created fixture is forcibly removed.
    client
        .worktrees()
        .remove(&WorktreeRemoveRequest::new(
            project.clone(),
            created.directory.clone(),
            true,
        ))
        .await
        .unwrap();
    assert!(!path.exists());
    assert!(
        !client
            .worktrees()
            .list(&project)
            .await
            .unwrap()
            .0
            .iter()
            .any(|item| item.directory == created.directory)
    );

    let external = root.join("external-fixture-checkout");
    git(
        &repo,
        &[
            "worktree",
            "add",
            "--detach",
            external.to_str().unwrap(),
            "HEAD",
        ],
    );
    assert!(
        !client
            .worktrees()
            .list(&project)
            .await
            .unwrap()
            .0
            .iter()
            .any(|item| Path::new(item.directory.as_str()) == external)
    );
    client.worktrees().refresh(&project).await.unwrap();
    assert!(
        client
            .worktrees()
            .list(&project)
            .await
            .unwrap()
            .0
            .iter()
            .any(|item| Path::new(item.directory.as_str()) == external
                && item.strategy.as_deref() == Some("git"))
    );
    git(&repo, &["worktree", "remove", external.to_str().unwrap()]);
    client.worktrees().refresh(&project).await.unwrap();
    assert!(
        !client
            .worktrees()
            .list(&project)
            .await
            .unwrap()
            .0
            .iter()
            .any(|item| Path::new(item.directory.as_str()) == external)
    );

    let error = client
        .worktrees()
        .remove(&WorktreeRemoveRequest::new(
            project.clone(),
            repo.to_str().unwrap().to_owned().into(),
            false,
        ))
        .await
        .unwrap_err();
    assert_eq!(error.status().unwrap(), 400);
    assert!(repo.join("tracked.txt").exists());
    let missing: ProjectId = "missing-project".to_owned().into();
    let Error::Http(error) = client.worktrees().refresh(&missing).await.unwrap_err() else {
        panic!("expected missing project");
    };
    assert_eq!(error.status(), 404);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::ProjectNotFound { .. })
    ));
}
