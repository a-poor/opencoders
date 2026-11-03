//! Read-only global live feed. Stop with Ctrl-C; no reconnection is attempted.
//! OPENCODE_BASE_URL is required; OPENCODE_PASSWORD and OPENCODE_EVENT_TYPE optional.
use opencoders::{Client, EventOptions};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base = env::var("OPENCODE_BASE_URL")
        .map_err(|_| "set OPENCODE_BASE_URL to your OpenCode server URL")?;
    let mut builder = Client::builder(base);
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let mut options = EventOptions::default();
    match env::var("OPENCODE_EVENT_TYPE") {
        Ok(kind) => options = options.types([kind]),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_EVENT_TYPE must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let mut events = client.events().subscribe(&options).await?;
    while let Some(message) = events.next().await {
        // Payloads can contain conversation text and private server data.
        println!("{}", message?.event.kind());
    }
    eprintln!("Live feed closed; refresh authoritative state before reopening.");
    Ok(())
}
