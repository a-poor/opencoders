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

- [ ] Complete the seven-operation stable shell goal (76 → 83 supported):
  `shell.create/get/list/output/remove`, `session.shell`, `session.environment`.
  Review v2.2.5 execution, output paging/UTF-8, retention and environment semantics;
  generate strict inputs and shared responses; preserve metadata/numbers and
  redact sensitive Debug output. Add model/protocol regressions and owned live
  fixtures for exits, timeouts, termination, paging, session events/history and
  environment replacement. Update guides/example/coverage and generation guards;
  pass workspace/feature/MSRV, formatting/lints/docs, all generation checks and
  pinned compatibility, then commit/push and verify Linux CI. Keep background
  ownership, experimental/deprecated/migration APIs and publishing out of scope.
  Implementation and local validation complete: 339 workspace tests/doctests
  (2 opt-in ignored), 38 Python tests, four Rust 1.88 feature builds and examples,
  fmt/clippy/docs, 14 generation checks, and pinned v2.2.5 smoke/active fixtures.
  Hosted Linux CI and compatibility verification remain before goal completion.

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
  live subscriptions, file reads and PTY sockets have separate handshake and idle
  deadlines.
- [ ] Extend remaining non-JSON endpoints using the streamed-response foundation.
  Empty HTTP 204 and file 200/206/304/416 responses are supported. Add typed API
  errors as resources need them.
- [ ] Apply the established request/ID and omitted/null/value conventions to
  additional mutations; session creation and updates are implemented.

### Initial validation and repository setup

- [ ] Extend mock-server coverage as additional operations are implemented,
  following the existing JSON body, empty-response, and mutation retry checks.

## Next: interactive conversation APIs (milestone 2)

### Events and session logs

- [ ] Verify retained logs across server restarts and forked sessions using a
  server configuration that enables payload persistence. The v2.2.5 CLI disables
  retention; current live replay/follow checks use explicitly seeded fixture rows.
- [ ] Exercise server queue overflow/slow-consumer behavior against the pinned
  implementation; deterministic tests already cover visible failures and EOF.

### Examples and first usable release

Version 0.1.0 was published and tagged on 2026-11-03; see the
[verified release record](RELEASE_READINESS.md#published-artifact).

## Soon: terminal and workspace APIs (milestone 3)

The non-experimental workspace and ordinary-PTY slice is complete, including
worktree lifecycle and VCS initialization. See the goal records under **Completed**.
The remaining 14 milestone-3 operations are experimental and stay out for now:
- [ ] Leave experimental filesystem write/watch/unwatch out for now; reconsider
  behind an opt-in feature after reviewing mutation scope and watch lifecycle.
- [ ] Leave experimental persistent PTYs out for now; reconsider behind an opt-in
  feature after reviewing ownership, handoff, persistence and replay separately.

## Later: broader API coverage and release hardening (milestone 4)

- [ ] Implement remaining stable session operations such as background and moves.
  Activity, viewed acknowledgements, synthetic input, commands, compaction,
  revert, context/diff, shell commands and environment replacement are implemented;
  see the goal records. Explicit
  skill invocation, instruction management and import/export are experimental in
  the current pin and stay out until a separate opt-in review.
- [ ] Implement remaining configuration, project, skill and reference APIs.
  Command/skill list discovery and stable project/config/shell/reference reads
  are implemented. Mutations and experimental operations need separate review.
- [ ] Implement provider, integration, and credential management APIs.
  Stable provider list/get discovery is implemented; connection/credential changes
  remain a separate goal.
- [ ] Implement MCP management/resources and client-hosted MCP WebSockets.
- [ ] Implement plugin management and generic plugin RPC calls/events with typed
  envelopes and support for dynamic payloads.
- [ ] Implement generation, voice, web search, and usage APIs.
- [ ] Implement schedule, artifact, and media APIs, preserving streamed
  binary responses where applicable.
- [ ] Implement experimental background-job discovery for shell, subagent and
  image work, including session/descendant, kind and status filters. Review
  unconsumed-job semantics against a pinned server before exposing the route.
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
- [ ] Reconsider hosted macOS testing if OS-specific functionality or dependency
  regressions justify its cost and runner delays. Local macOS testing remains
  the primary development path; hosted checks currently use Linux.
- [ ] Reduce repeated contract details between `docs/` and `DESIGN.md`, keeping
  guides focused on usage and design focused on rationale. README is now a
  minimal entry point; duplication caused stale coverage statements during review.
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
- [ ] Review resource naming and request construction as APIs expand beyond
  conversations; current resource accessors/builders are exercised by sixteen
  examples and documentation snippets, with no release-blocking change identified.
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
- [ ] Extend recovery validation to arbitrary concurrent changes, server restart
  and remote-network failures. Controlled live cuts now verify missed text/form
  changes and delayed old history with the shared observer after quiescence.
  Captured log watermarks are not atomic REST snapshot boundaries or retention
  guarantees; sequences can have gaps and a future `after` can exceed the marker.
  SSE IDs remain opaque metadata, separate from explicit log sequence checkpoints.

## Completed

- [x] Complete stable app-setup discovery: project list, config documents/shells,
  provider list/get and reference catalogs (70 → 76 supported operations),
  committed in `2b17889`. Preserve array/envelope shapes, location overrides,
  archive filters, server ordering, shared models and exact numbers. Detailed
  configuration types validate unions/bounds and redact Debug output; boxed
  document entries keep directory entries small. Missing providers are typed.
  Local validation: 330 workspace tests/doctests pass (2 opt-in ignored), final
  adapter regressions, four Rust 1.88 feature builds and examples, fmt/clippy/docs,
  13 generation targets, 38 Python tests and pinned v2.2.5 smoke/active suites.
  All seven jobs in [CI 42726525495](https://github.com/a-poor/opencoders/actions/runs/42726525495)
  and both jobs in [pinned compatibility 42726566487](https://github.com/a-poor/opencoders/actions/runs/42726566487)
  pass at the implementation revision. Archive-read checks use owned seeded
  metadata; remote references/providers, managed config, hot reload and Windows
  shells remain outside live evidence. Docs/example/coverage are updated;
  instruction management is correctly classified as experimental and left out.

- [x] Complete stable session activity, viewed acknowledgements and synthetic input
  (67 → 70 supported operations), committed in `5cb5fcf`.
  Typed process-local activity maps validate session keys; view requests preserve
  exact observed idle numbers and stale acknowledgements leave later turns unread.
  Synthetic inputs preserve omission/null/empty values, share inbox models,
  reconcile the first payload and retain staged reverts without automatic retries.
  Local macOS validation: 322 workspace tests/doctests pass (2 opt-in ignored),
  four Rust 1.88 feature builds plus examples, fmt/clippy/docs, all 12 generation
  checks, 35 Python checks and disposable v2.2.5 smoke/active suites.
  All seven jobs in [CI 42722059887](https://github.com/a-poor/opencoders/actions/runs/42722059887)
  and both jobs in [pinned compatibility 42722320149](https://github.com/a-poor/opencoders/actions/runs/42722320149)
  pass on the implementation revision. Docs, example and API coverage are updated;
  experimental, deprecated and migration-only APIs remain excluded.

- [x] Commit the workspace/session workflows, validate Linux, and complete stable
  saved-permission list/update/remove (64 → 67 supported operations).
  `3ff1edb` passed [CI 42713702937](https://github.com/a-poor/opencoders/actions/runs/42713702937)
  and [pinned compatibility 42713795590](https://github.com/a-poor/opencoders/actions/runs/42713795590).
  Saved-permission implementation `ea1df6f` passed all seven jobs in
  [CI 42715169271](https://github.com/a-poor/opencoders/actions/runs/42715169271)
  and both jobs in [pinned compatibility 42715208158](https://github.com/a-poor/opencoders/actions/runs/42715208158).
  Local macOS validation: 317 workspace tests/doctests pass (2 opt-in ignored),
  four Rust 1.88 feature builds and examples, fmt/clippy/docs, all 12 generation
  targets, 34 Python checks and disposable pinned smoke pass. Strict nullable
  updates reuse project IDs and preserve numeric metadata; tests cover project
  isolation/fallback, empty/null inputs, conflicts, unchanged pending requests,
  future evaluations, deny precedence, missing updates and repeated deletion.
  All changes are committed/pushed; experimental, deprecated and migration-only
  APIs remain out. See the read-only example and permission usage guide.

- [x] Complete stable session workflows: command/skill discovery, command execution,
  context and turn diffs, durable compaction, and staged/cleared/committed reverts.
  Nine new routes bring coverage from 55 to 64; experimental/persistent,
  deprecated and migration APIs remain out. Strict generated inputs reuse
  reviewed wire models; typed errors retain command/busy/server-reference details.
  The client adds no automatic mutation retries or UI state.
  Validation (2026-11-09, macOS arm64): 311 workspace tests/doctests pass with all
  features (2 opt-in tests ignored); Rust 1.88 builds all four feature combinations
  and examples. Formatting, strict Clippy/rustdoc, all 12 generation targets, 34
  Python tests and coverage checks pass; the complete disposable smoke suite
  (including WebSocket PTYs and worktree lifecycle) also passes. Active checks exercise
  commands/skills, real Git snapshots/diffs and restoration, busy errors, clearing
  a revert waking parked work, completed compaction/context, automatic boundary
  commit and permanent inclusive deletion. Linux validation passed at `3ff1edb` in CI 42713702937 and pinned compatibility
  42713795590 (2026-11-09).

- [x] Verify pushed schema/workspace/PTY commits on Linux, then complete stable
  workspace lifecycle: worktree create/refresh/remove and VCS initialization.
  - [x] Push `cf8237f` and `88224a1` to `main`.
  - [x] Verify ordinary CI and manual pinned-server smoke/active checks on
    `88224a1`: [CI](https://github.com/a-poor/opencoders/actions/runs/42707538439)
    and [compatibility](https://github.com/a-poor/opencoders/actions/runs/42707625108).
  - [x] Review pinned contracts/implementation, generate strict inputs and typed
    worktree failures, and add test-first resource methods with no SDK retries.
  - [x] Verify disposable Git/worktree lifecycle, force-required failures,
    discovery refresh and initialization conflicts without touching user data.
  - [x] Update usage/examples, explicit coverage (51 to 55 operations), and
    compatibility evidence; pass generation, feature/MSRV, fmt/lint/docs checks.
  Complete this goal before the proposed session-workflow slice (command/skill
  discovery and execution, context/diff, compaction and revert). Experimental,
  persistent, deprecated and migration-only APIs remain outside these goals.
  Validation (2026-11-09, macOS arm64): 304 workspace tests/doctests pass with
  all features (2 opt-in tests ignored), all four feature combinations build on
  Rust 1.88, examples compile, and fmt/clippy/docs, 11 generation targets, 33
  Python checks and the coverage report pass. The new lifecycle fixture passes
  against disposable v2.2.5 locally; hosted evidence above applies to `88224a1`.


- [x] Complete workspace reads and ordinary PTYs: filesystem list/find/streamed
  read; read-only VCS info/base/status/branches/diff and worktree listing; PTY
  lifecycle/resize, connection tickets, and optional WebSocket I/O. Keep runtime,
  rendering and terminal state caller-owned. Supporting VCS/worktree discovery
  makes workspace reads useful without expanding into repository mutations.
  - [x] Review pinned schemas/server behavior and record transport/API decisions.
  - [x] Add reviewed generated models and test-first typed resource methods.
  - [x] Preserve binary response status/headers, range/conditional semantics,
    bounded errors, cancellation/deadlines, and the default no-retry policy.
  - [x] Add optional WebSocket PTY transport with tested framing, tickets,
    input/output, resize, closure, and explicit reconnection boundaries.
  - [x] Verify disposable pinned-server workflows; add examples and usage/
    compatibility notes; update explicit operation coverage and generation CI.
  - [x] Pass workspace/feature/MSRV/lint/docs/generation and relevant live checks.
  Experimental filesystem writes/watch and persistent PTYs are deferred from
  this goal. Deprecated generation and V1 migration remain out of scope; future
  unstable API families may be exposed behind opt-in features after review.
  Validation (2026-11-09, local macOS arm64): 296 workspace tests/doctests with
  all features (2 opt-in live tests ignored); default/HTTP-only/HTTP-WebSocket
  suites pass; all four feature combinations and examples compile on Rust 1.88;
  fmt, all-feature clippy/docs, all 11 generation checks, 33 Python checks and
  coverage validation pass. The disposable v2.2.5 smoke run with WebSockets
  passes separately. New Linux live coverage is configured but not yet executed.
  Adds 16 operations (51/170 supported). Usage: `docs/workspace.md` and
  `docs/terminals.md`. This implementation follows the schema refresh in `cf8237f`.

- [x] Review and adopt the 2026-11-09 upstream schema drift from
  [the scheduled check](https://github.com/a-poor/opencoders/actions/runs/42687061589).
  Confirm the candidate matches its reported hash, update provenance and explicit
  coverage to 170 operations / 497 schemas, and record the full
  [contract review](openapi/README.md#2026-11-09-drift-review). Classify background-job
  discovery as milestone 4; preserve the separately tested v2.2.5 baseline.
  All nine pinned-toolchain generation checks reproduce unchanged Rust models;
  the full 497-component generation/compilation probe, workspace tests/doctests,
  nine schema tests, eight coverage tests and coverage report check pass.

- [x] Publish [opencoders 0.1.0](https://crates.io/crates/opencoders/0.1.0) from
  clean commit `d587570` on 2026-11-03 and push the annotated `v0.1.0` tag at that
  revision. Verify registry metadata, the downloaded archive's exact checksum and
  the successful [docs.rs build](https://docs.rs/opencoders/0.1.0/opencoders/).
- [x] Verify the final README/session-prompt example on Rust 1.88 and the clean
  package/dry run. On `d587570`, all five Linux CI jobs pass, manual schema checking
  finds no drift, and Linux smoke/active compatibility passes. Record exact run
  links and the published checksum in `RELEASE_READINESS.md`.

- [x] Reduce ordinary CI and manual live-server workflows to Linux. Keep local
  macOS runners and historical compatibility evidence; track reconsidering hosted
  macOS coverage in the backlog.
- [x] Add CI, docs.rs, crates.io and MIT/Apache-2.0 badges to the 59-line README,
  preserving the author's title cleanup. All badge endpoints return SVGs; docs
  and crate-version badges await publication. The updated 116-file package builds.
- [x] Add a read-only weekly/manual upstream schema check (Monday 09:17 UTC),
  ignoring JSON whitespace/key order and reporting bounded diffs. Nine offline
  behavioral tests pass and run in ordinary CI. A real 2026-11-03 download matches
  the pin byte-for-byte. Workflow lint, coverage checks, formatting and 272 local
  links/anchors pass. No schema, generated model or SDK behavior changes.

- [x] Commit/push hosted-validation records and finalize candidate `e86c6fa`:
  version 0.1.0 and planned changelog date 2026-11-03, matching the README.
  [CI 41686406437](https://github.com/a-poor/opencoders/actions/runs/41686406437)
  passes all seven jobs. The clean 116-file archive passes fresh extracted tests
  in both modes (231 each; two live tests ignored), stable/MSRV example checks,
  warning-free docs and a publish dry run without `--allow-dirty`. Record the
  exact checksum/revision in `RELEASE_READINESS.md`. No publication or tag.

- [x] Verify hosted OpenCode v2.2.5 smoke and all eight active execution/recovery
  scenarios on Linux x86-64 and macOS arm64 on 2026-11-03. All four jobs pass in
  [run 41680124254](https://github.com/a-poor/opencoders/actions/runs/41680124254)
  for `75f0f2a`, and all seven ordinary CI jobs pass in
  [run 41679740026](https://github.com/a-poor/opencoders/actions/runs/41679740026).
  No SDK changes were needed. Update the coverage baseline and compatibility/
  readiness records; remote providers, overflow and restart remain unverified.
- [x] Review and pin checkout v7.0.1 (`3d3c42e5`) using Node.js 24 in both
  workflows. Commit/push as `75f0f2a`, validate with actionlint, and rerun hosted
  CI and compatibility successfully after the update. Final logs have no
  action-runtime deprecation warnings.
- [x] Validate the updated 116-file working-tree package based on `75f0f2a`:
  fresh extracted default/HTTP-only tests (231 each; two live tests ignored),
  Rust 1.88 library/examples in both modes, warning-free API docs, and crates.io
  publish dry run. The dry-run archive matches the tested checksum recorded in
  `RELEASE_READINESS.md`. Coverage/tests, actionlint, formatting and 266 repository
  links/anchors pass. Evidence/docs remain reviewable; no publication or tag.

- [x] Replace the roughly 1,000-line README with a 54-line entry point: crates.io
  `0.1.0` installation, a read-only quick start, pinned API provenance and licenses.
  Move detailed usage into seven topic guides plus `docs/README.md`; retain the
  existing behavior and compatibility notes and update contributor/release links.
- [x] Include the guides in the crate archive and update the automation example's
  documentation pointer. Compile the new quick start and thirteen unchanged
  moved snippets on Rust 1.88 against the local release candidate. Verify 262
  repository links/anchors, all eight packaged documentation files, the 116-file
  package build, formatting and coverage checks. No publication or live run.

- [x] Push through `e0bd6de` to `origin/main` and wait for
  [CI run 41669812365](https://github.com/a-poor/opencoders/actions/runs/41669812365).
  All seven jobs pass: Linux/macOS default and HTTP-only tests, quality, MSRV,
  and generation. Record live-server validation as a separate pending workflow.
- [x] Review onboarding, API descriptions, generation/release instructions and
  contributor guidance. Add README workflow navigation; correct stale location,
  execution, generation, error and recovery guidance. Compile all fourteen README
  Rust snippets on Rust 1.88, check repository links/anchors and coverage, and
  verify generation instructions match all nine CI commands. Record duplicated
  workflow docs and the observed checkout-action warning as backlog follow-ups.

- [x] Prepare 0.1.0 package metadata and an explicit 108-file archive containing
  source, examples, tests/fixtures, README, release notes and both licenses.
  Keep schema/generation, coverage/live tooling and planning documents in the
  repository; preserve normal builds' independence from those tools.
- [x] Fill the MIT notice with the owner's confirmed `2026 Austin Poor`
  attribution and preserve the matching Apache appendix edit. Add an unreleased
  `CHANGELOG.md`, review public usage, make README setup/status and repository
  links explicit, and document the reproducible checks in `RELEASING.md`.
- [x] Build and inspect the archive, then verify it outside the checkout with a
  fresh build directory on macOS arm64: 217 library/integration tests and 14
  doctests pass with default and HTTP-only features; both live tests stay ignored.
  Compile the library and all sixteen examples on Rust 1.88 in both configurations;
  build warning-free docs. Cargo's publish dry run passes with registry access
  and performs no upload. Record the working-tree checksum and remaining hosted/
  final-revision gates in `RELEASE_READINESS.md`; no push, tag or publication.

- [x] Extend the isolated active fixture with a test-only GET relay and two live
  recovery scenarios using the background example's unchanged connection/state
  module. Verify real SSE transport cuts, stale snapshots/cleared previews,
  explicit reconnect, missed text/form changes, and a captured old history
  response delivered after settlement. Bounded refresh converges after quiescence;
  witness events only synchronize tests and never populate recovered state.
- [x] Fix the instruction-update decoder exposed by the unfiltered live feed.
  Guard the disjoint hash-or-`"removed"` schema before generating a scalar enum
  instead of an incorrect flattened struct. Add a failing-then-passing decoding
  regression and schema-drift guards; document the generated public type change.
- [x] Verify all eight active scenarios on v2.2.5/macOS arm64 (2026-11-03), with
  exactly twelve localhost provider requests and deletion of all eight sessions.
  Add two deterministic relay tests and an eighth pure active-fixture test.
  Validate workspace/HTTP-only suites (217 client tests, 40 generator tests,
  14 doctests), strict Clippy/docs, rustfmt, Rust 1.88 compilation of the library
  and recovery tests in both feature sets, all nine generation checks, 16 runner
  tests and 8 coverage tests/report checks. Record the unfinished-text recovery
  gap; arbitrary concurrent ordering, server overflow, restart, remote networks,
  the example CLI timer/backoff loop and Linux remain unverified live.

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
