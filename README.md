# `opencoders`

`opencoders` is an async Rust client for connecting to the [OpenCode v2 HTTP API](https://opencode.ai/v2/docs/api).

The public crate is still a scaffold. See [DESIGN.md](DESIGN.md) for the library
design and [TODO.md](TODO.md) for current tasks and the inbox.

The first completed experiment evaluates generated wire models against a pinned
schema. Its commands, tests, and limitations are documented in
[`tools/model-spike/README.md`](tools/model-spike/README.md). It is a private
workspace tool, not the client API. Use `cargo test --workspace --locked` to include
its tests; ordinary `cargo build` and `cargo test` select only the root library.
