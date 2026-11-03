# Workspace and model discovery

[Documentation index](README.md)

## Location-aware discovery

```rust
use opencoders::{Client, Location};

async fn discover() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("http://localhost:4096")
        .default_location(Location::directory("/remote/workspace"))
        .build()?;
    let location = client.location().get().await?;
    let agents = client.agents().list().await?;
    println!("{} agents at {}", agents.data.len(), agents.location.directory);

    // Override only this call; clones retain the configured default.
    let other = client.agents().list_at(Location::directory("/remote/other")).await?;
    let fallback = client.location().get_at(Location::Unspecified).await?;
    Ok(())
}
```

`Location::Unspecified` omits the location query even when the client has a
configured default. Server fallback rules still apply, including any location
headers supplied by the application or injected HTTP client. It does not send
JSON null, an empty string, or the literal string `"null"`.

Directory values use the nested `location[directory]` query on supported routes
only. They refer to the server's filesystem; the SDK neither reads local paths
nor canonicalizes them. Server-wide calls such as `server.info()` ignore the
configured directory. Agent lists retain their `location`/`data` envelope and
include hidden agents; applications choose what to display.

Run `cargo run --example discovery` with `OPENCODE_BASE_URL`, optional
`OPENCODE_PASSWORD`, and optional `OPENCODE_DIRECTORY`. The example resolves a
location, then lists agents at that resolved directory.

## Model and agent selection

```rust
use opencoders::{Client, ModelListOptions,
    models::{SessionId, SessionModelInput, SessionSwitchAgentRequest, SessionSwitchModelRequest}};

async fn configure(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    let catalog = client.models().list(&ModelListOptions::default().types(["chat"])).await?;
    let agent = client.agents().get(&"build".to_owned().into()).await?;
    if let Some(model) = client.models().default().await?.data {
        client.sessions().switch_agent(id, &SessionSwitchAgentRequest::new(agent.data.id)).await?;
        client.sessions().switch_model(id, &SessionSwitchModelRequest::new(
            SessionModelInput::new(model.provider_id, model.id),
        )).await?;
    }
    Ok(())
}
```

`models().list`, `models().default`, and `agents().get` retain `location`/`data`
envelopes and support per-call `list_at`, `default_at`, and `get_at` overrides.
Default-model `data` is required but nullable: `None` means no default is available.
Lists preserve server order and expose a snapshot, without waiting for catalog
changes. Empty `ModelListOptions::types` omits the filter (chat on v2.2.5); nonempty
lists send repeated `types` parameters. Types are open strings, including plugin
types. The SDK preserves duplicates/order; the tested server deduplicates requested
types and concatenates matching groups. Agent IDs are encoded as one path segment;
empty IDs and `.`/`..` return a pre-send encoding error.

Selection requests use strict, required, non-null agent/model fields. Use the
catalog's public `id` and `provider_id`, not its provider-native `model_id`.
`SessionModelInput::variant` selects an optional variant. Both switch methods
return `()` for HTTP 204 and ignore the discovery location default. They record
choices for subsequent turns without executing or validating their availability:
v2.2.5 accepts even unknown agent/provider/model IDs. There is no null reset operation.

On tested v2.2.5, selecting the same agent again adds another history entry.
An omitted model variant is stored as `"default"`; repeating the same provider,
model, and variant creates no additional model-selection entry. The SDK performs
no local deduplication. A response can be lost after a selection is applied; the
default transport does not retry either operation.

Run `cargo run --example configure_session` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, `OPENCODE_AGENT_ID` (for example, `build`), and optional
`OPENCODE_PASSWORD`. It discovers the selected agent and default model, creates
a fresh session, records both choices, retrieves it, and leaves it on the server.
If no default model exists, it exits before creating a session. It makes no model calls.
