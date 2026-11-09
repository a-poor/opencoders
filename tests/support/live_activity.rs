//! Called only after the harness verifies ownership of its disposable server.
use super::{LocalProvider, assistant_text, history, until};
use opencoders::{Client, EventOptions, SessionWaitOptions, models::*};
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
    assert!(
        !client
            .sessions()
            .active()
            .await
            .unwrap()
            .data
            .contains_key(id)
    );
}

pub(super) async fn verify(client: &Client, provider: &LocalProvider, workspace: &Path) {
    println!("active scenario: activity, viewed watermarks and synthetic input");
    let sessions = client.sessions();
    let session = sessions
        .create(
            &SessionCreateRequest::default()
                .location(SessionLocationInput::new(workspace.to_str().unwrap()))
                .agent("fixture".parse().unwrap())
                .model(SessionModelInput::new(
                    "fixture".parse().unwrap(),
                    "synthetic",
                )),
        )
        .await
        .unwrap()
        .data;
    let id = &session.id;
    assert!(sessions.active().await.unwrap().data.is_empty());
    sessions
        .view(id, &SessionViewRequest::new(0.into()))
        .await
        .unwrap();
    assert!(sessions.get(id).await.unwrap().data.time.viewed.is_none());
    let input_id: SessionMessageId = "msg_activity_synthetic".parse().unwrap();
    let admitted = sessions
        .synthetic(
            id,
            &SessionSyntheticRequest::new("synthetic fixture")
                .id(input_id.clone())
                .description("")
                .metadata(Default::default())
                .delivery(SessionInboxDelivery::Queue)
                .resume(false),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(admitted.id, input_id);
    assert_eq!(admitted.payload.description.as_deref(), Some(""));
    assert!(admitted.payload.metadata.is_empty());
    assert_eq!(sessions.inbox(id).list().await.unwrap().data.len(), 1);
    assert!(!sessions.active().await.unwrap().data.contains_key(id));
    assert!(provider.state().await["requests"]["synthetic"].is_null());
    // Same ID/type/session keeps the original content and pending delivery even
    // when the retry supplies different text. Waking is a separate decision.
    let replay = sessions
        .synthetic(
            id,
            &SessionSyntheticRequest::new("ignored replacement")
                .id(input_id.clone())
                .resume(false),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(
        serde_json::to_value(&replay).unwrap(),
        serde_json::to_value(&admitted).unwrap()
    );
    let mut events = client
        .events()
        .subscribe(&EventOptions::default().types([
            "session.text.delta",
            "session.execution.succeeded",
            "session.execution.failed",
            "session.viewed",
        ]))
        .await
        .unwrap();
    sessions
        .synthetic(
            id,
            &SessionSyntheticRequest::new("ignored replacement").id(input_id.clone()),
        )
        .await
        .unwrap();
    until(&mut events, "session.text.delta").await;
    let active = sessions.active().await.unwrap();
    assert_eq!(active.data.len(), 1);
    assert_eq!(active.data[id].type_, SessionActiveType::Running);
    assert!(sessions.inbox(id).list().await.unwrap().data.is_empty());
    provider.release("synthetic").await;
    until(&mut events, "session.execution.succeeded").await;
    idle(client, id).await;
    let first = sessions.get(id).await.unwrap().data;
    let first_idle = first.time.idle.unwrap();
    let messages = history(client, id).await;
    assert_eq!(assistant_text(&messages), "synthetic complete");
    assert!(messages.iter().any(|m| matches!(m, SessionMessageInfo::Synthetic(s) if s.id == input_id && s.text == "synthetic fixture")));
    // Reconciliation after promotion reconstructs from history, without another
    // message or execution. Delivery is supplied by this reconciliation request.
    let replay = sessions
        .synthetic(
            id,
            &SessionSyntheticRequest::new("ignored after promotion")
                .id(input_id.clone())
                .resume(false),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(replay.payload.text, admitted.payload.text);
    assert_eq!(history(client, id).await.len(), messages.len());
    let future = first_idle.as_u64().unwrap() + 60_000;
    sessions
        .view(id, &SessionViewRequest::new(future.into()))
        .await
        .unwrap();
    assert!(sessions.get(id).await.unwrap().data.time.viewed.is_none());
    sessions
        .switch_model(
            id,
            &SessionSwitchModelRequest::new(SessionModelInput::new(
                "fixture".parse().unwrap(),
                "synthetic_second",
            )),
        )
        .await
        .unwrap();
    sessions
        .synthetic(
            id,
            &SessionSyntheticRequest::new("synthetic fixture second"),
        )
        .await
        .unwrap();
    until(&mut events, "session.text.delta").await;
    assert!(sessions.active().await.unwrap().data.contains_key(id));
    provider.release("synthetic_second").await;
    until(&mut events, "session.execution.succeeded").await;
    idle(client, id).await;
    let second = sessions.get(id).await.unwrap().data;
    let second_idle = second.time.idle.unwrap();
    assert!(second_idle.as_u64().unwrap() > first_idle.as_u64().unwrap());
    assert!(second.time.viewed.is_none());
    // The UI's first acknowledgement arrives after a newer idle transition.
    // It advances only to the older observation, leaving the newer work unread.
    sessions
        .view(id, &SessionViewRequest::new(first_idle.clone()))
        .await
        .unwrap();
    let observed = until(&mut events, "session.viewed").await;
    assert!(
        matches!(observed.last(), Some(Event::SessionViewed(v)) if v.data.session_id == *id && v.data.idle == first_idle)
    );
    let viewed = sessions.get(id).await.unwrap().data;
    assert_eq!(viewed.time.viewed, Some(first_idle.clone()));
    assert_eq!(viewed.time.updated, second.time.updated);
    for old in [0.into(), first_idle.clone()] {
        sessions
            .view(id, &SessionViewRequest::new(old))
            .await
            .unwrap();
        assert_eq!(
            sessions.get(id).await.unwrap().data.time.viewed,
            Some(first_idle.clone())
        );
    }
    sessions
        .view(id, &SessionViewRequest::new(second_idle.clone()))
        .await
        .unwrap();
    let observed = until(&mut events, "session.viewed").await;
    assert!(
        matches!(observed.last(), Some(Event::SessionViewed(v)) if v.data.session_id == *id && v.data.idle == second_idle)
    );
    assert_eq!(
        sessions.get(id).await.unwrap().data.time.viewed,
        Some(second_idle)
    );
    // Unlike prompts/compaction, synthetic admission must retain staged history.
    let boundary = history(client, id).await.last().unwrap().id().clone();
    sessions
        .revert(id)
        .stage(&SessionRevertStageRequest::new(boundary.clone()).files(false))
        .await
        .unwrap();
    let held = sessions
        .synthetic(id, &SessionSyntheticRequest::new("held by revert"))
        .await
        .unwrap()
        .data;
    assert_eq!(
        sessions
            .get(id)
            .await
            .unwrap()
            .data
            .revert
            .unwrap()
            .message_id,
        boundary
    );
    assert!(!sessions.active().await.unwrap().data.contains_key(id));
    assert_eq!(sessions.inbox(id).list().await.unwrap().data.len(), 1);
    sessions.inbox(id).cancel(&held.id).await.unwrap();
    sessions.revert(id).clear().await.unwrap();
    idle(client, id).await;
    assert_eq!(provider.state().await["requests"]["synthetic"], 1);
    assert_eq!(provider.state().await["requests"]["synthetic_second"], 1);
    drop(events);
    sessions.remove(id).await.unwrap();
}
