//! Test-only GET relay. Forwards real responses; controls transport cuts and one
//! captured history response. Never manufactures OpenCode events or snapshots.
use std::{
    collections::BTreeMap,
    io,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::oneshot,
    task::{JoinHandle, JoinSet},
    time::timeout,
};

const DEADLINE: Duration = Duration::from_secs(10);
const MAX_BODY: usize = 1024 * 1024;

#[derive(Clone)]
pub struct RequestTrace {
    pub method: String,
    pub target: String,
    pub last_event_id: bool,
}

struct Gate {
    captured: oneshot::Sender<()>,
    release: oneshot::Receiver<()>,
}
pub struct HistoryHold {
    captured: oneshot::Receiver<()>,
    release: oneshot::Sender<()>,
}
impl HistoryHold {
    pub async fn captured(&mut self) {
        timeout(DEADLINE, &mut self.captured)
            .await
            .unwrap()
            .unwrap();
    }
    pub fn release(self) {
        self.release.send(()).unwrap();
    }
}

#[derive(Default)]
struct State {
    requests: Vec<RequestTrace>,
    streams: BTreeMap<usize, oneshot::Sender<()>>,
    history_gate: Option<Gate>,
}

pub struct Relay {
    origin: String,
    state: Arc<Mutex<State>>,
    task: JoinHandle<()>,
}
impl Relay {
    pub async fn start(upstream: &str) -> Self {
        let url: reqwest::Url = upstream.parse().unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.host_str(), Some("127.0.0.1"));
        assert!(url.port().is_some() && url.username().is_empty() && url.password().is_none());
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let state = Arc::new(Mutex::new(State::default()));
        let shared = state.clone();
        let http = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let task = tokio::spawn(async move {
            let mut handlers = JoinSet::new();
            loop {
                tokio::select! {
                    accepted = listener.accept() => {
                        let (socket, _) = accepted.unwrap();
                        handlers.spawn(serve(socket, url.clone(), http.clone(), shared.clone()));
                    }
                    Some(result) = handlers.join_next() => { result.unwrap(); }
                }
            }
        });
        Self {
            origin,
            state,
            task,
        }
    }
    pub fn url(&self) -> &str {
        &self.origin
    }
    pub fn requests(&self) -> Vec<RequestTrace> {
        self.state.lock().unwrap().requests.clone()
    }

    pub fn hold_history(&self) -> HistoryHold {
        let (captured, notification) = oneshot::channel();
        let (release, wait) = oneshot::channel();
        let mut state = self.state.lock().unwrap();
        assert!(state.history_gate.is_none());
        state.history_gate = Some(Gate {
            captured,
            release: wait,
        });
        HistoryHold {
            captured: notification,
            release,
        }
    }

    /// Abruptly close every event response, without a terminating HTTP chunk.
    /// Snapshot connections and the server's provider connection are untouched.
    pub async fn cut_events(&self) -> usize {
        let streams = std::mem::take(&mut self.state.lock().unwrap().streams);
        let count = streams.len();
        for (_, cut) in streams {
            cut.send(()).unwrap();
        }
        count
    }
    pub async fn close(mut self) {
        self.task.abort();
        let _ = (&mut self.task).await; // Dropping JoinSet aborts all handlers.
    }
}
impl Drop for Relay {
    fn drop(&mut self) {
        self.task.abort();
    }
}

async fn bytes(
    socket: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> io::Result<()> {
    socket.write_all(format!("HTTP/1.1 {status} Relay\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",body.len()).as_bytes()).await?;
    socket.write_all(body).await
}

async fn serve(
    mut socket: TcpStream,
    upstream: reqwest::Url,
    http: reqwest::Client,
    state: Arc<Mutex<State>>,
) {
    // Bounds and cancellation apply to fixture plumbing too. Client disconnection
    // is normal during tests; neither request headers nor bodies are logged.
    let _ = relay(&mut socket, upstream, http, state).await;
}

async fn relay(
    socket: &mut TcpStream,
    upstream: reqwest::Url,
    http: reqwest::Client,
    state: Arc<Mutex<State>>,
) -> io::Result<()> {
    let request = timeout(DEADLINE, async {
        let mut request = Vec::new();
        while !request.ends_with(b"\r\n\r\n") {
            request.push(socket.read_u8().await?);
            if request.len() > 8192 {
                return Err(io::Error::other("oversized fixture request"));
            }
        }
        String::from_utf8(request).map_err(io::Error::other)
    })
    .await
    .map_err(io::Error::other)??;
    let mut lines = request.split("\r\n");
    let mut first = lines.next().unwrap().split_whitespace();
    let method = first.next().unwrap_or("");
    let target = first.next().unwrap_or("");
    let mut headers = reqwest::header::HeaderMap::new();
    let mut last_event_id = false;
    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.eq_ignore_ascii_case("last-event-id") {
            last_event_id = true;
        }
        if name.eq_ignore_ascii_case("authorization") || name.eq_ignore_ascii_case("accept") {
            let mut value =
                reqwest::header::HeaderValue::from_str(value.trim()).map_err(io::Error::other)?;
            value.set_sensitive(name.eq_ignore_ascii_case("authorization"));
            headers.insert(
                reqwest::header::HeaderName::from_bytes(name.as_bytes())
                    .map_err(io::Error::other)?,
                value,
            );
        }
    }
    let route = target.split('?').next().unwrap();
    let index = {
        let mut state = state.lock().unwrap();
        assert!(state.requests.len() < 128, "fixture request budget");
        state.requests.push(RequestTrace {
            method: method.into(),
            target: target.into(),
            last_event_id,
        });
        state.requests.len()
    };
    if method != "GET" {
        return bytes(socket, 405, "application/json", b"{}").await;
    }
    if route != "/api/event" && !route.starts_with("/api/session/") {
        return bytes(socket, 404, "application/json", b"{}").await;
    }
    let mut response = timeout(
        DEADLINE,
        http.get(upstream.join(target).map_err(io::Error::other)?)
            .headers(headers)
            .send(),
    )
    .await
    .map_err(io::Error::other)?
    .map_err(io::Error::other)?;
    let status = response.status().as_u16();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_owned();
    if route == "/api/event" && status == 200 && content_type.starts_with("text/event-stream") {
        let (cut, mut receive) = oneshot::channel();
        state.lock().unwrap().streams.insert(index, cut);
        let result = async {
            socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n").await?;
            loop {
                tokio::select! {
                    biased;
                    _ = &mut receive => break,
                    _ = socket.read_u8() => break,
                    chunk = response.chunk() => {
                        let Some(chunk) = chunk.map_err(io::Error::other)? else {
                            socket.write_all(b"0\r\n\r\n").await?;
                            break;
                        };
                        if chunk.is_empty() { continue; }
                        socket.write_all(format!("{:x}\r\n",chunk.len()).as_bytes()).await?;
                        socket.write_all(&chunk).await?;
                        socket.write_all(b"\r\n").await?;
                    }
                }
            }
            Ok(())
        }.await;
        state.lock().unwrap().streams.remove(&index);
        result
    } else {
        let body = timeout(DEADLINE, async {
            let mut body = Vec::new();
            while let Some(chunk) = response.chunk().await.map_err(io::Error::other)? {
                if body.len() + chunk.len() > MAX_BODY {
                    return Err(io::Error::other("fixture body budget"));
                }
                body.extend_from_slice(&chunk);
            }
            Ok(body)
        })
        .await
        .map_err(io::Error::other)??;
        let gate = if route.ends_with("/message") {
            state.lock().unwrap().history_gate.take()
        } else {
            None
        };
        if let Some(gate) = gate {
            let _ = gate.captured.send(());
            timeout(DEADLINE, gate.release)
                .await
                .map_err(io::Error::other)?
                .map_err(io::Error::other)?;
        }
        bytes(socket, status, &content_type, &body).await
    }
}
