//! No execution: all synthetic inputs are parked in the owned smoke fixture.
use opencoders::{ApiError, Client, Error, models::*};
use std::path::Path;

fn conflict(error: Error) {
    let Error::Http(e) = error else {
        panic!("HTTP error")
    };
    assert_eq!(e.status(), 409);
    assert!(matches!(e.api_error(), Some(ApiError::Conflict { .. })));
}
pub(super) async fn verify(client: &Client, root: &Path) {
    let sessions = client.sessions();
    let request = SessionCreateRequest::default().location(SessionLocationInput::new(
        root.join("workspace + unicode 雪").to_str().unwrap(),
    ));
    let a = sessions.create(&request).await.unwrap().data;
    let b = sessions.create(&request).await.unwrap().data;
    assert!(sessions.active().await.unwrap().data.is_empty());
    sessions
        .view(&a.id, &SessionViewRequest::new(0.into()))
        .await
        .unwrap();
    assert!(
        sessions
            .get(&a.id)
            .await
            .unwrap()
            .data
            .time
            .viewed
            .is_none()
    );
    let input_id: SessionMessageId = "msg_smoke_synthetic".parse().unwrap();
    let request = SessionSyntheticRequest::new("")
        .id(input_id.clone())
        .resume(false);
    let admitted = sessions.synthetic(&a.id, &request).await.unwrap().data;
    assert_eq!(admitted.payload.text, "");
    assert_eq!(admitted.delivery, SessionInboxDelivery::Steer);
    let replay = sessions
        .synthetic(
            &a.id,
            &SessionSyntheticRequest::new("ignored")
                .id(input_id.clone())
                .resume(false),
        )
        .await
        .unwrap()
        .data;
    assert_eq!(
        serde_json::to_value(replay).unwrap(),
        serde_json::to_value(admitted).unwrap()
    );
    conflict(sessions.synthetic(&b.id, &request).await.unwrap_err());
    conflict(
        sessions
            .prompt(
                &a.id,
                &SessionPromptRequest::new("wrong type")
                    .id(input_id.clone())
                    .resume(false),
            )
            .await
            .unwrap_err(),
    );
    assert_eq!(sessions.inbox(&a.id).list().await.unwrap().data.len(), 1);
    sessions.inbox(&a.id).cancel(&input_id).await.unwrap();
    assert!(sessions.inbox(&a.id).list().await.unwrap().data.is_empty());
    let missing = "ses_missing_activity".parse().unwrap();
    for result in [
        sessions
            .view(&missing, &SessionViewRequest::new(0.into()))
            .await,
        sessions.synthetic(&missing, &request).await.map(|_| ()),
    ] {
        let Error::Http(e) = result.unwrap_err() else {
            panic!("HTTP error")
        };
        assert_eq!(e.status(), 404);
        assert!(matches!(
            e.api_error(),
            Some(ApiError::SessionNotFound { .. })
        ));
    }
    sessions.remove(&a.id).await.unwrap();
    sessions.remove(&b.id).await.unwrap();
}
