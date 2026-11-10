# opencoders

[![CI](https://github.com/a-poor/opencoders/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/a-poor/opencoders/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/opencoders)](https://docs.rs/opencoders)
[![crates.io](https://img.shields.io/crates/v/opencoders)](https://crates.io/crates/opencoders)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

An unofficial async Rust client for connecting to a running OpenCode server
through its [v2 HTTP API](https://opencode.ai/v2/docs/api).

See the [documentation](https://github.com/a-poor/opencoders/tree/main/docs) for
workflow guides, examples and compatibility details.

## Installation

Requires Rust 1.88+ and Tokio.

```toml
[dependencies]
opencoders = "0.1.0"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

HTTPS is enabled by default through Rustls. Use `default-features = false` for HTTP-only clients.

## Getting started

With OpenCode running and a provider/model configured, set `OPENCODE_PASSWORD`
to your server password, then create a session and send a message:

```rust
use opencoders::{Client, models::{SessionCreateRequest, SessionPromptRequest}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("http://localhost:4096")
        .basic_auth("opencode", std::env::var("OPENCODE_PASSWORD")?)
        .build()?;

    let session = client.sessions()
        .create(&SessionCreateRequest::default().title("Hello from Rust"))
        .await?;
    let prompt = SessionPromptRequest::new("Say hello in one sentence.").resume(true);
    client.sessions().prompt(&session.data.id, &prompt).await?;
    println!("Message accepted in session {}", session.data.id.as_str());
    Ok(())
}
```

The session uses the server's working directory. `prompt` acknowledges acceptance;
see [streaming responses](https://github.com/a-poor/opencoders/blob/main/docs/events.md)
to receive the assistant's output.

For non-interactive commands and session environments, see the
[shell guide](https://github.com/a-poor/opencoders/blob/main/docs/shells.md).

## Pinned API contract

Models use the [pinned OpenAPI 3.1.0 snapshot](https://github.com/a-poor/opencoders/blob/main/openapi/opencode-v2.json)
reviewed on 2026-11-09. Its schema version is `0.0.1`; the producing server release
is unspecified. The tested server baseline is **OpenCode v2.2.5**. See
[provenance](https://github.com/a-poor/opencoders/blob/main/openapi/provenance.json)
and [compatibility details](https://github.com/a-poor/opencoders/blob/main/docs/development.md).

## License

Available under either [MIT](https://github.com/a-poor/opencoders/blob/main/LICENSE-MIT.txt)
or [Apache-2.0](https://github.com/a-poor/opencoders/blob/main/LICENSE-APACHE.txt).
