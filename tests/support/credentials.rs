//! Runs only after the disposable smoke harness has checked server/data ownership.
use opencoders::{ApiError, Client, Error, Location, RequestField, models::*};
use serde_json::json;

fn invalid(error: Error, kind: &str) {
    let Error::Http(error) = error else {
        panic!("expected HTTP failure")
    };
    assert_eq!(error.status(), 400);
    assert!(
        matches!(error.api_error(),Some(ApiError::InvalidRequest{kind:Some(actual),..}) if actual==kind)
    );
}
async fn selected(client: &Client, id: &CredentialId) {
    let rows = client.credentials().list().await.unwrap().data;
    assert_eq!(rows.iter().filter(|r| r.active).count(), 1);
    assert!(rows.iter().any(|r| &r.id == id && r.active));
}
pub(super) async fn verify(client: &Client, fallback: &Client, other: &str) {
    println!("smoke scenario: global credentials and direct integration connections");
    // The runner uses isolated storage and strips all inherited provider credentials.
    assert!(client.credentials().list().await.unwrap().data.is_empty());
    let storage: IntegrationId = "opencoders-owned-storage".parse().unwrap();
    let first = CredentialCreateRequest::new(
        storage.clone(),
        CredentialValueInput::key("owned-dummy-key-a"),
    )
    .id("opencoders-credential-a / ? 雪".parse().unwrap())
    .activate(false);
    let a = client.credentials().create(&first).await.unwrap().data;
    assert!(a.active);
    assert_eq!(a.label, "default");
    // Global storage does not require this integration to be registered locally.
    assert!(
        !client
            .integrations()
            .list()
            .await
            .unwrap()
            .data
            .iter()
            .any(|i| i.id == storage)
    );
    let b = client
        .credentials()
        .create(
            &CredentialCreateRequest::new(
                storage.clone(),
                CredentialValueInput::key("owned-dummy-key-b"),
            )
            .id("opencoders-credential-b".parse().unwrap())
            .activate(false),
        )
        .await
        .unwrap()
        .data;
    assert!(!b.active);
    selected(client, &a.id).await;
    let oauth = CredentialValueInput::oauth(
        "unused-oauth-method".parse().unwrap(),
        "owned-dummy-refresh",
        "owned-dummy-access",
        9007199254740991,
    )
    .unwrap();
    let c = client
        .credentials()
        .create(
            &CredentialCreateRequest::new(storage.clone(), oauth)
                .id("opencoders-credential-c".parse().unwrap()),
        )
        .await
        .unwrap()
        .data;
    selected(client, &c.id).await;
    let d = client
        .credentials()
        .create(
            &CredentialCreateRequest::new(
                storage,
                CredentialValueInput::external("unused-external-method".parse().unwrap())
                    .metadata(serde_json::Map::from_iter([("owned".into(), json!(true))])),
            )
            .id("opencoders-credential-d".parse().unwrap())
            .activate(false),
        )
        .await
        .unwrap()
        .data;
    assert!(!d.active);
    let rows = fallback.credentials().list().await.unwrap().data;
    assert_eq!(rows.len(), 4);
    assert_eq!(rows.last().unwrap().id, c.id);
    let CredentialValue::Oauth(oauth) = &rows.iter().find(|r| r.id == c.id).unwrap().value else {
        panic!("oauth")
    };
    assert_eq!(oauth.expires.get(), 9007199254740991);
    assert!(oauth.access == "owned-dummy-access" && oauth.refresh == "owned-dummy-refresh");
    let Error::Http(conflict) = client.credentials().create(&first).await.unwrap_err() else {
        panic!("conflict")
    };
    assert_eq!(conflict.status(), 409);
    assert!(matches!(
        conflict.api_error(),
        Some(ApiError::Conflict { .. })
    ));
    client
        .credentials()
        .update(&a.id, &CredentialUpdateRequest::new(""))
        .await
        .unwrap();
    assert!(
        client
            .credentials()
            .list()
            .await
            .unwrap()
            .data
            .iter()
            .find(|r| r.id == a.id)
            .unwrap()
            .label
            .is_empty()
    );
    client.credentials().activate(&a.id).await.unwrap();
    client.credentials().activate(&a.id).await.unwrap();
    selected(client, &a.id).await;
    client.credentials().remove(&a.id).await.unwrap();
    selected(client, &d.id).await;
    client.credentials().remove(&b.id).await.unwrap();
    selected(client, &d.id).await;
    client.credentials().remove(&d.id).await.unwrap();
    selected(client, &c.id).await;
    client.credentials().remove(&c.id).await.unwrap();
    let missing = "opencoders-credential-missing".parse().unwrap();
    client
        .credentials()
        .update(&missing, &CredentialUpdateRequest::new("unused"))
        .await
        .unwrap();
    client.credentials().activate(&missing).await.unwrap();
    client.credentials().remove(&missing).await.unwrap();
    assert!(client.credentials().list().await.unwrap().data.is_empty());
    let denied: IntegrationId = "opencoders-discovery-denied".parse().unwrap();
    invalid(
        client
            .credentials()
            .create(&CredentialCreateRequest::new(
                denied.clone(),
                CredentialValueInput::key("owned-dummy-denied"),
            ))
            .await
            .unwrap_err(),
        "integration_policy",
    );

    let integration: IntegrationId = "opencoders-discovery".parse().unwrap();
    let method: IntegrationMethodId = "fixture-external".parse().unwrap();
    client
        .integrations()
        .connect_key(
            &integration,
            &IntegrationConnectKeyRequest::new("owned-dummy-key-form")
                .answer(serde_json::from_value(json!({"fixture":"key-data"})).unwrap())
                .label("key form"),
        )
        .await
        .unwrap();
    client
        .integrations()
        .connect_external(
            &integration,
            &IntegrationConnectExternalRequest::new(method.clone())
                .answer(serde_json::from_value(json!({"fixture":"external-data"})).unwrap()),
        )
        .await
        .unwrap();
    let mut nulls = IntegrationConnectKeyRequest::new("owned-dummy-key-null");
    nulls.answer = RequestField::Null;
    nulls.label = RequestField::Null;
    fallback
        .integrations()
        .connect_key_at(&integration, &nulls, Location::directory(other))
        .await
        .unwrap();
    fallback
        .integrations()
        .connect_external_at(
            &integration,
            &IntegrationConnectExternalRequest::new(method).label(""),
            Location::directory(other),
        )
        .await
        .unwrap();
    let rows = client.credentials().list().await.unwrap().data;
    assert_eq!(rows.len(), 4);
    assert!(rows.iter().all(|r| r.integration_id == integration));
    assert_eq!(rows.iter().filter(|r| r.active).count(), 1);
    let key = &rows.iter().find(|r| r.label == "key form").unwrap().value;
    let CredentialValue::Key(key) = key else {
        panic!("key")
    };
    assert!(key.key == "owned-dummy-key-form");
    assert_eq!(key.metadata.as_ref().unwrap()["fixture"], "key-data");
    let ext = &rows
        .iter()
        .find(|r| r.label == "Owned discovery integration")
        .unwrap()
        .value;
    let CredentialValue::External(ext) = ext else {
        panic!("external")
    };
    assert_eq!(ext.method_id.as_str(), "fixture-external");
    assert_eq!(ext.metadata.as_ref().unwrap()["fixture"], "external-data");
    let null = &rows
        .iter()
        .find(|r| r.label == "Owned discovery integration 2")
        .unwrap()
        .value;
    let CredentialValue::Key(null) = null else {
        panic!("key")
    };
    assert!(null.metadata.is_none());
    assert!(rows.last().unwrap().label.is_empty() && rows.last().unwrap().active);
    let snapshot = fallback
        .integrations()
        .get_at(&integration, Location::directory(other))
        .await
        .unwrap()
        .data;
    assert_eq!(snapshot.connections.len(), 5); // four global credentials plus the fixture environment
    let ConnectionInfo::Credential(connection) = &snapshot.connections[0] else {
        panic!("active credential first")
    };
    assert_eq!(connection.id, rows.last().unwrap().id);
    invalid(
        client
            .integrations()
            .connect_key(
                &integration,
                &IntegrationConnectKeyRequest::new("owned-dummy-invalid")
                    .answer(serde_json::from_value(json!({"fixture":17})).unwrap()),
            )
            .await
            .unwrap_err(),
        "integration_authorization",
    );
    invalid(
        client
            .integrations()
            .connect_external(
                &integration,
                &IntegrationConnectExternalRequest::new("missing-method".parse().unwrap()),
            )
            .await
            .unwrap_err(),
        "integration_authorization",
    );
    invalid(
        client
            .integrations()
            .connect_key(
                &denied,
                &IntegrationConnectKeyRequest::new("owned-dummy-denied"),
            )
            .await
            .unwrap_err(),
        "integration_policy",
    );
    invalid(
        client
            .integrations()
            .connect_external(
                &denied,
                &IntegrationConnectExternalRequest::new("missing-method".parse().unwrap()),
            )
            .await
            .unwrap_err(),
        "integration_policy",
    );
    let Error::Http(error) = client
        .integrations()
        .connect_key(
            &"missing-integration".parse().unwrap(),
            &IntegrationConnectKeyRequest::new("owned-dummy-missing"),
        )
        .await
        .unwrap_err()
    else {
        panic!("missing integration")
    };
    assert!(matches!(
        error.api_error(),
        Some(ApiError::IntegrationNotFound { .. })
    ));
    assert_eq!(client.credentials().list().await.unwrap().data.len(), 4);
    for row in rows {
        client.credentials().remove(&row.id).await.unwrap();
    }
    assert!(client.credentials().list().await.unwrap().data.is_empty());
    assert_eq!(
        client
            .integrations()
            .get(&integration)
            .await
            .unwrap()
            .data
            .connections
            .len(),
        1
    );
}
