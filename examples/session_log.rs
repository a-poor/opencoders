//! Read-only, bounded example of explicit log resumption and snapshot refresh.
//! Requires OPENCODE_BASE_URL and OPENCODE_SESSION_ID; OPENCODE_PASSWORD optional.
//! v2.2.5 CLI logs lack payload retention, so always refresh authoritative state.
use opencoders::{
    Client, SessionLogOptions,
    models::{SessionId, SessionLogItem},
};
use std::{env, error::Error, time::Duration};

async fn refresh(client: &Client, id: &SessionId) -> Result<(), Box<dyn Error>> {
    let snapshot = client.sessions().get(id).await?;
    println!("Refreshed session {}", snapshot.data.id.as_str());
    // An application would also refresh its message/inbox/permission snapshots.
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base = env::var("OPENCODE_BASE_URL").map_err(|_| "set OPENCODE_BASE_URL")?;
    let id: SessionId = env::var("OPENCODE_SESSION_ID")
        .map_err(|_| "set OPENCODE_SESSION_ID")?
        .parse()?;
    let mut builder = Client::builder(base);
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let mut checkpoint: Option<u64> = None;
    for pass in 0..2 {
        let mut options = SessionLogOptions::default().follow(pass == 1);
        options.after = checkpoint;
        let mut stream = client.sessions().experimental_log(&id, &options).await?;
        refresh(&client, &id).await?;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
        let mut synced = false;
        let mut received = 0;
        loop {
            let message = match tokio::time::timeout_at(deadline, stream.next()).await {
                Ok(Some(Ok(message))) => message,
                Ok(Some(Err(error))) => {
                    eprintln!("Observation failed: {error}");
                    break;
                }
                Ok(None) => {
                    eprintln!("Log closed; boundary reached: {synced}");
                    break;
                }
                Err(_) => {
                    eprintln!("Finished the bounded observation window");
                    break;
                }
            };
            if message.item.aggregate_id() != Some(id.as_str()) {
                // Unknown kinds have no inferred aggregate/cursor. Do not acknowledge
                // a later watermark without a deliberate policy for unknown data.
                return Err(
                    "unhandled log kind or unexpected aggregate; checkpoint unchanged".into(),
                );
            }
            match &message.item {
                SessionLogItem::Event(event) => {
                    println!("Observed {}", event.kind());
                    refresh(&client, &id).await?;
                    received += 1;
                }
                SessionLogItem::Synced(_) => {
                    refresh(&client, &id).await?;
                    synced = true;
                    println!("Captured replay boundary reached");
                }
                _ => return Err("unhandled log item; checkpoint unchanged".into()),
            }
            // Application-owned acknowledgement after successful handling. Never
            // use SSE IDs, regress a future cursor, or convert absent seq to zero.
            if let Some(seq) = message.item.sequence() {
                checkpoint = Some(checkpoint.map_or(seq, |old| old.max(seq)));
            }
            if (pass == 0 && synced) || received >= 10 {
                break;
            }
        }
        stream.close();
        refresh(&client, &id).await?;
        if pass == 0 {
            println!("Reopening explicitly after acknowledged sequence {checkpoint:?}");
        }
    }
    // Production clients decide retention assumptions, deduplication, retries,
    // durable checkpoint storage, and reconciliation of overlapping snapshots.
    Ok(())
}
