//! Explicitly invoked automation: this can call models/tools and modify the
//! server workspace. See README for required environment and retained-session cleanup.
#[path = "support/automation.rs"]
mod automation;

use automation::{Job, Limits, Notice, Outcome, run};
use opencoders::{Client, models::SessionModelInput};
use std::{
    env,
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn required(name: &str) -> Result<String, Box<dyn Error>> {
    env::var(name).map_err(|_| format!("set {name} to a valid Unicode value").into())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if env::args().skip(1).collect::<Vec<_>>() != ["--execute"] {
        return Err(
            "usage: cargo run --example automation -- --execute (calls the configured model/tools)"
                .into(),
        );
    }
    let base = required("OPENCODE_BASE_URL")?;
    // Caller-chosen IDs are printed before mutations for investigation after a
    // lost response. They are not idempotency keys or a promise that retry is safe.
    let nonce = format!(
        "{}_{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos(),
        std::process::id()
    );
    let job = Job {
        session_id: format!("ses_automation_{nonce}").parse()?,
        prompt_id: format!("msg_automation_{nonce}").parse()?,
        directory: required("OPENCODE_DIRECTORY")?,
        agent: required("OPENCODE_AGENT_ID")?.into(),
        model: SessionModelInput::new(
            required("OPENCODE_PROVIDER_ID")?.into(),
            required("OPENCODE_MODEL_ID")?,
        ),
        text: required("OPENCODE_PROMPT")?,
    };
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
    println!(
        "Session ID: {}; prompt ID: {}",
        job.session_id.as_str(),
        job.prompt_id.as_str()
    );
    println!(
        "This session is retained on every outcome. A local deadline does not stop server work."
    );
    let report = run(&client, &job, Limits::default(), |notice| match notice {
        Notice::Created => println!("Created the example session with the requested agent/model."),
        Notice::Admitted => println!("Prompt admitted; no completion claim yet."),
        Notice::Waiting => println!("Waiting for process-local idleness while observing progress."),
        Notice::TextFragment(chars) => println!("Observed {chars} transient text characters."),
        Notice::Execution(outcome) => {
            println!("Session execution event: {outcome:?} (not a prompt receipt).")
        }
        Notice::UnknownEvent => println!("Unknown event observed; retaining no payload."),
    })
    .await?;
    println!(
        "Observed outcome: {:?}; wait: {:?}; feed: {:?}",
        report.outcome, report.wait, report.trace.feed
    );
    println!(
        "Events: {}; text fragments: {}; unknown events: {}",
        report.trace.events_seen, report.trace.text_fragments, report.trace.unknown_events
    );
    if let Some(error) = &report.wait_error {
        eprintln!("Wait diagnostic: {error}");
    }
    if let Some(error) = &report.inspection_error {
        eprintln!("Inspection diagnostic: {error}");
    }
    if let Some(error) = &report.trace.stream_error {
        eprintln!("Feed diagnostic: {error}");
    }
    if let Some(view) = &report.inspection {
        println!(
            "History entries: {}; complete traversal: {}; pending inbox: {}; permissions: {}; forms: {}",
            view.history.len(),
            view.history_complete,
            view.inbox.data.len(),
            view.permissions.data.len(),
            view.forms.data.len()
        );
        if report.outcome == Outcome::Succeeded {
            // Only stored output passes this gate. No live text fragments are
            // appended: they can overlap history or be missing after disconnect.
            for text in view.answer_text() {
                println!("{text}");
            }
        }
    }
    if report.outcome != Outcome::Succeeded {
        return Err(
            "no observed successful completion; inspect the retained session before further actions"
                .into(),
        );
    }
    Ok(())
}
