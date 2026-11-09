//! Only called after the runner verifies server ownership and creates its Git fixture.
use opencoders::{ApiError, Client, Error, Location, SavedPermissionListOptions, models::*};
use std::{path::Path, time::Duration};
const ACTION: &str = "opencoders.saved.fixture";
async fn save(client: &Client, id: &SessionId, resource: &str) {
    let permissions = client.sessions().permissions(id);
    let pending = permissions
        .create(
            &SessionPermissionCreateRequest::new(ACTION, vec![resource.into()])
                .save(vec![resource.into()]),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(pending.effect, PermissionEffect::Ask);
    permissions
        .reply(
            &pending.id,
            &SessionPermissionReplyRequest::new(PermissionReply::Always),
        )
        .await
        .unwrap();
}
async fn evaluate(client: &Client, id: &SessionId, resource: &str, expected: PermissionEffect) {
    let p = client.sessions().permissions(id);
    let response = p
        .create(&SessionPermissionCreateRequest::new(
            ACTION,
            vec![resource.into()],
        ))
        .await
        .unwrap()
        .data;
    assert_eq!(response.effect, expected);
    if expected == PermissionEffect::Ask {
        p.reply(
            &response.id,
            &SessionPermissionReplyRequest::new(PermissionReply::Once),
        )
        .await
        .unwrap();
    }
}
pub async fn verify(client: &Client, root: &Path) {
    let mut sessions = Vec::new();
    for path in [
        root.join("lifecycle-repo"),
        root.join("workspace + unicode 雪"),
    ] {
        sessions.push(
            client
                .sessions()
                .create(
                    &SessionCreateRequest::default()
                        .location(SessionLocationInput::new(path.to_str().unwrap()))
                        .permissions(vec![PermissionRuleInput::new(
                            "opencoders.saved.*",
                            PermissionEffect::Ask,
                            "*",
                        )]),
                )
                .await
                .unwrap()
                .data,
        );
    }
    let (a, b) = (&sessions[0], &sessions[1]);
    let project_a = client
        .location()
        .get_at(Location::directory(a.location.directory.as_str()))
        .await
        .unwrap()
        .project
        .id;
    let project_b = client
        .location()
        .get_at(Location::directory(b.location.directory.as_str()))
        .await
        .unwrap()
        .project
        .id;
    assert_ne!(project_a, project_b);
    save(client, &a.id, "alpha").await;
    save(client, &a.id, "beta").await;
    save(client, &b.id, "alpha").await;
    let resource = client.permissions().saved();
    let filter = SavedPermissionListOptions::default().project(project_a.clone());
    let saved = resource.list(&filter).await.unwrap().data;
    assert_eq!(saved.len(), 2);
    assert!(saved.iter().all(|r| r.project_id == project_a));
    let alpha = saved.iter().find(|r| r.resource == "alpha").unwrap();
    let beta = saved.iter().find(|r| r.resource == "beta").unwrap();
    let all = resource
        .list(&SavedPermissionListOptions::default().project("".to_owned().into()))
        .await
        .unwrap()
        .data;
    assert!(all.iter().any(|r| r.project_id == project_a));
    assert!(all.iter().any(|r| r.project_id == project_b));
    let fallback = resource
        .list(&SavedPermissionListOptions::default())
        .await
        .unwrap()
        .data;
    assert!(fallback.iter().all(|r| r.project_id == project_b));
    assert!(
        resource
            .list(
                &SavedPermissionListOptions::default().project("missing-project".to_owned().into())
            )
            .await
            .unwrap()
            .data
            .is_empty()
    );
    // A caller header determines fallback; the SDK's conflicting default location
    // is ignored because this pinned route exposes only projectID as a query.
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "x-opencode-directory",
        a.location.directory.as_str().parse().unwrap(),
    );
    let by_header = Client::builder(client.base_url().as_str())
        .http_client(
            reqwest::Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        )
        .basic_auth(
            "opencode",
            std::env::var("OPENCODERS_LIVE_PASSWORD").unwrap(),
        )
        .default_headers(headers)
        .default_location(Location::directory(b.location.directory.as_str()))
        .build()
        .unwrap();
    assert_eq!(
        by_header
            .permissions()
            .saved()
            .list(&SavedPermissionListOptions::default())
            .await
            .unwrap()
            .data
            .len(),
        2
    );
    let unchanged = resource
        .update(&alpha.id, &PermissionSavedUpdateRequest::default())
        .await
        .unwrap()
        .data;
    assert_eq!(unchanged.resource, "alpha");
    let nulls: PermissionSavedUpdateRequest =
        serde_json::from_value(serde_json::json!({"action":null,"resource":null})).unwrap();
    let unchanged = resource.update(&alpha.id, &nulls).await.unwrap().data;
    assert_eq!(unchanged.action, ACTION);
    assert_eq!(unchanged.resource, "alpha");
    let Error::Http(conflict) = resource
        .update(
            &alpha.id,
            &PermissionSavedUpdateRequest::default().resource("beta"),
        )
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(conflict.status(), 409);
    assert!(
        matches!(conflict.api_error(),Some(ApiError::Conflict{resource:Some(id),..}) if id==alpha.id.as_str())
    );
    let changed = resource
        .update(
            &alpha.id,
            &PermissionSavedUpdateRequest::default().resource("gamma"),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(changed.id, alpha.id);
    assert_eq!(changed.project_id, project_a);
    assert_eq!(changed.resource, "gamma");
    evaluate(client, &a.id, "alpha", PermissionEffect::Ask).await;
    evaluate(client, &a.id, "gamma", PermissionEffect::Allow).await;
    evaluate(client, &b.id, "alpha", PermissionEffect::Allow).await;
    // Editing does not settle requests already pending, though new checks see it.
    let pending = client
        .sessions()
        .permissions(&a.id)
        .create(&SessionPermissionCreateRequest::new(
            ACTION,
            vec!["later".into()],
        ))
        .await
        .unwrap()
        .data;
    resource
        .update(
            &alpha.id,
            &PermissionSavedUpdateRequest::default().resource("later"),
        )
        .await
        .unwrap();
    assert_eq!(
        client
            .sessions()
            .permissions(&a.id)
            .get(&pending.id)
            .await
            .unwrap()
            .data
            .id,
        pending.id
    );
    evaluate(client, &a.id, "later", PermissionEffect::Allow).await;
    client
        .sessions()
        .permissions(&a.id)
        .reply(
            &pending.id,
            &SessionPermissionReplyRequest::new(PermissionReply::Once),
        )
        .await
        .unwrap();
    let empty = resource
        .update(
            &alpha.id,
            &PermissionSavedUpdateRequest::default()
                .action("")
                .resource(""),
        )
        .await
        .unwrap()
        .data;
    assert_eq!((empty.action.as_str(), empty.resource.as_str()), ("", ""));
    resource.remove(&alpha.id).await.unwrap();
    resource.remove(&alpha.id).await.unwrap();
    evaluate(client, &a.id, "later", PermissionEffect::Ask).await;
    let Error::Http(missing) = resource
        .update(&alpha.id, &PermissionSavedUpdateRequest::default())
        .await
        .unwrap_err()
    else {
        panic!("HTTP error")
    };
    assert_eq!(missing.status(), 404);
    assert!(
        matches!(missing.api_error(),Some(ApiError::PermissionSavedNotFound{id,..}) if id==alpha.id.as_str())
    );
    // Configured denies still override saved allows after edits.
    client
        .sessions()
        .update(
            &a.id,
            &SessionUpdateRequest::default().permissions(vec![PermissionRuleInput::new(
                ACTION,
                PermissionEffect::Deny,
                "*",
            )]),
        )
        .await
        .unwrap();
    resource
        .update(
            &beta.id,
            &PermissionSavedUpdateRequest::default().resource("denied"),
        )
        .await
        .unwrap();
    evaluate(client, &a.id, "denied", PermissionEffect::Deny).await;
    resource.remove(&beta.id).await.unwrap();
    assert!(resource.list(&filter).await.unwrap().data.is_empty());
    let b_rules = resource
        .list(&SavedPermissionListOptions::default().project(project_b.clone()))
        .await
        .unwrap()
        .data;
    for rule in b_rules.iter().filter(|r| r.action == ACTION) {
        resource.remove(&rule.id).await.unwrap();
    }
    for session in sessions {
        client.sessions().remove(&session.id).await.unwrap();
    }
}
