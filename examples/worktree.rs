//! Create/refresh/remove a worktree for an existing committed project.
//! Run with OPENCODE_BASE_URL, OPENCODE_DIRECTORY and --execute. Project setup
//! scripts can run. Cleanup uses force=false and reports any retained checkout.
use opencoders::{
    Client, Location,
    models::{WorktreeCreateRequest, WorktreeRemoveRequest},
};
use std::{env, error::Error};

fn optional_env(name: &str) -> Result<Option<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(format!("{name} must be valid Unicode").into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if !env::args().any(|arg| arg == "--execute") {
        return Err(
            "pass --execute to create/remove a checkout and allow project setup scripts".into(),
        );
    }
    let base = optional_env("OPENCODE_BASE_URL")?.ok_or("set OPENCODE_BASE_URL")?;
    let directory = optional_env("OPENCODE_DIRECTORY")?
        .ok_or("set OPENCODE_DIRECTORY to an existing committed project")?;
    let mut builder = Client::builder(base).default_location(Location::directory(directory));
    if let Some(password) = optional_env("OPENCODE_PASSWORD")? {
        builder = builder.basic_auth("opencode", password);
    }
    let client = builder.build()?;
    let project = client.location().get().await?.project.id;
    let mut request = WorktreeCreateRequest::new(project.clone());
    if let Some(parent) = optional_env("OPENCODE_WORKTREE_PARENT")? {
        request = request.directory(parent.into());
    }
    let created = client.worktrees().create(&request).await?;
    println!("Created {} (project {})", created.directory, project);
    let inspect = async {
        client.worktrees().refresh(&project).await?;
        let inventory = client.worktrees().list(&project).await?;
        println!("Saved worktrees: {}", inventory.0.len());
        Ok::<(), opencoders::Error>(())
    }
    .await;
    let cleanup = client
        .worktrees()
        .remove(&WorktreeRemoveRequest::new(
            project,
            created.directory.clone(),
            false,
        ))
        .await;
    if let Err(error) = &cleanup {
        eprintln!("Inspect retained checkout {}: {error}", created.directory);
    }
    inspect?;
    cleanup?;
    Ok(())
}
