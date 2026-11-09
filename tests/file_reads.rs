use opencoders::{Client, Error, FileReadOptions, Location, StatusCode, header};
use std::time::Duration;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header as matches_header, method},
};

#[tokio::test]
async fn cancelled_file_reads_preserve_idle_deadlines_and_close_the_connection() {
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = vec![0; 4096];
        let mut headers = Vec::new();
        while !headers.ends_with(b"\r\n\r\n") {
            assert!(headers.len() < 8192);
            headers.push(socket.read_u8().await.unwrap());
        }
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\n")
            .await
            .unwrap();
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(2), socket.read(&mut request))
                .await
                .unwrap()
                .unwrap(),
            0
        );
    });
    let client = Client::new(format!("http://{addr}")).unwrap();
    let mut body = client
        .files()
        .read(
            "file",
            &FileReadOptions::default().idle_timeout(Duration::from_millis(100)),
        )
        .await
        .unwrap()
        .into_body();
    assert!(
        tokio::time::timeout(Duration::from_millis(20), body.next())
            .await
            .is_err()
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
    let error = tokio::time::timeout(Duration::from_millis(20), body.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap_err();
    assert!(error.is_timeout());
    assert_eq!(error.status().unwrap(), 200);
    assert!(body.next().await.is_none());
    server.await.unwrap();
}

#[tokio::test]
async fn reads_preserve_encoded_paths_binary_bytes_partial_status_and_headers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(matches_header("range", "bytes=1-3"))
        .and(matches_header("if-range", "\"tag\""))
        .respond_with(
            ResponseTemplate::new(206)
                .insert_header("content-range", "bytes 1-3/9")
                .insert_header("etag", "\"tag\"")
                .set_body_bytes(vec![0, 255, 42]),
        )
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(format!("{}/proxy", server.uri()))
        .default_location(Location::directory("/work ?"))
        .max_response_bytes(1)
        .build()
        .unwrap();
    let mut result = client
        .files()
        .read(
            "folder/a?b#c% d",
            &FileReadOptions::default()
                .range("bytes=1-3")
                .if_range("\"tag\""),
        )
        .await
        .unwrap();
    assert_eq!(result.status(), StatusCode::PARTIAL_CONTENT);
    assert_eq!(result.headers()[header::CONTENT_RANGE], "bytes 1-3/9");
    let mut bytes = vec![];
    while let Some(chunk) = result.body_mut().next().await {
        bytes.extend_from_slice(&chunk.unwrap());
    }
    assert_eq!(bytes, vec![0, 255, 42]);
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests[0].url.path(),
        "/proxy/api/fs/read/folder%2Fa%3Fb%23c%25%20d"
    );
    assert_eq!(requests[0].url.query_pairs().next().unwrap().1, "/work ?");
}

#[tokio::test]
async fn conditional_and_unsatisfiable_reads_return_metadata_without_a_body() {
    for status in [304, 416] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(matches_header("if-none-match", "\"old\""))
            .respond_with(
                ResponseTemplate::new(status)
                    .insert_header("etag", "\"current\"")
                    .insert_header("content-range", "bytes */4"),
            )
            .mount(&server)
            .await;
        let client = Client::new(server.uri()).unwrap();
        let mut response = client
            .files()
            .read_at(
                "file",
                &FileReadOptions::default().if_none_match("\"old\""),
                Location::Unspecified,
            )
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), status);
        assert_eq!(response.headers()[header::ETAG], "\"current\"");
        assert!(response.body_mut().next().await.is_none());
    }
}

#[tokio::test]
async fn file_byte_budget_terminates_stream_and_close_releases_it() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![1; 32]))
        .mount(&server)
        .await;
    let client = Client::new(server.uri()).unwrap();
    let mut response = client
        .files()
        .read("file", &FileReadOptions::default().max_bytes(3))
        .await
        .unwrap();
    assert!(matches!(
        response.body_mut().next().await.unwrap(),
        Err(Error::ResponseTooLarge(_))
    ));
    assert!(response.body_mut().next().await.is_none());
    let mut response = client
        .files()
        .read("file", &FileReadOptions::default())
        .await
        .unwrap();
    response.body_mut().close();
    assert!(response.body_mut().next().await.is_none());
}

#[tokio::test]
async fn file_errors_are_bounded_and_invalid_headers_fail_before_send() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(403).set_body_string("private-server-response"))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .max_error_body_bytes(4)
        .build()
        .unwrap();
    let error = client
        .files()
        .read("file", &FileReadOptions::default())
        .await
        .unwrap_err();
    assert_eq!(error.operation(), "fs.read");
    let Error::Http(error) = error else {
        panic!("HTTP error expected")
    };
    assert_eq!(error.body(), b"priv");
    assert!(error.is_body_truncated());
    let error = client
        .files()
        .read(
            "file",
            &FileReadOptions::default().range("private\r\ninvalid"),
        )
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Encode(_)));
    assert!(!format!("{error:?}").contains("private"));
}

#[tokio::test]
async fn file_handshake_deadline_is_separate_from_ordinary_request_deadline() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_millis(80))
                .set_body_string("ok"),
        )
        .mount(&server)
        .await;
    let client = Client::builder(server.uri())
        .request_timeout(Duration::from_millis(1))
        .build()
        .unwrap();
    let error = client
        .files()
        .read(
            "file",
            &FileReadOptions::default().handshake_timeout(Duration::from_millis(10)),
        )
        .await
        .unwrap_err();
    assert!(error.is_timeout());
    let mut response = client
        .files()
        .read("file", &FileReadOptions::default())
        .await
        .unwrap();
    assert_eq!(response.body_mut().next().await.unwrap().unwrap(), "ok");
}
