use opencoders::{
    Client, Error,
    models::{PermissionReply, SessionPermissionCreateRequest, SessionPermissionReplyRequest},
};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    time::timeout,
};

#[tokio::test]
async fn permission_mutations_are_not_retried_after_the_server_receives_the_body() {
    for reply in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = Client::new(format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut bytes = Vec::new();
            while !bytes.ends_with(b"\r\n\r\n") {
                bytes.push(socket.read_u8().await.unwrap());
                assert!(bytes.len() < 8192);
            }
            let head = String::from_utf8(bytes).unwrap();
            assert!(head.starts_with("POST /api/session/ses_test/permission"));
            let length: usize = head
                .lines()
                .find_map(|l| {
                    l.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .map(str::to_owned)
                })
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            let mut body = vec![0; length];
            socket.read_exact(&mut body).await.unwrap();
            let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(
                json[if reply { "decision" } else { "action" }],
                if reply { "always" } else { "read" }
            );
            drop(socket);
            assert!(
                timeout(Duration::from_millis(150), listener.accept())
                    .await
                    .is_err()
            );
        });
        let id = "ses_test".parse().unwrap();
        let resource = client.sessions().permissions(&id);
        let error = if reply {
            resource
                .reply(
                    &"per_test".parse().unwrap(),
                    &SessionPermissionReplyRequest::new(PermissionReply::Always),
                )
                .await
                .unwrap_err()
        } else {
            resource
                .create(&SessionPermissionCreateRequest::new("read", vec![]))
                .await
                .unwrap_err()
        };
        assert!(matches!(error, Error::Transport(_)));
        assert!(error.status().is_none());
        assert_eq!(
            error.operation(),
            if reply {
                "session.permission.reply"
            } else {
                "session.permission.create"
            }
        );
        server.await.unwrap();
    }
}

#[tokio::test]
async fn permission_deadline_includes_error_body_reads() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = Client::builder(format!("http://{}", listener.local_addr().unwrap()))
        .request_timeout(Duration::from_millis(150))
        .build()
        .unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut bytes = Vec::new();
        while !bytes.ends_with(b"\r\n\r\n") {
            bytes.push(socket.read_u8().await.unwrap());
        }
        socket
            .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 100\r\n\r\n{")
            .await
            .unwrap();
        assert_eq!(socket.read(&mut [0; 1]).await.unwrap(), 0);
    });
    let error = client
        .sessions()
        .permissions(&"ses_test".parse().unwrap())
        .get(&"per_test".parse().unwrap())
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    assert_eq!(error.status().unwrap(), 404);
    assert_eq!(error.operation(), "session.permission.get");
    timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}
