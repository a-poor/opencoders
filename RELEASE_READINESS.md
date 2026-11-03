# Milestone 2 release readiness

Audit and example follow-ups: **2026-11-02**, against the checked-in schema and the recorded OpenCode
**v2.2.5 / macOS arm64** smoke run. This audit does not rerun that server test or
establish Linux compatibility. It proposes the remaining work for the first
usable release; no release or publication has been performed.

The core conversation resources are implemented, but **milestone 2 is not yet
complete**. The remaining work is chiefly live recovery and active-execution
validation, and release preparation. PTY support is milestone 3 and
does not gate the first conversation-library release. No UI is being built.

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
| Session listing and history | Runnable examples, protocol tests and imported settled transcripts on the pinned server. | Concurrent/active snapshots are not validated. |
| Incremental output | Typed SSE plus the background observer's bounded transient previews, with deterministic HTTP/SSE tests. | No active model-output run. Previews are not reconstructed transcripts. |
| Permission/form replies | Typed lifecycle APIs, six form field kinds, reply effects and manual live fixtures. | No real waiting-tool resumption after a reply. |
| Execution outcomes | Admission/inbox/control APIs and a bounded automation example with deterministic outcome-correlation tests. | Live prompts are parked with `resume(false)`; interrupts/waits exercise idle sessions. Active execution remains unverified. |
| Recovery after disconnection | Background observer and deterministic missed-change, overlap, lag, cancellation and reconnect tests; explicit log cursors remain available separately. | Live reconnect ordering/overflow remains unverified; stock v2.2.5 CLI retains no event payloads. |
| Library use outside a UI | Bounded automation creates/selects a session, submits one prompt, consumes progress and inspects outcomes/pending work. | Its fresh-session policy is mock-tested; live model/tool behavior remains a separate validation goal. |

## Prioritized completion plan

### 1. Demonstrate application-owned event state and recovery

Completed as [`examples/background_session.rs`](examples/background_session.rs),
with example-owned state and [behavioral tests](tests/background_session.rs).
It uses the existing API and caller-owned Tokio runtime, without a GUI/TUI
dependency or public state-store abstraction. Its bounded queue reports failure
out of band; snapshots and transient text previews remain separate.

Implemented checks (deterministic fixtures, not a new live-server claim):

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

### 2. Demonstrate bounded automation and active execution

Completed the bounded example as [`examples/automation.rs`](examples/automation.rs)
with [deterministic outcome tests](tests/automation.rs). It creates/selects a fresh
session, identifies its prompt before admission, observes progress while waiting,
and inspects bounded history/pending work. Success, failure, interruption, pending
input, requests requiring an answer and inconclusive evidence are distinct.
It retains all sessions and does not equate admission/idleness with success.
An explicit `--execute` invocation is required to run it against a server.

Recommended next goal: investigate and build an isolated deterministic
provider/tool fixture for active compatibility testing against the pinned server.
Keep real credentials and paid calls outside that goal unless separately requested.

Extend opt-in compatibility testing to actual execution, including incremental
output, active interruption/settlement and permission/form tool resumption.
First investigate a deterministic local provider/tool fixture supported by the
pinned server. If real credentials are necessary, use a separate explicitly
invoked test with documented cost and workspace effects. Ordinary tests and the
current smoke runner must remain credential-free and avoid model calls.

Record unsupported or untested outcome paths individually. A package may be
described as a limited preview before these checks pass, but that would not
satisfy the full milestone 2 acceptance criteria above.

### 3. Prepare the package and release notes

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
- Settled transcript imports do not verify active history/fork filtering, model
  execution or tool resumption. Forks beyond the 500-row copy boundary and form
  expiry/restart behavior remain explicit compatibility gaps.

Continue tracking concrete tasks in [TODO.md](TODO.md); keep operation status and
evidence in [openapi/coverage.json](openapi/coverage.json) synchronized as work lands.
