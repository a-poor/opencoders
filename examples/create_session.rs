//! Creates one empty session and retrieves it; it never submits a prompt.
//! Uses OPENCODE_BASE_URL, optional OPENCODE_PASSWORD, and required OPENCODE_DIRECTORY.

use opencoders::{
    Client,
    models::{SessionCreateRequest, SessionLocationInput},
};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OPENCODE_BASE_URL")
        .map_err(|_| "set OPENCODE_BASE_URL to your OpenCode server URL")?;
    let directory = env::var("OPENCODE_DIRECTORY")
        .map_err(|_| "set OPENCODE_DIRECTORY to the creation directory on the server")?;
    let mut builder = Client::builder(base_url);
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let request = SessionCreateRequest::default()
        .title("Created with opencoders")
        .location(SessionLocationInput::new(directory));
    // On timeout/disconnect, creation might already have succeeded. Inspect server
    // state before deciding whether to issue another create request.
    let created = client.sessions().create(&request).await?;
    println!("Created session {}", created.data.id.as_str());
    let retrieved = client.sessions().get(&created.data.id).await?;
    println!(
        "{} at {}",
        retrieved.data.title.as_deref().unwrap_or("(untitled)"),
        retrieved.data.location.directory
    );
    // The session remains on the server after the example exits.
    Ok(())
}
