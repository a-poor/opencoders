//! Read-only background observation with application-owned state and recovery.
//! Set OPENCODE_BASE_URL and OPENCODE_SESSION_ID; OPENCODE_PASSWORD is optional.
//! Runs for at most 30 seconds with at most three explicit connection attempts.
#[path = "support/session_observer.rs"]
mod observer;

use observer::{Connection, Failure, State};
use opencoders::{Client, models::SessionId};
use std::{env, error::Error, time::Duration};
use tokio::time::{Instant, MissedTickBehavior, interval, sleep, timeout_at};

fn report(state: &State, buffered: usize) {
    // Avoid printing conversation/form/permission payloads. A consuming UI can
    // inspect this application-owned state and choose its own presentation.
    println!(
        "{:?}; refresh needed={}; queued={buffered}; unknown events={}",
        state.phase, state.needs_refresh, state.unknown_events
    );
    if let Some(snapshot) = &state.snapshot {
        println!(
            "  session={}; recent messages={}; inbox={}; permissions={}; forms={}",
            snapshot.session.id.as_str(),
            snapshot.history.data.len(),
            snapshot.inbox.data.len(),
            snapshot.permissions.data.len(),
            snapshot.forms.data.len()
        );
        // The opaque cursor is retained for caller-controlled pagination. This
        // example loads only one recent page and does not claim complete history.
    }
    if let Some(preview) = &state.preview {
        println!(
            "  transient fragment for {}: {} characters; truncated={}",
            preview.message_id.as_str(),
            preview.text.chars().count(),
            preview.truncated
        );
    }
}

async fn consume(connection: &mut Connection, state: &mut State) -> Result<(), Failure> {
    connection.refresh(state).await?;
    report(state, connection.buffered_events());
    let mut refresh = interval(Duration::from_secs(1));
    refresh.set_missed_tick_behavior(MissedTickBehavior::Skip);
    refresh.tick().await; // Consume the immediate tick after the initial snapshot.
    loop {
        tokio::select! {
            // A ready refresh must not starve behind a continuously ready feed.
            biased;
            _ = refresh.tick() => {
                println!("Refreshing snapshots; the previous view may be stale.");
                connection.refresh(state).await?;
            }
            result = connection.next(state) => { result?; }
        }
        report(state, connection.buffered_events());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base = env::var("OPENCODE_BASE_URL").map_err(|_| "set OPENCODE_BASE_URL")?;
    let id: SessionId = env::var("OPENCODE_SESSION_ID")
        .map_err(|_| "set OPENCODE_SESSION_ID")?
        .parse()?;
    let mut builder = Client::builder(base)
        .request_timeout(Duration::from_secs(10))
        .max_response_bytes(1024 * 1024);
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let mut state = State::default();
    let deadline = Instant::now() + Duration::from_secs(30);
    for attempt in 1..=3 {
        state.connecting(); // Retains the old snapshot, explicitly marked incomplete.
        println!("Connecting, attempt {attempt}/3; snapshots require refresh.");
        match timeout_at(deadline, Connection::open(&client, &id, 32)).await {
            Ok(Ok(mut connection)) => {
                match timeout_at(deadline, consume(&mut connection, &mut state)).await {
                    Ok(Err(error)) => {
                        eprintln!("Observation ended: {error}");
                        report(&state, connection.buffered_events());
                    }
                    Err(_) => println!("Observation window ended."),
                    Ok(Ok(())) => {}
                }
                // Cancelling consume stops local awaits; close explicitly aborts
                // and joins the reader, including when refresh was in progress.
                connection.close().await;
            }
            Ok(Err(error)) => eprintln!("Connection failed; snapshots remain stale: {error}"),
            Err(_) => break,
        }
        if attempt == 3
            || timeout_at(deadline, sleep(Duration::from_millis(500)))
                .await
                .is_err()
        {
            break;
        }
        // Application policy, not SDK retry: only read-only requests are repeated.
        // Subscribe first, then refresh all snapshots on every new connection.
    }
    state.stopped();
    report(&state, 0);
    if state.snapshot.is_none() {
        return Err("observation ended without a complete snapshot".into());
    }
    if Instant::now() < deadline {
        return Err("connection attempts exhausted; last snapshots may be stale".into());
    }
    Ok(())
}
