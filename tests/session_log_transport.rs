use opencoders::{Client, Error, SessionLogOptions, StreamErrorKind, models::SessionLogItem};
use serde_json::json;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::oneshot,
    time::{sleep, timeout},
};
#[path = "support/sse.rs"]
mod support;
use support::*;

fn event(seq: u64) -> String {
    format!(
        "id: opaque-{seq}\ndata: {}\n\n",
        json!({"id":format!("evt_{seq}"),"created":1,"type":"session.renamed","durable":{"aggregateID":"ses_log","seq":seq,"version":1},"data":{"sessionID":"ses_log","title":"雪"}})
    )
}
fn marker(seq: u64) -> String {
    format!(
        "data: {}\n\n",
        json!({"type":"log.synced","aggregateID":"ses_log","seq":seq})
    )
}

#[tokio::test]
async fn reconnect_uses_only_acknowledged_sequences_across_replay_follow_and_disconnect() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = Client::new(format!("http://{}", listener.local_addr().unwrap())).unwrap();
    let task = tokio::spawn(async move {
        let (mut first, _) = listener.accept().await.unwrap();
        let req = String::from_utf8(request(&mut first).await).unwrap();
        assert!(req.starts_with("GET /api/experimental/session/ses_log/log?follow=true HTTP/1.1"));
        headers(&mut first).await;
        chunk(
            &mut first,
            format!(
                "{}{}{}{}data: unfinished",
                event(0),
                event(4),
                marker(7),
                event(9)
            )
            .as_bytes(),
        )
        .await;
        drop(first);
        let (mut second, _) = listener.accept().await.unwrap();
        let req = String::from_utf8(request(&mut second).await).unwrap();
        assert!(req.starts_with(
            "GET /api/experimental/session/ses_log/log?after=9&follow=false HTTP/1.1"
        ));
        assert!(!req.to_ascii_lowercase().contains("last-event-id"));
        headers(&mut second).await;
        chunk(
            &mut second,
            format!("{}{}", event(10), marker(12)).as_bytes(),
        )
        .await;
        second.write_all(b"0\r\n\r\n").await.unwrap();
    });
    let id = "ses_log".parse().unwrap();
    let mut first = client
        .sessions()
        .experimental_log(&id, &SessionLogOptions::default().follow(true))
        .await
        .unwrap();
    let mut acknowledged = None;
    let mut received = Vec::new();
    while let Some(message) = first.next().await {
        match message {
            Ok(m) => {
                let sequence = m.item.sequence().unwrap();
                received.push(sequence);
                acknowledged = Some(sequence);
            }
            Err(e) => {
                assert!(matches!(e, Error::Transport(_)));
                assert_eq!(e.operation(), "experimental.session.log");
                break;
            }
        }
    }
    assert_eq!(received, [0, 4, 7, 9]);
    assert!(first.next().await.is_none());
    let mut second = client
        .sessions()
        .experimental_log(
            &id,
            &SessionLogOptions::default()
                .after(acknowledged.unwrap())
                .follow(false),
        )
        .await
        .unwrap();
    assert_eq!(
        second.next().await.unwrap().unwrap().item.sequence(),
        Some(10)
    );
    assert_eq!(
        second.next().await.unwrap().unwrap().item.sequence(),
        Some(12)
    );
    assert!(second.next().await.is_none());
    joined(task).await;
}

#[tokio::test]
async fn cancelled_read_preserves_fragmented_log_and_close_cancels_following() {
    let (release, wait) = oneshot::channel();
    let (client, task) = server(|mut s| async move {
        headers(&mut s).await;
        chunk(&mut s, b"\xef\xbb\xbf: heartbeat\r\ndata: ").await;
        wait.await.unwrap();
        let data = json!({"type":"log.synced","aggregateID":"ses_log","seq":0}).to_string();
        for b in format!("{data}\r\n\r\n").as_bytes() {
            chunk(&mut s, &[*b]).await;
        }
        let mut b = [0; 1];
        assert_eq!(s.read(&mut b).await.unwrap(), 0);
    })
    .await;
    let client = Client::builder(client.base_url().to_string())
        .request_timeout(Duration::from_millis(20))
        .max_response_bytes(1)
        .build()
        .unwrap();
    let mut stream = client
        .sessions()
        .experimental_log(
            &"ses_log".parse().unwrap(),
            &SessionLogOptions::default()
                .follow(true)
                .handshake_timeout(Duration::from_millis(100)),
        )
        .await
        .unwrap();
    assert!(
        timeout(Duration::from_millis(150), stream.next())
            .await
            .is_err()
    );
    release.send(()).unwrap();
    assert_eq!(
        stream.next().await.unwrap().unwrap().item.sequence(),
        Some(0)
    );
    stream.close();
    assert!(stream.next().await.is_none());
    joined(task).await;
}

#[tokio::test]
async fn log_handshake_and_idle_timeouts_keep_operation_and_status() {
    for handshake in [true, false] {
        let (client, task) = server(move |mut s| async move {
            if !handshake {
                headers(&mut s).await;
                chunk(&mut s, marker(3).as_bytes()).await;
            }
            let mut b = [0; 1];
            assert_eq!(s.read(&mut b).await.unwrap(), 0);
        })
        .await;
        let options = SessionLogOptions::default()
            .handshake_timeout(Duration::from_millis(100))
            .idle_timeout(Duration::from_millis(100));
        let result = client
            .sessions()
            .experimental_log(&"ses_log".parse().unwrap(), &options)
            .await;
        let error = if handshake {
            result.unwrap_err()
        } else {
            let mut stream = result.unwrap();
            assert!(matches!(
                stream.next().await.unwrap().unwrap().item,
                SessionLogItem::Synced(_)
            ));
            let error = stream.next().await.unwrap().unwrap_err();
            assert!(stream.next().await.is_none());
            error
        };
        assert!(error.is_timeout());
        assert_eq!(error.operation(), "experimental.session.log");
        assert_eq!(
            error.status().map(|s| s.as_u16()),
            if handshake { None } else { Some(200) }
        );
        let Error::Stream(e) = error else {
            panic!("stream")
        };
        assert_eq!(
            e.kind(),
            if handshake {
                StreamErrorKind::HandshakeTimeout
            } else {
                StreamErrorKind::IdleTimeout
            }
        );
        joined(task).await;
    }
}

#[tokio::test]
async fn dropping_a_log_leaves_another_subscription_alive() {
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
        chunk(&mut second, marker(8).as_bytes()).await;
        assert_eq!(second.read(&mut b).await.unwrap(), 0);
    });
    let id = "ses_log".parse().unwrap();
    let options = SessionLogOptions::default().follow(true);
    let first = client
        .sessions()
        .experimental_log(&id, &options)
        .await
        .unwrap();
    let mut second = client
        .sessions()
        .experimental_log(&id, &options)
        .await
        .unwrap();
    drop(first);
    assert_eq!(
        second.next().await.unwrap().unwrap().item.sequence(),
        Some(8)
    );
    drop(second);
    joined(task).await;
}

#[tokio::test]
async fn clean_eof_does_not_invent_a_missing_watermark_or_reconnect() {
    let (client, task) = server(|mut s| async move {
        headers(&mut s).await;
        chunk(
            &mut s,
            format!("{}data: {{\"type\":\"log.synced\"}}", event(0)).as_bytes(),
        )
        .await;
        s.write_all(b"0\r\n\r\n").await.unwrap();
        sleep(Duration::from_millis(10)).await;
    })
    .await;
    let mut stream = client
        .sessions()
        .experimental_log(&"ses_log".parse().unwrap(), &SessionLogOptions::default())
        .await
        .unwrap();
    assert!(matches!(
        stream.next().await.unwrap().unwrap().item,
        SessionLogItem::Event(_)
    ));
    assert!(stream.next().await.is_none());
    assert!(stream.next().await.is_none());
    joined(task).await;
}
