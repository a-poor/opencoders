use opencoders::{ApiError, Client, Error, Location, VcsInitOptions, models::*};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path, query_param},
};

#[tokio::test]
async fn lost_mutation_responses_do_not_retry_initialization_or_worktree_operations() {
    use std::time::Duration;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };
    for operation in ["create", "refresh", "remove", "init"] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut headers = Vec::new();
            while !headers.ends_with(b"\r\n\r\n") {
                assert!(headers.len() < 8192);
                headers.push(socket.read_u8().await.unwrap());
            }
            let headers = String::from_utf8(headers).unwrap();
            let length: usize = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse().unwrap())
                })
                .unwrap_or(0);
            let mut body = vec![0; length];
            socket.read_exact(&mut body).await.unwrap();
            if operation == "init" {
                assert!(body.is_empty());
            } else {
                let body: Value = serde_json::from_slice(&body).unwrap();
                assert_eq!(body["projectID"], "fixture");
                if operation == "remove" {
                    assert_eq!(body["force"], false);
                }
            }
            socket.shutdown().await.unwrap();
            drop(socket);
            assert!(
                tokio::time::timeout(Duration::from_millis(150), listener.accept())
                    .await
                    .is_err(),
                "unexpected retry of {operation}"
            );
        });
        let client = Client::new(format!("http://{addr}")).unwrap();
        let project: ProjectId = "fixture".to_owned().into();
        let result = match operation {
            "create" => client
                .worktrees()
                .create(&WorktreeCreateRequest::new(project))
                .await
                .map(|_| ()),
            "refresh" => client.worktrees().refresh(&project).await,
            "remove" => {
                client
                    .worktrees()
                    .remove(&WorktreeRemoveRequest::new(
                        project,
                        "/private-checkout".to_owned().into(),
                        false,
                    ))
                    .await
            }
            _ => client.vcs().init(&VcsInitOptions::default()).await,
        };
        let error = result.unwrap_err();
        assert!(matches!(error, Error::Transport(_)));
        assert!(error.status().is_none());
        assert!(!format!("{error:?}").contains("private-checkout"));
        server.await.unwrap();
    }
}

#[tokio::test]
async fn malformed_worktree_error_metadata_remains_unrecognized() {
    for body in [
        json!({"_tag":"WorktreeError","name":"OtherError","data":{"message":"failed","forceRequired":true}}),
        json!({"_tag":"WorktreeError","name":"WorktreeError","data":{"message":"failed","forceRequired":"yes"}}),
        json!({"_tag":"WorktreeError","data":{"message":"failed","forceRequired":true}}),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(400).set_body_json(body.clone()))
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new(server.uri()).unwrap();
        let Error::Http(error) = client
            .worktrees()
            .refresh(&"p".to_owned().into())
            .await
            .unwrap_err()
        else {
            panic!("expected HTTP error");
        };
        assert!(matches!(error.api_error(), Some(ApiError::Unrecognized(raw)) if raw == &body));
    }
}

#[test]
fn worktree_inputs_preserve_omission_and_empty_values_and_require_explicit_force() {
    let project: ProjectId = "project ?".to_owned().into();
    assert_eq!(
        serde_json::to_value(WorktreeCreateRequest::new(project.clone())).unwrap(),
        json!({"projectID":"project ?"})
    );
    let request = WorktreeCreateRequest::new(project.clone())
        .from("/source".to_owned().into())
        .directory("/parent".to_owned().into())
        .branch("")
        .name("");
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({"projectID":"project ?","from":"/source","directory":"/parent","branch":"","name":""})
    );
    for field in ["from", "directory", "branch", "name", "unknown"] {
        assert!(
            serde_json::from_value::<WorktreeCreateRequest>(
                json!({"projectID":"project ?",field:null})
            )
            .is_err()
        );
    }
    for force in [false, true] {
        let request =
            WorktreeRemoveRequest::new(project.clone(), "/checkout".to_owned().into(), force);
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            json!({"projectID":"project ?","directory":"/checkout","force":force})
        );
    }
    for value in [
        json!({"projectID":"p","directory":"/checkout"}),
        json!({"projectID":"p","directory":"/checkout","force":null}),
        json!({"projectID":"p","directory":"/checkout","force":false,"extra":1}),
    ] {
        assert!(serde_json::from_value::<WorktreeRemoveRequest>(value).is_err());
    }
    let info: WorktreeInfo =
        serde_json::from_value(json!({"directory":"/created","future":42})).unwrap();
    assert_eq!(info.extra["future"], 42);
}

#[tokio::test]
async fn worktree_lifecycle_uses_project_bodies_without_location_and_preserves_created_metadata() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(path("/proxy/api/worktree"))
        .and(body_json(json!({"projectID":"project ?","from":"/source","branch":"topic","directory":"/parent + 雪","name":"review"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"directory":"/parent + 雪/review-2","future":true})))
        .expect(1).mount(&server).await;
    Mock::given(method("POST"))
        .and(path("/proxy/api/worktree/refresh"))
        .and(body_json(json!({"projectID":"project ?"})))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/proxy/api/worktree"))
        .and(body_json(
            json!({"projectID":"project ?","directory":"/parent + 雪/review-2","force":false}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let project: ProjectId = "project ?".to_owned().into();
    let info = client
        .worktrees()
        .create(
            &WorktreeCreateRequest::new(project.clone())
                .from("/source".to_owned().into())
                .branch("topic")
                .directory("/parent + 雪".to_owned().into())
                .name("review"),
        )
        .await
        .unwrap();
    assert_eq!(info.directory.as_str(), "/parent + 雪/review-2");
    assert_eq!(info.extra["future"], true);
    client.worktrees().refresh(&project).await.unwrap();
    client
        .worktrees()
        .remove(&WorktreeRemoveRequest::new(project, info.directory, false))
        .await
        .unwrap();
    for request in server.received_requests().await.unwrap() {
        assert!(request.url.query().is_none());
        assert_eq!(request.headers["content-type"], "application/json");
    }
}

#[tokio::test]
async fn vcs_initialization_is_bodyless_and_preserves_default_override_and_empty_provider() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/proxy/api/vcs/init"))
        .respond_with(ResponseTemplate::new(204))
        .expect(3)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/work + 雪"))
        .build()
        .unwrap();
    client.vcs().init(&VcsInitOptions::default()).await.unwrap();
    client
        .vcs()
        .init_at(
            &VcsInitOptions::default().provider("hg"),
            Location::directory("/override"),
        )
        .await
        .unwrap();
    client
        .vcs()
        .init_at(
            &VcsInitOptions::default().provider(""),
            Location::Unspecified,
        )
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests[0].url.query_pairs().collect::<Vec<_>>(),
        vec![("location[directory]".into(), "/work + 雪".into())]
    );
    assert_eq!(
        requests[1].url.query_pairs().collect::<Vec<_>>(),
        vec![
            ("location[directory]".into(), "/override".into()),
            ("provider".into(), "hg".into())
        ]
    );
    assert_eq!(requests[2].url.query(), Some("provider="));
    assert!(requests.iter().all(|request| request.body.is_empty()));
}

#[tokio::test]
async fn force_required_remains_a_typed_error_without_an_implicit_forced_retry() {
    for force in [
        None,
        Some(Value::Null),
        Some(json!(false)),
        Some(json!(true)),
    ] {
        let server = MockServer::start().await;
        let mut data = json!({"message":"private-error"});
        if let Some(force) = force.clone() {
            data["forceRequired"] = force;
        }
        Mock::given(method("DELETE"))
            .and(body_json(
                json!({"projectID":"p","directory":"/private-checkout","force":false}),
            ))
            .respond_with(
                ResponseTemplate::new(400).set_body_json(
                    json!({"_tag":"WorktreeError","name":"WorktreeError","data":data}),
                ),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new(server.uri()).unwrap();
        let error = client
            .worktrees()
            .remove(&WorktreeRemoveRequest::new(
                "p".to_owned().into(),
                "/private-checkout".to_owned().into(),
                false,
            ))
            .await
            .unwrap_err();
        assert!(!format!("{error:?}").contains("private-"));
        assert_eq!(error.status().unwrap(), 400);
        let Error::Http(error) = error else {
            panic!("expected HTTP error");
        };
        assert!(
            matches!(error.api_error(),Some(ApiError::Worktree {force_required,..}) if *force_required==force.and_then(|f| f.as_bool()))
        );
    }
}

#[tokio::test]
async fn unsupported_vcs_provider_is_typed_and_unexpected_success_is_not_accepted() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(query_param("provider","custom"))
        .respond_with(ResponseTemplate::new(501).set_body_json(json!({"_tag":"VcsInitNotSupportedError","providerID":"custom","message":"private-error"})))
        .expect(1).mount(&server).await;
    Mock::given(method("POST"))
        .and(query_param("provider", "git"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(true)))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let Error::Http(error) = client
        .vcs()
        .init(&VcsInitOptions::default().provider("custom"))
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error");
    };
    assert!(
        matches!(error.api_error(),Some(ApiError::VcsInitNotSupported{provider_id,..}) if provider_id=="custom")
    );
    assert_eq!(
        client
            .vcs()
            .init(&VcsInitOptions::default().provider("git"))
            .await
            .unwrap_err()
            .status()
            .unwrap(),
        200
    );
}
