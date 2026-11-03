//! Check fault injection independently of the opt-in OpenCode run.
#[path = "support/recovery_proxy.rs"]
mod proxy;

use opencoders::{Client, EventOptions, models::Event};
use proxy::Relay;
use serde_json::json;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    time::timeout,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

#[tokio::test]
async fn relays_real_frames_then_cuts_transport_without_reopening_upstream() {
    let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", upstream.local_addr().unwrap());
    let served = tokio::spawn(async move {
        let (mut socket, _) = upstream.accept().await.unwrap();
        let mut headers = Vec::new();
        while !headers.ends_with(b"\r\n\r\n") {
            headers.push(socket.read_u8().await.unwrap());
        }
        let request = String::from_utf8(headers).unwrap();
        assert!(request.starts_with("GET /api/event?types=server.connected "));
        assert!(
            request
                .lines()
                .filter_map(|line| line.split_once(':'))
                .any(|(name, value)| name.eq_ignore_ascii_case("authorization")
                    && value.trim() == "Basic dXNlcjpwYXNz")
        );
        let event = "data: {\"type\":\"server.connected\",\"id\":\"evt_relay\",\"data\":{}}\n\n";
        socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n{event}\r\n",event.len()).as_bytes()).await.unwrap();
        // Dropping the relay's response must also release the real upstream stream.
        assert_eq!(
            timeout(Duration::from_secs(5), socket.read_u8())
                .await
                .unwrap()
                .unwrap_err()
                .kind(),
            std::io::ErrorKind::UnexpectedEof
        );
        assert!(
            timeout(Duration::from_millis(100), upstream.accept())
                .await
                .is_err()
        );
    });
    let relay = Relay::start(&origin).await;
    let client = Client::builder(relay.url())
        .basic_auth("user", "pass")
        .http_client(reqwest::Client::builder().no_proxy().build().unwrap())
        .build()
        .unwrap();
    let mut events = client
        .events()
        .subscribe(&EventOptions::default().types(["server.connected"]))
        .await
        .unwrap();
    assert!(matches!(
        events.next().await.unwrap().unwrap().event,
        Event::ServerConnected(_)
    ));
    assert_eq!(relay.cut_events().await, 1);
    assert!(
        timeout(Duration::from_secs(5), events.next())
            .await
            .unwrap()
            .unwrap()
            .is_err()
    );
    assert!(events.next().await.is_none());
    assert_eq!(relay.requests().len(), 1);
    assert_eq!(
        relay.requests()[0].target,
        "/api/event?types=server.connected"
    );
    assert!(!relay.requests()[0].last_event_id);
    served.await.unwrap();
    relay.close().await;
}

#[tokio::test]
async fn holds_captured_history_while_new_gets_see_new_state_and_rejects_mutations() {
    let upstream = MockServer::start().await;
    let old = json!({"data":[],"cursor":{}});
    Mock::given(method("GET"))
        .and(path("/api/session/ses_relay/message"))
        .and(header("authorization", "Basic dXNlcjpwYXNz"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&old))
        .mount(&upstream)
        .await;
    let relay = Relay::start(&upstream.uri()).await;
    let http = reqwest::Client::builder().no_proxy().build().unwrap();
    let mut hold = relay.hold_history();
    let request = http
        .get(format!(
            "{}/api/session/ses_relay/message?limit=50",
            relay.url()
        ))
        .basic_auth("user", Some("pass"))
        .send();
    tokio::pin!(request);
    tokio::select! {
        _ = &mut request => panic!("history response escaped the gate"),
        _ = hold.captured() => {},
    }
    upstream.reset().await;
    let new = json!({"data":[{"revision":2}],"cursor":{}});
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&new))
        .mount(&upstream)
        .await;
    let fresh = http
        .get(format!("{}/api/session/ses_relay/message", relay.url()))
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&fresh).unwrap(),
        new
    );
    hold.release();
    let stale = request.await.unwrap().bytes().await.unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&stale).unwrap(),
        old
    );
    let rejected = http
        .post(format!("{}/api/session/ses_relay/prompt", relay.url()))
        .send()
        .await
        .unwrap();
    assert_eq!(rejected.status(), 405);
    assert_eq!(upstream.received_requests().await.unwrap().len(), 1);
    assert_eq!(
        relay
            .requests()
            .iter()
            .filter(|request| request.method == "GET")
            .count(),
        2
    );
    relay.close().await;
}
