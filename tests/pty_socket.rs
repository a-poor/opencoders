#![cfg(feature = "websocket")]
use futures_util::{SinkExt, StreamExt};
use opencoders::{Client, Error, Location, PtyConnectOptions, PtyOutput};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::{
    accept_async, accept_hdr_async,
    tungstenite::{Message, protocol::CloseFrame, protocol::frame::coding::CloseCode},
};

#[tokio::test]
async fn rejected_upgrades_preserve_bounded_http_errors_and_deadline_status() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    for stalled in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = vec![0; 4096];
            let mut headers = Vec::new();
            while !headers.ends_with(b"\r\n\r\n") {
                assert!(headers.len() < 8192);
                headers.push(socket.read_u8().await.unwrap());
            }
            socket
                .write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 1000\r\n\r\n")
                .await
                .unwrap();
            if !stalled {
                socket.write_all(b"private-response").await.unwrap();
            }
            let _ = socket.read(&mut buffer).await;
            assert!(
                tokio::time::timeout(Duration::from_millis(100), listener.accept())
                    .await
                    .is_err()
            );
        });
        let client = Client::builder(format!("http://{addr}"))
            .max_error_body_bytes(4)
            .build()
            .unwrap();
        let error = client
            .ptys()
            .connect(
                &"pty_test".parse().unwrap(),
                &PtyConnectOptions::default()
                    .ticket("secret")
                    .handshake_timeout(Duration::from_millis(100)),
            )
            .await
            .unwrap_err();
        assert_eq!(error.status().unwrap(), 403);
        assert!(!format!("{error:?}").contains("secret"));
        if stalled {
            assert!(error.is_timeout());
        } else {
            let Error::Http(error) = error else {
                panic!("expected HTTP error");
            };
            assert_eq!(error.body(), b"priv");
        }
        server.await.unwrap();
    }
}

#[tokio::test]
async fn invalid_accept_headers_are_rejected_before_socket_io() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buffer = vec![0; 4096];
        let mut headers = Vec::new();
        while !headers.ends_with(b"\r\n\r\n") {
            assert!(headers.len() < 8192);
            headers.push(socket.read_u8().await.unwrap());
        }
        socket.write_all(b"HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: invalid\r\n\r\n").await.unwrap();
        let _ = socket.read(&mut buffer).await;
    });
    let client = Client::new(format!("http://{addr}")).unwrap();
    let error = client
        .ptys()
        .connect(&"pty_test".parse().unwrap(), &PtyConnectOptions::default())
        .await
        .unwrap_err();
    let Error::PtySocket(error) = error else {
        panic!("expected handshake rejection")
    };
    assert_eq!(
        error.kind(),
        opencoders::PtySocketErrorKind::InvalidHandshake
    );
    server.await.unwrap();
}

#[tokio::test]
#[allow(
    clippy::result_large_err,
    reason = "tungstenite fixes the handshake callback error type"
)]
async fn pty_socket_preserves_transport_headers_cursor_frames_and_utf8_input() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws = accept_hdr_async(
            stream,
            |request: &tokio_tungstenite::tungstenite::handshake::server::Request, response| {
                assert_eq!(request.uri().path(), "/proxy/api/pty/pty_test/connect");
                assert!(
                    request
                        .uri()
                        .query()
                        .unwrap()
                        .contains("ticket=private-ticket")
                );
                assert!(request.uri().query().unwrap().contains("cursor=-1"));
                assert_eq!(
                    request.headers()["authorization"],
                    "Basic b3BlbmNvZGU6c2VjcmV0"
                );
                assert_eq!(request.headers()["x-injected"], "yes");
                Ok(response)
            },
        )
        .await
        .unwrap();
        ws.send(Message::Text("replay 🌻".into())).await.unwrap();
        ws.send(Message::Binary(b"\0{\"cursor\":42}".to_vec().into()))
            .await
            .unwrap();
        ws.send(Message::Ping(b"ping".to_vec().into()))
            .await
            .unwrap();
        loop {
            match ws.next().await.unwrap().unwrap() {
                Message::Text(text) => {
                    assert_eq!(text.as_str(), "hello 🌻\r");
                    break;
                }
                Message::Pong(_) => {}
                other => panic!("unexpected {other:?}"),
            }
        }
        ws.send(Message::Text("live".into())).await.unwrap();
        ws.close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "finished".into(),
        }))
        .await
        .unwrap();
        while let Some(message) = ws.next().await {
            if matches!(message, Ok(Message::Close(_))) {
                break;
            }
        }
    });
    let injected = reqwest::Client::builder()
        .default_headers(opencoders::header::HeaderMap::from_iter([(
            opencoders::header::HeaderName::from_static("x-injected"),
            opencoders::header::HeaderValue::from_static("yes"),
        )]))
        .build()
        .unwrap();
    let client = Client::builder(format!("http://{addr}/proxy"))
        .http_client(injected)
        .basic_auth("opencode", "secret")
        .default_location(Location::directory("/work ?"))
        .build()
        .unwrap();
    let options = PtyConnectOptions::default()
        .ticket("private-ticket")
        .cursor(-1);
    assert!(!format!("{options:?}").contains("private-ticket"));
    let mut connection = client
        .ptys()
        .connect(&"pty_test".parse().unwrap(), &options)
        .await
        .unwrap();
    assert!(
        matches!(connection.next().await.unwrap().unwrap(),PtyOutput::Data(text) if text=="replay 🌻")
    );
    assert!(matches!(
        connection.next().await.unwrap().unwrap(),
        PtyOutput::Cursor(42)
    ));
    let (mut writer, mut reader) = connection.split();
    writer.send("hello 🌻\r".to_owned()).await.unwrap();
    assert!(matches!(reader.next().await.unwrap().unwrap(),PtyOutput::Data(text) if text=="live"));
    assert!(
        matches!(reader.next().await.unwrap().unwrap(),PtyOutput::Closed {code:Some(1000),reason} if reason=="finished")
    );
    assert!(reader.next().await.is_none());
    task.await.unwrap();
}

#[tokio::test]
async fn malformed_control_frames_and_oversized_messages_terminate_the_socket() {
    for message in [
        Message::Binary(b"\0{\"cursor\":-1}".to_vec().into()),
        Message::Binary(b"\0{}".to_vec().into()),
        Message::Binary(b"unknown binary".to_vec().into()),
        Message::Text("x".repeat(200).into()),
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut ws = accept_async(stream).await.unwrap();
            ws.send(message).await.unwrap();
            let _ = ws.next().await;
        });
        let client = Client::new(format!("http://{addr}")).unwrap();
        let mut connection = client
            .ptys()
            .connect(
                &"pty_test".parse().unwrap(),
                &PtyConnectOptions::default().max_message_bytes(100),
            )
            .await
            .unwrap();
        assert!(matches!(
            connection.next().await.unwrap(),
            Err(Error::PtySocket(_))
        ));
        assert!(connection.next().await.is_none());
        task.await.unwrap();
    }
}

#[tokio::test]
async fn cancelled_reads_keep_the_idle_deadline_and_invalid_cursors_do_not_connect() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws = accept_async(stream).await.unwrap();
        let _ = ws.next().await;
    });
    let client = Client::new(format!("http://{addr}")).unwrap();
    let id = "pty_test".parse().unwrap();
    assert!(matches!(
        client
            .ptys()
            .connect(&id, &PtyConnectOptions::default().cursor(-2))
            .await,
        Err(Error::Encode(_))
    ));
    let mut connection = client
        .ptys()
        .connect(
            &id,
            &PtyConnectOptions::default().idle_timeout(Duration::from_millis(100)),
        )
        .await
        .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(20), connection.next())
            .await
            .is_err()
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
    let error = tokio::time::timeout(Duration::from_millis(20), connection.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap_err();
    assert!(error.is_timeout());
    assert!(connection.next().await.is_none());
    task.await.unwrap();
}
