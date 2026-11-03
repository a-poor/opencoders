# Milestone 2 release readiness

Audit: **2026-11-02**, against the checked-in schema and the recorded OpenCode
**v2.2.5 / macOS arm64** smoke run. This audit does not rerun that server test or
establish Linux compatibility. It proposes the remaining work for the first
usable release; no release or publication has been performed.

The core conversation resources are implemented, but **milestone 2 is not yet
complete**. The remaining work is chiefly integrated examples, recovery and
execution validation, and release preparation. PTY support is milestone 3 and
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
| Incremental output | Typed live SSE, parser/failure/deadline tests and manual mutation events. | No integrated background consumer or active model-output run. |
| Permission/form replies | Typed lifecycle APIs, six form field kinds, reply effects and manual live fixtures. | No real waiting-tool resumption after a reply. |
| Execution outcomes | Prompt admission, pending inbox control, interruption and bounded waits. | Live prompts are parked with `resume(false)`; interrupts/waits exercise idle sessions. Wait returning does not prove prompt success. |
| Recovery after disconnection | Explicit log cursors, finite/follow reads, snapshot-refresh example and mock failure handling. | No integrated disconnect/reconnect reconciliation demonstration; stock v2.2.5 CLI retains no event payloads. |
| Library use outside a UI | Individual examples share the same client and runtime. | A complete bounded automation example remains to be written. |

## Prioritized completion plan

### 1. Demonstrate application-owned event state and recovery

Next goal: add a small **background session observer example**, with behavioral
tests for its reconciliation logic. Use the existing API and caller-owned Tokio
runtime. Keep state in the example, without a GUI/TUI dependency or a public
state-store abstraction.

Completion checks:

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

Add an automation example that creates/selects a session, admits input, observes
progress, waits with a deadline and inspects resulting history/pending work.
Make success, failure, interruption and unresolved input distinguishable. Never
equate admission or process-local idleness with a successful response.

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
