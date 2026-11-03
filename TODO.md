# Tasks and backlog

This file tracks current work, upcoming milestones, bugs, and longer-term ideas
for `opencoders`. See [DESIGN.md](DESIGN.md) for architecture, scope, and rationale,
[API_COVERAGE.md](API_COVERAGE.md) for route status, and
[RELEASE_READINESS.md](RELEASE_READINESS.md) for the milestone 2 acceptance audit.

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

## Next: interactive conversation APIs (milestone 2)

### Events and session logs

- [ ] Verify retained logs across server restarts and forked sessions using a
  server configuration that enables payload persistence. The v2.2.5 CLI disables
  retention; current live replay/follow checks use explicitly seeded fixture rows.
- [ ] Exercise server queue overflow/slow-consumer behavior against the pinned
  implementation; deterministic tests already cover visible failures and EOF.
- [ ] Next goal: extend the active localhost fixture to verify disconnect,
  reconnect and snapshot refresh during gated execution. Keep previews separate
  from persisted text; record which recovery steps remain application-owned.

### Examples and first usable release

- [ ] Prepare package metadata, review the archive's included files, and replace
  the MIT copyright placeholder with the owner's correct holder/year. Audit-only
  package listing succeeds; packaged-build and publish dry-run checks remain.
- [ ] Write initial release notes and finish the documentation/API ergonomics
  review. Setup, authentication, runtime, operation coverage and tested-version
  guidance already exist; retain the documented execution/recovery limits.
- [ ] Verify milestone 2 acceptance criteria and release-candidate checks from
  `RELEASE_READINESS.md`, including hosted Linux/macOS CI/live runs, before
  declaring the first usable conversation-library release ready.

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
- [ ] Extend the explicit planned routes in `API_COVERAGE.md` as consumer needs
  justify them. Debug, deprecated generation and V1 migration routes are deferred.
- [ ] Document schema-update and release procedures, including compatibility
  checks and review of generated changes.
- [ ] Maintain the milestone 2 package/release checks as additional features land.

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

No unresolved client implementation bugs are recorded.

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
- [ ] Verify timeline changes during pagination and experimental media-reference
  negotiation. The active fixture covers unfinished text and running tools; text
  deltas do not update persisted text until the block ends. The smoke baseline
  verifies fixed imported history, both directions, separate type filters,
  cursor/order rejection, and empty boundary pages without model/tool execution.
- [ ] Verify catalog changes/settlement, live absence of a default model, and
  selection during active execution. Current v2.2.5 tests verify stored selections
  and history markers without model calls; success does not establish availability.
  Nullable defaults are covered by mocks. Active execution selects a configured
  localhost provider; remote-provider readiness remains unverified.
- [ ] Verify delivery during active runs, concurrent admission/cancellation, and
  ID reuse after delivery/cancellation. Active fixtures verify fresh-session
  `resume(true)` waking/delivery. Smoke checks use `resume(false)`, confirming pending-ID
  reconciliation and same-mode update conflicts. Synthetic/compaction/move inbox
  lifecycle and experimental inbox media references also remain unverified.
- [ ] Verify active tool interruption, repeated interruption during stopping,
  resume-true steering/control execution, concurrent admissions, and waits through
  successor runs. Active fixtures verify held-provider interruption, connection
  cancellation, wait timeouts/settlement and success/failure/interrupted outcomes.
- [ ] Verify permission/plugin evaluation, concurrent replies and partial batch
  failures, waiting-tool reject/always/feedback, duplicate pending IDs, and pending
  lifetime across location reload/server restart. The active read fixture verifies
  `once` resumption and provider receipt of the file content. Manual fixtures cover
  reply batching, saved rules, deny precedence and session/location ownership.
- [ ] Verify fork instruction inheritance, active assistant/shell/compaction
  exclusion during actual execution, concurrent mutations, and copies exceeding
  the 500-row batch size. The import fixture path filters unfinished rows before
  storing them. Settled history, boundaries, compaction references, usage reset,
  current configuration, inbox exclusion, and independent ownership are live-tested.
- [ ] Verify form terminal expiry, restart/location teardown, concurrent settlement,
  and question dismissal without feedback. Active fixtures verify answer and
  cancellation-feedback resumption. Manual v2.2.5 fixtures cover discovery,
  all field kinds, validation/default/condition/hidden semantics, answering and
  cancellation, immediate terminal retrieval, ownership, and form events. The
  ten-minute terminal retention is source-reviewed, not tested by waiting.
- [ ] Verify snapshot reconciliation during concurrent execution and disconnection.
  Captured log watermarks are not atomic REST snapshot boundaries or retention
  guarantees; sequences can have gaps and a future `after` can exceed the marker.
  SSE IDs remain opaque metadata, separate from explicit log sequence checkpoints.
- [ ] Determine the earliest useful PTY/WebSocket subset and document any protocol
  details missing from OpenAPI.

## Completed

- [x] Add the separate opt-in `tools/live/active.py` runner and
  `tests/live_active.rs`, reusing the checksum-pinned server and isolated
  environment with a scripted authenticated localhost provider. Restrict tools
  to asking permission for one disposable file and answering questions; disable
  other providers, warming, compaction, sharing and automatic titles. Make no
  real-credential or paid calls; ordinary tests skip the active fixture.
- [x] Verify all six active scenarios on v2.2.5/macOS arm64 (2026-11-03): gated
  text deltas, provider interruption/cancellation, provider failure, permission
  `once` resuming a read, and question answer/cancellation-feedback resumption.
  Assert wait timeouts/settlement, running/terminal history, matching outcomes,
  empty pending work, exactly nine provider calls and deletion of all sessions.
  Record that text deltas remain transient until the block ends, and that a tool
  failure with feedback can still lead to successful execution.
- [x] Add seven pure active-fixture tests and an opt-in manual workflow job;
  update coverage, usage/design, compatibility and readiness records. Validate
  workspace/HTTP-only suites (214 client tests, 39 generator tests, 14 doctests),
  strict Clippy/docs, rustfmt, Rust 1.88 compilation of the new test in both feature
  sets, 15 runner tests, 8 coverage tests/report checks, and actionlint. Linux,
  remote providers, live recovery and other waiting-tool modes remain explicit gaps.

- [x] Add `examples/automation.rs` and example-owned orchestration/outcome policy.
  Explicit `--execute` creates/selects a fresh session, identifies one prompt,
  subscribes before admission, consumes progress while waiting, and inspects
  session/history/inbox/permission/form snapshots. Share a work deadline across
  setup/admission/wait, with a separate bounded inspection window and event/page
  limits. Retain all sessions and document explicit inspection/cleanup.
- [x] Add fourteen deterministic tests distinguishing admission and idleness from
  observed success/failure/interruption, pending input, unanswered requests and
  inconclusive evidence. Cover ambiguous admission, identity mismatches, partial
  or competing history, pagination bounds, failed/timed-out waits/inspection,
  feed degradation and recognized instruction notices. Never retry mutations or
  automatically interrupt, approve requests or delete work.
- [x] Update usage/design/readiness and operation evidence; source-review pinned
  message ordering, terminal projections and instruction notices without claiming
  live execution. Validate workspace/HTTP-only suites (214 client tests, 39
  generator tests, fourteen doctests), all sixteen examples on Rust 1.88 with
  both configurations, strict Clippy, formatting, warning-free docs,
  coverage/eight checker tests and local documentation links. No live model/tool
  calls or automatic commit.
- [x] Add `examples/background_session.rs` with example-owned state and a bounded
  background SSE reader. Subscribe before reading session, one recent history
  page and inbox/permission/form snapshots; refresh periodically and after
  explicit reconnects. Keep bounded text-fragment previews separate from history
  and treat buffered/unknown events as invalidations rather than snapshot patches.
- [x] Report lag/EOF/errors independently of the full event queue, retain stale
  snapshots on failure, publish only complete fetch bundles, and abort/join the
  reader on shutdown. Add nine deterministic HTTP/SSE tests for missed-change
  convergence, snapshot overlap, scoped previews, full queues including during
  refresh, failures/cancellation and independent task lifetime. No UI, public
  state store, model calls or automatic mutation retries.
- [x] Update README/design/readiness and coverage evidence for the observer. Its
  recovery policy is mock-tested; live concurrent ordering, active model output
  and server queue overflow remain unverified. Validate 200 client tests in
  default/HTTP-only configurations, 39 generator tests, fourteen doctests, all
  fifteen examples on Rust 1.88 with both configurations, strict Clippy,
  formatting, warning-free docs, coverage checks/eight checker tests and local
  documentation links.
  No new live-server run or automatic commit of this example.
- [x] Audit all 169 pinned operations: 35 supported with public methods/protocol
  references, 129 planned (34 milestone 3, 95 milestone 4), and 5 explicit
  deferrals. Keep live scope, source-reviewed findings and unverified behavior
  separate in reviewed `openapi/coverage.json` and generated `API_COVERAGE.md`.
- [x] Add offline inventory generation/checking and CI integration. Reject schema,
  route, baseline and reference drift; require explicit new-operation review.
  Document the limits of reference/literal checks rather than claiming behavioral
  coverage from them. Track milestone 2 acceptance gaps and prioritized goals in
  `RELEASE_READINESS.md`; correct stale provenance and README error guidance.
- [x] Validate eight coverage-checker tests, deterministic report checking, both
  workflow files with actionlint, local documentation links and the package file
  list. Package metadata warnings and packaged-build/publish dry-run checks remain
  on the release checklist. No Rust behavior changes or new live-server run were
  part of this audit; no automatic commit or publication.
- [x] Complete typed session forking with TDD: `sessions().fork(&id, &request)`,
  strict generated `SessionForkRequest`, omitted/null/value `before` semantics,
  shared response/provenance types, and generator guards for message IDs and the
  create/get/fork envelope. Preserve encoded IDs, bounded diagnostics, typed
  errors, ordinary deadlines, and no automatic mutation retries.
- [x] Verify forks against disposable v2.2.5/macOS arm64 without model calls:
  full/null/exclusive/before-first history, missing/empty/foreign-boundary errors,
  new message IDs, inherited current configuration/location, reset archive/usage,
  pending inbox exclusion, attachment readability, compaction keep-ID remapping,
  fork events, root/provenance distinction, and survival after source deletion.
- [x] Add `examples/session_fork.rs` plus usage/design/compatibility notes. Explain
  that fork provenance is separate from parent/child ownership, copies use
  projected history, and failures can follow successful creation. Record active
  execution/instruction/concurrency/batch-size/retained-log validation gaps.
- [x] Validate 191 client tests, 39 generator tests, fourteen doctests/examples,
  HTTP-only tests, both Rust 1.88 builds, strict Clippy, docs, formatting, all nine
  generation checks, eight runner tests, and the pinned live smoke test. All
  sixteen live fixture sessions are removed. No automatic commit.

- [x] Complete the typed form lifecycle goal with TDD: location-aware
  `forms().list/list_at` and `sessions().forms(&id).list/get/create/reply/cancel`.
  Preserve envelopes, encoded IDs, request optionality, bodyless cancellation
  feedback, typed form errors, bounded diagnostics, and no automatic retries.
- [x] Add reviewed `--forms` generation and the ninth CI generation check,
  nonempty strict field inputs, six field kinds, scalar/list answers, and explicit
  state dispatch. Share reviewed event response models behind equivalence guards.
  Fix the existing event answer/condition scalar-union decoding bug with a failing
  regression test followed by generator normalization.
- [x] Verify manual forms on disposable v2.2.5/macOS arm64: location and session
  isolation, null/generated/explicit IDs, duplicate IDs, all field kinds,
  default/condition/hidden validation, invalid answers, reply/cancellation states,
  and typed form events. No model calls or external URL fetches.
- [x] Add `examples/forms.rs` and usage/design/compatibility documentation. Validate
  184 client tests, 38 generator tests, thirteen doctests/examples, HTTP-only
  tests, both Rust 1.88 builds, formatting, strict Clippy, docs, all nine generation
  checks, eight runner tests, actionlint, and the pinned live smoke test. Timed
  expiry, real tool resumption, concurrent settlement, and hosted Linux execution
  remain separate validation work. No automatic commit.

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
