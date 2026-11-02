# Tasks and backlog

This file tracks current work, upcoming milestones, bugs, and longer-term ideas
for `opencoders`. See [DESIGN.md](DESIGN.md) for architecture, scope, and rationale.

We are building a Rust library for GUI/TUI developers, with automation supported
by the same APIs. Building an end-user interface is outside the project's scope.

## Inbox

Drop quick notes, issues, tasks, or ideas here. No categorization, prioritization,
or detailed write-up is required; the agent can organize them later. Keep one
blank bullet at the end so the next note is easy to add.

-

## Keeping this file useful

- Agents may triage the **Inbox** while updating this file or when explicitly
  asked. Move entries into a milestone, **Backlog**, **Bugs**, or **Investigations
  and decisions** as appropriate. Merge duplicates while preserving the author's
  intent, details, and links. Keep unclear entries in the inbox with a short note
  about what needs clarification.
- Use **Backlog** for categorized work that has no milestone yet. Keep a blank
  bullet at the end of both **Inbox** and **Backlog**, including after triage.
- Move tasks into **In progress** when work starts. Keep blocked tasks there with
  a short description of the dependency or unresolved question.
- Check off completed tasks and move them to **Completed**, adding a commit, PR,
  or validation note when useful. Split large tasks into smaller checkboxes as
  their implementation becomes concrete.
- Record confirmed defects under **Bugs** and unverified behavior under
  **Investigations and decisions**. Longer-term ideas are candidates, not release
  commitments.
- Update this file as work changes; update `DESIGN.md` when an architectural
  decision or project boundary changes.

## In progress

No active tasks. The client-foundation goal is recorded under **Completed**.
The next candidate goal is location configuration and a representative
location-scoped discovery method, exercising nested query encoding and envelopes.

## Now: contract and foundation (milestone 1)

### Contract and model generation

- [ ] Select a pinned OpenCode server release for compatibility testing.
- [ ] Extend the reviewed `--server-info` generation path to additional models
  and decide when to rename the private experiment into the permanent tool.
  Ordinary crate builds must remain independent of generation and schema fetching.
- [ ] Apply the spike findings to broader production models: additive response fields,
  complete event/error dispatch, three-state request fields, numeric constraints,
  and inline endpoint schemas. See `tools/model-spike/README.md`.
- [ ] Create an operation-ID inventory with supported, planned, and deferred
  status so API coverage can be tracked.

### Crate and transport

- [ ] Add default location and per-request overrides to the client on applicable
  routes; verify nested location query encoding and dynamic path-segment encoding.
- [ ] Separate ordinary-request timeouts from streaming and long-running
  operations; document local cancellation versus server-side interruption.
- [ ] Extend response handling to endpoints with documented empty bodies and
  multiple success statuses. Add typed API errors as new resources need them.
- [ ] Establish request/response conventions for IDs, omitted versus null values,
  response metadata, and pagination cursors.
- [ ] Implement representative location-scoped methods to exercise envelopes
  and public request conventions beyond the direct `server.info()` response.
- [ ] Document ambiguous mutation outcomes after transport failures when adding
  write operations; keep the default transport's automatic retries disabled.

### Initial validation and repository setup

- [ ] Extend mock-server coverage to nested query/request encoding, envelopes,
  and documented empty responses as additional operations are implemented.
- [ ] Add opt-in smoke tests against the pinned server using disposable data.
- [ ] Add CI for formatting, Clippy, tests, documentation/examples, supported
  features, the minimum Rust version, and reproducible generation.
- [ ] Replace the MIT license's copyright placeholders with the correct details.

## Next: interactive conversation APIs (milestone 2)

### Core resources

- [ ] Implement session listing with explicit pagination and filters.
- [ ] Implement session creation, retrieval, updates, deletion, and forking.
- [ ] Implement message history and message retrieval while retaining cursors
  and relevant response metadata.
- [ ] Implement agent/model discovery and session agent/model selection.
- [ ] Implement prompt submission, delivery modes, and inbox inspection/update/
  cancellation. Keep admission distinct from execution completion.
- [ ] Implement interruption and execution observation/waiting.
- [ ] Implement pending permission discovery, session permission requests, and
  permission replies without imposing an approval policy.
- [ ] Implement pending form discovery and session form lifecycle/replies.
- [ ] Add protocol tests for these resource methods and unknown server errors.

### Events and session logs

- [ ] Implement global event subscriptions returning typed asynchronous streams,
  including event filters and connection markers.
- [ ] Model text/reasoning deltas, tool activity, session lifecycle, permissions,
  and forms needed by interactive consumers.
- [ ] Add an unknown-event fallback retaining raw data; keep malformed known
  events distinguishable as decoding errors.
- [ ] Implement SSE framing, comments/heartbeats, multiline data, EOF, and the
  upstream stream-failure event.
- [ ] Implement durable session log reads with exclusive `after` sequences,
  optional following, SSE identifiers, and synchronization markers.
- [ ] Verify independent subscription lifetimes, drop cancellation, bounded
  buffering, and visible disconnect/lag behavior.
- [ ] Test fragmented SSE frames, failure events, unknown variants, cancellation,
  disconnection, and durable replay boundaries.
- [ ] Verify snapshot/event ordering and recovery behavior against upstream;
  document which steps remain the consuming application's responsibility.

### Examples and first usable release

- [ ] Add runnable Rust examples for session listing, history, prompt submission,
  event consumption, permission replies, and form replies. Use ordinary output
  or assertions, with no GUI/TUI implementation.
- [ ] Add an example of background event consumption and recovery after
  disconnection using minimal application-owned state.
- [ ] Add an automation example using the same API surface.
- [ ] Expand crate documentation and the README with setup, authentication,
  runtime requirements, supported operations, and tested server versions.
- [ ] Verify milestone 2 acceptance criteria from `DESIGN.md` and prepare the
  first usable release.

## Soon: terminal and workspace APIs (milestone 3)

- [ ] Implement PTY lifecycle and resize operations.
- [ ] Verify PTY connection tickets, WebSocket framing, input/output, closure,
  and cursor behavior against the server or official client.
- [ ] Add WebSocket transport behind an optional Cargo feature.
- [ ] Implement persistent PTY operations and connections, recording any deferred
  operations in the coverage inventory.
- [ ] Implement filesystem listing/search and streamed reads, including headers,
  range requests, and conditional responses.
- [ ] Implement filesystem write/watch/unwatch operations.
- [ ] Implement worktree and VCS APIs used by interactive consumers.
- [ ] Add protocol tests and small library examples for PTY and workspace APIs;
  verify builds with and without WebSocket support.

## Later: broader API coverage and release hardening (milestone 4)

- [ ] Implement remaining session operations such as commands, skills, synthetic
  messages, compaction, revert, context/diff, import/export, and instructions.
- [ ] Implement configuration, project, command, skill, and reference APIs.
- [ ] Implement provider, integration, and credential management APIs.
- [ ] Implement MCP management/resources and client-hosted MCP WebSockets.
- [ ] Implement plugin management and generic plugin RPC calls/events with typed
  envelopes and support for dynamic payloads.
- [ ] Implement generation, voice, web search, and usage APIs.
- [ ] Implement shell, schedule, artifact, and media APIs, preserving streamed
  binary responses where applicable.
- [ ] Review all remaining operations, including debug and migration routes;
  implement them or record explicit deferrals in the coverage inventory.
- [ ] Document schema-update and release procedures, including compatibility
  checks and review of generated changes.
- [ ] Validate package metadata, included files, Cargo feature combinations,
  examples, and documentation before publishing.

## Backlog

Categorized tasks and ideas without a scheduled milestone belong here. The
initial entries are longer-term candidates to revisit when consumer needs justify
the additional API surface. Move an item into a milestone when it is prioritized.

- [ ] Automatic pagination helpers that preserve filter and ordering semantics.
- [ ] Shared event subscriptions with explicit buffering, subscriber lifetimes,
  and lag behavior.
- [ ] Optional recovery helpers based on verified replay/resynchronization
  semantics.
- [ ] Retry policies for operations with established idempotency guarantees.
- [ ] Local server discovery and optional process-management helpers.
- [ ] Browser/WASM support and corresponding runtime/transport tests.
- [ ] A blocking API if there is demonstrated demand.
- [ ] Typed plugin RPC adapters for externally supplied plugin contracts.
- [ ] Optional state-reconstruction helpers, keeping state ownership explicit.
- [ ] Generation of routine endpoint methods if manual maintenance becomes costly.
-

## Bugs

No confirmed client implementation bugs are recorded yet.

For each new bug, add an unchecked task with the affected operation, expected
versus actual behavior, reproduction steps or a regression fixture, relevant
crate/server versions, and impact. Link the fix and regression coverage when
resolved. Keep credentials and other secrets out of examples and logs.

## Investigations and decisions

Record findings alongside the related implementation task, then update the design
when a decision is made.

- [ ] Confirm supported authentication and pairing behavior beyond the schema's
  empty security declarations; determine which convenience helpers to expose.
- [ ] Resolve nested location encoding for routes without an explicit OpenAPI
  serialization style.
- [ ] Choose ergonomic resource names and request construction patterns after
  exercising representative methods in library examples.
- [ ] Confirm the semantics of omitted versus explicit null fields before choosing
  their public Rust representation.
- [ ] Confirm event ordering, connection-marker guarantees, log watermarks, and
  snapshot reconciliation without assuming gap-free global events.
- [ ] Choose how to expose SSE IDs, log sequences, pagination, and location
  metadata while keeping common calls convenient.
- [ ] Determine the earliest useful PTY/WebSocket subset and document any protocol
  details missing from OpenAPI.

## Completed

- [x] Complete the client-foundation goal with test-first configuration and
  protocol coverage: cloneable `Client`/`ClientBuilder`, explicit base URL,
  authentication/headers, deadlines, injected reqwest client, and typed
  `server.info()`. Reverse-proxy prefixes are preserved; the default transport
  disables redirects and retries. The starter scaffold is replaced.
- [x] Add distinguishable transport, HTTP/API, decode, and oversized-response
  errors with status/operation context, bounded diagnostic prefixes, safe default
  formatting, and inspectable unknown server errors.
- [x] Generate reviewed, non-exhaustive server-info response models with preserved
  additive fields and validated PID bounds. Check output using
  `cargo run -p opencoders-model-spike -- --server-info --check`.
- [x] Select reqwest 0.13, Tokio, Serde, and optional/default Rustls; declare Rust
  1.88 and verify the locked library build on that compiler. Add the runnable
  `examples/server_info.rs` and configuration/error documentation.
- [x] Validate 30 client tests, 20 generation-experiment tests, and the crate
  documentation example; check HTTP-only builds/tests, formatting, Clippy, API
  docs, example compilation, and both checked-in model outputs. Tests use mocks;
  live OpenCode compatibility and CI automation remain open.
- [x] Complete the first contract/model-generation goal: pin the reviewed schema
  and provenance, select Typify 0.8.0 with an explicit non-null override, and
  document required production adaptations in `tools/model-spike/README.md`.
  All 490 component schemas generate and compile; the reduced probe has 20
  passing tests. The associated server release remains unknown and is recorded
  as null in provenance; choosing a server baseline remains open.
- [x] Validate omitted/null/value request handling and explicit event dispatch
  through failing tests followed by working adapters. Record raw generator
  limitations as characterization tests, not desired client behavior.
- [x] Record scoped goals and pragmatic TDD preferences in `AGENTS.md`.
- [x] Create the Rust library crate and add MIT/Apache-2.0 license files and the
  Cargo license declaration. Copyright placeholder cleanup remains above.
- [x] Review the v2 HTTP API, OpenAPI schema, and official client documentation.
- [x] Write `DESIGN.md` with a library-only scope and GUI/TUI developers as the
  primary audience.
- [x] Create this backlog for implementation tasks, bugs, and longer-term work.
