# Changelog

## Unreleased

- Add command/skill discovery, slash-command execution, context and turn diffs,
  durable compaction admission, and staged/cleared/committed session reverts.
  Reuse existing wire models, preserve nullable/omitted inputs, expose typed
  command/busy/server-reference failures, and document irreversible history and
  execution side effects. No SDK mutation retries.

- Add worktree create/refresh/remove and explicit VCS initialization. Requests
  preserve optional fields and require an explicit removal force decision;
  typed worktree failures preserve `forceRequired` without automatic escalation.
  Pinned disposable tests cover Git initialization, detached checkouts, dirty
  removal and inventory reconciliation. See [workspace APIs](docs/workspace.md).

- Add filesystem listing/search and streamed binary reads with range/conditional
  response metadata, independent deadlines and optional byte bounds.
- Add read-only VCS info/base/status/branches/diffs and project worktree listing.
- Add ordinary PTY lifecycle, resize and single-use connection tickets. Optional
  `websocket` exposes caller-driven UTF-8 I/O, replay cursors and close metadata
  through the configured HTTP transport, without automatic retries/reconnection.
  New models, protocol tests, examples and pinned macOS live checks cover these
  16 operations; persistent PTYs and experimental filesystem mutations remain out
  of scope. See [workspace reads](docs/workspace.md) and [PTYs](docs/terminals.md).

- Refresh the reviewed OpenAPI pin after the 2026-11-09 upstream drift report.
  The contract now has 170 operations and 497 component schemas; background-job
  discovery is planned for milestone 4. Generated Rust models and the separately
  tested OpenCode v2.2.5 baseline are unchanged. See the
  [contract review](openapi/README.md#2026-11-09-drift-review) for compatibility details.

## 0.1.0 — 2026-11-03

Initial async Rust library for applications using the OpenCode v2 HTTP API,
primarily GUI/TUI clients. Applications own rendering, runtime and state.

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
- A minimal README and [topic guides](docs/README.md), plus sixteen examples
  including application-owned background observation and bounded automation with
  deterministic tests. Source, guides, examples, test fixtures and both license
  texts are included in the crate archive.

### Compatibility and limits

The compatibility baseline is **OpenCode v2.2.5 on macOS arm64 and Linux x86-64**,
with hosted smoke and active runs passing on both platforms on 2026-11-03.
Smoke fixtures exercise the supported endpoints. Eight active
scenarios use an isolated localhost provider and disposable read/question tools
to verify execution, interruption, waiting-tool resumption, and controlled
disconnect/reconnect recovery. They do not establish real-model reasoning or
remote-provider compatibility. Other platforms remain unverified.

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
