//! Creates /bin/cat, exchanges one line, detaches and removes it. Unix server only.
//! Run: cargo run --features websocket --example pty -- --execute
use opencoders::{
    Client, Location, PtyConnectOptions, PtyOutput,
    models::{PtyCreateRequest, PtyUpdateRequest},
};
use std::{env, error::Error, time::Duration};

fn optional_env(name: &str) -> Result<Option<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(format!("{name} must be valid Unicode").into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if !env::args().any(|arg| arg == "--execute") {
        return Err("pass --execute to start and remove a PTY process".into());
    }
    let mut builder =
        Client::builder(optional_env("OPENCODE_BASE_URL")?.ok_or("set OPENCODE_BASE_URL")?);
    if let Some(password) = optional_env("OPENCODE_PASSWORD")? {
        builder = builder.basic_auth("opencode", password);
    }
    if let Some(directory) = optional_env("OPENCODE_DIRECTORY")? {
        builder = builder.default_location(Location::directory(directory));
    }
    let client = builder.build()?;
    let created = client
        .ptys()
        .create(
            &PtyCreateRequest::new()
                .command("/bin/cat")
                .title("opencoders example"),
        )
        .await?;
    let id = created.data.id;
    // Keep cleanup outside the exchange future so errors/timeouts still remove
    // this process. Process aborts need caller-managed cleanup by the printed ID.
    println!("Created PTY {}", id.as_str());
    let location = Location::directory(created.location.directory.as_str());
    let exchange = tokio::time::timeout(Duration::from_secs(15), async {
        client
            .ptys()
            .update_at(&id, &PtyUpdateRequest::new().size(24, 80), location.clone())
            .await?;
        let ticket = client
            .ptys()
            .connect_token_at(&id, location.clone())
            .await?;
        let mut socket = client
            .ptys()
            .connect_at(
                &id,
                &PtyConnectOptions::default()
                    .ticket(ticket.data.ticket)
                    .cursor(-1)
                    .idle_timeout(Duration::from_secs(5)),
                location.clone(),
            )
            .await?;
        socket.send("opencoders example\n").await?;
        let mut observed = String::new();
        while let Some(message) = socket.next().await {
            match message? {
                PtyOutput::Data(text) => {
                    observed.push_str(&text);
                    if observed.contains("opencoders example") {
                        break;
                    }
                    if observed.len() > 4096 {
                        return Err("unexpected terminal output".into());
                    }
                }
                PtyOutput::Closed { .. } => return Err("PTY closed before echo".into()),
                _ => {}
            }
        }
        if !observed.contains("opencoders example") {
            return Err("PTY ended before echo".into());
        }
        socket.close().await?;
        println!("Received echoed input; detached from PTY");
        Ok::<(), Box<dyn Error>>(())
    })
    .await;
    let cleanup = client.ptys().remove_at(&id, location).await;
    if let Err(error) = &cleanup {
        eprintln!("Cleanup failed for PTY {}: {error}", id.as_str());
    }
    exchange??;
    cleanup?;
    Ok(())
}
