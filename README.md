# `opencoders`

An async Rust client for the [OpenCode v2 HTTP API](https://opencode.ai/v2/docs/api),
for GUI/TUI developers and automated consumers. Your application owns its UI and state.

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

Set `OPENCODE_PASSWORD` to your server password, then connect to your OpenCode server:

```rust
use opencoders::{Client, SessionListOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("http://localhost:4096")
        .basic_auth("opencode", std::env::var("OPENCODE_PASSWORD")?)
        .build()?;

    let info = client.server().info().await?;
    let sessions = client.sessions()
        .list(&SessionListOptions::default().limit(10))
        .await?;
    println!("OpenCode {}: {} sessions in this page", info.version, sessions.data.len());
    Ok(())
}
```

## Pinned API contract

Models use the [pinned OpenAPI 3.1.0 snapshot](https://github.com/a-poor/opencoders/blob/main/openapi/opencode-v2.json)
retrieved on 2026-11-02. Its schema version is `0.0.1`; the producing server release
is unspecified. The tested server baseline is **OpenCode v2.2.5**. See
[provenance](https://github.com/a-poor/opencoders/blob/main/openapi/provenance.json)
and [compatibility details](https://github.com/a-poor/opencoders/blob/main/docs/development.md).

## License

Available under either [MIT](https://github.com/a-poor/opencoders/blob/main/LICENSE-MIT.txt)
or [Apache-2.0](https://github.com/a-poor/opencoders/blob/main/LICENSE-APACHE.txt).
