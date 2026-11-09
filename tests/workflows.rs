use opencoders::{Client, Location, RequestField, SessionDiffOptions, models::*};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path, query_param},
};

#[test]
fn workflow_inputs_preserve_null_omission_empty_lists_and_false() {
    let mut command = SessionCommandRequest::new("review", "");
    assert_eq!(
        serde_json::to_value(&command).unwrap(),
        json!({"name":"review","text":""})
    );
    command.delivery = RequestField::Null;
    assert_eq!(
        serde_json::to_value(command.files(vec![])).unwrap(),
        json!({"name":"review","text":"","delivery":null,"files":[]})
    );
    for field in ["files", "agents", "skills", "unknown"] {
        assert!(
            serde_json::from_value::<SessionCommandRequest>(
                json!({"name":"review","text":"",field:null})
            )
            .is_err()
        );
    }
    assert!(
        serde_json::from_value::<SessionCommandRequest>(
            json!({"name":"review","text":"","files":[{"uri":"file:///x","unknown":true}]})
        )
        .is_err()
    );
    let compact = SessionCompactRequest::default();
    assert_eq!(serde_json::to_value(&compact).unwrap(), json!({}));
    let compact: SessionCompactRequest =
        serde_json::from_value(json!({"id":null,"delivery":null})).unwrap();
    assert_eq!(
        serde_json::to_value(compact).unwrap(),
        json!({"id":null,"delivery":null})
    );
    let revert = SessionRevertStageRequest::new("msg_a".parse().unwrap()).files(false);
    assert_eq!(
        serde_json::to_value(revert).unwrap(),
        json!({"messageID":"msg_a","files":false})
    );
    assert!(
        serde_json::from_value::<SessionRevertStageRequest>(
            json!({"messageID":"msg_a","extra":false})
        )
        .is_err()
    );
}

#[tokio::test]
async fn workflow_mutations_use_session_paths_exact_statuses_and_preserve_envelopes() {
    let server = MockServer::start().await;
    for (verb, suffix, body, response) in [
        ("POST", "command", json!({"name":"review","text":""}), ResponseTemplate::new(204)),
        ("POST", "compact", json!({"id":"msg_c","delivery":"queue"}), ResponseTemplate::new(200).set_body_json(json!({"data":{"id":"msg_c","sessionID":"ses_a","type":"compaction","time":{"created":1},"payload":{},"delivery":"queue","future":1},"extra":true}))),
        ("POST", "revert/stage", json!({"messageID":"msg_a","files":false}), ResponseTemplate::new(200).set_body_json(json!({"data":{"messageID":"msg_a","files":[],"future":2},"extra":true}))),
    ] {
        Mock::given(method(verb)).and(path(format!("/proxy/api/session/ses_a/{suffix}"))).and(body_json(body)).respond_with(response).expect(1).mount(&server).await;
    }
    for (verb, suffix) in [("DELETE", "revert"), ("POST", "revert/commit")] {
        Mock::given(method(verb))
            .and(path(format!("/proxy/api/session/ses_a/{suffix}")))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&server)
            .await;
    }
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/ignored"))
        .build()
        .unwrap();
    let id = "ses_a".parse().unwrap();
    client
        .sessions()
        .command(&id, &SessionCommandRequest::new("review", ""))
        .await
        .unwrap();
    let compact = client
        .sessions()
        .compact(
            &id,
            &SessionCompactRequest::default()
                .id("msg_c".parse().unwrap())
                .delivery(SessionInboxDelivery::Queue),
        )
        .await
        .unwrap();
    assert_eq!(compact.extra["extra"], true);
    assert_eq!(compact.data.extra["future"], 1);
    let staged = client
        .sessions()
        .revert(&id)
        .stage(&SessionRevertStageRequest::new("msg_a".parse().unwrap()).files(false))
        .await
        .unwrap();
    assert_eq!(staged.data.extra["future"], 2);
    client.sessions().revert(&id).clear().await.unwrap();
    client.sessions().revert(&id).commit().await.unwrap();
    for request in server.received_requests().await.unwrap() {
        assert!(request.url.query().is_none());
        if request.url.path().ends_with("revert") || request.url.path().ends_with("commit") {
            assert!(request.body.is_empty());
            assert!(!request.headers.contains_key("content-type"));
        }
    }
}

#[tokio::test]
async fn workflow_reads_use_catalog_locations_and_session_diff_filters() {
    let server = MockServer::start().await;
    let location = json!({"projectID":"p","directory":"/work"});
    for (resource, data) in [
        ("command", json!([{"name":"review","future":1}])),
        (
            "skill",
            json!([{"id":"skill","name":"review","path":"/work/SKILL.md","content":"fixture","autoinvoke":false,"future":2}]),
        ),
    ] {
        Mock::given(method("GET"))
            .and(path(format!("/api/{resource}")))
            .and(query_param("location[directory]", "/work ? 雪"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"location":location,"data":data,"future":true})),
            )
            .expect(1)
            .mount(&server)
            .await;
    }
    Mock::given(method("GET")).and(path("/api/session/ses_a/context")).respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"msg_idle","type":"idle","outcome":"succeeded","time":{"created":1},"future":true}],"future":true}))).expect(1).mount(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/session/ses_a/diff"))
        .and(query_param("from", "msg_a"))
        .and(query_param("to", "msg_b"))
        .and(query_param("context", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[],"future":true})))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .default_location(Location::directory("/work ? 雪"))
        .build()
        .unwrap();
    assert_eq!(
        client.commands().list().await.unwrap().data[0].extra["future"],
        1
    );
    assert_eq!(
        client
            .skills()
            .list_at(Location::directory("/work ? 雪"))
            .await
            .unwrap()
            .data[0]
            .extra["future"],
        2
    );
    let id = "ses_a".parse().unwrap();
    let context = client.sessions().context(&id).await.unwrap();
    assert_eq!(
        serde_json::to_value(&context.data[0]).unwrap()["future"],
        true
    );
    let diff = client
        .sessions()
        .diff(
            &id,
            &SessionDiffOptions::default()
                .from("msg_a".parse().unwrap())
                .to("msg_b".parse().unwrap())
                .context(0),
        )
        .await
        .unwrap();
    assert_eq!(diff.extra["future"], Value::Bool(true));
    for request in server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter(|r| r.url.path().contains("/session/"))
    {
        assert!(
            !request
                .url
                .query_pairs()
                .any(|(k, _)| k.starts_with("location"))
        );
    }
}

#[tokio::test]
async fn workflow_errors_preserve_typed_details_metadata_and_redact_debug() {
    use opencoders::{ApiError, Error};
    for (status, body) in [
        (
            404,
            json!({"_tag":"CommandNotFoundError","command":"private-command","message":"private-detail"}),
        ),
        (
            500,
            json!({"_tag":"CommandExecutionError","command":"private-command","message":"private-detail"}),
        ),
        (
            409,
            json!({"_tag":"SessionBusyError","sessionID":"private-session","message":"private-detail"}),
        ),
        (
            500,
            json!({"_tag":"UnknownError","message":"private-detail","ref":"private-reference"}),
        ),
        (
            500,
            json!({"_tag":"UnknownError","message":"private-detail","ref":42}),
        ),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(status)
                    .insert_header("x-request-id", "fixture")
                    .set_body_json(body.clone()),
            )
            .expect(1)
            .mount(&server)
            .await;
        let client = Client::new(server.uri()).unwrap();
        let Error::Http(e) = client
            .sessions()
            .command(
                &"ses_a".parse().unwrap(),
                &SessionCommandRequest::new("private-command", ""),
            )
            .await
            .unwrap_err()
        else {
            panic!("HTTP error")
        };
        assert_eq!(serde_json::from_slice::<Value>(e.body()).unwrap(), body);
        let api = e.api_error().unwrap();
        match body["_tag"].as_str().unwrap() {
            "CommandNotFoundError" => assert!(
                matches!(api,ApiError::CommandNotFound{command,..} if command=="private-command")
            ),
            "CommandExecutionError" => assert!(
                matches!(api,ApiError::CommandExecution{command,..} if command=="private-command")
            ),
            "SessionBusyError" => assert!(
                matches!(api,ApiError::SessionBusy{session_id,..} if session_id=="private-session")
            ),
            _ if body["ref"].is_string() => assert!(
                matches!(api,ApiError::Unknown{reference:Some(r),..} if r=="private-reference")
            ),
            _ => assert!(matches!(api,ApiError::Unrecognized(raw) if raw==&body)),
        }
        assert!(!format!("{api:?} {e:?}").contains("private-"));
    }
}

#[tokio::test]
async fn workflow_mutations_reject_other_success_statuses_without_retry() {
    use opencoders::Error;
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({})))
        .expect(4)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let sessions = client.sessions();
    let id = "ses_a".parse().unwrap();
    let results = [
        sessions
            .command(&id, &SessionCommandRequest::new("c", ""))
            .await,
        sessions
            .compact(&id, &SessionCompactRequest::default())
            .await
            .map(|_| ()),
        sessions
            .revert(&id)
            .stage(&SessionRevertStageRequest::new("msg_a".parse().unwrap()))
            .await
            .map(|_| ()),
        sessions.revert(&id).commit().await,
        sessions.revert(&id).clear().await,
    ];
    for result in results {
        assert!(matches!(result, Err(Error::Http(_))));
    }
}

#[tokio::test]
async fn lost_workflow_mutation_responses_do_not_retry_callbacks_or_reverts() {
    use std::time::Duration;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };
    for operation in [
        "command",
        "compact",
        "stage",
        "commit",
        "clear",
        "view",
        "synthetic",
    ] {
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
            if matches!(operation, "commit" | "clear") {
                assert!(body.is_empty());
            } else {
                assert!(serde_json::from_slice::<Value>(&body).unwrap().is_object());
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
        let id = "ses_a".parse().unwrap();
        let result = match operation {
            "view" => {
                client
                    .sessions()
                    .view(&id, &SessionViewRequest::new(1.into()))
                    .await
            }
            "synthetic" => client
                .sessions()
                .synthetic(&id, &SessionSyntheticRequest::new("private-command"))
                .await
                .map(|_| ()),
            "command" => {
                client
                    .sessions()
                    .command(&id, &SessionCommandRequest::new("private-command", ""))
                    .await
            }
            "compact" => client
                .sessions()
                .compact(&id, &SessionCompactRequest::default())
                .await
                .map(|_| ()),
            "stage" => client
                .sessions()
                .revert(&id)
                .stage(&SessionRevertStageRequest::new("msg_a".parse().unwrap()))
                .await
                .map(|_| ()),
            "commit" => client.sessions().revert(&id).commit().await,
            _ => client.sessions().revert(&id).clear().await,
        };
        let error = result.unwrap_err();
        assert!(matches!(error, opencoders::Error::Transport(_)));
        assert!(error.status().is_none());
        assert!(!format!("{error:?}").contains("private-command"));
        server.await.unwrap();
    }
}
