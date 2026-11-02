//! Explicit cursor pagination (at most three pages), without a UI.
//! Uses OPENCODE_BASE_URL, optional OPENCODE_TOKEN, and optional OPENCODE_DIRECTORY.

use opencoders::{Client, SessionListOptions, SessionOrder};
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
    if let Some(token) = optional_env("OPENCODE_TOKEN")? {
        builder = builder.bearer_auth(token);
    }
    let client = builder.build()?;
    let mut options = SessionListOptions::default()
        .limit(10)
        .order(SessionOrder::Desc);
    if let Some(directory) = optional_env("OPENCODE_DIRECTORY")? {
        options = options.directory(directory);
    }

    for page_number in 1..=3 {
        let page = client.sessions().list(&options).await?;
        println!(
            "Page {page_number} (previous page available: {})",
            page.cursor.previous.is_some()
        );
        for session in page.data {
            println!(
                "{}: {} [{}]",
                session.id.as_str(),
                session.title.as_deref().unwrap_or("(untitled)"),
                session.location.directory
            );
        }
        let Some(next) = page.cursor.next else { break };
        // Retain filters, omit the first-page order, and pass the opaque cursor back.
        // To go backwards, use page.cursor.previous in exactly the same way.
        options.cursor = Some(next);
        options.order = None;
    }
    Ok(())
}
