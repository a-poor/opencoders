//! Discovers an agent/models, creates a fresh session, and records selections.
//! Requires OPENCODE_BASE_URL, OPENCODE_DIRECTORY, OPENCODE_AGENT_ID; optional OPENCODE_PASSWORD.
//! Leaves the created session on the server and makes no model calls.

use opencoders::{
    Client, Location, ModelListOptions,
    models::{
        SessionCreateRequest, SessionLocationInput, SessionModelInput, SessionSwitchAgentRequest,
        SessionSwitchModelRequest,
    },
};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OPENCODE_BASE_URL")
        .map_err(|_| "set OPENCODE_BASE_URL to your OpenCode server URL")?;
    let directory = env::var("OPENCODE_DIRECTORY")
        .map_err(|_| "set OPENCODE_DIRECTORY to a directory on the server")?;
    let agent_id = env::var("OPENCODE_AGENT_ID")
        .map_err(|_| "set OPENCODE_AGENT_ID to the agent to select (for example, build)")?;
    let mut builder = Client::builder(base_url).default_location(Location::directory(directory));
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let agent = client.agents().get(&agent_id.into()).await?;
    let catalog = client.models().list(&ModelListOptions::default()).await?;
    println!("{} chat models in the current catalog", catalog.data.len());
    let default = client.models().default().await?;
    let Some(model) = default.data else {
        println!("No default model is available; no session was created.");
        return Ok(());
    };
    // Discovery defaults do not supply the creation location; set it explicitly.
    let created = client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .title("Configured with opencoders")
                .location(SessionLocationInput::new(
                    default.location.directory.to_string(),
                )),
        )
        .await?;
    let id = &created.data.id;
    println!("Created session {}", id.as_str());
    // Use public catalog IDs, not the provider-native model_id. Responses can be
    // lost after a selection is applied; inspect this session before retrying.
    client
        .sessions()
        .switch_agent(id, &SessionSwitchAgentRequest::new(agent.data.id))
        .await?;
    client
        .sessions()
        .switch_model(
            id,
            &SessionSwitchModelRequest::new(SessionModelInput::new(model.provider_id, model.id)),
        )
        .await?;
    let retrieved = client.sessions().get(id).await?;
    println!("Selections recorded for {}", retrieved.data.id.as_str());
    // Selection does not validate provider credentials or invoke the model.
    Ok(())
}
