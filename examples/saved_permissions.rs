//! Read saved rules for a concrete discovered project; do not change approvals.
use opencoders::{Client, Location, SavedPermissionListOptions};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = Client::builder(env::var("OPENCODE_BASE_URL")?);
    if let Ok(password) = env::var("OPENCODE_PASSWORD") {
        builder = builder.basic_auth("opencode", password);
    }
    if let Ok(directory) = env::var("OPENCODE_DIRECTORY") {
        builder = builder.default_location(Location::directory(directory));
    }
    let client = builder.build()?;
    let project = client.location().get().await?.project.id;
    let rules = client
        .permissions()
        .saved()
        .list(&SavedPermissionListOptions::default().project(project))
        .await?;
    for rule in rules.data {
        println!("{}: allow {} on {}", rule.id, rule.action, rule.resource);
    }
    Ok(())
}
