//! Runs only after the parent harness verifies ownership of the disposable server.
use super::{LocalProvider, assistant_text, history, until};
use opencoders::{
    ApiError, Client, Error, EventOptions, SessionDiffOptions, SessionWaitOptions, models::*,
};
use std::{path::Path, time::Duration};

async fn idle(client: &Client, id: &SessionId) {
    client
        .sessions()
        .experimental_wait(
            id,
            &SessionWaitOptions::default().timeout(Duration::from_secs(15)),
        )
        .await
        .unwrap();
}
fn busy(error: Error) {
    let Error::Http(e) = error else {
        panic!("HTTP error")
    };
    assert_eq!(e.status(), 409);
    assert!(matches!(e.api_error(), Some(ApiError::SessionBusy { .. })));
}
pub(super) async fn verify(client: &Client, provider: &LocalProvider, workspace: &Path) {
    println!("active scenario: session workflows, snapshots and compaction");
    let sessions = client.sessions();
    let session = sessions
        .create(
            &SessionCreateRequest::default()
                .location(SessionLocationInput::new(workspace.to_str().unwrap()))
                .agent("fixture".parse().unwrap())
                .model(SessionModelInput::new(
                    "fixture".parse().unwrap(),
                    "command",
                )),
        )
        .await
        .unwrap()
        .data;
    let id = &session.id;
    // Execution awaits plugin activation, then resolves command registration.
    let Error::Http(e) = sessions
        .command(
            id,
            &SessionCommandRequest::new("missing-fixture-command", ""),
        )
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(e.status(), 404);
    assert!(matches!(
        e.api_error(),
        Some(ApiError::CommandNotFound { .. })
    ));
    let commands = client.commands().list().await.unwrap();
    assert!(commands.data.iter().any(|c| c.name == "fixture-command"));
    assert_eq!(
        commands.location.directory.as_str(),
        workspace.to_str().unwrap()
    );
    let skills = client.skills().list().await.unwrap();
    let skill = skills
        .data
        .iter()
        .find(|s| s.name.as_str() == "workflow-fixture")
        .expect("registered fixture skill");
    assert_eq!(skill.autoinvoke, Some(false));
    assert!(skill.content.contains("Local skill content."));
    assert!(sessions.context(id).await.unwrap().data.is_empty());
    assert!(
        sessions
            .diff(id, &SessionDiffOptions::default())
            .await
            .unwrap()
            .data
            .is_empty()
    );
    let Error::Http(e) = sessions
        .command(id, &SessionCommandRequest::new("fixture-command-fails", ""))
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(e.status(), 500);
    assert!(matches!(
        e.api_error(),
        Some(ApiError::CommandExecution { .. })
    ));
    let mut events = client
        .events()
        .subscribe(&EventOptions::default().types([
            "session.text.delta",
            "session.execution.succeeded",
            "session.execution.failed",
        ]))
        .await
        .unwrap();
    sessions
        .command(
            id,
            &SessionCommandRequest::new("fixture-command", "argument")
                .skills(vec![PromptInputSkillAttachment::new(skill.id.clone())]),
        )
        .await
        .unwrap();
    until(&mut events, "session.text.delta").await;
    let active = history(client, id).await;
    let first = active
        .iter()
        .find_map(|m| {
            if let SessionMessageInfo::User(v) = m {
                Some(v.id.clone())
            } else {
                None
            }
        })
        .unwrap();
    busy(
        sessions
            .revert(id)
            .stage(&SessionRevertStageRequest::new(first.clone()).files(false))
            .await
            .unwrap_err(),
    );
    busy(sessions.revert(id).clear().await.unwrap_err());
    busy(sessions.revert(id).commit().await.unwrap_err());
    // The first text delta follows capture of the start snapshot. This controlled
    // write is inside the owned fixture and will be captured at the end of the step.
    let file = workspace.join("workflow.txt");
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "before workflow\n");
    std::fs::write(&file, "after workflow\n").unwrap();
    provider.release("command").await;
    until(&mut events, "session.execution.succeeded").await;
    idle(client, id).await;
    let original = history(client, id).await;
    assert_eq!(assistant_text(&original), "first second");
    assert!(
        serde_json::to_string(&original)
            .unwrap()
            .contains("workflow fixture argument")
    );
    let last = original.last().unwrap().id().clone();
    let diff = sessions
        .diff(
            id,
            &SessionDiffOptions::default()
                .from(first.clone())
                .to(first.clone())
                .context(0),
        )
        .await
        .unwrap();
    assert!(diff.data.iter().any(|d| d.file.as_str() == "workflow.txt"));
    let Error::Http(e) = sessions
        .diff(id, &SessionDiffOptions::default().from(last.clone()))
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(e.status(), 400);
    assert!(matches!(
        e.api_error(),
        Some(ApiError::InvalidRequest { .. })
    ));
    let staged = sessions
        .revert(id)
        .stage(&SessionRevertStageRequest::new(first.clone()).files(false))
        .await
        .unwrap()
        .data;
    assert_eq!(staged.message_id, first);
    assert!(staged.files.is_empty());
    assert_eq!(history(client, id).await.len(), original.len());
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "after workflow\n");
    sessions.revert(id).clear().await.unwrap();
    idle(client, id).await;
    let staged = sessions
        .revert(id)
        .stage(&SessionRevertStageRequest::new(first.clone()))
        .await
        .unwrap()
        .data;
    assert!(
        staged
            .files
            .iter()
            .any(|d| d.file.as_str() == "workflow.txt")
    );
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "before workflow\n");
    sessions.revert(id).clear().await.unwrap();
    idle(client, id).await;
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "after workflow\n");
    sessions
        .switch_model(
            id,
            &SessionSwitchModelRequest::new(SessionModelInput::new(
                "fixture".parse().unwrap(),
                "clear",
            )),
        )
        .await
        .unwrap();
    let parked_id: SessionMessageId = "msg_workflow_parked".parse().unwrap();
    sessions
        .prompt(
            id,
            &SessionPromptRequest::new("queued before staged revert")
                .id(parked_id.clone())
                .resume(false),
        )
        .await
        .unwrap();
    sessions
        .revert(id)
        .stage(&SessionRevertStageRequest::new(first.clone()).files(false))
        .await
        .unwrap();
    assert_eq!(sessions.inbox(id).list().await.unwrap().data.len(), 1);
    sessions.revert(id).clear().await.unwrap();
    until(&mut events, "session.execution.succeeded").await;
    idle(client, id).await;
    assert!(
        history(client, id)
            .await
            .iter()
            .any(|m| m.id() == &parked_id)
    );
    assert!(sessions.inbox(id).list().await.unwrap().data.is_empty());
    sessions
        .switch_model(
            id,
            &SessionSwitchModelRequest::new(SessionModelInput::new(
                "fixture".parse().unwrap(),
                "compact",
            )),
        )
        .await
        .unwrap();
    // Staging the old idle marker removes it and the subsequent model selection
    // when compaction automatically commits this boundary; selected model persists.
    sessions
        .revert(id)
        .stage(&SessionRevertStageRequest::new(last.clone()).files(false))
        .await
        .unwrap();
    let compact_id: SessionMessageId = "msg_workflow_compact".parse().unwrap();
    let admitted = sessions
        .compact(id, &SessionCompactRequest::default().id(compact_id.clone()))
        .await
        .unwrap()
        .data;
    assert_eq!(admitted.id, compact_id);
    assert_eq!(admitted.session_id, *id);
    until(&mut events, "session.execution.succeeded").await;
    idle(client, id).await;
    assert!(sessions.get(id).await.unwrap().data.revert.is_none());
    let full = history(client, id).await;
    assert!(!full.iter().any(|m| m.id() == &last));
    let context = sessions.context(id).await.unwrap().data;
    assert!(context.len() < full.len());
    let Some(SessionMessageInfo::Compaction(compact)) = context.first() else {
        panic!("context should start at compaction")
    };
    assert!(
        matches!(compact.as_ref(),SessionMessageCompaction::Completed(c) if c.id==compact_id && c.summary.contains("workflow"))
    );
    sessions
        .revert(id)
        .stage(&SessionRevertStageRequest::new(compact_id.clone()).files(false))
        .await
        .unwrap();
    sessions.revert(id).commit().await.unwrap();
    assert!(
        !history(client, id)
            .await
            .iter()
            .any(|m| m.id() == &compact_id)
    );
    assert!(sessions.get(id).await.unwrap().data.revert.is_none());
    sessions.revert(id).commit().await.unwrap();
    sessions.revert(id).clear().await.unwrap();
    idle(client, id).await;
    drop(events);
    sessions.remove(id).await.unwrap();
}
