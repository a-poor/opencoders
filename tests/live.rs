//! Opt-in only: `python3 tools/live/run.py` owns the server, credentials, and data.

use opencoders::{
    ApiError, Client, Error, Location, MessageListOptions, MessageOrder, ParentFilter,
    RequestField, SessionListOptions, SessionOrder, header,
    models::{
        PermissionEffect, PermissionRuleInput, SessionCreateRequest, SessionInfo,
        SessionLocationInput, SessionMessageInfo, SessionMessageType, SessionUpdateRequest,
    },
};
use serde_json::json;
use std::{collections::BTreeSet, env, path::PathBuf, time::Duration};

fn required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} missing; use python3 tools/live/run.py"))
}

#[tokio::test]
#[ignore = "requires a disposable pinned server; run python3 tools/live/run.py"]
async fn pinned_server_discovery_and_session_pagination() {
    let url = required("OPENCODERS_LIVE_URL");
    let root = PathBuf::from(required("OPENCODERS_LIVE_ROOT"))
        .canonicalize()
        .unwrap();
    let workspace = root.join("workspace + unicode 雪");
    let other = root.join("other-workspace");
    let workspace = workspace.to_str().unwrap();
    let other = other.to_str().unwrap();
    let mut headers = header::HeaderMap::new();
    let mut auth: header::HeaderValue = required("OPENCODERS_LIVE_AUTH").parse().unwrap();
    auth.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth);
    let http = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .timeout(Duration::from_secs(30))
        .default_headers(headers.clone())
        .build()
        .unwrap();
    let client = Client::builder(&url)
        .http_client(http.clone())
        .basic_auth("opencode", required("OPENCODERS_LIVE_PASSWORD"))
        .default_location(Location::directory(workspace))
        .build()
        .unwrap();
    assert_eq!(client.base_url().scheme(), "http");
    assert_eq!(client.base_url().host_str(), Some("127.0.0.1"));

    // Verify ownership before any fixture writes. Never attach this test to a user's server.
    let info = client.server().info().await.unwrap();
    assert_eq!(info.version, required("OPENCODERS_LIVE_VERSION"));
    assert_eq!(
        info.pid,
        required("OPENCODERS_LIVE_PID").parse::<u64>().unwrap()
    );
    assert!(PathBuf::from(&info.paths.tmp).starts_with(&root));
    let empty = client
        .sessions()
        .list(&SessionListOptions::default())
        .await
        .unwrap();
    assert!(empty.data.is_empty());
    assert!(empty.cursor.next.is_none());

    let unauthenticated = Client::builder(&url)
        .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
        .build()
        .unwrap();
    let Error::Http(error) = unauthenticated.server().info().await.unwrap_err() else {
        panic!("expected 401")
    };
    assert_eq!(error.status(), 401);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::Unauthorized { .. })
    ));

    let location = client.location().get().await.unwrap();
    assert_eq!(location.directory.as_str(), workspace);
    let agents = client.agents().list().await.unwrap();
    assert_eq!(agents.location.directory.as_str(), workspace);
    assert!(!agents.data.is_empty());
    assert_eq!(
        client
            .location()
            .get_at(Location::directory(other))
            .await
            .unwrap()
            .directory
            .as_str(),
        other
    );
    headers.insert("x-opencode-directory", other.parse().unwrap());
    let fallback = Client::builder(&url)
        .http_client(http.clone())
        .default_headers(headers)
        .default_location(Location::directory(workspace))
        .build()
        .unwrap();
    assert_eq!(
        fallback
            .location()
            .get_at(Location::Unspecified)
            .await
            .unwrap()
            .directory
            .as_str(),
        other
    );
    assert_eq!(
        fallback.location().get().await.unwrap().directory.as_str(),
        workspace
    );

    // Empty sessions exercise SDK mutations without submitting prompts.
    let mut ids = Vec::new();
    for title in ["live alpha", "live beta", "live gamma"] {
        let request = SessionCreateRequest::default()
            .title(title)
            .location(SessionLocationInput::new(workspace))
            .metadata(serde_json::Map::from_iter([(
                "fixture".into(),
                json!(true),
            )]))
            .permissions(vec![]);
        let session = client.sessions().create(&request).await.unwrap().data;
        assert_eq!(session.title.as_deref(), Some(title));
        let retrieved = client.sessions().get(&session.id).await.unwrap().data;
        assert_eq!(retrieved.id, session.id);
        assert_eq!(retrieved.title, session.title);
        assert_eq!(retrieved.location.directory.as_str(), workspace);
        assert_eq!(
            serde_json::to_value(retrieved.metadata).unwrap(),
            json!({"fixture": true})
        );
        ids.push(session.id.to_string());
    }
    client
        .sessions()
        .update(
            &ids[1].parse().unwrap(),
            &SessionUpdateRequest::default().archived(true),
        )
        .await
        .unwrap();
    let child = client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .title("live child")
                .parent_id(ids[0].parse().unwrap())
                .location(SessionLocationInput::new(other)),
        )
        .await
        .unwrap()
        .data;
    // A concrete parent wins over an explicitly conflicting location.
    assert_eq!(child.parent_id.as_ref().unwrap().as_str(), ids[0]);
    assert_eq!(child.location.directory.as_str(), workspace);
    let elsewhere = client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .title("live other")
                .location(SessionLocationInput::new(other)),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(
        client
            .sessions()
            .get(&elsewhere.id)
            .await
            .unwrap()
            .data
            .location
            .directory
            .as_str(),
        other
    );

    // The default SDK location must not limit this server-wide resource.
    let all = client
        .sessions()
        .list(&SessionListOptions::default())
        .await
        .unwrap();
    assert_eq!(all.data.len(), 5);
    let archived = client
        .sessions()
        .list(&SessionListOptions::default().archived(true))
        .await
        .unwrap();
    assert_eq!(archived.data.len(), 1);
    assert_eq!(archived.data[0].id.as_str(), ids[1]);
    let children = client
        .sessions()
        .list(
            &SessionListOptions::default()
                .parent(ParentFilter::ChildrenOf(ids[0].parse().unwrap())),
        )
        .await
        .unwrap();
    assert_eq!(children.data.len(), 1);

    let mut options = SessionListOptions::default()
        .directory(workspace)
        .parent(ParentFilter::RootOnly)
        .archived(false)
        .search("live ")
        .limit(1)
        .order(SessionOrder::Desc);
    let first = client.sessions().list(&options).await.unwrap();
    assert_eq!(first.data.len(), 1);
    options.cursor = Some(first.cursor.next.clone().expect("second page"));
    options.order = None;
    let second = client.sessions().list(&options).await.unwrap();
    assert_eq!(second.data.len(), 1);
    let changed_filters = SessionListOptions::default()
        .cursor(first.cursor.next.clone().unwrap())
        .limit(10)
        .directory(other)
        .archived(true)
        .search("does not match")
        .order(SessionOrder::Asc);
    let bound = client.sessions().list(&changed_filters).await.unwrap();
    assert_eq!(bound.data.len(), 1);
    assert_eq!(bound.data[0].id, second.data[0].id);
    assert!(bound.cursor.next.is_some());
    assert_eq!(
        BTreeSet::from([first.data[0].id.as_str(), second.data[0].id.as_str()]),
        BTreeSet::from([ids[0].as_str(), ids[2].as_str()])
    );
    // Cursors mark nonempty page boundaries, not proof that another row exists.
    options.cursor = Some(second.cursor.next.clone().expect("full-page continuation"));
    let end = client.sessions().list(&options).await.unwrap();
    assert!(end.data.is_empty());
    assert!(end.cursor.next.is_none());
    options.cursor = Some(second.cursor.previous.expect("previous page"));
    let previous = client.sessions().list(&options).await.unwrap();
    assert_eq!(previous.data[0].id, first.data[0].id);

    options.cursor = Some("invalid-cursor".to_owned().into());
    let Error::Http(error) = client.sessions().list(&options).await.unwrap_err() else {
        panic!("expected invalid cursor")
    };
    assert_eq!(error.status(), 400);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::InvalidCursor { .. })
    ));

    let error = client
        .sessions()
        .list(&SessionListOptions::default().limit(0))
        .await
        .unwrap_err();
    assert_eq!(error.status(), Some(opencoders::StatusCode::BAD_REQUEST));
    let missing = "ses_opencoders_missing".parse().unwrap();
    for result in [
        client.sessions().get(&missing).await,
        client
            .sessions()
            .create(&SessionCreateRequest::default().parent_id(missing.clone()))
            .await,
    ] {
        let Error::Http(error) = result.unwrap_err() else {
            panic!("expected missing session")
        };
        assert_eq!(error.status(), 404);
        assert!(
            matches!(error.api_error(), Some(ApiError::SessionNotFound { session_id, .. }) if session_id == missing.as_str())
        );
    }

    // Exercise server defaults and explicit null separately; neither request uses
    // the SDK discovery default to rewrite its body.
    let defaults_client = Client::builder(&url)
        .http_client(http.clone())
        .default_location(Location::directory(other))
        .build()
        .unwrap();
    let defaults = defaults_client
        .sessions()
        .create(&SessionCreateRequest::default())
        .await
        .unwrap()
        .data;
    assert_eq!(defaults.location.directory.as_str(), workspace);
    let mut nulls = SessionCreateRequest::default();
    nulls.id = RequestField::Null;
    nulls.parent_id = RequestField::Null;
    nulls.title = RequestField::Null;
    nulls.agent = RequestField::Null;
    nulls.model = RequestField::Null;
    nulls.location = RequestField::Null;
    nulls.metadata = RequestField::Null;
    nulls.permissions = RequestField::Null;
    let null_session = defaults_client
        .sessions()
        .create(&nulls)
        .await
        .unwrap()
        .data;
    assert_eq!(null_session.location.directory.as_str(), workspace);
    assert!(null_session.parent_id.is_none());
    // All sessions have no messages. This exercises updates without provider calls.
    let rule = PermissionRuleInput::new("read", PermissionEffect::Ask, "*");
    let update = SessionUpdateRequest::default()
        .title("Updated remote session")
        .metadata(serde_json::Map::from_iter([("old".into(), json!(true))]))
        .permissions(vec![rule])
        .archived(true);
    client
        .sessions()
        .update(&elsewhere.id, &update)
        .await
        .unwrap();
    let updated = client.sessions().get(&elsewhere.id).await.unwrap().data;
    assert_eq!(updated.title.as_deref(), Some("Updated remote session"));
    assert_eq!(
        serde_json::to_value(&updated.metadata).unwrap(),
        json!({"old": true})
    );
    assert_eq!(
        serde_json::to_value(&updated.permissions).unwrap(),
        json!([{"action": "read", "effect": "ask", "resource": "*"}])
    );
    assert!(updated.time.archived.is_some());
    let snapshot = serde_json::to_value(&updated).unwrap();

    // Empty, explicit-null, and repeated archive requests must not clear fields.
    let null_update: SessionUpdateRequest = serde_json::from_value(json!({
        "title": null, "metadata": null, "permissions": null, "archived": null
    }))
    .unwrap();
    for request in [
        SessionUpdateRequest::default(),
        null_update,
        SessionUpdateRequest::default().archived(true),
    ] {
        client
            .sessions()
            .update(&elsewhere.id, &request)
            .await
            .unwrap();
        assert_eq!(
            serde_json::to_value(client.sessions().get(&elsewhere.id).await.unwrap().data).unwrap(),
            snapshot
        );
    }

    client
        .sessions()
        .update(
            &elsewhere.id,
            &SessionUpdateRequest::default()
                .metadata(serde_json::Map::from_iter([(
                    "replacement".into(),
                    json!("yes"),
                )]))
                .permissions(vec![PermissionRuleInput::new(
                    "write",
                    PermissionEffect::Deny,
                    "*",
                )])
                .archived(false),
        )
        .await
        .unwrap();
    let restored = client.sessions().get(&elsewhere.id).await.unwrap().data;
    assert_eq!(
        serde_json::to_value(restored.metadata).unwrap(),
        json!({"replacement": "yes"})
    );
    assert_eq!(
        serde_json::to_value(restored.permissions).unwrap(),
        json!([{"action": "write", "effect": "deny", "resource": "*"}])
    );
    assert!(restored.time.archived.is_none());

    client
        .sessions()
        .update(
            &elsewhere.id,
            &SessionUpdateRequest::default()
                .metadata(serde_json::Map::new())
                .permissions(vec![]),
        )
        .await
        .unwrap();
    let cleared = client.sessions().get(&elsewhere.id).await.unwrap().data;
    assert_eq!(serde_json::to_value(cleared.metadata).unwrap(), json!({}));
    assert_eq!(
        serde_json::to_value(cleared.permissions).unwrap(),
        json!([])
    );
    // v2.2.5 title generation returns before any model call without a user message.
    // Empty title is not a request to clear the title; populated sessions can invoke a model.
    client
        .sessions()
        .update(&elsewhere.id, &SessionUpdateRequest::default().title(""))
        .await
        .unwrap();
    assert_eq!(
        client
            .sessions()
            .get(&elsewhere.id)
            .await
            .unwrap()
            .data
            .title
            .as_deref(),
        Some("Updated remote session")
    );

    for result in [
        client
            .sessions()
            .update(&missing, &SessionUpdateRequest::default())
            .await,
        client.sessions().remove(&missing).await,
    ] {
        let Error::Http(error) = result.unwrap_err() else {
            panic!("expected missing session")
        };
        assert_eq!(error.status(), 404);
        assert!(matches!(
            error.api_error(),
            Some(ApiError::SessionNotFound { .. })
        ));
    }

    let grandchild = client
        .sessions()
        .create(
            &SessionCreateRequest::default()
                .parent_id(child.id.clone())
                .title("live grandchild"),
        )
        .await
        .unwrap()
        .data;
    let parent = ids[0].parse().unwrap();
    client.sessions().remove(&parent).await.unwrap();
    for id in [&parent, &child.id, &grandchild.id] {
        let Error::Http(error) = client.sessions().get(id).await.unwrap_err() else {
            panic!("expected deleted session")
        };
        assert_eq!(error.status(), 404);
        assert!(matches!(
            error.api_error(),
            Some(ApiError::SessionNotFound { .. })
        ));
    }
    assert_eq!(
        client.sessions().get(&elsewhere.id).await.unwrap().data.id,
        elsewhere.id
    );
    let Error::Http(error) = client.sessions().remove(&parent).await.unwrap_err() else {
        panic!("expected repeated-delete error")
    };
    assert_eq!(error.status(), 404);
    verify_message_history(&client, &http, &url, &elsewhere).await;
    verify_selection(&client, &fallback, &elsewhere.id, workspace, other).await;
    verify_prompt_inbox(&client, &ids[1].parse().unwrap(), &elsewhere.id).await;
    verify_execution(&client, &ids[1].parse().unwrap()).await;
    verify_events(&client, &ids[1].parse().unwrap(), &elsewhere.id).await;
    verify_session_logs(&client, &ids[1].parse().unwrap(), &elsewhere.id).await;
    verify_permissions(
        &client,
        &fallback,
        [
            &ids[1].parse().unwrap(),
            &ids[2].parse().unwrap(),
            &elsewhere.id,
        ],
        workspace,
        other,
    )
    .await;
    let remaining = client
        .sessions()
        .list(&SessionListOptions::default())
        .await
        .unwrap();
    assert_eq!(remaining.data.len(), 6);
    for session in remaining.data {
        client.sessions().remove(&session.id).await.unwrap();
    }
    assert!(
        client
            .sessions()
            .list(&SessionListOptions::default())
            .await
            .unwrap()
            .data
            .is_empty()
    );
}

async fn verify_permissions(
    client: &Client,
    fallback: &Client,
    ids: [&opencoders::models::SessionId; 3],
    workspace: &str,
    other: &str,
) {
    use opencoders::{
        EventOptions,
        models::{
            PermissionReply, PermissionSourceInput, SessionId, SessionPermissionCreateRequest,
            SessionPermissionReplyRequest,
        },
    };
    for id in ids {
        client
            .sessions()
            .update(
                id,
                &SessionUpdateRequest::default().permissions(vec![
                    PermissionRuleInput::new("opencoders.live.*", PermissionEffect::Ask, "*"),
                    PermissionRuleInput::new("opencoders.live.allow", PermissionEffect::Allow, "*"),
                    PermissionRuleInput::new("opencoders.live.deny", PermissionEffect::Deny, "*"),
                ]),
            )
            .await
            .unwrap();
    }
    let p = client.sessions().permissions(ids[0]);
    let sibling = client.sessions().permissions(ids[1]);
    let remote = client.sessions().permissions(ids[2]);
    let mut events = client
        .events()
        .subscribe(&EventOptions::default().types(["permission.asked", "permission.replied"]))
        .await
        .unwrap();
    for (action, effect) in [
        ("opencoders.live.allow", PermissionEffect::Allow),
        ("opencoders.live.deny", PermissionEffect::Deny),
    ] {
        let result = p
            .create(&SessionPermissionCreateRequest::new(
                action,
                vec!["a".into()],
            ))
            .await
            .unwrap();
        assert_eq!(result.data.effect, effect);
        assert!(p.list().await.unwrap().data.is_empty());
        assert_permission_missing(p.get(&result.data.id).await.unwrap_err());
    }
    let mut request = SessionPermissionCreateRequest::new(
        "opencoders.live.ask",
        vec!["file + 雪".into(), "file + 雪".into()],
    )
    .id("per_live_first".parse().unwrap())
    .save(vec![])
    .metadata(serde_json::from_value(json!({"fixture":true})).unwrap())
    .source(PermissionSourceInput::tool(
        "unprefixed-message",
        "unprefixed-tool",
    ));
    request.agent = RequestField::Null;
    let first = p.create(&request).await.unwrap().data;
    assert_eq!(first.effect, PermissionEffect::Ask);
    let got = p.get(&first.id).await.unwrap().data;
    assert_eq!(got.resources, request.resources);
    assert_eq!(got.metadata["fixture"], true);
    assert_eq!(got.source.unwrap().message_id, "unprefixed-message");
    let mut null_id = SessionPermissionCreateRequest::new("opencoders.live.ask", vec!["b".into()]);
    null_id.id = RequestField::Null;
    let second = p.create(&null_id).await.unwrap().data;
    let third = sibling
        .create(&SessionPermissionCreateRequest::new(
            "opencoders.live.ask",
            vec!["c".into()],
        ))
        .await
        .unwrap()
        .data;
    let outside = remote
        .create(&SessionPermissionCreateRequest::new(
            "opencoders.live.ask",
            vec!["d".into()],
        ))
        .await
        .unwrap()
        .data;
    let listed = client.permissions().list().await.unwrap();
    assert_eq!(listed.location.directory.as_str(), workspace);
    assert_eq!(
        listed
            .data
            .iter()
            .map(|r| r.id.as_str())
            .collect::<Vec<_>>(),
        [first.id.as_str(), second.id.as_str(), third.id.as_str()]
    );
    assert_eq!(
        client
            .permissions()
            .list_at(Location::directory(other))
            .await
            .unwrap()
            .data[0]
            .id,
        outside.id
    );
    assert_eq!(
        fallback
            .permissions()
            .list_at(Location::Unspecified)
            .await
            .unwrap()
            .data[0]
            .id,
        outside.id
    );
    assert_eq!(fallback.permissions().list().await.unwrap().data.len(), 3);
    assert_eq!(p.list().await.unwrap().data.len(), 2);
    assert_permission_missing(sibling.get(&first.id).await.unwrap_err());
    assert_permission_missing(
        sibling
            .reply(
                &first.id,
                &SessionPermissionReplyRequest::new(PermissionReply::Reject),
            )
            .await
            .unwrap_err(),
    );
    assert_eq!(p.list().await.unwrap().data.len(), 2);
    p.reply(
        &first.id,
        &SessionPermissionReplyRequest::new(PermissionReply::Once).message(""),
    )
    .await
    .unwrap();
    assert_permission_missing(p.get(&first.id).await.unwrap_err());
    assert_permission_missing(
        p.reply(
            &first.id,
            &SessionPermissionReplyRequest::new(PermissionReply::Once),
        )
        .await
        .unwrap_err(),
    );
    assert_eq!(p.list().await.unwrap().data.len(), 1);
    let fourth = p
        .create(&SessionPermissionCreateRequest::new(
            "opencoders.live.ask",
            vec!["e".into()],
        ))
        .await
        .unwrap()
        .data;
    p.reply(
        &second.id,
        &SessionPermissionReplyRequest::new(PermissionReply::Reject).message("fixture feedback"),
    )
    .await
    .unwrap();
    assert!(p.list().await.unwrap().data.is_empty());
    assert_permission_missing(p.get(&fourth.id).await.unwrap_err());
    assert_eq!(sibling.list().await.unwrap().data.len(), 1);
    assert_eq!(remote.list().await.unwrap().data.len(), 1);
    let mut reply = SessionPermissionReplyRequest::new(PermissionReply::Reject);
    reply.message = RequestField::Null;
    sibling.reply(&third.id, &reply).await.unwrap();
    remote.reply(&outside.id, &reply).await.unwrap();

    // All requests are manually created fixtures; no running tool/model is waiting.
    // Saved allow rules only touch the runner-owned temporary database/project.
    let save = SessionPermissionCreateRequest::new("opencoders.live.saved", vec!["saved/a".into()])
        .save(vec!["saved/*".into()]);
    let saved = p.create(&save).await.unwrap().data;
    let matching = sibling
        .create(&SessionPermissionCreateRequest::new(
            "opencoders.live.saved",
            vec!["saved/b".into()],
        ))
        .await
        .unwrap()
        .data;
    let unmatched = sibling
        .create(&SessionPermissionCreateRequest::new(
            "opencoders.live.saved",
            vec!["elsewhere".into()],
        ))
        .await
        .unwrap()
        .data;
    p.reply(
        &saved.id,
        &SessionPermissionReplyRequest::new(PermissionReply::Always),
    )
    .await
    .unwrap();
    assert_permission_missing(sibling.get(&matching.id).await.unwrap_err());
    assert_eq!(sibling.list().await.unwrap().data[0].id, unmatched.id);
    assert_eq!(
        p.create(&save).await.unwrap().data.effect,
        PermissionEffect::Allow
    );
    sibling
        .reply(
            &unmatched.id,
            &SessionPermissionReplyRequest::new(PermissionReply::Reject),
        )
        .await
        .unwrap();
    for save in [None, Some(vec![])] {
        let mut req =
            SessionPermissionCreateRequest::new("opencoders.live.unsaved", vec!["a".into()]);
        req.save = save;
        let before = p.create(&req).await.unwrap().data;
        p.reply(
            &before.id,
            &SessionPermissionReplyRequest::new(PermissionReply::Always),
        )
        .await
        .unwrap();
        let after = p.create(&req).await.unwrap().data;
        assert_eq!(after.effect, PermissionEffect::Ask);
        p.reply(
            &after.id,
            &SessionPermissionReplyRequest::new(PermissionReply::Reject),
        )
        .await
        .unwrap();
    }
    // Configured denies still win over project-saved allow rules.
    client
        .sessions()
        .update(
            ids[0],
            &SessionUpdateRequest::default().permissions(vec![PermissionRuleInput::new(
                "opencoders.live.saved",
                PermissionEffect::Deny,
                "*",
            )]),
        )
        .await
        .unwrap();
    assert_eq!(
        p.create(&save).await.unwrap().data.effect,
        PermissionEffect::Deny
    );
    for id in ids {
        assert!(
            client
                .sessions()
                .permissions(id)
                .list()
                .await
                .unwrap()
                .data
                .is_empty()
        );
    }
    let missing: SessionId = "ses_permission_missing".parse().unwrap();
    let resource = client.sessions().permissions(&missing);
    for error in [
        resource.list().await.unwrap_err(),
        resource.get(&first.id).await.unwrap_err(),
        resource.create(&request).await.unwrap_err(),
        resource.reply(&first.id, &reply).await.unwrap_err(),
    ] {
        assert_eq!(error.status().unwrap(), 404);
        let Error::Http(error) = error else {
            panic!("missing session")
        };
        assert!(matches!(
            error.api_error(),
            Some(ApiError::SessionNotFound { .. })
        ));
    }
    let error = client
        .permissions()
        .list_at(Location::directory(
            "/opencoders-missing-permission-location",
        ))
        .await
        .unwrap_err();
    assert_eq!(error.status().unwrap(), 404);
    let Error::Http(error) = error else {
        panic!("missing location")
    };
    assert!(matches!(
        error.api_error(),
        Some(ApiError::LocationNotFound { .. })
    ));
    let mut saw_asked = false;
    let mut saw_replied = false;
    tokio::time::timeout(Duration::from_secs(5), async {
        while !saw_asked || !saw_replied {
            match events.next().await.unwrap().unwrap().event {
                opencoders::models::Event::PermissionAsked(e) if e.data.id == first.id => {
                    saw_asked = true
                }
                opencoders::models::Event::PermissionReplied(e)
                    if e.data.request_id == first.id =>
                {
                    assert_eq!(e.data.reply, PermissionReply::Once);
                    saw_replied = true;
                }
                _ => {}
            }
        }
    })
    .await
    .unwrap();
    events.close();
}
fn assert_permission_missing(error: Error) {
    assert_eq!(error.status().unwrap(), 404);
    let Error::Http(error) = error else {
        panic!("missing permission")
    };
    assert!(matches!(
        error.api_error(),
        Some(ApiError::PermissionNotFound { .. })
    ));
}

async fn verify_execution(client: &Client, id: &opencoders::models::SessionId) {
    use opencoders::models::{SessionInboxDelivery, SessionPromptRequest};
    use opencoders::{SessionInterruptOptions, SessionWaitOptions};
    let wait = SessionWaitOptions::default().timeout(Duration::from_secs(5));
    assert!(
        client
            .sessions()
            .inbox(id)
            .list()
            .await
            .unwrap()
            .data
            .is_empty()
    );
    for resume in [None, Some(false), Some(true)] {
        let mut options = SessionInterruptOptions::default();
        options.resume = resume;
        assert!(
            !client
                .sessions()
                .interrupt(id, &options)
                .await
                .unwrap()
                .interrupted
        );
        client
            .sessions()
            .experimental_wait(id, &wait)
            .await
            .unwrap();
    }
    // Waiting does not start work. resume(true) must not promote a queued prompt.
    // Never exercise resume(true) with steering/control input: that can call a model.
    let queued = client
        .sessions()
        .prompt(
            id,
            &SessionPromptRequest::new("held execution fixture")
                .id("msg_live_execution_queue".parse().unwrap())
                .delivery(SessionInboxDelivery::Queue)
                .resume(false),
        )
        .await
        .unwrap()
        .data;
    client
        .sessions()
        .experimental_wait(id, &wait)
        .await
        .unwrap();
    assert!(
        !client
            .sessions()
            .interrupt(id, &SessionInterruptOptions::default().resume(true))
            .await
            .unwrap()
            .interrupted
    );
    client
        .sessions()
        .experimental_wait(id, &wait)
        .await
        .unwrap();
    let pending = client.sessions().inbox(id).list().await.unwrap();
    assert_eq!(pending.data.len(), 1);
    assert_eq!(pending.data[0].id(), &queued.id);
    client
        .sessions()
        .inbox(id)
        .cancel(&queued.id)
        .await
        .unwrap();
    let steering = client
        .sessions()
        .prompt(
            id,
            &SessionPromptRequest::new("held steering fixture")
                .id("msg_live_execution_steer".parse().unwrap())
                .delivery(SessionInboxDelivery::Steer)
                .resume(false),
        )
        .await
        .unwrap()
        .data;
    for options in [
        SessionInterruptOptions::default(),
        SessionInterruptOptions::default().resume(false),
    ] {
        assert!(
            !client
                .sessions()
                .interrupt(id, &options)
                .await
                .unwrap()
                .interrupted
        );
        client
            .sessions()
            .experimental_wait(id, &wait)
            .await
            .unwrap();
        let pending = client.sessions().inbox(id).list().await.unwrap();
        assert_eq!(pending.data.len(), 1);
        assert_eq!(pending.data[0].id(), &steering.id);
    }
    assert!(
        client
            .sessions()
            .messages(id)
            .list(&MessageListOptions::default())
            .await
            .unwrap()
            .data
            .is_empty()
    );
    client
        .sessions()
        .inbox(id)
        .cancel(&steering.id)
        .await
        .unwrap();
    assert!(
        client
            .sessions()
            .inbox(id)
            .list()
            .await
            .unwrap()
            .data
            .is_empty()
    );
    let missing = "ses_live_execution_missing".parse().unwrap();
    for error in [
        client
            .sessions()
            .interrupt(&missing, &SessionInterruptOptions::default())
            .await
            .unwrap_err(),
        client
            .sessions()
            .experimental_wait(&missing, &wait)
            .await
            .unwrap_err(),
    ] {
        assert_eq!(error.status().unwrap(), 404);
        let Error::Http(error) = error else {
            panic!("expected missing session")
        };
        assert!(matches!(
            error.api_error(),
            Some(ApiError::SessionNotFound { .. })
        ));
    }
}

async fn verify_session_logs(
    client: &Client,
    id: &opencoders::models::SessionId,
    other: &opencoders::models::SessionId,
) {
    use opencoders::{
        EventOptions, SessionLogOptions, SessionLogStream,
        models::{Event, SessionLogItem},
    };
    async fn read(stream: &mut SessionLogStream) -> Option<opencoders::SessionLogMessage> {
        tokio::time::timeout(Duration::from_secs(5), stream.next())
            .await
            .expect("log deadline")
            .map(|v| v.expect("log item"))
    }
    async fn collect(
        client: &Client,
        id: &opencoders::models::SessionId,
        options: &SessionLogOptions,
    ) -> Vec<opencoders::SessionLogMessage> {
        let mut stream = client
            .sessions()
            .experimental_log(id, options)
            .await
            .unwrap();
        let mut items = Vec::new();
        while let Some(item) = read(&mut stream).await {
            items.push(item);
            assert!(items.len() < 100);
        }
        items
    }
    fn seed(events: &[serde_json::Value]) {
        use std::{
            io::Write,
            process::{Command, Stdio},
        };
        let mut child = Command::new("python3")
            .arg("tools/live/seed_log.py")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&serde_json::to_vec(events).unwrap())
            .unwrap();
        assert!(child.wait().unwrap().success());
    }
    let options = SessionLogOptions::default();
    // Stock CLI behavior: sequence counters exist, but event payloads are not retained.
    let initial = collect(client, id, &options).await;
    assert_eq!(initial.len(), 1);
    assert!(matches!(initial[0].item, SessionLogItem::Synced(_)));
    let initial_seq = initial[0].item.sequence().unwrap();
    assert!(initial_seq > 0);
    assert!(initial[0].id.is_none());
    let mut stock = client
        .sessions()
        .experimental_log(id, &options.clone().follow(true))
        .await
        .unwrap();
    assert!(matches!(
        read(&mut stock).await.unwrap().item,
        SessionLogItem::Synced(_)
    ));
    client
        .sessions()
        .update(
            id,
            &SessionUpdateRequest::default().title("Log stock retention check"),
        )
        .await
        .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(150), stock.next())
            .await
            .is_err()
    );
    stock.close();
    let advanced = collect(client, id, &options).await;
    assert_eq!(advanced.len(), 1);
    assert!(advanced[0].item.sequence().unwrap() > initial_seq);
    // Seed only real captured events into the owned DB; projections/counters stay untouched.
    let mut feed = client
        .events()
        .subscribe(&EventOptions::default().types(["session.renamed"]))
        .await
        .unwrap();
    assert!(matches!(
        feed.next().await.unwrap().unwrap().event,
        Event::ServerConnected(_)
    ));
    let mut recorded = Vec::new();
    for (session, title) in [
        (id, "Log replay first"),
        (other, "Log other aggregate"),
        (id, "Log replay second"),
    ] {
        client
            .sessions()
            .update(session, &SessionUpdateRequest::default().title(title))
            .await
            .unwrap();
        let message = tokio::time::timeout(Duration::from_secs(5), feed.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(message.event.durable_aggregate_id(), Some(session.as_str()));
        recorded.push(serde_json::to_value(message.event).unwrap());
    }
    seed(&recorded);
    let replay = collect(client, id, &options).await;
    assert_eq!(replay.len(), 3);
    let first = recorded[0]["durable"]["seq"].as_u64().unwrap();
    let second = recorded[2]["durable"]["seq"].as_u64().unwrap();
    assert_eq!(
        replay
            .iter()
            .map(|v| v.item.sequence().unwrap())
            .collect::<Vec<_>>(),
        [first, second, second]
    );
    assert!(matches!(replay[2].item, SessionLogItem::Synced(_)));
    // The Effect SSE encoder does not add IDs for these payloads.
    assert!(replay.iter().all(|v| v.id.is_none()));
    let after = collect(client, id, &options.clone().after(first).follow(false)).await;
    assert_eq!(after.len(), 2);
    assert_eq!(after[0].item.sequence(), Some(second));
    for after in [second, second + 100] {
        let items = collect(client, id, &options.clone().after(after)).await;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].item.sequence(), Some(second));
    }
    let elsewhere = collect(client, other, &options).await;
    assert_eq!(elsewhere.len(), 2);
    assert_eq!(elsewhere[0].item.aggregate_id(), Some(other.as_str()));
    let mut follow = client
        .sessions()
        .experimental_log(id, &options.clone().after(second).follow(true))
        .await
        .unwrap();
    assert!(matches!(
        read(&mut follow).await.unwrap().item,
        SessionLogItem::Synced(_)
    ));
    client
        .sessions()
        .update(
            id,
            &SessionUpdateRequest::default().title("Log retained follow fixture"),
        )
        .await
        .unwrap();
    let event = tokio::time::timeout(Duration::from_secs(5), feed.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
        .event;
    let seq = event.durable_sequence().unwrap();
    seed(&[serde_json::to_value(event).unwrap()]);
    // A later ordinary commit wakes the pinned DB-backed tail after fixture seeding.
    client
        .sessions()
        .update(
            id,
            &SessionUpdateRequest::default().title("Log follow wake"),
        )
        .await
        .unwrap();
    let item = read(&mut follow).await.unwrap();
    assert_eq!(item.item.sequence(), Some(seq));
    assert!(matches!(item.item, SessionLogItem::Event(_)));
    follow.close();
    drop(feed);
    let resumed = collect(client, id, &options.clone().after(seq)).await;
    assert_eq!(resumed.len(), 1);
    assert!(resumed[0].item.sequence().unwrap() > seq);
    let Error::Http(e) = client
        .sessions()
        .experimental_log(&"ses_missing_log".parse().unwrap(), &options)
        .await
        .unwrap_err()
    else {
        panic!("missing session")
    };
    assert_eq!(e.status(), 404);
    assert!(matches!(
        e.api_error(),
        Some(ApiError::SessionNotFound { .. })
    ));
    let error = client
        .sessions()
        .experimental_log(id, &options.after(u64::MAX))
        .await
        .unwrap_err();
    assert_eq!(error.status().unwrap(), 400);
}

async fn verify_events(
    client: &Client,
    id: &opencoders::models::SessionId,
    other: &opencoders::models::SessionId,
) {
    use opencoders::{
        EventOptions, EventStream,
        models::{Event, SessionPromptRequest},
    };
    async fn next(stream: &mut EventStream) -> opencoders::EventMessage {
        tokio::time::timeout(Duration::from_secs(5), stream.next())
            .await
            .expect("live event deadline")
            .expect("live feed closed")
            .expect("live event failed")
    }
    // The client has a default directory. Events still span server locations.
    let options = EventOptions::default().types([
        "session.renamed",
        "session.inbox.enqueued",
        "session.inbox.cancelled",
    ]);
    let mut first = client.events().subscribe(&options).await.unwrap();
    let mut second = client
        .events()
        .subscribe(&EventOptions::default().types(["session.renamed"]))
        .await
        .unwrap();
    for stream in [&mut first, &mut second] {
        let marker = next(stream).await;
        assert!(matches!(marker.event, Event::ServerConnected(_)));
        assert_eq!(marker.event_name, "message");
        assert!(marker.id.is_none());
    }
    for (session, title) in [(id, "SSE first location"), (other, "SSE second location")] {
        client
            .sessions()
            .update(session, &SessionUpdateRequest::default().title(title))
            .await
            .unwrap();
        for stream in [&mut first, &mut second] {
            let message = next(stream).await;
            let Event::SessionRenamed(event) = message.event else {
                panic!("expected rename")
            };
            assert_eq!(&event.data.session_id, session);
            assert_eq!(event.data.title, title);
            assert!(event.durable.seq.0.get() > 0);
        }
    }
    let item = client
        .sessions()
        .prompt(
            id,
            &SessionPromptRequest::new("SSE pending fixture").resume(false),
        )
        .await
        .unwrap()
        .data;
    let Event::SessionInboxEnqueued(event) = next(&mut first).await.event else {
        panic!("expected enqueue")
    };
    assert_eq!(&event.data.session_id, id);
    assert_eq!(event.data.inbox_id, item.id);
    client.sessions().inbox(id).cancel(&item.id).await.unwrap();
    assert!(matches!(
        next(&mut first).await.event,
        Event::SessionInboxCancelled(_)
    ));
    drop(first);
    client
        .sessions()
        .update(
            other,
            &SessionUpdateRequest::default().title("SSE surviving subscription"),
        )
        .await
        .unwrap();
    // A filtered subscription must skip the preceding inbox events.
    let Event::SessionRenamed(event) = next(&mut second).await.event else {
        panic!("expected filtered rename")
    };
    assert_eq!(event.data.title, "SSE surviving subscription");
    second.close();
    assert!(second.next().await.is_none());
    assert!(
        client
            .sessions()
            .inbox(id)
            .list()
            .await
            .unwrap()
            .data
            .is_empty()
    );
}

async fn verify_prompt_inbox(
    client: &Client,
    id: &opencoders::models::SessionId,
    other: &opencoders::models::SessionId,
) {
    use opencoders::models::{
        PromptInputAgentAttachment, PromptInputFileAttachment, PromptInputSkillAttachment,
        SessionInboxDelivery, SessionInboxInfo, SessionInboxUpdateRequest, SessionPromptRequest,
    };
    let inbox = client.sessions().inbox(id);
    assert!(inbox.list().await.unwrap().data.is_empty());
    // Every successful prompt explicitly suppresses waking execution. Delivery
    // mode alone does not prevent execution, so never rely on queue for isolation.
    let request = SessionPromptRequest::new("held prompt")
        .id("msg_live_admission_b".parse().unwrap())
        .delivery(SessionInboxDelivery::Queue)
        .resume(false)
        .files(vec![PromptInputFileAttachment::new(
            "data:text/plain;base64,aGk=",
        )])
        .agents(vec![PromptInputAgentAttachment::new("build")])
        .skills(vec![])
        .metadata(serde_json::from_value(json!({"fixture":{"nested":true}})).unwrap());
    let admitted = client.sessions().prompt(id, &request).await.unwrap().data;
    assert_eq!(admitted.id.as_str(), "msg_live_admission_b");
    assert_eq!(&admitted.session_id, id);
    assert_eq!(admitted.delivery, SessionInboxDelivery::Queue);
    assert_eq!(admitted.payload.text, "held prompt");
    assert_eq!(admitted.payload.files[0].data.as_str(), "aGk=");
    assert_eq!(admitted.payload.files[0].mime, "text/plain");
    assert_eq!(admitted.payload.agents[0].name, "build");
    assert_eq!(admitted.payload.metadata["fixture"], json!({"nested":true}));
    let duplicate = client
        .sessions()
        .prompt(
            id,
            &SessionPromptRequest::new("changed text")
                .id(admitted.id.clone())
                .delivery(SessionInboxDelivery::Steer)
                .resume(false),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(
        serde_json::to_value(&duplicate).unwrap(),
        serde_json::to_value(&admitted).unwrap()
    );
    assert_eq!(inbox.list().await.unwrap().data.len(), 1);
    assert!(
        client
            .sessions()
            .messages(id)
            .list(&MessageListOptions::default())
            .await
            .unwrap()
            .data
            .is_empty()
    );

    let conflict = client.sessions().prompt(other, &request).await.unwrap_err();
    assert_inbox_conflict(conflict);
    // Wrong-session cancellation is a no-op and must not remove the real item.
    client
        .sessions()
        .inbox(other)
        .cancel(&admitted.id)
        .await
        .unwrap();
    assert_eq!(inbox.list().await.unwrap().data.len(), 1);
    assert_inbox_conflict(
        client
            .sessions()
            .inbox(other)
            .update(
                &admitted.id,
                &SessionInboxUpdateRequest::new(SessionInboxDelivery::Queue),
            )
            .await
            .unwrap_err(),
    );

    // ID order deliberately differs from enqueue order. Omitted/null delivery
    // resolves to steer, but resume(false) still prevents waking an idle loop.
    let omitted = client
        .sessions()
        .prompt(
            id,
            &SessionPromptRequest::new("second")
                .id("msg_live_admission_a".parse().unwrap())
                .resume(false),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(omitted.delivery, SessionInboxDelivery::Steer);
    let mut null_delivery = SessionPromptRequest::new("").resume(false);
    null_delivery.id = RequestField::Null;
    null_delivery.delivery = RequestField::Null;
    let nulls = client
        .sessions()
        .prompt(id, &null_delivery)
        .await
        .unwrap()
        .data;
    assert_eq!(nulls.delivery, SessionInboxDelivery::Steer);
    assert_eq!(nulls.payload.text, "");
    let listed = inbox.list().await.unwrap();
    assert_eq!(
        listed.data.iter().map(|item| item.id()).collect::<Vec<_>>(),
        [&admitted.id, &omitted.id, &nulls.id]
    );
    assert!(
        listed
            .data
            .iter()
            .all(|item| matches!(item, SessionInboxInfo::User(_)))
    );
    let queued = SessionInboxUpdateRequest::new(SessionInboxDelivery::Queue);
    inbox.update(&omitted.id, &queued).await.unwrap();
    assert_inbox_conflict(inbox.update(&omitted.id, &queued).await.unwrap_err());
    let listed = inbox.list().await.unwrap();
    assert_eq!(listed.data[1].delivery(), SessionInboxDelivery::Queue);
    assert_eq!(listed.data[1].id(), &omitted.id);
    // Unavailable updates conflict; unavailable cancellations succeed.
    let missing = "msg_live_missing".parse().unwrap();
    assert_inbox_conflict(inbox.update(&missing, &queued).await.unwrap_err());
    inbox.cancel(&missing).await.unwrap();
    for pending in [&admitted.id, &omitted.id, &nulls.id] {
        inbox.cancel(pending).await.unwrap();
        inbox.cancel(pending).await.unwrap();
        assert_inbox_conflict(inbox.update(pending, &queued).await.unwrap_err());
    }
    assert!(inbox.list().await.unwrap().data.is_empty());
    assert!(
        client
            .sessions()
            .messages(id)
            .list(&MessageListOptions::default())
            .await
            .unwrap()
            .data
            .is_empty()
    );

    // Preparation failures do not admit an item. No real files, URLs, or skills
    // are loaded: one unsupported URI and one deliberately missing skill ID.
    for bad in [
        SessionPromptRequest::new("bad file")
            .resume(false)
            .files(vec![PromptInputFileAttachment::new("unsupported:fixture")]),
        SessionPromptRequest::new("bad skill")
            .resume(false)
            .skills(vec![PromptInputSkillAttachment::new(
                "opencoders-missing-skill".to_owned().into(),
            )]),
    ] {
        let Error::Http(error) = client.sessions().prompt(id, &bad).await.unwrap_err() else {
            panic!("expected preparation error")
        };
        assert_eq!(error.status(), 400);
        assert!(matches!(
            error.api_error(),
            Some(ApiError::InvalidRequest { .. })
        ));
    }
    assert!(inbox.list().await.unwrap().data.is_empty());
    let missing_session = "ses_live_missing".parse().unwrap();
    for result in [
        client
            .sessions()
            .prompt(
                &missing_session,
                &SessionPromptRequest::new("no session").resume(false),
            )
            .await
            .map(|_| ()),
        client
            .sessions()
            .inbox(&missing_session)
            .list()
            .await
            .map(|_| ()),
        client
            .sessions()
            .inbox(&missing_session)
            .update(&missing, &queued)
            .await,
        client
            .sessions()
            .inbox(&missing_session)
            .cancel(&missing)
            .await,
    ] {
        let Error::Http(error) = result.unwrap_err() else {
            panic!("expected missing session")
        };
        assert_eq!(error.status(), 404);
        assert!(matches!(
            error.api_error(),
            Some(ApiError::SessionNotFound { .. })
        ));
    }
}

fn assert_inbox_conflict(error: Error) {
    let Error::Http(error) = error else {
        panic!("expected HTTP conflict")
    };
    assert_eq!(error.status(), 409);
    assert!(matches!(error.api_error(), Some(ApiError::Conflict { .. })));
}

async fn verify_selection(
    client: &Client,
    fallback: &Client,
    id: &opencoders::models::SessionId,
    workspace: &str,
    other: &str,
) {
    use opencoders::{
        ModelListOptions,
        models::{SessionModelInput, SessionSwitchAgentRequest, SessionSwitchModelRequest},
    };
    let models = client
        .models()
        .list(&ModelListOptions::default())
        .await
        .unwrap();
    assert_eq!(models.location.directory.as_str(), workspace);
    let default = client.models().default().await.unwrap();
    assert_eq!(default.location.directory.as_str(), workspace);
    let chat = client
        .models()
        .list(&ModelListOptions::default().types(["chat", "chat"]))
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(&chat.data).unwrap(),
        serde_json::to_value(&models.data).unwrap()
    );
    let images = client
        .models()
        .list(&ModelListOptions::default().types(["image"]))
        .await
        .unwrap();
    assert!(images.data.iter().all(|m| {
        m.type_
            .as_ref()
            .is_some_and(|kind| kind.as_str() == "image")
    }));
    let combined = client
        .models()
        .list(&ModelListOptions::default().types(["chat", "image"]))
        .await
        .unwrap();
    let expected = chat
        .data
        .iter()
        .chain(images.data.iter())
        .map(|m| (&m.provider_id, &m.id))
        .collect::<Vec<_>>();
    assert_eq!(
        combined
            .data
            .iter()
            .map(|m| (&m.provider_id, &m.id))
            .collect::<Vec<_>>(),
        expected
    );
    assert!(
        client
            .models()
            .list(&ModelListOptions::default().types(["opencoders-unknown-type"]))
            .await
            .unwrap()
            .data
            .is_empty()
    );
    assert_eq!(
        client
            .models()
            .list_at(&ModelListOptions::default(), Location::directory(other))
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        other
    );
    assert_eq!(
        fallback
            .models()
            .default_at(Location::Unspecified)
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        other
    );
    assert_eq!(
        fallback
            .models()
            .default()
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        workspace
    );
    let agents = client.agents().list().await.unwrap();
    let agent = &agents.data[0];
    let got = client.agents().get(&agent.id).await.unwrap();
    assert_eq!(got.location.directory.as_str(), workspace);
    assert_eq!(
        serde_json::to_value(&got.data).unwrap(),
        serde_json::to_value(agent).unwrap()
    );
    assert_eq!(
        fallback
            .agents()
            .get_at(&agent.id, Location::Unspecified)
            .await
            .unwrap()
            .location
            .directory
            .as_str(),
        other
    );
    let unknown = "opencoders-nonexistent-agent".to_owned().into();
    let Error::Http(error) = client.agents().get(&unknown).await.unwrap_err() else {
        panic!("expected missing agent")
    };
    assert!(matches!(
        error.api_error(),
        Some(ApiError::AgentNotFound { .. })
    ));
    assert_eq!(error.status(), 404);

    let choice = SessionSwitchAgentRequest::new(agent.id.clone());
    client.sessions().switch_agent(id, &choice).await.unwrap();
    client.sessions().switch_agent(id, &choice).await.unwrap();
    let model = default
        .data
        .as_ref()
        .map(|m| SessionModelInput::new(m.provider_id.clone(), &m.id))
        .unwrap_or_else(|| SessionModelInput::new("fixture".to_owned().into(), "fixture"));
    let selected = SessionSwitchModelRequest::new(model.clone());
    client.sessions().switch_model(id, &selected).await.unwrap();
    let normalized = serde_json::to_value(model.clone().variant("default")).unwrap();
    assert_eq!(
        serde_json::to_value(client.sessions().get(id).await.unwrap().data.model.unwrap()).unwrap(),
        normalized
    );
    client
        .sessions()
        .switch_model(
            id,
            &SessionSwitchModelRequest::new(model.clone().variant("default")),
        )
        .await
        .unwrap();
    let before_variant = client.sessions().get(id).await.unwrap().data;
    assert_eq!(before_variant.agent.as_ref().unwrap(), &agent.id);
    assert_eq!(
        serde_json::to_value(before_variant.model.as_ref().unwrap()).unwrap(),
        normalized
    );
    let changed = SessionSwitchModelRequest::new(model.variant("fixture-variant"));
    client.sessions().switch_model(id, &changed).await.unwrap();
    let after = client.sessions().get(id).await.unwrap().data;
    assert_eq!(
        serde_json::to_value(after.model.unwrap()).unwrap(),
        serde_json::to_value(&changed.model).unwrap()
    );
    let history = client
        .sessions()
        .messages(id)
        .list(&MessageListOptions::default())
        .await
        .unwrap();
    assert_eq!(
        history
            .data
            .iter()
            .filter(|m| m.message_type() == SessionMessageType::AgentSwitched)
            .count(),
        2
    );
    assert_eq!(
        history
            .data
            .iter()
            .filter(|m| m.message_type() == SessionMessageType::ModelSwitched)
            .count(),
        2
    );
    // Selection is stored without catalog validation; it does not invoke a model.
    client
        .sessions()
        .switch_agent(id, &SessionSwitchAgentRequest::new(unknown.clone()))
        .await
        .unwrap();
    let unknown_model = SessionSwitchModelRequest::new(SessionModelInput::new(
        "opencoders-nonexistent-provider".to_owned().into(),
        "unknown-model",
    ));
    client
        .sessions()
        .switch_model(id, &unknown_model)
        .await
        .unwrap();
    let stored = client.sessions().get(id).await.unwrap().data;
    assert_eq!(stored.agent.unwrap(), unknown);
    assert_eq!(
        serde_json::to_value(stored.model.unwrap()).unwrap(),
        serde_json::to_value(unknown_model.model.variant("default")).unwrap()
    );
    for result in [
        client
            .sessions()
            .switch_agent(&"ses_missing".parse().unwrap(), &choice)
            .await,
        client
            .sessions()
            .switch_model(&"ses_missing".parse().unwrap(), &selected)
            .await,
    ] {
        let Error::Http(error) = result.unwrap_err() else {
            panic!("expected missing session")
        };
        assert_eq!(error.status(), 404);
        assert!(matches!(
            error.api_error(),
            Some(ApiError::SessionNotFound { .. })
        ));
    }
}

async fn verify_message_history(
    client: &Client,
    http: &reqwest::Client,
    url: &str,
    template: &SessionInfo,
) {
    let empty = client
        .sessions()
        .messages(&template.id)
        .list(&MessageListOptions::default())
        .await
        .unwrap();
    assert!(empty.data.is_empty());
    assert!(empty.cursor.next.is_none());
    let missing_message = "msg_missing".parse().unwrap();
    let Error::Http(error) = client
        .sessions()
        .messages(&template.id)
        .get(&missing_message)
        .await
        .unwrap_err()
    else {
        panic!("expected missing message")
    };
    assert!(matches!(
        error.api_error(),
        Some(ApiError::MessageNotFound { .. })
    ));
    assert_eq!(error.status(), 404);
    let missing_session = "ses_missing".parse().unwrap();
    for result in [
        client
            .sessions()
            .messages(&missing_session)
            .list(&MessageListOptions::default())
            .await
            .map(|_| ()),
        client
            .sessions()
            .messages(&missing_session)
            .get(&missing_message)
            .await
            .map(|_| ()),
    ] {
        let Error::Http(error) = result.unwrap_err() else {
            panic!("expected missing session")
        };
        assert!(matches!(
            error.api_error(),
            Some(ApiError::SessionNotFound { .. })
        ));
    }

    // Import only into this runner-owned disposable server. The pinned import
    // handler stores projected messages directly; it does not schedule execution.
    // These model/tool outputs are fixtures, never actual model or tool calls.
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/messages.json")).unwrap();
    let mut info = serde_json::to_value(template).unwrap();
    info["id"] = json!("ses_live_history");
    let response=http.post(format!("{url}/api/experimental/session/import"))
        .header(header::CONTENT_TYPE,"application/json")
        .body(serde_json::to_vec(&json!({"info":info,"messages":fixtures,"location":{"directory":template.location.directory}})).unwrap())
        .send().await.unwrap();
    assert_eq!(response.status(), 200, "disposable history import failed");
    let id = "ses_live_history".parse().unwrap();
    let messages = client.sessions().messages(&id);
    let all = messages.list(&MessageListOptions::default()).await.unwrap();
    assert_eq!(all.data.len(), 7);
    assert_eq!(all.data[0].id().as_str(), "msg_live_07");
    let expected: Vec<SessionMessageInfo> = serde_json::from_value(fixtures).unwrap();
    for message in &expected {
        let got = messages.get(message.id()).await.unwrap().data;
        assert_eq!(
            serde_json::to_value(got).unwrap(),
            serde_json::to_value(message).unwrap()
        );
    }
    let foreign = client
        .sessions()
        .messages(&template.id)
        .get(expected[0].id())
        .await
        .unwrap_err();
    let Error::Http(error) = foreign else {
        panic!("expected wrong-session message error")
    };
    assert!(matches!(
        error.api_error(),
        Some(ApiError::MessageNotFound { .. })
    ));

    for order in [MessageOrder::Asc, MessageOrder::Desc] {
        let mut options = MessageListOptions::default()
            .limit(2)
            .order(order)
            .message_type(SessionMessageType::User);
        let first = messages.list(&options).await.unwrap();
        let first_ids: Vec<_> = first
            .data
            .iter()
            .map(|m| m.id().as_str().to_owned())
            .collect();
        assert_eq!(
            first_ids,
            if order == MessageOrder::Asc {
                vec!["msg_live_01", "msg_live_03"]
            } else {
                vec!["msg_live_05", "msg_live_03"]
            }
        );
        options.cursor = first.cursor.next;
        options.order = None;
        let second = messages.list(&options).await.unwrap();
        assert_eq!(second.data.len(), 1);
        assert_eq!(
            second.data[0].id().as_str(),
            if order == MessageOrder::Asc {
                "msg_live_05"
            } else {
                "msg_live_01"
            }
        );
        let back = messages
            .list(
                &options
                    .clone()
                    .cursor(second.cursor.previous.clone().unwrap()),
            )
            .await
            .unwrap();
        assert_eq!(
            back.data
                .iter()
                .map(|m| m.id().as_str().to_owned())
                .collect::<Vec<_>>(),
            first_ids
        );
        // Cursors mark boundaries even when fewer than limit items were returned.
        options.cursor = second.cursor.next;
        let terminal = messages.list(&options).await.unwrap();
        assert!(terminal.data.is_empty());
        assert!(terminal.cursor.next.is_none());
    }
    let first = messages
        .list(&MessageListOptions::default().limit(1))
        .await
        .unwrap();
    for options in [
        MessageListOptions::default().cursor("invalid"),
        MessageListOptions::default()
            .cursor(first.cursor.next.clone().unwrap())
            .order(MessageOrder::Asc),
    ] {
        let Error::Http(error) = messages.list(&options).await.unwrap_err() else {
            panic!("expected invalid cursor")
        };
        assert_eq!(error.status(), 400);
        assert!(matches!(
            error.api_error(),
            Some(ApiError::InvalidCursor { .. })
        ));
    }
    let Error::Http(error) = messages
        .list(&MessageListOptions::default().limit(0))
        .await
        .unwrap_err()
    else {
        panic!("expected invalid limit")
    };
    assert_eq!(error.status(), 400);
    assert!(matches!(
        error.api_error(),
        Some(ApiError::InvalidRequest { .. })
    ));
    // The type filter is separate from the cursor, unlike session-list filters.
    let filtered = messages
        .list(
            &MessageListOptions::default()
                .limit(1)
                .order(MessageOrder::Asc)
                .message_type(SessionMessageType::User),
        )
        .await
        .unwrap();
    let unfiltered = messages
        .list(
            &MessageListOptions::default()
                .limit(2)
                .cursor(filtered.cursor.next.unwrap()),
        )
        .await
        .unwrap();
    assert_eq!(
        unfiltered.data[0].message_type(),
        SessionMessageType::Assistant
    );
    let no_match = messages
        .list(&MessageListOptions::default().message_type(SessionMessageType::Compaction))
        .await
        .unwrap();
    assert!(no_match.data.is_empty());
}
