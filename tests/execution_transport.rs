use opencoders::{Client, Error, SessionInterruptOptions, SessionWaitOptions};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::oneshot,
    time::timeout,
};

async fn request(socket: &mut TcpStream) -> String {
    let mut bytes = Vec::new();
    while !bytes.ends_with(b"\r\n\r\n") {
        bytes.push(socket.read_u8().await.unwrap());
        assert!(bytes.len() < 8192);
    }
    String::from_utf8(bytes).unwrap()
}
async fn setup() -> (Client, TcpListener) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = Client::new(format!("http://{}", listener.local_addr().unwrap())).unwrap();
    (client, listener)
}

#[tokio::test]
async fn wait_completes_at_204_headers_without_waiting_for_connection_close() {
    let (client, listener) = setup().await;
    let (release, hold) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        assert!(
            request(&mut socket)
                .await
                .starts_with("POST /api/experimental/session/ses_test/wait HTTP/1.1")
        );
        socket
            .write_all(b"HTTP/1.1 204 No Content\r\nConnection: keep-alive\r\n\r\n")
            .await
            .unwrap();
        hold.await.unwrap();
    });
    timeout(
        Duration::from_secs(2),
        client
            .sessions()
            .experimental_wait(&"ses_test".parse().unwrap(), &SessionWaitOptions::default()),
    )
    .await
    .unwrap()
    .unwrap();
    release.send(()).unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn wait_deadline_covers_error_body_reads_and_retains_received_status() {
    let (client, listener) = setup().await;
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        request(&mut socket).await;
        socket
            .write_all(b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 100\r\n\r\n{")
            .await
            .unwrap();
        assert_eq!(socket.read(&mut [0; 1]).await.unwrap(), 0);
    });
    let error = client
        .sessions()
        .experimental_wait(
            &"ses_test".parse().unwrap(),
            &SessionWaitOptions::default().timeout(Duration::from_millis(100)),
        )
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    assert_eq!(error.operation(), "experimental.session.wait");
    assert_eq!(error.status().unwrap(), 503);
    timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn truncated_responses_keep_operation_status_and_do_not_retry() {
    for wait in [false, true] {
        let (client, listener) = setup().await;
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            request(&mut socket).await;
            if wait {
                socket
                    .write_all(b"HTTP/1.1 503 Unavailable\r\nContent-Length: 100\r\n\r\n{")
                    .await
                    .unwrap();
            } else {
                socket
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\n\r\n{")
                    .await
                    .unwrap();
            }
            drop(socket);
            assert!(
                timeout(Duration::from_millis(150), listener.accept())
                    .await
                    .is_err()
            );
        });
        let id = "ses_test".parse().unwrap();
        let error = if wait {
            client
                .sessions()
                .experimental_wait(&id, &SessionWaitOptions::default())
                .await
                .unwrap_err()
        } else {
            client
                .sessions()
                .interrupt(&id, &SessionInterruptOptions::default())
                .await
                .unwrap_err()
        };
        assert!(matches!(error, Error::Transport(_)));
        assert_eq!(
            error.operation(),
            if wait {
                "experimental.session.wait"
            } else {
                "session.interrupt"
            }
        );
        assert_eq!(error.status().unwrap(), if wait { 503 } else { 200 });
        server.await.unwrap();
    }
}

#[tokio::test]
async fn disconnect_after_post_acceptance_does_not_retry_either_operation() {
    for wait in [false, true] {
        let (client, listener) = setup().await;
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let received = request(&mut socket).await;
            assert!(received.starts_with("POST "));
            assert!(!received.to_lowercase().contains("transfer-encoding:"));
            drop(socket);
            assert!(
                timeout(Duration::from_millis(150), listener.accept())
                    .await
                    .is_err()
            );
        });
        let id = "ses_test".parse().unwrap();
        let error = if wait {
            client
                .sessions()
                .experimental_wait(&id, &SessionWaitOptions::default())
                .await
                .unwrap_err()
        } else {
            client
                .sessions()
                .interrupt(&id, &SessionInterruptOptions::default().resume(true))
                .await
                .unwrap_err()
        };
        assert!(matches!(error, Error::Transport(_)));
        assert!(error.status().is_none());
        assert_eq!(
            error.operation(),
            if wait {
                "experimental.session.wait"
            } else {
                "session.interrupt"
            }
        );
        server.await.unwrap();
    }
}

#[tokio::test]
async fn dropping_one_wait_cancels_its_connection_without_interrupting_or_cancelling_another() {
    let (client, listener) = setup().await;
    let (first_ready, ready) = oneshot::channel();
    let (second_ready, ready2) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (mut first, _) = listener.accept().await.unwrap();
        assert!(
            request(&mut first)
                .await
                .starts_with("POST /api/experimental/session/ses_one/wait ")
        );
        first_ready.send(()).unwrap();
        let (mut second, _) = listener.accept().await.unwrap();
        assert!(
            request(&mut second)
                .await
                .starts_with("POST /api/experimental/session/ses_two/wait ")
        );
        second_ready.send(()).unwrap();
        assert_eq!(
            timeout(Duration::from_secs(2), first.read(&mut [0; 1]))
                .await
                .unwrap()
                .unwrap(),
            0
        );
        second
            .write_all(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        assert!(
            timeout(Duration::from_millis(150), listener.accept())
                .await
                .is_err()
        );
    });
    let sessions = client.sessions();
    let id1 = "ses_one".parse().unwrap();
    let id2 = "ses_two".parse().unwrap();
    let options = SessionWaitOptions::default();
    let mut first = Box::pin(sessions.experimental_wait(&id1, &options));
    tokio::select! { result = &mut first => panic!("unexpected completion: {result:?}"), _ = ready => {} }
    let mut second = Box::pin(sessions.experimental_wait(&id2, &options));
    tokio::select! { result = &mut second => panic!("unexpected completion: {result:?}"), _ = ready2 => {} }
    drop(first);
    timeout(Duration::from_secs(2), second)
        .await
        .unwrap()
        .unwrap();
    server.await.unwrap();
}
