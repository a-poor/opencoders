//! Owned disposable server only; commands use builtins and bounded local sleeps.
use opencoders::{
    ApiError, Client, Error, EventOptions, EventStream, Location, MessageListOptions,
    SessionShellOptions, ShellOutputOptions, models::*,
};
use std::{
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};

async fn terminal(client: &Client, id: &ShellId) -> ShellInfo {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let info = client.shells().get(id).await.unwrap().data;
            if info.status != ShellStatus::Running {
                return info;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("shell never terminated")
}
fn missing(error: Error) {
    let Error::Http(e) = error else {
        panic!("HTTP error")
    };
    assert_eq!(e.status(), 404);
    assert!(matches!(
        e.api_error(),
        Some(ApiError::ShellNotFound { .. })
    ));
}
async fn event(events: &mut EventStream, kind: &str) -> Event {
    tokio::time::timeout(Duration::from_secs(10), async {
        for _ in 0..100 {
            let e = events
                .next()
                .await
                .expect("closed shell event feed")
                .unwrap()
                .event;
            if e.kind() == kind {
                return e;
            }
        }
        panic!("unbounded event traffic")
    })
    .await
    .expect("missing shell event")
}
fn quoted(path: &Path) -> String {
    format!("'{}'", path.to_str().unwrap().replace('\'', "'\\''"))
}
async fn exited_process(pid: u64) {
    tokio::time::timeout(Duration::from_secs(10), async {
        while Command::new("/bin/kill")
            .args(["-0", &pid.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success()
        {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("owned command process survived removal");
}

pub(super) async fn verify(client: &Client, root: &Path, other: &str) {
    println!("smoke scenario: standalone and session shell commands");
    let shells = client.shells();
    let workspace = root.join("workspace + unicode 雪");
    let command = ShellCreateRequest::new("printf 'A雪B'")
        .timeout_ms(5000)
        .metadata(serde_json::from_value(serde_json::json!({"fixture":"shell"})).unwrap());
    let created = shells.create(&command).await.unwrap();
    assert_eq!(
        created.location.directory.as_str(),
        workspace.to_str().unwrap()
    );
    let id = &created.data.id;
    let done = terminal(client, id).await;
    assert_eq!(done.status, ShellStatus::Exited);
    assert_eq!(done.exit.unwrap().as_i64(), Some(0));
    assert!(done.time.completed.is_some());
    assert_eq!(done.metadata["fixture"], "shell");
    assert!(Path::new(&done.file).starts_with(root));
    assert_eq!(done.cwd, workspace.to_str().unwrap());
    assert!(
        !shells
            .list()
            .await
            .unwrap()
            .data
            .iter()
            .any(|s| &s.id == id)
    );
    let full = shells
        .output(id, &ShellOutputOptions::default())
        .await
        .unwrap()
        .data;
    assert_eq!(full.output, "A雪B");
    assert_eq!(full.size.get(), 5);
    assert_eq!(full.cursor.get(), 5);
    assert!(!full.truncated);
    let aligned = shells
        .output(id, &ShellOutputOptions::default().limit(4))
        .await
        .unwrap()
        .data;
    assert_eq!(aligned.output, "A雪");
    assert_eq!(aligned.cursor.get(), 4);
    assert!(!aligned.truncated);
    let split = shells
        .output(id, &ShellOutputOptions::default().limit(2))
        .await
        .unwrap()
        .data;
    assert_eq!(split.output, "A�");
    assert_eq!(split.cursor.get(), 2);
    let rest = shells
        .output(
            id,
            &ShellOutputOptions::default()
                .cursor(split.cursor.get())
                .limit(3),
        )
        .await
        .unwrap()
        .data;
    assert!(rest.output.contains('�'));
    assert!(rest.output.ends_with('B'));
    assert_eq!(rest.cursor.get(), 5);
    let end = shells
        .output(id, &ShellOutputOptions::default().cursor(100).limit(0))
        .await
        .unwrap()
        .data;
    assert_eq!(end.output, "");
    assert_eq!(end.cursor.get(), 5);
    missing(
        shells
            .get_at(id, Location::directory(other))
            .await
            .unwrap_err(),
    );
    shells.remove(id).await.unwrap();
    assert!(!Path::new(&done.file).exists());
    missing(shells.get(id).await.unwrap_err());
    missing(
        shells
            .output(id, &ShellOutputOptions::default())
            .await
            .unwrap_err(),
    );
    shells.remove(id).await.unwrap(); // pinned removal is idempotent; SDK still never retries

    let failed = shells
        .create(&ShellCreateRequest::new("printf 'out'; printf 'err' >&2; exit 7").timeout_ms(5000))
        .await
        .unwrap()
        .data;
    assert_eq!(
        terminal(client, &failed.id).await.exit.unwrap().as_i64(),
        Some(7)
    );
    let output = shells
        .output(&failed.id, &ShellOutputOptions::default())
        .await
        .unwrap()
        .data
        .output;
    assert!(output.contains("out") && output.contains("err"));
    shells.remove(&failed.id).await.unwrap();
    let timed = shells
        .create(&ShellCreateRequest::new("exec /bin/sleep 30").timeout_ms(50))
        .await
        .unwrap()
        .data;
    assert_eq!(
        terminal(client, &timed.id).await.status,
        ShellStatus::Timeout
    );
    shells.remove(&timed.id).await.unwrap();
    let running = shells
        .create(&ShellCreateRequest::new("exec /bin/sleep 30").timeout_ms(10000))
        .await
        .unwrap()
        .data;
    assert!(
        shells
            .list()
            .await
            .unwrap()
            .data
            .iter()
            .any(|s| s.id == running.id)
    );
    shells.remove(&running.id).await.unwrap();
    exited_process(running.pid.unwrap().get()).await;
    missing(shells.get(&running.id).await.unwrap_err());
    assert!(
        shells
            .list_at(Location::directory(other))
            .await
            .unwrap()
            .data
            .is_empty()
    );
    let remote = shells
        .create_at(
            &ShellCreateRequest::new("printf other")
                .cwd(other)
                .timeout_ms(5000),
            Location::directory(other),
        )
        .await
        .unwrap();
    assert_eq!(remote.location.directory.as_str(), other);
    // Explicit location is retained for get/output/remove; no ambient default switch.
    tokio::time::timeout(Duration::from_secs(10), async {
        while shells
            .get_at(&remote.data.id, Location::directory(other))
            .await
            .unwrap()
            .data
            .status
            == ShellStatus::Running
        {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        shells
            .output_at(
                &remote.data.id,
                &ShellOutputOptions::default(),
                Location::directory(other)
            )
            .await
            .unwrap()
            .data
            .output,
        "other"
    );
    shells
        .remove_at(&remote.data.id, Location::directory(other))
        .await
        .unwrap();

    let sessions = client.sessions();
    let session = sessions
        .create(
            &SessionCreateRequest::default()
                .location(SessionLocationInput::new(workspace.to_str().unwrap())),
        )
        .await
        .unwrap()
        .data;
    let sid = &session.id;
    let mut events = client
        .events()
        .subscribe(&EventOptions::default().types(["session.shell.started", "session.shell.ended"]))
        .await
        .unwrap();
    let print_env = "printf '%s|%s|%s|%s|%s' \"${OPENCODE_DISABLE_MODELS_FETCH-unset}\" \"${SHELL_FIXTURE_KEEP-unset}\" \"${SHELL_FIXTURE_DROP-unset}\" \"$TERM\" \"$OPENCODE_TERMINAL\"";
    for (index, environment, expected) in [
        (0, None, "1|unset|unset|xterm-256color|1"),
        (
            1,
            Some(
                [
                    ("SHELL_FIXTURE_KEEP".into(), "first".into()),
                    ("SHELL_FIXTURE_DROP".into(), "present".into()),
                ]
                .into(),
            ),
            "unset|first|present|xterm-256color|1",
        ),
        (
            2,
            Some([("SHELL_FIXTURE_KEEP".into(), "second".into())].into()),
            "unset|second|unset|xterm-256color|1",
        ),
        (
            3,
            Some(Default::default()),
            "unset|unset|unset|xterm-256color|1",
        ),
    ] {
        if let Some(variables) = environment {
            sessions
                .environment(sid, &SessionEnvironmentRequest::new(variables))
                .await
                .unwrap();
        }
        let message_id: SessionMessageId = format!("msg_shell_env_{index}").parse().unwrap();
        let request =
            SessionShellRequest::new(format!("{print_env}; exit 7")).id(message_id.clone());
        sessions
            .shell(
                sid,
                &request,
                &SessionShellOptions::default().timeout(Duration::from_secs(10)),
            )
            .await
            .unwrap();
        let Event::SessionShellStarted(start) = event(&mut events, "session.shell.started").await
        else {
            unreachable!()
        };
        let Event::SessionShellEnded(end) = event(&mut events, "session.shell.ended").await else {
            unreachable!()
        };
        assert_eq!(start.data.session_id, *sid);
        assert_eq!(start.data.shell.id, end.data.shell.id);
        assert_eq!(start.id.as_str(), format!("evt_shell_env_{index}"));
        assert_eq!(end.data.shell.exit.as_ref().unwrap().as_i64(), Some(7));
        assert_eq!(end.data.output.output, expected);
        let history = sessions
            .messages(sid)
            .list(&MessageListOptions::default())
            .await
            .unwrap();
        let message = history
            .data
            .iter()
            .find_map(|m| match m {
                SessionMessageInfo::Shell(s) if s.id == message_id => Some(s),
                _ => None,
            })
            .unwrap();
        assert_eq!(message.output.as_ref().unwrap().output, expected);
        assert!(message.time.completed.is_some());
        assert!(
            matches!(&message.exit,Some(SessionMessageShellExit::Number(n)) if n.as_i64()==Some(7))
        );
        assert!(!sessions.active().await.unwrap().data.contains_key(sid));
        let pending = sessions.inbox(sid).list().await.unwrap();
        assert_eq!(pending.data.len(), index + 1);
        assert!(
            pending
                .data
                .iter()
                .all(|i| matches!(i, SessionInboxInfo::Synthetic(_)))
        );
        shells.remove(&end.data.shell.id).await.unwrap();
    }

    // A local HTTP timeout must not kill server-owned session completion.
    let gate = root.join("shell-release");
    let command = format!(
        "i=0; while [ ! -f {} ] && [ $i -lt 200 ]; do i=$((i+1)); /bin/sleep 0.05; done; printf released",
        quoted(&gate)
    );
    let request = SessionShellRequest::new(command).id("msg_shell_disconnect".parse().unwrap());
    let error = sessions
        .shell(
            sid,
            &request,
            &SessionShellOptions::default().timeout(Duration::from_millis(250)),
        )
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    let Event::SessionShellStarted(start) = event(&mut events, "session.shell.started").await
    else {
        unreachable!()
    };
    assert_eq!(
        shells.get(&start.data.shell.id).await.unwrap().data.status,
        ShellStatus::Running
    );
    std::fs::write(&gate, b"release").unwrap();
    let Event::SessionShellEnded(end) = event(&mut events, "session.shell.ended").await else {
        unreachable!()
    };
    assert_eq!(end.data.output.output, "released");
    tokio::time::timeout(Duration::from_secs(10), async {
        while sessions.inbox(sid).list().await.unwrap().data.len() != 5 {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .unwrap();
    shells.remove(&end.data.shell.id).await.unwrap();
    sessions.remove(sid).await.unwrap();
    assert!(shells.list().await.unwrap().data.is_empty());
}
