//! Fork a fresh session's selection history, without model calls or a UI.
//! Requires OPENCODE_BASE_URL and OPENCODE_DIRECTORY; optional OPENCODE_PASSWORD.
use opencoders::{
    Client, MessageListOptions, MessageOrder,
    models::{
        SessionCreateRequest, SessionForkRequest, SessionId, SessionLocationInput,
        SessionSwitchAgentRequest,
    },
};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base = env::var("OPENCODE_BASE_URL").map_err(|_| "set OPENCODE_BASE_URL")?;
    let directory = env::var("OPENCODE_DIRECTORY").map_err(|_| "set OPENCODE_DIRECTORY")?;
    let mut builder = Client::builder(base);
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let source = client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .title("Fork example")
                .location(SessionLocationInput::new(directory)),
        )
        .await?
        .data;
    println!("Created source {}", source.id.as_str());
    let mut forks: Vec<SessionId> = vec![];
    let result = async {
        // Empty sources cannot be forked on v2.2.5. Recording an agent selection
        // creates a projected history entry without executing a model or a tool.
        client
            .sessions()
            .switch_agent(
                &source.id,
                &SessionSwitchAgentRequest::new("build".to_owned().into()),
            )
            .await?;
        let history = client
            .sessions()
            .messages(&source.id)
            .list(&MessageListOptions::default().order(MessageOrder::Asc))
            .await?;
        let boundary = history
            .data
            .first()
            .ok_or("agent selection returned no history entry")?
            .id()
            .clone();
        let full = client
            .sessions()
            .fork(&source.id, &SessionForkRequest::default())
            .await?
            .data;
        println!("Fork with copied history: {}", full.id.as_str());
        forks.push(full.id);
        let before = client
            .sessions()
            .fork(&source.id, &SessionForkRequest::default().before(boundary))
            .await?
            .data;
        println!("Fork before the first entry: {}", before.id.as_str());
        forks.push(before.id);
        // Full history and an explicit `before` boundary both create independent
        // sessions with fork provenance, not parent/child ownership.
        Ok::<(), Box<dyn Error>>(())
    }
    .await;
    // Attempt to clean up every known fixture even if an operation failed.
    // A lost fork response can leave an unknown new session; do not retry blindly.
    let mut cleanup_error = None;
    for id in forks.iter().chain(std::iter::once(&source.id)) {
        if let Err(error) = client.sessions().remove(id).await {
            cleanup_error.get_or_insert(error);
        }
    }
    result?;
    if let Some(error) = cleanup_error {
        return Err(error.into());
    }
    println!("Removed the example source and forks");
    Ok(())
}
