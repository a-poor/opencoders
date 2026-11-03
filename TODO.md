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

No active implementation goal.

## Now: contract and foundation (milestone 1)

### Contract and model generation

- [ ] Extend the reviewed production generation paths to additional models
  and decide when to rename the private experiment into the permanent tool.
  Ordinary crate builds must remain independent of generation and schema fetching.
- [ ] Apply the spike findings to broader production models: additive response fields,
  complete event/error dispatch, three-state request fields, numeric constraints,
  and inline endpoint schemas. See `tools/model-spike/README.md`.
- [ ] Create an operation-ID inventory with supported, planned, and deferred
  status so API coverage can be tracked.

### Crate and transport

- [ ] Choose timeout behavior for future long-running operations. Experimental
  session waits have per-call total deadlines, independent of ordinary requests;
  live subscriptions have separate handshake and idle deadlines.
- [ ] Extend response handling to endpoints with multiple success statuses.
  Empty HTTP 204 responses are supported. Add typed API errors as resources need them.
- [ ] Apply the established request/ID and omitted/null/value conventions to
  additional mutations; session creation and updates are implemented.

### Initial validation and repository setup

- [ ] Extend mock-server coverage as additional operations are implemented,
  following the existing JSON body, empty-response, and mutation retry checks.
- [ ] After pushing the workflows, confirm hosted CI and manually run the pinned
  server smoke workflow on Linux/macOS. Local macOS compatibility is verified;
  Linux execution remains unverified.
- [ ] Replace the MIT license's copyright placeholders with the correct details.

## Next: interactive conversation APIs (milestone 2)

### Core resources

- [ ] Implement session forking. Creation, retrieval, updates, and deletion are
  implemented with typed requests and appropriate response handling.
- [ ] Implement pending form discovery and session form lifecycle/replies.
- [ ] Add protocol tests for these resource methods and unknown server errors.

### Events and session logs

- [ ] Verify retained logs across server restarts and forked sessions using a
  server configuration that enables payload persistence. The v2.2.5 CLI disables
  retention; current live replay/follow checks use explicitly seeded fixture rows.
- [ ] Exercise server queue overflow/slow-consumer behavior against the pinned
  implementation; deterministic tests already cover visible failures and EOF.
- [ ] Verify snapshot/event ordering and recovery behavior against upstream;
  document which steps remain the consuming application's responsibility.

### Examples and first usable release

- [ ] Add a runnable Rust example for form replies. Use ordinary output
  or assertions, with no GUI/TUI implementation. Session listing, creation, and
  update/deletion lifecycle, model/agent configuration, message history, and
  prompt/inbox, permission requests/replies, interruption/waiting, live event, and explicit log resumption
  examples are implemented.
- [ ] Add an example of background event consumption using minimal
  application-owned state. Explicit log resumption and snapshot refresh are
  demonstrated in `examples/session_log.rs` without a background task.
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
- [ ] Implement saved-permission listing/update/removal. Request replies can save
  project allow rules, but their management APIs are not yet exposed.
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

- [ ] Verify pairing/session-cookie behavior beyond tested Basic authentication;
  determine which additional convenience helpers to expose.
- [ ] Resolve nested location encoding for routes without an explicit OpenAPI
  serialization style. Discovery routes use documented `deepObject` encoding;
  explicit-null query semantics remain unverified. The v2.2.5 live test confirms
  directory query encoding, header fallback, and query precedence.
- [ ] Choose ergonomic resource names and request construction patterns after
  exercising representative methods in library examples.
- [ ] Verify omitted/null behavior for future mutations. `RequestField<T>` preserves
  all three wire states; v2.2.5 create accepts null for all eight fields and uses
  server-cwd fallback for omitted/null location without a parent. All four session
  update fields are no-ops when omitted/null; metadata/permissions replace contents.
- [ ] Verify generated titling on populated sessions and deletion during active
  execution/concurrent changes when those workflows are implemented. Current live
  checks use empty, idle sessions without model calls; no atomicity is promised.
- [ ] Verify maximum session-list limits, project/subpath filters, and page
  consistency during concurrent changes. v2.2.5 smoke tests confirm cursor-bound
  filters/order, boundary cursors that can lead to empty pages, and zero-limit
  rejection; see `tools/live/README.md`.
- [ ] Verify message snapshots during active execution, timeline changes during
  pagination, and experimental media-reference negotiation. The v2.2.5 baseline
  verifies fixed imported history, both directions, separate type filters,
  cursor/order rejection, and empty boundary pages without model/tool execution.
- [ ] Verify catalog changes/settlement, live absence of a default model, and
  selection during active execution. Current v2.2.5 tests verify stored selections
  and history markers without model calls; success does not establish availability.
  Nullable defaults are covered by mocks. Provider execution remains unverified.
- [ ] Verify prompt waking/execution, delivery during active runs, concurrent
  admission/cancellation, and ID reuse after delivery/cancellation. Current live
  checks use `resume(false)` and queue-only updates, confirming pending-ID
  reconciliation and same-mode update conflicts. Synthetic/compaction/move inbox
  lifecycle and experimental inbox media references also remain unverified.
- [ ] Verify interruption of active provider/tool work, asynchronous cleanup,
  repeated interruption during stopping/settlement, resume-true steering/control
  execution, concurrent admissions, and waits through successor runs. These are
  source-reviewed; current live checks cover only idle and parked-input behavior.
- [ ] Verify permission/plugin evaluation, concurrent replies and partial batch
  failures, actual tool resumption/feedback, duplicate pending IDs, and pending
  lifetime across location reload/server restart. Current live fixtures manually
  evaluate requests without executing tools; they confirm reply batching, saved
  allow effects, configured-deny precedence, and session/location ownership.
- [ ] Verify snapshot reconciliation during concurrent execution and disconnection.
  Captured log watermarks are not atomic REST snapshot boundaries or retention
  guarantees; sequences can have gaps and a future `after` can exceed the marker.
  SSE IDs remain opaque metadata, separate from explicit log sequence checkpoints.
- [ ] Determine the earliest useful PTY/WebSocket subset and document any protocol
  details missing from OpenAPI.

## Completed

- [x] Complete permission request APIs with TDD: location-aware
  `permissions().list/list_at` and `sessions().permissions(&id).list/get/create/reply`.
  Preserve envelopes, server order, input wire states, encoded IDs, typed
  `PermissionNotFound`, bounded diagnostics, and the default no-retry policy.
- [x] Add reviewed `--permissions` generation and the eighth CI generation check.
  Reuse event IDs/reply/source models and existing effect/session/location types;
  keep strict inputs separate from tolerant responses. Guard nullable/optional
  adapters and tool-source shape; provide explicit input constructors.
- [x] Verify allow/deny/ask evaluation, pending ownership/location isolation,
  once/reject/always, batch rejection, cross-session saved-rule effects, deny
  precedence, nullable inputs, and live permission events on disposable
  v2.2.5/macOS arm64 without model/tool execution. Record the source-reviewed and
  live-tested boundaries; keep approval policy application-owned.
- [x] Add `examples/permissions.rs` and usage/design/compatibility documentation.
  Validate 176 client tests, 36 generator tests, twelve doctests/examples,
  HTTP-only tests, both Rust 1.88 builds, formatting, Clippy, docs, all eight
  generation checks, eight runner tests, actionlint, and the live smoke test.
  Actual tool resumption, concurrency/reload behavior, saved-rule management,
  and hosted Linux checks remain tracked separately. No automatic commit.

- [x] Complete session execution control with TDD: `sessions().interrupt(&id,
  &options)` preserves resume omission/false/true and returns the generated
  `SessionInterruptResponse`; `experimental_wait(&id, &options)` accepts HTTP 204
  with an independent optional total deadline. Share bounded responses, typed
  errors, safe path encoding, and the existing default no-retry policy.
- [x] Test required/extended response fields, both interruption booleans, request
  encoding, statuses/errors, body bounds, injected/ordinary/per-call deadlines,
  cancellation of independent waits, empty keep-alive responses, and disconnects.
  Document acceptance versus cleanup, process-local idle versus execution outcome,
  resume-true behavior, and local cancellation versus server interruption.
- [x] Verify idle no-ops, every resume query choice on empty sessions, parked queue
  behavior with resume true, held steering input with omitted/false resume, waits
  with pending input, and missing sessions on disposable v2.2.5/macOS arm64 without
  model calls. Add `examples/execution.rs` and update usage/design/compatibility docs.
- [x] Validate 166 client tests, 34 generator tests, eleven doctests/examples,
  HTTP-only tests, both Rust 1.88 builds, formatting, Clippy, docs, all seven
  generation checks, eight runner tests, actionlint, and the live smoke test.
  Active provider execution and hosted Linux validation remain tracked separately;
  no automatic commit.

- [x] Complete durable session logs with TDD: `sessions().experimental_log(&id,
  &options)` supports exclusive `after`, explicit finite/follow modes, typed
  `log.synced` markers, and separate SSE metadata. Reuse bounded caller-driven
  transport with explicit cancellation, deadlines, terminal errors, and no retries.
- [x] Extend reviewed event generation to 62 kinds across live/log models,
  including all 47 durable session events. Preserve empty versus zero watermarks,
  unknown payloads, extensions, and numeric bounds; reject known transient log
  kinds and guard durable-envelope/marker adapters against schema drift.
- [x] Test replay gaps, future cursors, marker handling, explicit resumption,
  cancelled reads, independent connections, and errors. Confirm the v2.2.5 CLI
  advances markers without retaining payloads; separately verify replay/follow
  with actual captured events seeded into its disposable database, without model
  calls. Record the retention limitation and keep snapshot policy application-owned.
- [x] Add `examples/session_log.rs`, public API guidance, and architecture/live
  compatibility notes. Validate 153 client tests, 34 generator tests, ten doctests
  and examples, HTTP-only tests, both Rust 1.88 builds, formatting, Clippy, docs,
  all seven generation checks, eight runner tests, actionlint, and the live smoke
  test. Hosted Linux checks remain pending; no automatic commit.

- [x] Complete live event subscriptions with TDD: `events().subscribe(&options)`
  returns an independent `EventStream`, with repeated type filters, SSE metadata,
  separate handshake/idle deadlines, explicit close/drop cancellation, and one
  terminal error or clean EOF. No background queue, retry, replay, or UI.
- [x] Generate 52 reviewed event kinds through `--events`, with strict dispatch,
  raw fallback for other kinds, preserved envelope/extensions, safe integers,
  exact durable versions, and nonempty content/form fields. Validate every kind
  with fixtures and guard generation adapters against schema drift.
- [x] Implement bounded incremental SSE framing, BOM/UTF-8 and CR/LF/CRLF handling,
  comments, multiline data, metadata persistence, partial-event EOF discard, and
  protocol failure frames. Test fragmented streams, heartbeat/timeout behavior,
  injected transport deadlines, cancelled reads, and independent connection cleanup.
- [x] Verify filtered global feeds, connected markers, cross-location renames,
  inbox admission/cancellation, and surviving subscriptions on disposable
  OpenCode v2.2.5/macOS arm64 without model calls. Add `examples/events.rs`, usage,
  architecture/compatibility notes, and the seventh CI generation check.
- [x] Validate 141 client tests, 33 generator tests, nine doctests/examples,
  HTTP-only tests, both Rust 1.88 builds, formatting, Clippy, docs, all seven
  generation checks, actionlint, runner tests, and the live smoke test. Durable
  logs/recovery, overflow-specific live behavior, and hosted Linux checks remain
  tracked separately. No automatic commit.


- [x] Complete prompt/inbox APIs with TDD: `sessions().prompt(&id, &request)` and
  `sessions().inbox(&id).list/update/cancel`. Preserve admitted/list envelopes,
  enqueue order, encoded IDs, and all request wire states; distinguish admission
  from execution and pending state. Expose typed conflicts with bounded diagnostics.
- [x] Add reviewed `--inbox` generation and CI coverage. Reuse shared IDs and
  prepared attachment response models; keep strict URI-based inputs separate.
  Explicitly dispatch all four inbox kinds, validate non-null opaque payloads,
  preserve nullable/optional/empty values, and guard schema changes. Correct the
  generator's permissive nested file-source decoding with a tested strict adapter.
- [x] Verify request bodies, errors, response bounds, unexpected statuses, and no
  mutation retries after disconnects. Live v2.2.5/macOS arm64 checks confirm held
  admission, inline attachment preparation, pending-ID reconciliation, conflicts,
  enqueue order, steer-to-queue changes, and cancellation without model calls.
- [x] Add `examples/prompt_inbox.rs`, public usage docs, and compatibility notes.
  Validate 119 client tests, 32 generator tests, eight doctests, HTTP-only tests,
  both Rust 1.88 builds, formatting, Clippy, docs, eight examples, all six generation
  checks, actionlint, and the disposable live smoke test. Hosted Linux validation
  remains pending; no publishing, pushing, or automatic commit.
- [x] Complete model and agent selection with TDD: location-aware `models().list`
  and `default`, individual `agents().get`, and session `switch_agent`/`switch_model`.
  Preserve envelopes, server order, repeated type filters, and caller location
  overrides; expose typed missing-agent errors and accept only HTTP 204 for switches.
- [x] Extend reviewed discovery/session generation with open model-type/reasoning
  strings, required nullable default data, signed safe-integer limits, redacted
  catalog debug output, and strict non-null selection inputs. Guard schema changes
  and reuse shared types; no new generation command or dependency is needed.
- [x] Verify request encoding, malformed responses, nullable defaults, typed/unknown
  errors, unexpected statuses, and no mutation retry after disconnects. Live
  v2.2.5/macOS arm64 checks confirm discovery, stored choices, unknown-ID acceptance,
  omitted variant normalization, and repeated-agent/model history behavior without
  model calls. Add `examples/configure_session.rs` and update compatibility docs.
- [x] Validate 109 client tests, 29 generator tests, seven doctests, HTTP-only tests,
  both Rust 1.88 builds, formatting, Clippy, docs, seven examples, all five generation
  checks, and the disposable live smoke test. Hosted Linux validation remains pending;
  no publishing, pushing, or automatic commit.
- [x] Complete message history/retrieval with TDD: `sessions().messages(&id)`
  exposes typed `list(&MessageListOptions)` and `get(&message_id)` methods. Preserve
  data/cursor envelopes, server ordering, additive fields, and safe ID encoding;
  expose typed missing-message errors and explicit limit/order/cursor/type filters.
- [x] Add reviewed `--messages` generation and CI coverage. Reuse shared IDs and
  models; dispatch all eleven timeline kinds and nested assistant/tool/compaction
  unions explicitly. Preserve numeric precision, enforce reviewed bounds and
  nonempty tool content, and guard schema/tag/optionality changes before adaptation.
- [x] Verify mock decoding, request/query encoding, pagination, malformed/unknown
  values, and HTTP errors. Extend the live v2.2.5/macOS arm64 test with seven fixed
  imported messages, attachments, reasoning/tool records, bidirectional filtered
  traversal, cursor/order errors, wrong-session retrieval, and empty pages.
  Fixture import schedules no model/tool execution and is not a public SDK method.
- [x] Add `examples/history.rs`, crate/README usage, and compatibility notes.
  Validate 96 client tests, 27 generator tests, six doctests, the live smoke test,
  HTTP-only tests, both Rust 1.88 builds, formatting, Clippy, docs, six examples,
  all five regeneration checks, and actionlint. Hosted Linux checks remain pending;
  no publishing or automatic commit.
- [x] Complete session update/deletion with TDD: `sessions().update(&id, &request)`
  and `sessions().remove(&id)` use encoded ID paths, ignore the discovery location
  default, and accept only HTTP 204 without JSON decoding. Share deadlines, bounded
  diagnostics, typed errors, and the default no-retry policy with existing methods.
- [x] Generate `SessionUpdateRequest` through `--sessions`, preserving all four
  omitted/null/value states and reusing strict nested input types. Guard request
  shapes and add fluent title/metadata/permissions/archive setters.
- [x] Verify request bodies, empty responses including keep-alive, unexpected
  statuses, typed failures, deadlines, and no retries after disconnects. Extend the
  pinned v2.2.5/macOS arm64 test to cover rename, metadata/permission replacement
  and clearing, archive/restore, omitted/null no-ops, recursive deletion through
  three levels, and repeated-delete errors. All fixtures now use SDK methods.
- [x] Add `examples/session_lifecycle.rs` and document server-generated titling,
  partial/ambiguous mutation outcomes, and recursive deletion. Validate 83 client
  tests, 24 generator tests, five doctests, the live smoke test, HTTP-only tests,
  both Rust 1.88 feature builds, formatting, Clippy, docs, five examples, and all
  four regeneration checks. No model calls, publishing, or automatic commit.
- [x] Complete typed session creation and retrieval: `sessions().create(&request)`
  and `sessions().get(&id)` preserve the `SessionResponse` envelope, encode IDs as
  single path segments, and expose typed missing-session/parent errors.
- [x] Extend `--sessions` with inline create/get schemas, strict nested input
  types, and `RequestField<T>` for omitted/null/value creation properties. Keep
  response extensions separate from inputs; guard request/envelope/variant schema
  adaptations. Add constructors and fluent value setters.
- [x] Add JSON POST support sharing deadlines, bounded responses, and diagnostics;
  force the JSON content type and distinguish pre-send encoding errors. Verify
  that disconnects after a POST body do not retry, and document ambiguous outcomes
  and caller-owned policies on injected transports.
- [x] Replace live fixture creation with SDK methods; verify create/get round trips,
  metadata, empty permissions, cross-directory retrieval, parent precedence,
  missing IDs/parents, empty requests, and all eight properties explicitly null on
  OpenCode v2.2.5/macOS arm64 without model calls. Add `examples/create_session.rs`.
- [x] Validate 73 client tests, 23 generator tests, four doctests, the live smoke
  test, HTTP-only tests, both Rust 1.88 feature builds, formatting, Clippy, docs,
  all four examples, and all four regeneration checks. Hosted Linux validation
  remains pending separately.
- [x] Complete the CI/live-baseline goal: add GitHub Actions checks for formatting,
  Clippy, workspace tests, docs/examples, default/HTTP-only features, Rust 1.88
  library builds, and all four generation targets (Rust/rustfmt 1.99.1).
- [x] Pin OpenCode v2.2.5, its commit, and macOS arm64/Linux x86-64 archive hashes.
  Add an opt-in disposable runner and manual workflow. Verify server info,
  location/agent discovery, and session listing against the real macOS release
  with temporary state and empty session fixtures; make no model calls.
- [x] Add and test `ClientBuilder::basic_auth`, correct example authentication,
  and document v2.2.5 cursor-bound filters/order and boundary cursors that can lead
  to empty pages. Exercised responses need no generated-model changes. Record
  the tested scope and remaining limitations in `tools/live/README.md`.
- [x] Validate 60 client tests, 22 generator tests, three doctests, six runner
  tests, and the live smoke test; HTTP-only tests, both MSRV feature builds,
  formatting, Clippy, docs/examples, all regeneration checks, and actionlint pass.
  Hosted workflow execution awaits a push; changes are not committed automatically.
- [x] Complete session listing with TDD: typed `SessionListOptions`, ordering and
  parent filters, direct query encoding, `client.sessions().list(&options)`, and
  typed invalid-cursor errors. Preserve omission versus explicit false/empty/root
  filters; the SDK's default location does not affect this list route.
- [x] Add reviewed session models through `--sessions`, reusing shared discovery
  types. Preserve nested extensions, opaque next/previous cursors, and server
  page order. Validate ID prefixes, safe-integer file-diff counts, and true-only
  flags; numeric fields retain supported integer precision through JSON numbers.
- [x] Add the bounded caller-controlled `examples/sessions.rs`, public API docs,
  and README pagination guidance. Validate 59 client tests, 22 generator tests,
  three doctests, all examples, HTTP-only tests, Rust 1.88, formatting, Clippy,
  docs, and all four regeneration checks. Live cursor/filter semantics, limit
  ranges, and consistency during concurrent changes remain to be verified.
- [x] Complete location-aware discovery with TDD: default location configuration,
  per-call directory overrides/omission, `location.get`, and `agent.list`.
  Queries preserve server path strings; the SDK default never leaks onto
  server-wide routes. Agent responses retain resolved location metadata and
  additive fields. Caller location headers survive query omission.
- [x] Add typed missing-location/unavailable-service errors and extend reviewed
  generation with `--discovery`. Guard schema adaptations for positive agent
  step limits, string cache intervals, and number-or-false provider timeouts.
  Redact provider request configuration in default debug formatting.
- [x] Add `examples/discovery.rs`, crate/README usage, and documented response
  compatibility policies. Validate 44 client tests, 21 generator tests, two
  doctests, both examples, HTTP-only tests, Rust 1.88, formatting, Clippy, docs,
  and all three model regeneration checks. Live-server/header fallback and
  explicit-null query behavior remain unverified; no live compatibility claim.
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
