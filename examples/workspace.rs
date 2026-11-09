//! Read-only workspace discovery. Optional OPENCODE_FILE streams a bounded file.
use opencoders::{Client, FileListOptions, FileReadOptions, Location, VcsBranchOptions};
use std::{env, error::Error, time::Duration};

fn optional_env(name: &str) -> Result<Option<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(format!("{name} must be valid Unicode").into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut builder =
        Client::builder(optional_env("OPENCODE_BASE_URL")?.ok_or("set OPENCODE_BASE_URL")?);
    if let Some(password) = optional_env("OPENCODE_PASSWORD")? {
        builder = builder.basic_auth("opencode", password);
    }
    if let Some(directory) = optional_env("OPENCODE_DIRECTORY")? {
        builder = builder.default_location(Location::directory(directory));
    }
    let client = builder.build()?;
    let resolved = client.location().get().await?;
    let location = Location::directory(resolved.directory.as_str());
    for entry in client
        .files()
        .list_at(&FileListOptions::default(), location.clone())
        .await?
        .data
    {
        println!("{} {}", entry.type_, entry.path);
    }
    let vcs = client.vcs().get_at(location.clone()).await?;
    println!("Current branch: {:?}", vcs.data.branch.current);
    println!(
        "Branches: {:?}",
        client
            .vcs()
            .branches_at(&VcsBranchOptions::default().limit(10), location.clone())
            .await?
            .data
            .0
    );
    println!(
        "Worktrees: {}",
        client.worktrees().list(&resolved.project.id).await?.0.len()
    );
    if let Some(path) = optional_env("OPENCODE_FILE")? {
        let response = client
            .files()
            .read_at(
                &path,
                &FileReadOptions::default()
                    .max_bytes(1024 * 1024)
                    .idle_timeout(Duration::from_secs(10)),
                location,
            )
            .await?;
        println!("Read status: {}", response.status());
        let mut body = response.into_body();
        let mut bytes = 0;
        while let Some(chunk) = body.next().await {
            bytes += chunk?.len();
        }
        println!("Read {bytes} bytes");
    }
    Ok(())
}
