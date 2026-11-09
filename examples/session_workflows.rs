//! Read command/skill catalogs, active context and diffs; never submit execution.
use opencoders::{Client, Location, SessionDiffOptions, models::SessionId};
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
    let id: SessionId = env::var("OPENCODE_SESSION_ID")?.parse()?;
    let client = builder.build()?;
    println!("Commands: {}", client.commands().list().await?.data.len());
    println!("Skills: {}", client.skills().list().await?.data.len());
    println!(
        "Context messages: {}",
        client.sessions().context(&id).await?.data.len()
    );
    println!(
        "Changed files in latest turn: {}",
        client
            .sessions()
            .diff(&id, &SessionDiffOptions::default().context(3))
            .await?
            .data
            .len()
    );
    Ok(())
}
