//! Inspect app-setup catalogs; print counts only, never configuration or credentials.
use opencoders::{Client, Location, ProjectListOptions};
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
    println!(
        "Projects: {}",
        client
            .projects()
            .list(&ProjectListOptions::default())
            .await?
            .len()
    );
    println!(
        "Configuration sources: {}",
        client.config().get().await?.len()
    );
    println!(
        "Shell candidates: {}",
        client.config().shells().await?.len()
    );
    let providers = client.providers().list().await?;
    println!("Available providers: {}", providers.data.len());
    if let Some(provider) = providers.data.first() {
        let _detail = client.providers().get(&provider.id).await?;
        println!("First provider details retrieved");
    }
    println!(
        "References: {}",
        client.references().list().await?.data.len()
    );
    Ok(())
}
