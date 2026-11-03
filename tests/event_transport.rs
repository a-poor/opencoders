//! Real socket tests for incremental reads and ownership of live connections.
use opencoders::{Client, Error, EventOptions, StreamErrorKind};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::oneshot,
    time::{sleep, timeout},
};

const CONNECTED: &str = "data: {\"id\":\"evt_test\",\"type\":\"server.connected\",\"data\":{}}\n\n";
#[path = "support/sse.rs"]
mod support;
use support::*;

#[tokio::test]
async fn fragmented_utf8_and_multiple_frames_survive_network_chunk_boundaries() {
    let (client, task) = server(|mut s| async move {
        headers(&mut s).await;
        for b in "\u{feff}: ping\r\ndata: {\"type\":\"future.雪\",\r\ndata: \"value\":1}\r\n\r\n"
            .as_bytes()
        {
            chunk(&mut s, &[*b]).await;
        }
        chunk(
            &mut s,
            format!("{CONNECTED}{CONNECTED}data: unfinished").as_bytes(),
        )
        .await;
        s.write_all(b"0\r\n\r\n").await.unwrap();
    })
    .await;
    let mut stream = client
        .events()
        .subscribe(&EventOptions::default())
        .await
        .unwrap();
    assert_eq!(
        stream.next().await.unwrap().unwrap().event.kind(),
        "future.雪"
    );
    for _ in 0..2 {
        assert_eq!(
            stream.next().await.unwrap().unwrap().event.kind(),
            "server.connected"
        );
    }
    assert!(stream.next().await.is_none());
    assert!(stream.next().await.is_none());
    joined(task).await;
}

#[tokio::test]
async fn ordinary_and_handshake_deadlines_do_not_limit_stream_lifetime() {
    let (client, task) = server(|mut s| async move {
        headers(&mut s).await;
        sleep(Duration::from_millis(200)).await;
        chunk(&mut s, CONNECTED.as_bytes()).await;
        s.write_all(b"0\r\n\r\n").await.unwrap();
    })
    .await;
    let client = Client::builder(client.base_url().to_string())
        .request_timeout(Duration::from_millis(50))
        .max_response_bytes(1)
        .build()
        .unwrap();
    let mut stream = client
        .events()
        .subscribe(&EventOptions::default().handshake_timeout(Duration::from_millis(100)))
        .await
        .unwrap();
    assert!(stream.next().await.unwrap().is_ok());
    assert!(stream.next().await.is_none());
    joined(task).await;
}

#[tokio::test]
async fn handshake_timeout_covers_headers_and_error_body_and_keeps_status() {
    for status in [None, Some(503)] {
        let (client, task) = server(move |mut s| async move {
            if status.is_some() {
                s.write_all(
                    b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 100\r\n\r\nprivate",
                )
                .await
                .unwrap();
            }
            let mut buf = [0; 1];
            assert_eq!(s.read(&mut buf).await.unwrap(), 0);
        })
        .await;
        let error = client
            .events()
            .subscribe(&EventOptions::default().handshake_timeout(Duration::from_millis(100)))
            .await
            .unwrap_err();
        assert!(error.is_timeout());
        assert_eq!(error.status().map(|s| s.as_u16()), status);
        let Error::Stream(e) = error else {
            panic!("handshake error")
        };
        assert_eq!(e.kind(), StreamErrorKind::HandshakeTimeout);
        joined(task).await;
    }
}

#[tokio::test]
async fn heartbeats_reset_idle_deadline_then_inactivity_terminates_and_closes() {
    let (client, task) = server(|mut s| async move {
        headers(&mut s).await;
        for _ in 0..4 {
            sleep(Duration::from_millis(50)).await;
            chunk(&mut s, b": heartbeat\n\n").await;
        }
        chunk(&mut s, CONNECTED.as_bytes()).await;
        let mut b = [0; 1];
        assert_eq!(s.read(&mut b).await.unwrap(), 0);
    })
    .await;
    let mut stream = client
        .events()
        .subscribe(&EventOptions::default().idle_timeout(Duration::from_millis(150)))
        .await
        .unwrap();
    assert!(stream.next().await.unwrap().is_ok());
    let error = stream.next().await.unwrap().unwrap_err();
    assert!(error.is_timeout());
    let Error::Stream(e) = error else {
        panic!("idle error")
    };
    assert_eq!(e.kind(), StreamErrorKind::IdleTimeout);
    assert!(stream.next().await.is_none());
    joined(task).await;
}

#[tokio::test]
async fn cancelling_next_keeps_partial_frame_and_close_releases_connection() {
    let (release, wait) = oneshot::channel();
    let (client, task) = server(|mut s| async move {
        headers(&mut s).await;
        chunk(&mut s, &CONNECTED.as_bytes()[..10]).await;
        wait.await.unwrap();
        chunk(&mut s, &CONNECTED.as_bytes()[10..]).await;
        let mut b = [0; 1];
        assert_eq!(s.read(&mut b).await.unwrap(), 0);
    })
    .await;
    let mut stream = client
        .events()
        .subscribe(&EventOptions::default())
        .await
        .unwrap();
    assert!(
        timeout(Duration::from_millis(50), stream.next())
            .await
            .is_err()
    );
    release.send(()).unwrap();
    assert!(stream.next().await.unwrap().is_ok());
    stream.close();
    assert!(stream.next().await.is_none());
    joined(task).await;
}

#[tokio::test]
async fn subscriptions_are_independent_and_dropping_one_closes_only_its_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = Client::new(format!("http://{}", listener.local_addr().unwrap())).unwrap();
    let task = tokio::spawn(async move {
        let (mut first, _) = listener.accept().await.unwrap();
        request(&mut first).await;
        headers(&mut first).await;
        let (mut second, _) = listener.accept().await.unwrap();
        request(&mut second).await;
        headers(&mut second).await;
        let mut b = [0; 1];
        assert_eq!(first.read(&mut b).await.unwrap(), 0);
        chunk(&mut second, CONNECTED.as_bytes()).await;
        assert_eq!(second.read(&mut b).await.unwrap(), 0);
    });
    let first = client
        .events()
        .subscribe(&EventOptions::default())
        .await
        .unwrap();
    let mut second = client
        .events()
        .subscribe(&EventOptions::default())
        .await
        .unwrap();
    drop(first);
    assert!(second.next().await.unwrap().is_ok());
    drop(second);
    joined(task).await;
}

#[tokio::test]
async fn injected_reqwest_deadline_remains_effective() {
    let (client, task) = server(|mut s| async move {
        headers(&mut s).await;
        let mut b = [0; 1];
        assert_eq!(s.read(&mut b).await.unwrap(), 0);
    })
    .await;
    let http = reqwest::Client::builder()
        .timeout(Duration::from_millis(150))
        .build()
        .unwrap();
    let client = Client::builder(client.base_url().to_string())
        .http_client(http)
        .build()
        .unwrap();
    let mut stream = client
        .events()
        .subscribe(&EventOptions::default())
        .await
        .unwrap();
    let error = stream.next().await.unwrap().unwrap_err();
    assert!(error.is_timeout());
    assert!(matches!(error, Error::Transport(_)));
    assert!(stream.next().await.is_none());
    joined(task).await;
}

#[tokio::test]
async fn abrupt_chunked_body_disconnect_is_a_terminal_transport_error() {
    let (client, task) = server(|mut s| async move {
        headers(&mut s).await;
        chunk(&mut s, CONNECTED.as_bytes()).await;
        // Deliberately omit the terminating HTTP chunk.
    })
    .await;
    let mut stream = client
        .events()
        .subscribe(&EventOptions::default())
        .await
        .unwrap();
    assert!(stream.next().await.unwrap().is_ok());
    let error = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(error, Error::Transport(_)));
    assert_eq!(error.status().unwrap(), 200);
    assert!(stream.next().await.is_none());
    joined(task).await;
}
