//! Wait for an existing session; pass --interrupt to request a stop first.
//! Requires OPENCODE_BASE_URL and OPENCODE_SESSION_ID; OPENCODE_PASSWORD optional.
use opencoders::{Client, SessionInterruptOptions, SessionWaitOptions, models::SessionId};
use std::{env, error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let interrupt = match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [] => false,
        [arg] if arg == "--interrupt" => true,
        _ => return Err("usage: cargo run --example execution -- [--interrupt]".into()),
    };
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
    if interrupt {
        let response = client
            .sessions()
            .interrupt(&id, &SessionInterruptOptions::default().resume(false))
            .await?;
        println!(
            "Interruption accepted for active execution: {}",
            response.interrupted
        );
        // Either boolean still requires waiting if cleanup/settlement matters.
    }
    let options = SessionWaitOptions::default().timeout(Duration::from_secs(30));
    match client.sessions().experimental_wait(&id, &options).await {
        Ok(()) => println!("This server process observed the session idle"),
        Err(error) if error.is_timeout() => {
            eprintln!("Stopped waiting after the deadline; execution may still be active");
            return Err(error.into());
        }
        Err(error) => return Err(error.into()),
    }
    // Idle is not a prompt receipt or a successful-execution result. Concurrent
    // work can begin again. Inspect snapshots and session.execution.* events.
    let session = client.sessions().get(&id).await?;
    let inbox = client.sessions().inbox(&id).list().await?;
    println!(
        "Latest recorded outcome: {:?}; pending inputs: {}",
        session.data.outcome,
        inbox.data.len()
    );
    // A UI can await in its own task/select!; dropping the future stops only local
    // waiting. Only the explicit interrupt request above requests a stop.
    Ok(())
}
