# `opencoders`

`opencoders` is an async Rust client for connecting to the [OpenCode v2 HTTP API](https://opencode.ai/v2/docs/api).

The library currently supports server information, location resolution, and agent
discovery through a configurable async client. It targets developers building
GUI/TUI clients and also supports automated consumers. Applications own their UI
and state. See [DESIGN.md](DESIGN.md) and [TODO.md](TODO.md) for the plan and backlog.

## Usage

Requires Rust 1.88 or newer and a caller-owned Tokio runtime.

```rust
use opencoders::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("http://localhost:4096")
        .bearer_auth("your-server-token")
        .build()?;

    let info = client.server().info().await?;
    println!("OpenCode {} (pid {})", info.version, info.pid);
    Ok(())
}
```

Run the included example against a server you configure:

```sh
export OPENCODE_BASE_URL=http://localhost:4096
# Set OPENCODE_TOKEN in your environment if bearer authentication is required.
cargo run --example server_info
```

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
`OPENCODE_TOKEN`, and optional `OPENCODE_DIRECTORY`. The example resolves a
location, then lists agents at that resolved directory.

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

`BuildError` covers invalid configuration. API calls return `Error::Transport`,
`Error::Http`, `Error::Decode`, or `Error::ResponseTooLarge`. Use `operation()`,
`status()`, and `is_timeout()` for diagnostics and control flow. All three methods
expect exactly HTTP 200; unexpected redirects and other 2xx statuses are HTTP
errors too.

HTTP errors retain a bounded raw body and recognize `InvalidRequestError`,
`UnauthorizedError`, `LocationNotFoundError`, and `ServiceUnavailableError`.
Other JSON, including malformed known error shapes, is
available as `ApiError::Unrecognized`. Known payloads' additional fields remain
available in the raw body. Truncated or non-JSON bodies have no parsed API error.
Body-read failures are transport errors and retain any received HTTP status.

Default client/error `Debug` and error `Display` omit URLs, headers, response
bodies, and source messages. Explicit body access and error sources may contain
sensitive server-supplied text; inspect them deliberately. Dropping a request
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

The pinned schema and provenance live in [`openapi/`](openapi/). Validation so far
uses that contract and mocks, not a pinned live OpenCode release. Selecting and
testing a live server remains open. Other endpoints and event streams are not yet
implemented.

The private [`tools/model-spike/`](tools/model-spike/README.md) workspace tool
contains the initial experiment and reviewed server-info/discovery generation commands.
Models are checked in; normal builds do not generate code or fetch a schema.
Ordinary `cargo build` and `cargo test` select the root library; `--workspace`
also checks the tool.
