# Changelog

## 0.1.0 — Unreleased

Initial async Rust library for applications using the OpenCode v2 HTTP API,
primarily GUI/TUI clients. Applications own rendering, runtime and state.
This version has not been published; hosted validation and a final release
revision review remain pending.

### Supported APIs

- Server information and location, agent and model discovery.
- Session creation, retrieval, updates, deletion, forks, pagination, and model
  or agent selection.
- Projected message history and retrieval; prompt admission and pending inbox
  inspection, updates and cancellation.
- Execution interruption and experimental bounded waiting.
- Permission requests/replies and form discovery, answering and cancellation.
- Typed live SSE subscriptions and experimental session log replay/follow.

The [coverage inventory](https://github.com/a-poor/opencoders/blob/main/API_COVERAGE.md)
records 35 supported operations from the 169-operation pinned contract.
PTY/WebSocket, filesystem, worktree, configuration, provider management and
other APIs remain outside this release's implemented surface.

### Integration

- Rust 1.88+, Rust 2024, and a caller-owned Tokio runtime. Cloned clients share
  their HTTP connection pool; the SDK adds no background worker or UI framework.
- Default Rustls HTTPS support; `default-features = false` for HTTP-only use.
  Caller-supplied Reqwest clients are supported.
- HTTP Basic authentication, configurable request/stream bounds, typed errors,
  explicit cursors, and request models preserving omission versus null.
- No automatic mutation retries or event reconnection. Cancellation stops local
  waiting and does not undo server work; admission and idleness are not success.
- Sixteen examples, including application-owned background observation and
  bounded automation, with deterministic tests. Source, examples, test fixtures
  and both license texts are included in the crate archive.

### Compatibility and limits

The local compatibility baseline is **OpenCode v2.2.5 on macOS arm64**, tested
2026-11-02/03. Smoke fixtures exercise the supported endpoints. Eight active
scenarios use an isolated localhost provider and disposable read/question tools
to verify execution, interruption, waiting-tool resumption, and controlled
disconnect/reconnect recovery. They do not establish real-model reasoning or
remote-provider compatibility. Linux and hosted runs remain unverified.

Live events are transient. Missing text cannot be reconstructed from history
until its text block ends. Snapshot reads are not atomic; the shared observer
converges after controlled changes settle. Server overflow, restart recovery,
arbitrary concurrent changes and the example CLI timer/backoff loop remain
untested live. Stock v2.2.5 disables event-payload retention, so log replay tests
use seeded rows rather than proving natural durability.

The [compatibility record](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md)
details other unverified behaviors, including pairing/cookies, tool reply modes,
form lifetime and active fork boundaries. The pinned schema does not identify its
producing server release; its version is separate from the tested baseline.

### Correction during development

The unfiltered live observer exposed an instruction-update decoding defect.
`EventSessionInstructionsUpdatedDataDeltaValue` is now a scalar enum accepting a
checked instruction hash or `"removed"`, replacing the incorrect struct with
`subtype_*` fields. Early checkout consumers constructing that generated type
must adapt to its enum variants. Malformed values still fail decoding.

### License

Available under either [MIT](LICENSE-MIT.txt) or [Apache-2.0](LICENSE-APACHE.txt).
