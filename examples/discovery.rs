//! Resolve a server location and list its agents without loading a UI.
//! Uses OPENCODE_BASE_URL, optional OPENCODE_PASSWORD, and optional OPENCODE_DIRECTORY.

use opencoders::{Client, Location};
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
    let base_url = optional_env("OPENCODE_BASE_URL")?
        .ok_or("set OPENCODE_BASE_URL to your OpenCode server URL")?;
    let mut builder = Client::builder(base_url);
    if let Some(password) = optional_env("OPENCODE_PASSWORD")? {
        builder = builder.basic_auth("opencode", password);
    }
    if let Some(directory) = optional_env("OPENCODE_DIRECTORY")? {
        builder = builder.default_location(Location::directory(directory));
    }
    let client = builder.build()?;
    let location = client.location().get().await?;
    println!(
        "Location: {} (project {})",
        location.directory, location.project.id
    );
    // Use the resolved directory so both calls select the same location even if
    // the server default changes between requests.
    let agents = client
        .agents()
        .list_at(Location::directory(location.directory.to_string()))
        .await?;
    println!("Agents at {}:", agents.location.directory);
    for agent in agents.data {
        println!(
            "- {}: {} (mode: {}, hidden: {})",
            agent.id, agent.name, agent.mode, agent.hidden
        );
    }
    Ok(())
}
