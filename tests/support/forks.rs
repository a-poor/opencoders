//! Fork checks run only inside the disposable, pinned live-server harness.
use opencoders::{
    ApiError, Client, Error, EventOptions, MessageListOptions, MessageOrder, ParentFilter,
    RequestField, SessionListOptions, models::*,
};
use serde_json::{Value, json};
use std::{collections::BTreeSet, time::Duration};

fn boundary(info: &SessionInfo, source: &SessionId, kind: &str, message: &SessionMessageId) {
    assert!(info.parent_id.is_none());
    let fork = info.fork.as_ref().unwrap();
    assert_eq!(&fork.session_id, source);
    assert_eq!(
        serde_json::to_value(&fork.boundary).unwrap(),
        json!({"type":kind,"messageID":message})
    );
}
async fn history(client: &Client, id: &SessionId) -> Vec<SessionMessageInfo> {
    client
        .sessions()
        .messages(id)
        .list(
            &MessageListOptions::default()
                .order(MessageOrder::Asc)
                .limit(100),
        )
        .await
        .unwrap()
        .data
}
fn content(message: &SessionMessageInfo) -> Value {
    let mut raw = serde_json::to_value(message).unwrap();
    raw.as_object_mut().unwrap().remove("id");
    raw
}
fn missing_message(error: Error) {
    let Error::Http(error) = error else {
        panic!("HTTP error")
    };
    assert_eq!(error.status(), 404);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::MessageNotFound { .. })
    ));
}
fn empty(error: Error) {
    let Error::Http(error) = error else {
        panic!("HTTP error")
    };
    assert_eq!(error.status(), 400);
    assert!(
        matches!(error.api_error(),Some(ApiError::InvalidRequest{kind:Some(k),..}) if k=="empty_session")
    );
}

pub async fn verify(
    client: &Client,
    http: &reqwest::Client,
    url: &str,
    empty_id: &SessionId,
    template: &SessionInfo,
) {
    let sessions = client.sessions();
    let source: SessionId = "ses_live_history".parse().unwrap();
    let default = SessionForkRequest::default();
    let mut null = default.clone();
    null.before = RequestField::Null;
    for request in [&default, &null] {
        empty(sessions.fork(empty_id, request).await.unwrap_err());
    }
    let Error::Http(error) = sessions
        .fork(&"ses_fork_missing".parse().unwrap(), &default)
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(error.status(), 404);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::SessionNotFound { .. })
    ));
    let first_id: SessionMessageId = "msg_live_01".parse().unwrap();
    missing_message(
        sessions
            .fork(empty_id, &default.clone().before(first_id.clone()))
            .await
            .unwrap_err(),
    );
    missing_message(
        sessions
            .fork(
                &source,
                &default.clone().before("msg_missing".parse().unwrap()),
            )
            .await
            .unwrap_err(),
    );

    // Current configuration is inherited even when copying an earlier boundary.
    sessions
        .switch_agent(
            &source,
            &SessionSwitchAgentRequest::new("build".to_owned().into()),
        )
        .await
        .unwrap();
    sessions
        .switch_model(
            &source,
            &SessionSwitchModelRequest::new(
                SessionModelInput::new("fixture".to_owned().into(), "fork-fixture-model")
                    .variant("fork-fixture-variant"),
            ),
        )
        .await
        .unwrap();
    sessions
        .update(
            &source,
            &SessionUpdateRequest::default()
                .title("Fork fixture")
                .metadata(
                    serde_json::from_value::<SessionMetadata>(json!({"fork-fixture":true}))
                        .unwrap(),
                )
                .permissions(vec![PermissionRuleInput::new(
                    "fixture.*",
                    PermissionEffect::Deny,
                    "*",
                )])
                .archived(true),
        )
        .await
        .unwrap();
    let source_info = sessions.get(&source).await.unwrap().data;
    let original = history(client, &source).await;
    assert_eq!(original.len(), 9);
    let pending = sessions
        .prompt(
            &source,
            &SessionPromptRequest::new("held fork fixture").resume(false),
        )
        .await
        .unwrap();
    let mut events = client
        .events()
        .subscribe(&EventOptions::default().types(["session.forked"]))
        .await
        .unwrap();
    let full = sessions.fork(&source, &default).await.unwrap().data;
    let null_fork = sessions.fork(&source, &null).await.unwrap().data;
    let before_id: SessionMessageId = "msg_live_04".parse().unwrap();
    let before = sessions
        .fork(&source, &default.clone().before(before_id.clone()))
        .await
        .unwrap()
        .data;
    let first = sessions
        .fork(&source, &default.clone().before(first_id.clone()))
        .await
        .unwrap()
        .data;
    assert_ne!(full.id, null_fork.id);
    for fork in [&full, &null_fork, &before, &first] {
        assert_eq!(fork.location.directory, source_info.location.directory);
        assert_eq!(fork.project_id, source_info.project_id);
        assert_eq!(fork.subpath, source_info.subpath);
        assert_eq!(
            serde_json::to_value(&fork.agent).unwrap(),
            serde_json::to_value(&source_info.agent).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&fork.model).unwrap(),
            serde_json::to_value(&source_info.model).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&fork.metadata).unwrap(),
            serde_json::to_value(&source_info.metadata).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&fork.permissions).unwrap(),
            serde_json::to_value(&source_info.permissions).unwrap()
        );
        assert!(fork.time.archived.is_none());
        assert!(fork.time.busy.is_none());
        assert!(fork.time.idle.is_none());
        assert!(fork.outcome.is_none());
        assert!(fork.revert.is_none());
        assert_eq!(fork.title.as_deref(), Some("Fork fixture (fork #1)"));
        assert!(
            sessions
                .inbox(&fork.id)
                .list()
                .await
                .unwrap()
                .data
                .is_empty()
        );
        let fetched = sessions.get(&fork.id).await.unwrap().data;
        assert_eq!(
            serde_json::to_value(&fetched).unwrap(),
            serde_json::to_value(fork).unwrap()
        );
    }
    boundary(&full, &source, "through", original.last().unwrap().id());
    boundary(
        &null_fork,
        &source,
        "through",
        original.last().unwrap().id(),
    );
    boundary(&before, &source, "before", &before_id);
    boundary(&first, &source, "before", &first_id);
    for (fork, expected) in [
        (&full, &original[..]),
        (&null_fork, &original[..]),
        (&before, &original[..3]),
        (&first, &original[..0]),
    ] {
        let copied = history(client, &fork.id).await;
        assert_eq!(copied.len(), expected.len());
        let ids: BTreeSet<_> = copied.iter().map(|m| m.id().clone()).collect();
        assert_eq!(ids.len(), copied.len());
        for (new, old) in copied.iter().zip(expected) {
            assert_ne!(new.id(), old.id());
            assert_eq!(content(new), content(old));
            assert_eq!(
                serde_json::to_value(
                    sessions
                        .messages(&fork.id)
                        .get(new.id())
                        .await
                        .unwrap()
                        .data
                )
                .unwrap(),
                serde_json::to_value(new).unwrap()
            );
        }
        if !copied.is_empty() {
            missing_message(
                sessions
                    .messages(&fork.id)
                    .get(&first_id)
                    .await
                    .unwrap_err(),
            );
        }
    }
    empty(sessions.fork(&first.id, &default).await.unwrap_err());
    assert_eq!(
        serde_json::to_value(history(client, &source).await).unwrap(),
        serde_json::to_value(&original).unwrap()
    );
    assert_eq!(sessions.inbox(&source).list().await.unwrap().data.len(), 1);
    sessions
        .inbox(&source)
        .cancel(&pending.data.id)
        .await
        .unwrap();
    let roots = sessions
        .list(
            &SessionListOptions::default()
                .parent(ParentFilter::RootOnly)
                .limit(100),
        )
        .await
        .unwrap();
    assert!(roots.data.iter().any(|s| s.id == full.id));
    assert!(
        sessions
            .list(&SessionListOptions::default().parent(ParentFilter::ChildrenOf(source.clone())))
            .await
            .unwrap()
            .data
            .is_empty()
    );
    // A fork of a fork retains immediate provenance and survives source removal.
    let grand = sessions.fork(&full.id, &default).await.unwrap().data;
    assert_eq!(grand.title.as_deref(), Some("Fork fixture (fork #2)"));
    assert_eq!(grand.fork.as_ref().unwrap().session_id, full.id);
    sessions.remove(&full.id).await.unwrap();
    assert_eq!(sessions.get(&grand.id).await.unwrap().data.id, grand.id);
    assert_eq!(history(client, &grand.id).await.len(), original.len());
    // The copied inline attachment remains readable after its source is removed.
    assert_eq!(
        content(&history(client, &grand.id).await[0]),
        content(&original[0])
    );
    let (mut saw_full, mut saw_before) = (false, false);
    tokio::time::timeout(Duration::from_secs(5), async {
        while !saw_full || !saw_before {
            if let Event::SessionForked(e) = events.next().await.unwrap().unwrap().event {
                if e.data.session_id == full.id {
                    assert_eq!(e.data.parent_id, source);
                    assert_eq!(
                        serde_json::to_value(&e.data.boundary).unwrap(),
                        serde_json::to_value(&full.fork.as_ref().unwrap().boundary).unwrap()
                    );
                    saw_full = true;
                }
                if e.data.session_id == before.id {
                    assert_eq!(e.data.parent_id, source);
                    assert_eq!(
                        serde_json::to_value(&e.data.boundary).unwrap(),
                        serde_json::to_value(&before.fork.as_ref().unwrap().boundary).unwrap()
                    );
                    saw_before = true;
                }
            }
        }
    })
    .await
    .unwrap();
    events.close();
    for fork in [&null_fork, &before, &first, &grand] {
        sessions.remove(&fork.id).await.unwrap();
    }
    verify_compaction(client, http, url, template).await;
}

async fn verify_compaction(
    client: &Client,
    http: &reqwest::Client,
    url: &str,
    template: &SessionInfo,
) {
    // Import settled projections only. The import route drops unfinished rows,
    // so active-work exclusion remains source-reviewed rather than verified here.
    // No compaction or model runs here.
    let messages = json!([
        {"id":"msg_fork_z","type":"user","time":{"created":1},"text":"settled"},
        {"id":"msg_fork_c","type":"assistant","time":{"created":2,"completed":3},"agent":"build","model":{"id":"fixture","providerID":"fixture"},"content":[]},
        {"id":"msg_fork_f","type":"compaction","status":"completed","reason":"manual","summary":"fixture","time":{"created":4},"metadata":{"keepID":"msg_fork_z"}}
    ]);
    // Validate the fixture against the same public models before importing it.
    let _: Vec<SessionMessageInfo> = serde_json::from_value(messages.clone()).unwrap();
    let mut info = serde_json::to_value(template).unwrap();
    info["id"] = json!("ses_live_fork_compaction");
    info["cost"] = json!(7);
    info["tokens"] = json!({"input":8,"output":9,"reasoning":1,"cache":{"read":2,"write":3}});
    let response=http.post(format!("{url}/api/experimental/session/import")).header(opencoders::header::CONTENT_TYPE,"application/json").body(serde_json::to_vec(&json!({"info":info,"messages":messages,"location":{"directory":template.location.directory}})).unwrap()).send().await.unwrap();
    assert_eq!(
        response.status(),
        200,
        "disposable fork fixture import failed"
    );
    let id = "ses_live_fork_compaction".parse().unwrap();
    let sessions = client.sessions();
    assert_eq!(history(client, &id).await.len(), 3);
    let source_info = sessions.get(&id).await.unwrap().data;
    assert_eq!(serde_json::to_value(&source_info.cost).unwrap(), json!(7));
    assert_eq!(
        serde_json::to_value(&source_info.tokens).unwrap(),
        json!({"input":8,"output":9,"reasoning":1,"cache":{"read":2,"write":3}})
    );
    let fork = sessions
        .fork(&id, &SessionForkRequest::default())
        .await
        .unwrap()
        .data;
    boundary(&fork, &id, "through", &"msg_fork_f".parse().unwrap());
    let copied = history(client, &fork.id).await;
    assert_eq!(copied.len(), 3);
    assert_eq!(
        copied.iter().map(|m| m.message_type()).collect::<Vec<_>>(),
        [
            SessionMessageType::User,
            SessionMessageType::Assistant,
            SessionMessageType::Compaction
        ]
    );
    let summary = serde_json::to_value(&copied[2]).unwrap();
    assert_eq!(summary["metadata"]["keepID"], copied[0].id().as_str());
    assert_eq!(serde_json::to_value(&fork.cost).unwrap(), json!(0));
    assert_eq!(
        serde_json::to_value(&fork.tokens).unwrap(),
        json!({"input":0,"output":0,"reasoning":0,"cache":{"read":0,"write":0}})
    );
    assert_eq!(history(client, &id).await.len(), 3);
    sessions.remove(&id).await.unwrap();
    assert_eq!(history(client, &fork.id).await.len(), 3);
    sessions.remove(&fork.id).await.unwrap();
}
