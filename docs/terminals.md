# Ordinary PTYs

`Client::ptys()` exposes list, create, get, update/resize, remove and connection
tickets in every build. Enable `websocket` for `connect`/`connect_at`:

```toml
opencoders = { path = "../opencoders", features = ["websocket"] }
```

These APIs are unreleased; use the repository revision containing them until the
next crate release. `websocket` is optional and does not imply TLS; the default
`rustls` feature supplies TLS through reqwest. HTTP-only builds can use
`--no-default-features --features websocket`.

Create with `PtyCreateRequest::new().command(...).args(...).cwd(...).title(...)`;
`env` accepts a string map. Omitted values remain omitted; explicit empty args/maps
are retained, and null is rejected. The pinned server inherits its environment,
defaults an omitted/empty command to a shell, and can add login arguments to known
shells. Choose a concrete command when those defaults are unwanted. Request Debug
omits command arguments and environment values.

`update(id, &PtyUpdateRequest::new().title("build").size(24, 80))` sets title and
viewport rows/columns. Dimensions must be positive safe integers. The pinned
server ignores an empty title and only resizes running PTYs. An exited PTY can
remain listed with `exit_code`; the server retains a bounded number. `remove`
terminates and removes the process and requires HTTP 204. Dropping a client or
socket does not remove it. The SDK adds no lifecycle retries; an injected HTTP
client retains the retry policy explicitly configured by its caller.

## Connections, tickets and ownership

```rust,no_run
# async fn example(client: &opencoders::Client, id: &opencoders::models::PtyId) -> opencoders::Result<()> {
use opencoders::{Location, PtyConnectOptions, PtyOutput};
let ticket = client.ptys().connect_token(id).await?;
let location = Location::directory(ticket.location.directory.as_str());
let mut socket = client.ptys().connect_at(id,
    &PtyConnectOptions::default().ticket(ticket.data.ticket).cursor(-1), location).await?;
socket.send("hello\n").await?;
while let Some(message) = socket.next().await {
    match message? {
        PtyOutput::Data(text) => { /* feed caller's terminal parser */ }
        PtyOutput::Cursor(cursor) => { /* record replay boundary */ }
        PtyOutput::Closed { code, reason } => break,
        _ => {}
    }
}
socket.close().await?;
# Ok(()) }
```

Tickets expire after 60 seconds on v2.2.5 and are single-use, bound to the PTY and
resolved location. Minting supplies the mandatory `x-opencode-ticket: 1` header.
Use the returned directory when consuming one. Configured HTTP authentication
also works without a ticket. Credentials/tickets are omitted from transport error
Debug and ticket model Debug; terminal output and peer close reasons are data and
their Debug includes their contents.

The socket upgrades through the configured reqwest client, preserving HTTP auth,
TLS and proxy policies. It requires a valid HTTP/1.1 101 response and validates
the WebSocket accept header; the schema's boolean/200 description is not the wire
protocol. No compression or subprotocol is requested. Handshake failures retain
bounded HTTP diagnostics and status; socket errors expose a redacted category.

`PtyConnection` implements `Stream<Item = Result<PtyOutput>>` and `Sink<String>`.
Use `split()` with `StreamExt`/`SinkExt` for concurrent caller-owned tasks. Text
frames carry terminal output; binary `0x00` followed by JSON carries the replay
cursor. Ping/pong is serviced while polling the connection. Input is UTF-8 text;
the library supplies no terminal emulator, stdin policy or rendering state.

Omitting `cursor` replays retained output, `-1` tails from the current end, and a
nonnegative cursor resumes at an absolute **UTF-16 code-unit** offset. The cursor
control frame follows replay. For subsequent text, advance using
`text.encode_utf16().count()`, not UTF-8 byte length. The pinned server retains
roughly 2 MiB of JavaScript string units and sends no explicit gap marker if the
requested prefix has been lost. Callers decide whether to reconnect or reset.

The default upgrade deadline is 30 seconds; optional idle timeout covers polled
reads. Maximum incoming message/frame and outgoing message size defaults to
8 MiB. Cancelling a pending read preserves its deadline. Writes can have partial
delivery on cancellation/failure: do not blindly resend input. No tasks,
reconnection, ticket minting or input replay occur implicitly. Local `close()`
sends a close frame and detaches without awaiting the peer; `disconnect()`/drop
release immediately. A peer close produces one `Closed` with code/reason, then
EOF. Query `get()` for the process exit code. v2.2.5 can close with 4404 when
connecting to an exited process or racing removal.

Run `cargo run --features websocket --example pty -- --execute` with
`OPENCODE_BASE_URL`, optional `OPENCODE_PASSWORD`/`OPENCODE_DIRECTORY`. It starts
`/bin/cat` on a Unix server, resizes, exchanges a fixed line, detaches and removes
the PTY, including on ordinary errors/timeouts. A hard process abort may leave the
printed PTY ID for manual removal. Persistent PTYs remain deferred.
