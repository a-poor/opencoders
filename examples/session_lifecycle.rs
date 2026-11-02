//! Creates one empty session, updates/archives/restores it, then deletes it.
//! Uses OPENCODE_BASE_URL, optional OPENCODE_PASSWORD, and required OPENCODE_DIRECTORY.
//! No existing session ID is accepted and no prompt is submitted.

use opencoders::{
    Client,
    models::{SessionCreateRequest, SessionLocationInput, SessionUpdateRequest},
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
    let created = client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .title("Lifecycle example")
                .location(SessionLocationInput::new(directory)),
        )
        .await?;
    let id = created.data.id;
    println!("Created session {}", id.as_str());
    // If a response is lost, inspect the session before deciding whether to retry.
    client
        .sessions()
        .update(
            &id,
            &SessionUpdateRequest::default()
                .title("Renamed example")
                .archived(true),
        )
        .await?;
    let archived = client.sessions().get(&id).await?;
    println!("Archived: {}", archived.data.time.archived.is_some());
    client
        .sessions()
        .update(&id, &SessionUpdateRequest::default().archived(false))
        .await?;
    // Removal also deletes descendants. This example removes only its newly
    // created session; on an earlier error it leaves the printed ID for inspection.
    client.sessions().remove(&id).await?;
    println!("Deleted session {}", id.as_str());
    Ok(())
}
