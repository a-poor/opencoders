//! Admit input without waking execution, inspect it, change delivery, and cancel it.
//! Requires OPENCODE_BASE_URL and OPENCODE_DIRECTORY; optional OPENCODE_PASSWORD.
//! Creates its own session and removes it after completing the demonstration.

use opencoders::{
    Client,
    models::{
        SessionCreateRequest, SessionInboxDelivery, SessionInboxUpdateRequest,
        SessionLocationInput, SessionPromptRequest,
    },
};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OPENCODE_BASE_URL")
        .map_err(|_| "set OPENCODE_BASE_URL to your OpenCode server URL")?;
    let directory = env::var("OPENCODE_DIRECTORY")
        .map_err(|_| "set OPENCODE_DIRECTORY to a directory on the server")?;
    let mut builder = Client::builder(base_url);
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let session = client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .title("Prompt/inbox example")
                .location(SessionLocationInput::new(directory)),
        )
        .await?
        .data;
    println!("Created session {}", session.id.as_str());
    // Explicit false suppresses waking the loop. Queue delivery by itself does
    // not suppress waking, and false does not pause a loop that is already running.
    let admitted = client
        .sessions()
        .prompt(
            &session.id,
            &SessionPromptRequest::new("Example pending input")
                .delivery(SessionInboxDelivery::Steer)
                .resume(false),
        )
        .await?
        .data;
    println!("Admitted pending input {}", admitted.id.as_str());
    let inbox = client.sessions().inbox(&session.id);
    println!("{} pending items", inbox.list().await?.data.len());
    // Queue changes delivery without waking execution. On v2.2.5 setting the
    // already recorded mode returns a conflict; do not blindly retry mutations.
    inbox
        .update(
            &admitted.id,
            &SessionInboxUpdateRequest::new(SessionInboxDelivery::Queue),
        )
        .await?;
    inbox.cancel(&admitted.id).await?;
    client.sessions().remove(&session.id).await?;
    println!("Cancelled the pending input and removed the example session");
    // If an earlier request fails, inspect the printed session ID before retrying.
    Ok(())
}
