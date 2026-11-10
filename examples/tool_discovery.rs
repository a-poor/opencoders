//! Inspect location-aware integration/tool catalogs; print counts, not payloads.
use opencoders::{Client, Location};
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
    // On v2.2.5 this waits for plugin activation; MCP connections can still be pending.
    let integrations = client.integrations().list().await?;
    println!("Integrations: {}", integrations.data.len());
    if let Some(integration) = integrations.data.first() {
        let detail = client.integrations().get(&integration.id).await?;
        println!("First integration's methods: {}", detail.data.methods.len());
    }
    println!(
        "Configured MCP servers: {}",
        client.mcp().list().await?.data.len()
    );
    // Catalog retrieval contacts connected MCP servers; it does not read contents.
    let catalog = client.mcp().resource_catalog().await?;
    println!(
        "Resources: {}, templates: {}",
        catalog.data.resources.len(),
        catalog.data.templates.len()
    );
    println!("Plugins: {}", client.plugins().list().await?.data.len());
    println!(
        "Search providers: {}",
        client.websearch().providers().await?.data.len()
    );
    Ok(())
}
