# `opencoders`

`opencoders` is an async Rust client for connecting to the [OpenCode v2 HTTP API](https://opencode.ai/v2/docs/api).

The library currently supports server information, location/agent discovery,
session management/listing, and message history/retrieval. It targets developers building
GUI/TUI clients and also supports automated consumers. Applications own their UI
and state. See [DESIGN.md](DESIGN.md) and [TODO.md](TODO.md) for the plan and backlog.

## Usage

Requires Rust 1.88 or newer and a caller-owned Tokio runtime.

```rust
use opencoders::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("http://localhost:4096")
        .basic_auth("opencode", "your-server-password")
        .build()?;

    let info = client.server().info().await?;
    println!("OpenCode {} (pid {})", info.version, info.pid);
    Ok(())
}
```

Run the included example against a server you configure:

```sh
export OPENCODE_BASE_URL=http://localhost:4096
# Set OPENCODE_PASSWORD in your environment to your server password.
cargo run --example server_info
```

OpenCode v2.2.5 uses HTTP Basic authentication with username `opencode` and the
server password. Use HTTPS when connecting to a remote server. `bearer_auth`
remains available for proxies that require it; the stock server does not accept
Bearer tokens. Pairing/cookie management is not implemented.

Custom authentication schemes and application headers can be supplied through
`ClientBuilder::default_headers`, using the re-exported `opencoders::header` types.
No credentials are discovered from local OpenCode files.

## Location-aware discovery

```rust
use opencoders::{Client, Location};

async fn discover() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("http://localhost:4096")
        .default_location(Location::directory("/remote/workspace"))
        .build()?;
    let location = client.location().get().await?;
    let agents = client.agents().list().await?;
    println!("{} agents at {}", agents.data.len(), agents.location.directory);

    // Override only this call; clones retain the configured default.
    let other = client.agents().list_at(Location::directory("/remote/other")).await?;
    let fallback = client.location().get_at(Location::Unspecified).await?;
    Ok(())
}
```

`Location::Unspecified` omits the location query even when the client has a
configured default. Server fallback rules still apply, including any location
headers supplied by the application or injected HTTP client. It does not send
JSON null, an empty string, or the literal string `"null"`.

Directory values use the nested `location[directory]` query on supported routes
only. They refer to the server's filesystem; the SDK neither reads local paths
nor canonicalizes them. Server-wide calls such as `server.info()` ignore the
configured directory. Agent lists retain their `location`/`data` envelope and
include hidden agents; applications choose what to display.

Run `cargo run --example discovery` with `OPENCODE_BASE_URL`, optional
`OPENCODE_PASSWORD`, and optional `OPENCODE_DIRECTORY`. The example resolves a
location, then lists agents at that resolved directory.

## Session listing and pagination

```rust
use opencoders::{Client, ParentFilter, SessionListOptions, SessionOrder};

async fn browse(client: &Client) -> opencoders::Result<()> {
    let mut options = SessionListOptions::default()
        .limit(20).order(SessionOrder::Desc)
        .parent(ParentFilter::RootOnly).archived(false);
    let first = client.sessions().list(&options).await?;
    if let Some(next) = first.cursor.next {
        options.cursor = Some(next);
        options.order = None;
        let second = client.sessions().list(&options).await?;
        // second.cursor.previous can retrieve the preceding page.
    }
    Ok(())
}
```

`SessionListOptions::default()` omits all filters. Options cover limit, order,
search, parent, archived state, directory, project, subpath, and cursor. Root-only
selection sends `parentID=null`; selecting children uses a typed `SessionId`.
Omitted archive filtering differs from explicit `false`. Public option fields
can be cleared with `None`; provided empty strings are sent unchanged. The SDK
represents limits as `u32` and leaves accepted ranges to the server.

Session listing ignores the SDK's default location and uses explicit, direct
`directory`/`project` filters. Each call returns one `data`/`cursor` page in server
order. Pass either opaque cursor back unchanged, retaining applicable filters;
the order option is documented for the first page. Setting a cursor does not
silently change other options. Missing or null cursors become `None`; an empty
cursor remains `Some`. The SDK performs no automatic pagination or retries.

On tested OpenCode v2.2.5, cursors carry the original filters and ordering; changing
those requires clearing the cursor. `limit` still controls page size. A next or
previous cursor marks a boundary and can lead to an empty page, even when the
current page is shorter than the requested limit.

Run `cargo run --example sessions` with the same environment variables as the
discovery example. It follows up to three pages, keeping pagination in caller
code. Session models reuse discovery ID/location/model/permission types and
preserve added fields. Numeric timestamps, costs, and token values use
`serde_json::Number` to retain supported integer precision. Session/message ID
prefixes and nested file-diff bounds/flags are checked during decoding.

## Creating and retrieving sessions

```rust
use opencoders::{Client, RequestField, models::{SessionCreateRequest, SessionLocationInput}};

async fn create(client: &Client) -> opencoders::Result<()> {
    let mut request = SessionCreateRequest::default()
        .title("New conversation")
        .location(SessionLocationInput::new("/remote/workspace"));
    // Public fields also allow explicit null or omission:
    request.metadata = RequestField::Null;
    let created = client.sessions().create(&request).await?;
    let retrieved = client.sessions().get(&created.data.id).await?;
    println!("{}", retrieved.data.id.as_str());
    Ok(())
}
```

Both calls return `SessionResponse`, preserving `data` and envelope extensions.
Creation sends a JSON body (`{}` for a default request) and does not submit a
prompt. `RequestField::Omitted`, `Null`, and `Value(value)` preserve all three wire
states for the eight optional create fields. Fluent setters set values; public
fields can be reset to omission or null. Null does not imply a general clearing
policy. v2.2.5 accepts all eight fields as null; null/omitted location and parent
use the server working directory with no parent.

Creation uses the body's location or the parent's location when a concrete
`parentID` is supplied; the parent takes precedence even over a conflicting body
location on v2.2.5. Create/get ignore `ClientBuilder::default_location`. Retrieval
encodes the typed session ID as one path segment. Input location/model/permission
types are separate from tolerant response models, so response extensions are not
automatically echoed into writes. `SessionModelInput::variant` permits omission
or a string, not null; arbitrary JSON belongs in `SessionMetadata`.

The default transport does not retry creation or follow redirects. A timeout,
disconnect, or unreadable response can happen **after the session was created**.
Inspect server state before resubmitting; an explicit session ID is not a promise
of idempotency. An injected reqwest client's retry/redirect policy is retained.

Run `cargo run --example create_session` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, and optional `OPENCODE_PASSWORD`. It creates one empty
session, retrieves it, and leaves it on the server.

## Updating and deleting sessions

```rust
use opencoders::{Client, models::{SessionId, SessionUpdateRequest}};

async fn manage(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    client.sessions().update(id, &SessionUpdateRequest::default()
        .title("Renamed conversation").archived(true)).await?;
    client.sessions().update(id, &SessionUpdateRequest::default()
        .archived(false)).await?;
    // Removal also deletes this session's descendants.
    client.sessions().remove(id).await?;
    Ok(())
}
```

Both methods return `()` for HTTP 204, without decoding JSON. They encode the ID
as one path segment and ignore the SDK's discovery location default. Updates
support `title`, `metadata`, `permissions`, and `archived`, each using
`RequestField<T>` to preserve omission, null, and values.

On tested v2.2.5, omitted and null fields leave existing values unchanged.
Metadata and permission values replace their contents; an empty map or list
clears those contents. `archived(true)` archives, and `archived(false)` restores.
An empty title requests server-generated titling and can invoke a model when the
session has conversation history; it does not clear the title.

Deletion removes the session and its descendants; applications own confirmation
and recovery decisions. Repeated deletion returns a typed missing-session error
on v2.2.5. Updates can be partially applied before an error, and a lost response
can follow a completed update or deletion. The default transport does not retry
either method; inspect server state before deciding to repeat a mutation.

Run `cargo run --example session_lifecycle` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, and optional `OPENCODE_PASSWORD`. It creates a fresh empty
session, renames it, archives/restores it, and deletes it without a model call.
If a later step fails, the printed session ID is available for inspection.

## Message history and retrieval

```rust
use opencoders::{Client, MessageListOptions, MessageOrder,
    models::{SessionId, SessionMessageType}};

async fn history(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    let messages = client.sessions().messages(id);
    let mut options = MessageListOptions::default().limit(20)
        .order(MessageOrder::Desc).message_type(SessionMessageType::User);
    let first = messages.list(&options).await?;
    if let Some(message) = first.data.first() {
        let retrieved = messages.get(message.id()).await?;
        println!("{}", retrieved.data.message_type());
    }
    if let Some(next) = first.cursor.next {
        options.cursor = Some(next);
        options.order = None;
        let second = messages.list(&options).await?;
    }
    Ok(())
}
```

`list` returns `SessionMessagesResponse` with `data` and `cursor`; `get` returns
`SessionMessageResponse` with `data`. Both preserve additive fields and use
session/message IDs encoded as individual path segments. They ignore the SDK's
default discovery location and do not fetch extra pages automatically.

Options cover `limit`, `order`, `cursor`, and `message_type` (sent as `type`).
Defaults omit all parameters; v2.2.5 defaults to the newest 50 messages. Follow
either opaque cursor unchanged, **omit order and repeat the same type filter**.
The server rejects a nonempty cursor combined with order. Message cursors carry
ordering but not the type filter. Setting a cursor does not change other options;
public fields can be cleared with `None`. Boundary cursors can lead to empty pages.

`SessionMessageInfo` is an enum covering all eleven pinned timeline kinds, with
`id()` and `message_type()` helpers. Match `User`, `Assistant`, or other variants
to inspect their boxed payloads. Assistant content includes typed text, reasoning,
and tool states; compaction states and attachments remain typed. These are
projected history records, not live events or pending inbox items. Added object
fields are retained; unknown tags and malformed known variants are decode errors.
Message metadata and provider-native data may contain sensitive content.

Run `cargo run --example history` with `OPENCODE_BASE_URL`, `OPENCODE_SESSION_ID`,
and optional `OPENCODE_PASSWORD`. It reads at most three pages and retrieves one
message by ID, without submitting input or changing the session.

## Configuration and behavior

- Base URLs accept an optional reverse-proxy prefix: `https://host/opencode`
  resolves to `https://host/opencode/api/info`. The trailing slash is optional.
  Supply the server base, without an `/api` suffix. URL credentials, queries, and
  fragments are rejected; use headers for authentication.
- Client clones share the transport. The default transport has a 10-second
  connection timeout, a 30-second request deadline including body reads, and no
  redirects or automatic retries.
- `http_client` accepts a configured `reqwest::Client`. Its DNS, proxy, TLS,
  redirect, retry, and connection policies remain in effect. SDK headers and the
  SDK request timeout override transport defaults. Configure connection timeouts
  on the injected client; combining it with `connect_timeout` returns a build
  error. `without_request_timeout` removes only the SDK deadline.
- The default `rustls` feature enables HTTPS. `--no-default-features` supports
  HTTP or a TLS backend enabled through the supplied reqwest dependency.
- Success responses are limited to 8 MiB by default. HTTP/decode diagnostic
  prefixes are limited to 8 KiB; both limits are configurable. Error accessors
  indicate truncation. The limit applies to the accumulated body; the HTTP stack
  may deliver a larger individual chunk.
- Server-info models preserve unknown fields in `extra` maps, including nested
  paths and capabilities. Deserialization enforces the schema's nonnegative
  safe-integer range for `pid`. Optional null and absent response values become
  `None` and serialize as absent.
- Discovery models preserve additional object fields and typed agent/provider
  configuration. Agent `steps`, when present, must be a positive safe integer;
  provider timeout values accept numbers or `false`. Unknown enum variants remain
  decoding errors. Generated optional fields generally accept null as absent;
  the `steps` adapter rejects explicit null. These are response-only models.

## Errors

`BuildError` covers invalid configuration. API calls return `Error::Encode`, `Error::Transport`,
`Error::Http`, `Error::Decode`, or `Error::ResponseTooLarge`. Use `operation()`,
`status()`, and `is_timeout()` for diagnostics and control flow. JSON response
methods expect exactly HTTP 200; session update/remove expect HTTP 204 with no
JSON decoding. Unexpected redirects and other success statuses are HTTP errors
too. Encoding errors happen before any request is sent and have no status.
JSON-body operations set `Content-Type: application/json`, overriding defaults.

HTTP errors retain a bounded raw body and recognize `InvalidRequestError`,
`UnauthorizedError`, `LocationNotFoundError`, `ServiceUnavailableError`, and
`InvalidCursorError`, `SessionNotFoundError` (including missing parents), and
`MessageNotFoundError` (also used when a message belongs to another session).
Other JSON, including malformed known error shapes, is
available as `ApiError::Unrecognized`. Known payloads' additional fields remain
available in the raw body. Truncated or non-JSON bodies have no parsed API error.
Body-read failures are transport errors and retain any received HTTP status.

Default client/error `Debug` and error `Display` omit URLs, headers, response
bodies, and source messages. Explicit body access and error sources may contain
sensitive request or server-supplied text; inspect them deliberately. Dropping a request
future cancels local consumption and does not undo server-side work.

`ProviderRequest` also redacts its `Debug` output because provider configuration
can contain credentials. Its fields and serialization preserve the original
values; treat explicitly accessed or serialized response data as sensitive.

## Development and compatibility

```sh
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo doc --workspace --no-deps --locked
```

Tests use local mock HTTP servers; they need permission to bind localhost ports
but do not require OpenCode, provider credentials, or model calls. The runnable
examples are built by the checks and contact a real server only when invoked.

The pinned schema and provenance live in [`openapi/`](openapi/). The implemented
operations also pass an opt-in smoke test against **OpenCode v2.2.5 on macOS
arm64**, using disposable data and no model calls. Run `python3 tools/live/run.py`;
see [compatibility details and CI](tools/live/README.md) for prerequisites, release
checksums, verified behavior, and limitations. Other endpoints and event streams
are not yet implemented.

The private [`tools/model-spike/`](tools/model-spike/README.md) workspace tool
contains the initial experiment and reviewed server-info/discovery/session/message generation commands.
Models are checked in; normal builds do not generate code or fetch a schema.
Ordinary `cargo build` and `cargo test` select the root library; `--workspace`
also checks the tool.
