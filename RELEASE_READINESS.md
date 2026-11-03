# Milestone 2 release readiness

Updated **2026-11-03**, against the checked-in schema, the recorded OpenCode
**v2.2.5 / macOS arm64** smoke run and a new active localhost-provider run.
This does not establish Linux or remote-provider compatibility. It tracks remaining
work for the first usable release; no release or publication has been performed.

The core conversation resources are implemented, but **milestone 2 is not yet
complete**. The remaining work is chiefly package/release preparation and hosted
validation. PTY support is milestone 3 and does not gate the first
conversation-library release. No UI is being built.

## Coverage and evidence

[API_COVERAGE.md](API_COVERAGE.md) classifies all **169** pinned operation IDs:

| Status | Count | Meaning |
| --- | ---: | --- |
| Supported | 35 | Public Rust methods with protocol tests; live scope varies. |
| Planned, milestone 3 | 34 | Terminal, filesystem, worktree and VCS APIs. |
| Planned, milestone 4 | 95 | Broader session, configuration, integration and other APIs. |
| Deferred | 5 | Three debug routes, deprecated text-generation alias and V1 migration status. |

These counts measure route implementation, not behavioral coverage. The report
links each supported operation to its method, representative protocol assertions,
live assertions and limitations. Generated model availability and raw fixture
imports do not count as public endpoint support. Planned routes have no release
date commitment; deferrals can be revisited for concrete consumer needs.

The schema's producing server release is unknown. The separately pinned test
baseline is not a claim that the schema came from v2.2.5. Detailed observed and
source-reviewed behavior remains in [the compatibility notes](tools/live/README.md).

## Milestone 2 acceptance audit

The criteria below come from [DESIGN.md](DESIGN.md#implementation-milestones).

| Criterion | Present evidence | Remaining gap |
| --- | --- | --- |
| Session listing and history | Runnable examples, protocol tests, imported transcripts and active streaming/waiting-tool snapshots on the pinned server. | Concurrent timeline changes during pagination remain unverified. |
| Incremental output | Typed SSE, bounded transient previews, gated live text deltas, and snapshot recovery after a missed fragment. | An unfinished block is absent from persisted text until it ends; reconnect cannot reconstruct its missed deltas. |
| Permission/form replies | Typed lifecycle APIs, manual fixtures, and actual read/question tool resumption after SDK replies. Cancellation with feedback also resumes execution. | Other waiting-tool reply modes, dismissal without feedback, races and restart lifetime remain unverified. |
| Execution outcomes | Bounded automation tests plus active success/failure/interruption, provider connection cancellation and wait settlement on the pinned server. | Tool interruption, steering/successor runs, remote providers and concurrent clients remain unverified. |
| Recovery after disconnection | Shared background observer passes live transport-cut, missed text/form events, delayed old history and explicit reconnect/refresh scenarios. Deterministic tests cover queue lag and cancellation. | General concurrent ordering, server overflow, restart recovery and the CLI timer/backoff loop remain unverified; stock v2.2.5 CLI retains no event payloads. |
| Library use outside a UI | Bounded automation creates/selects a session, submits one prompt, consumes progress and inspects outcomes/pending work. The active fixture exercises the SDK primitives. | The example's complete fresh-session policy remains mock-tested. |

## Prioritized completion plan

### 1. Demonstrate application-owned event state and recovery

Completed as [`examples/background_session.rs`](examples/background_session.rs),
with example-owned state and [behavioral tests](tests/background_session.rs).
It uses the existing API and caller-owned Tokio runtime, without a GUI/TUI
dependency or public state-store abstraction. Its bounded queue reports failure
out of band; snapshots and transient text previews remain separate.

Implemented deterministic checks, supplemented by the controlled live cases below:

- Read an initial session/history/pending-request snapshot and consume relevant
  events without describing the combination as an atomic snapshot.
- Show incremental updates and handling of unknown events. Bound buffering and
  make lag, stream errors and EOF visible to the application.
- On disconnection, explicitly reconnect and refresh snapshots; demonstrate
  convergence after a missed change with deterministic fixtures. Do not assume
  that SSE IDs or a log watermark establish a complete recovery boundary.
- Document task ownership, cancellation and shutdown, including incomplete local
  state while resynchronizing. Do not automatically retry mutations.

Durable log replay can supplement this path only when payload retention is
available. Snapshot refresh is necessary for the stock CLI baseline. Verifying
natural retention across restart/forks remains open; it need not block a release
whose documented recovery path relies on refreshed snapshots.

Completed the [controlled live recovery cases](tests/support/live_recovery.rs):
the test relay cuts three real SSE transports while OpenCode keeps running. The
shared observer reports stale state, reconnects only when explicitly instructed,
and refreshes missed text/form changes. A captured old history response is released
after execution settles; queued events invalidate it and a subsequent refresh
converges without duplicating text. Relay traces contain only GETs and no replay
cursor. Independent witness events synchronize tests but never populate recovered
state. These are controlled quiescent scenarios, not atomicity or general recovery
guarantees. Server overflow/restart remain separately tracked.

The unfiltered feed uncovered a generated instruction-delta decoder bug. The fix
changes `EventSessionInstructionsUpdatedDataDeltaValue` from a flattened struct
to a scalar enum matching the hash-or-`"removed"` schema. Include this source-level
type correction in initial release notes and the public API review.

### 2. Demonstrate bounded automation and active execution

Completed the bounded example as [`examples/automation.rs`](examples/automation.rs)
with [deterministic outcome tests](tests/automation.rs). It creates/selects a fresh
session, identifies its prompt before admission, observes progress while waiting,
and inspects bounded history/pending work. Success, failure, interruption, pending
input, requests requiring an answer and inconclusive evidence are distinct.
It retains all sessions and does not equate admission/idleness with success.
An explicit `--execute` invocation is required to run it against a server.

Completed the separate [`tools/live/active.py`](tools/live/active.py) fixture:
eight active scenarios pass on v2.2.5/macOS arm64 using twelve scripted localhost
provider requests, disposable state and no real credentials/paid calls. It verifies
text deltas versus snapshots, active interruption and provider cancellation,
success/failure/interrupted outcomes, permission-approved reading, and question
answer/cancellation-feedback resumption. Each checks settlement and empty pending
work; all eight sessions are deleted through the SDK. Ordinary tests compile and
skip this fixture; the existing smoke runner still makes no model calls.

The conversation workflow now has controlled live evidence. Keep unsupported or
untested outcome paths explicit in release notes; package preparation and hosted
validation still gate release readiness.

### 3. Prepare the package and release notes

Recommended next goal: prepare the package metadata, archive contents, licensing
attribution and initial release notes, then validate the packaged crate without
publishing. Confirm the license holder/year with the owner if not established.

The audit ran `cargo package --list --allow-dirty --locked --offline`. Listing
succeeds, but Cargo warns about missing package metadata. This checks the file
list, not compilation of a packaged archive or publication readiness.

- Add a package description and repository URL; review documentation links,
  version and README rendering. Cargo currently discovers the root README.
- Replace the project copyright placeholder in `LICENSE-MIT.txt` with the
  owner's correct holder/year. The Apache license's standard appendix template
  is not the same unresolved project attribution.
- Review the package file list. It currently includes the schema, tests,
  workflows and coverage/live tooling; decide which are useful in the published
  archive. The private model-generator workspace package is not included.
- Write initial release notes with supported APIs, runtime/MSRV/features, tested
  server/platform, experimental behavior and known limitations. Check whether
  any public API ergonomics should change before the first release.
- Build and test the intended package archive in isolation, then perform a Cargo
  publish dry run. Neither has been performed by this audit.

### 4. Collect release-candidate validation

After the implementation work, run the repository checks on the intended release
revision: workspace and HTTP-only tests, Rust 1.88 library builds, formatting,
strict Clippy, warning-free docs, examples, all nine generation checks, coverage
inventory/tests and live-runner tests. The workflows already encode most checks;
the coverage checker is now included in the quality job.

Confirm hosted CI and run the manual pinned smoke workflow on Linux x86-64 and
macOS arm64. Linux and hosted execution are still unverified; workflow files alone
are not evidence of passing runs. Repeat the package check after final changes
and record revision/platform/results with the release. Publishing remains a
separate action.

## Limits to retain in release documentation

- Basic authentication is exercised; pairing/cookies and other server releases
  are not. Transport injection can change redirect/retry/TLS policies.
- Live events are not replayable history. Seeded log fixtures test the server
  encoder/replay path, not natural payload durability or restart recovery.
- Slow-consumer overflow and snapshot/event races need additional validation.
  Local cancellation does not undo server-side work; mutations can have ambiguous
  outcomes and are not automatically retried.
- Active local-provider tests do not validate remote-provider behavior, arbitrary
  tools or model reasoning. Active fork filtering, copies beyond the 500-row
  boundary and form expiry/restart behavior remain explicit compatibility gaps.

Continue tracking concrete tasks in [TODO.md](TODO.md); keep operation status and
evidence in [openapi/coverage.json](openapi/coverage.json) synchronized as work lands.
