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
