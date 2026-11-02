use opencoders::{Client, Error};
use std::{error::Error as _, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::path};

#[tokio::test]
async fn create_disconnect_after_the_body_does_not_retry_or_expose_request_data() {
    use opencoders::models::SessionCreateRequest;
    let request = SessionCreateRequest::default().title("private request text");
    let body = serde_json::to_vec(&request).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        read_request_headers(&mut socket).await;
        let mut received = vec![0; body.len()];
        socket.read_exact(&mut received).await.unwrap();
        assert_eq!(received, body);
        drop(socket);
        assert!(
            tokio::time::timeout(Duration::from_millis(200), listener.accept())
                .await
                .is_err()
        );
    });
    let error = Client::builder(format!("http://{address}/private-prefix"))
        .request_timeout(Duration::from_secs(1))
        .build()
        .unwrap()
        .sessions()
        .create(&request)
        .await
        .unwrap_err();
    assert_eq!(error.operation(), "session.create");
    assert!(error.status().is_none());
    assert!(!error.is_timeout());
    assert!(!format!("{error:?} {error}").contains("private request text"));
    let Error::Transport(error) = error else {
        panic!("expected transport error")
    };
    assert!(
        !error
            .source()
            .unwrap()
            .to_string()
            .contains("private-prefix")
    );
    task.await.unwrap();
}

async fn read_request_headers(socket: &mut TcpStream) {
    let mut bytes = Vec::new();
    while !bytes.ends_with(b"\r\n\r\n") {
        assert!(
            bytes.len() < 4096,
            "test request headers exceed buffer limit"
        );
        bytes.push(socket.read_u8().await.unwrap());
    }
}

#[tokio::test]
async fn disconnect_is_a_transport_error_with_no_url_in_its_source() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        read_request_headers(&mut socket).await;
        // Drop the connection before sending response headers.
    });
    let error = Client::new(format!("http://{address}/private-path"))
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err();
    assert_eq!(error.operation(), "server.info");
    assert!(error.status().is_none());
    assert!(!error.is_timeout());
    let Error::Transport(error) = error else {
        panic!("expected transport error");
    };
    assert!(!error.source().unwrap().to_string().contains("private-path"));
    task.await.unwrap();
}

#[tokio::test]
async fn request_deadline_includes_response_body_reads() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (headers_written, received_headers) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        read_request_headers(&mut socket).await;
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        headers_written.send(()).unwrap();
        tokio::time::sleep(Duration::from_secs(2)).await;
    });
    let client = Client::builder(format!("http://{address}"))
        .request_timeout(Duration::from_millis(200))
        .build()
        .unwrap();
    let request = tokio::spawn(async move { client.server().info().await });
    received_headers.await.unwrap();
    let error = request.await.unwrap().unwrap_err();
    assert!(error.is_timeout());
    assert_eq!(error.status().unwrap().as_u16(), 200);
    task.abort();
}

#[tokio::test]
async fn chunked_error_body_is_bounded_without_content_length() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        read_request_headers(&mut socket).await;
        socket.write_all(b"HTTP/1.1 500 Internal Server Error\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n3\r\nabc\r\n3\r\ndef\r\n0\r\n\r\n").await.unwrap();
    });
    let Error::Http(error) = Client::builder(format!("http://{address}"))
        .max_error_body_bytes(4)
        .build()
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error");
    };
    assert_eq!(error.body(), b"abcd");
    assert!(error.is_body_truncated());
    task.await.unwrap();
}

#[tokio::test]
async fn exact_diagnostic_limit_does_not_claim_truncation() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(500).set_body_string("1234"))
        .mount(&server)
        .await;
    let Error::Http(error) = Client::builder(server.uri())
        .max_error_body_bytes(4)
        .build()
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error");
    };
    assert_eq!(error.body(), b"1234");
    assert!(!error.is_body_truncated());
}

#[tokio::test]
async fn zero_diagnostic_limit_retains_status_without_body() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(500).set_body_string("private"))
        .mount(&server)
        .await;
    let Error::Http(error) = Client::builder(server.uri())
        .max_error_body_bytes(0)
        .build()
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err()
    else {
        panic!("expected HTTP error");
    };
    assert_eq!(error.status().as_u16(), 500);
    assert!(error.body().is_empty());
    assert!(error.is_body_truncated());
}

#[tokio::test]
async fn decode_error_diagnostics_are_also_bounded() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(200).set_body_string("invalid-json-body"))
        .mount(&server)
        .await;
    let Error::Decode(error) = Client::builder(server.uri())
        .max_error_body_bytes(7)
        .build()
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err()
    else {
        panic!("expected decode error");
    };
    assert_eq!(error.body(), b"invalid");
    assert!(error.is_body_truncated());
    assert_eq!(error.operation(), "server.info");
    assert_eq!(error.status().as_u16(), 200);
    assert!(error.source().is_some());
}

#[tokio::test]
async fn unexpected_success_status_is_not_decoded_as_server_info() {
    let server = MockServer::start().await;
    Mock::given(path("/api/info"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let error = Client::new(server.uri())
        .unwrap()
        .server()
        .info()
        .await
        .unwrap_err();
    assert_eq!(error.status().unwrap().as_u16(), 204);
    assert!(matches!(error, Error::Http(_)));
}
