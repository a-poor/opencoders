//! Manual form creation and answering in a fresh session; no model calls or UI.
//! Requires OPENCODE_BASE_URL and OPENCODE_DIRECTORY; OPENCODE_PASSWORD optional.
use opencoders::{Client, Location, SessionFormCancelOptions, models::*};
use std::{collections::HashMap, env, error::Error};

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
                .title("Form API example")
                .location(SessionLocationInput::new(directory)),
        )
        .await?
        .data;
    println!("Created example session {}", session.id.as_str());
    let result = async {
        let forms = client.sessions().forms(&session.id);
        let mut field = FormBooleanFieldInput::new("confirmed");
        field.title = Some("Confirm this example fixture".into());
        field.required = Some(true);
        let request = SessionFormCreateRequest::new("Manual example", FormFieldsInput::new(field));
        let form = forms.create(&request).await?.data;
        let pending = client.forms().list().await?;
        println!("{} pending forms at this location", pending.data.len());
        assert!(matches!(
            forms.get(&form.id).await?.data.state,
            FormState::Pending(_)
        ));
        // This answer is explicit and applies only to this example's own fixture.
        // A GUI/TUI would render fields and collect its user's answers here.
        let answer = HashMap::from([("confirmed".into(), FormValue::Boolean(false))]);
        forms
            .reply(&form.id, &SessionFormReplyRequest::new(answer.into()))
            .await?;
        assert!(matches!(
            forms.get(&form.id).await?.data.state,
            FormState::Answered(_)
        ));
        let cancelled = forms.create(&request).await?.data;
        forms
            .cancel(
                &cancelled.id,
                &SessionFormCancelOptions::default().message("Finished demonstration"),
            )
            .await?;
        assert!(matches!(
            forms.get(&cancelled.id).await?.data.state,
            FormState::Cancelled(_)
        ));
        println!("Answered one form and cancelled another");
        Ok::<(), opencoders::Error>(())
    }
    .await;
    // Attempt cleanup even if a form operation failed. Terminal form data is
    // retained separately in server memory and is not durable session history.
    let cleanup = client.sessions().remove(&session.id).await;
    result?;
    cleanup?;
    println!("Removed the example session");
    Ok(())
}
