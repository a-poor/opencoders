//! Demonstrate a manual permission request in a fresh session, with no model calls.
//! Requires OPENCODE_BASE_URL and OPENCODE_DIRECTORY; OPENCODE_PASSWORD optional.
use opencoders::{
    Client, Location,
    models::{
        PermissionEffect, PermissionReply, PermissionRuleInput, SessionCreateRequest,
        SessionLocationInput, SessionPermissionCreateRequest, SessionPermissionReplyRequest,
    },
};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base = env::var("OPENCODE_BASE_URL").map_err(|_| "set OPENCODE_BASE_URL")?;
    let directory = env::var("OPENCODE_DIRECTORY").map_err(|_| "set OPENCODE_DIRECTORY")?;
    let mut builder =
        Client::builder(base).default_location(Location::directory(directory.clone()));
    match env::var("OPENCODE_PASSWORD") {
        Ok(password) => builder = builder.basic_auth("opencode", password),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_PASSWORD must be valid Unicode".into());
        }
    }
    let client = builder.build()?;
    let session = client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .title("Permission API example")
                .location(SessionLocationInput::new(directory))
                .permissions(vec![PermissionRuleInput::new(
                    "opencoders.example.inspect",
                    PermissionEffect::Ask,
                    "*",
                )]),
        )
        .await?
        .data;
    println!("Created example session {}", session.id.as_str());
    let permissions = client.sessions().permissions(&session.id);
    let result = permissions
        .create(&SessionPermissionCreateRequest::new(
            "opencoders.example.inspect",
            vec!["example-resource".into()],
        ))
        .await?
        .data;
    println!("Permission evaluation: {:?}", result.effect);
    if result.effect == PermissionEffect::Ask {
        let pending = permissions.get(&result.id).await?.data;
        println!(
            "Pending request {} for {}",
            pending.id.as_str(),
            pending.action
        );
        let location_pending = client.permissions().list().await?;
        println!(
            "{} pending requests at this location",
            location_pending.data.len()
        );
        // This example explicitly rejects only its own manually-created fixture.
        // A real client presents the request and lets its caller choose the reply.
        // Reject can also clear other pending requests in the same session.
        permissions
            .reply(
                &pending.id,
                &SessionPermissionReplyRequest::new(PermissionReply::Reject)
                    .message("Finished the permission API demonstration"),
            )
            .await?;
        println!(
            "{} requests remain in the example session",
            permissions.list().await?.data.len()
        );
    }
    // Evaluation never executes the named action. No prompt/tool was submitted.
    client.sessions().remove(&session.id).await?;
    println!("Removed the example session");
    // On an earlier error, inspect the printed session ID before retrying.
    Ok(())
}
