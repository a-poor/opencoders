//! Read at most three history pages, then retrieve one message by ID. No writes.
//! Requires OPENCODE_BASE_URL and OPENCODE_SESSION_ID; OPENCODE_PASSWORD is optional.

use opencoders::{
    Client, MessageListOptions, MessageOrder,
    models::{SessionId, SessionMessageAssistantContentItem, SessionMessageInfo},
};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OPENCODE_BASE_URL")
        .map_err(|_| "set OPENCODE_BASE_URL to your OpenCode server URL")?;
    let id: SessionId = env::var("OPENCODE_SESSION_ID")
        .map_err(|_| "set OPENCODE_SESSION_ID to the session to read")?
        .parse()?;
    let mut builder = Client::builder(base_url);
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let messages = client.sessions().messages(&id);
    let mut options = MessageListOptions::default()
        .limit(10)
        .order(MessageOrder::Desc);
    let mut first_id = None;
    for page_number in 1..=3 {
        let page = messages.list(&options).await?;
        println!("Page {page_number} ({} messages)", page.data.len());
        for message in &page.data {
            if first_id.is_none() {
                first_id = Some(message.id().clone());
            }
            println!("{} [{}]", message.id().as_str(), message.message_type());
            match message {
                SessionMessageInfo::User(user) => println!("{}", user.text),
                SessionMessageInfo::Assistant(assistant) => {
                    for part in &assistant.content {
                        if let SessionMessageAssistantContentItem::Text(text) = part {
                            println!("{}", text.text);
                        }
                    }
                }
                _ => {} // Applications choose how to display other timeline entries.
            }
        }
        let Some(next) = page.cursor.next else { break };
        options.cursor = Some(next);
        options.order = None; // Never combine a cursor with order; retain any type filter.
    }
    if let Some(id) = first_id {
        let retrieved = messages.get(&id).await?;
        println!(
            "Retrieved {} [{}]",
            retrieved.data.id().as_str(),
            retrieved.data.message_type()
        );
    }
    Ok(())
}
