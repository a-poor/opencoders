# Milestone 2 release readiness

Updated **2026-11-03**, against the checked-in schema and successful hosted
OpenCode **v2.2.5 / macOS arm64 and Linux x86-64** smoke and active
localhost-provider runs. Remote-provider behavior remains unverified.
This tracks remaining work for the first usable release; no release or
publication has been performed.

The core conversation resources and hosted functional validation are complete.
The candidate version is **0.1.0**, matching Cargo and the README dependency;
release notes are dated **2026-11-03**. This is the planned release date, not a
publication record. The clean committed candidate recorded below passes package
validation and hosted CI. Subsequent badge, documentation and maintenance-workflow
edits still need final CI and package checks before publication.
PTY support is
milestone 3 and does not gate the first conversation-library release. No UI is
being built.

## Coverage and evidence

Current hosted policy is Linux-only for ordinary CI and manual live tests;
macOS testing continues locally. Earlier macOS runs below remain valid historical
evidence. The separate weekly schema check reports upstream drift for review and
does not update the pinned contract or gate ordinary CI.

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
eight active scenarios pass on v2.2.5/macOS arm64 and Linux x86-64 using twelve
scripted localhost provider requests, disposable state and no real credentials/paid calls. It verifies
text deltas versus snapshots, active interruption and provider cancellation,
success/failure/interrupted outcomes, permission-approved reading, and question
answer/cancellation-feedback resumption. Each checks settlement and empty pending
work; all eight sessions are deleted through the SDK. Ordinary tests compile and
skip this fixture; the existing smoke runner still makes no model calls.

The conversation workflow now has controlled live evidence. Keep unsupported or
untested outcome paths explicit in release notes. Clean candidate validation is
recorded below; publication remains pending.

### 3. Prepare the package and release notes

The manifest now supplies a description, verified repository URL, explicit README,
keywords/categories and a file include list. The existing 0.1.0 version, Rust 1.88
minimum and feature/dependency declarations are unchanged. No documentation URL
claims an existing hosted build; Cargo's normal docs.rs discovery can apply after
publication. The owner confirmed **2026 Austin Poor** for the MIT notice; the
Apache appendix now has the same attribution. Both full license texts are packaged.

The archive includes source, all sixteen examples/support modules, tests/fixtures,
README, the [usage guides](docs/README.md), [draft release notes](CHANGELOG.md),
licenses and Cargo's generated metadata. Schema, generation, coverage/live tools,
workflows and planning documents remain in the repository; links to them use
repository URLs. The README uses the planned crates.io `0.1.0` dependency, as
requested for release preparation; this is not evidence of publication. The
changelog and release records retain the actual unreleased status.

Public usage review covered client/runtime ownership, resource accessors, typed
IDs and request builders, omitted/null handling, pagination, stream policy and
error inspection. Existing examples retain explicit application decisions. No
additional API changes were identified for this goal; the instruction-delta enum
correction is included in the release notes for early checkout consumers.

The [package procedure](RELEASING.md) builds and tests an extracted archive outside
the checkout, checks both feature configurations and the MSRV, and uses only a
publish dry run. Local results on **2026-11-03, macOS arm64, Rust 1.99.1**:

| Check | Result |
| --- | --- |
| Package listing/build | Pass with `--allow-dirty --locked --offline`; no missing-metadata warnings. |
| Archive inspection | 108 files, 1,421,235 uncompressed bytes / 218,386 compressed bytes. Source matches the package-preparation checkout; no generation tools, workspace/path dependencies or build script. |
| Extracted default and HTTP-only tests | Both pass: 217 library/integration tests and 14 doctests; the two live tests remain ignored. Fresh directory outside the checkout. |
| Rust 1.88 library and all sixteen examples | Both feature configurations pass with locked, cached dependencies. |
| Extracted API documentation | Pass with warnings denied. |
| Publish dry run | Pass against crates.io with `--allow-dirty --locked --dry-run`; Cargo confirms upload was aborted. Registry access is required, so the initial offline attempt could not run. |
| Repository consistency | Formatting, diff whitespace, coverage report, and documentation path checks pass. No SDK behavior or generated models changed. |

The table above records the earlier package-preparation snapshot. This is
working-tree evidence based on commit `899ec51`, **not a clean release
artifact**. The source/documentation snapshot was subsequently committed as
`e0bd6de`; the archive's VCS metadata records the earlier dirty state. The dry run
produced the same tested archive, SHA-256
`6219b427195d24ed0820a27e0326b85a890dccc888ab83cf29109f3e233449ae`.
Later README/changelog edits change the archive. Repeat validation after final
release edits and record the clean revision.

The subsequent documentation split was checked locally on 2026-11-03: packaging
and extracted-library verification pass with **116 files**, including all eight
`docs/` files. All fourteen README/guide Rust snippets compile on Rust 1.88; the
crates.io dependency in the quick start was patched to the local 0.1.0 candidate
for that check. Repository links/anchors and archive-relative documentation links
resolve. These documentation checks do not replace the final clean-revision test
and publish-dry-run procedure above.

After hosted validation, the updated working-tree candidate based on `75f0f2a`
was checked on **2026-11-03, macOS arm64, Rust 1.99.1**:

- The 116-file archive builds and contains the current source, guides, fixtures
  and licenses: 1,424,099 uncompressed bytes / 219,416 compressed bytes. Its
  normalized manifest has no workspace/path dependencies or build script.
- Extracted tests in a fresh directory pass in both default and HTTP-only modes:
  217 library/integration tests plus 14 doctests each, with both live tests ignored.
  Rust 1.88 checks pass for the library and all sixteen examples in both modes;
  extracted API documentation builds with warnings denied.
- The crates.io publish dry run passes and explicitly aborts the upload. Its
  archive matches the tested package, SHA-256
  `d611f39a2e2b309c6c1b3652ddae62b107104dca41e6d50d02a9c71b0190bf8c`.

This archive includes uncommitted compatibility-documentation updates and records
`dirty: true`; it is not a clean release artifact. Formatting, workflow lint,
the coverage report and its eight tests, and 266 repository links/anchors pass.
The clean candidate results below supersede this working-tree package for release
readiness.

### 4. Collect release-candidate validation

Clean candidate **`e86c6fab8a3cf3a2fd43fcdb667f82aa44dcaea1`** finalizes 0.1.0
and the planned 2026-11-03 changelog date. Cargo, the lockfile and the README agree
on the version. Validation on **2026-11-03, macOS arm64**:

| Check | Result |
| --- | --- |
| [CI 41686406437](https://github.com/a-poor/opencoders/actions/runs/41686406437) | All seven jobs pass for the candidate: Linux/macOS default and HTTP-only tests, quality, Rust 1.88 and generation. |
| Clean archive | `cargo package --locked --offline` passes without `--allow-dirty`. VCS metadata records `e86c6fa` with no dirty flag; 116 files, 1,423,949 uncompressed bytes / 219,350 compressed bytes. |
| Extracted tests | Fresh directory outside the checkout; default and HTTP-only each pass 217 library/integration tests and 14 doctests, with the two live tests ignored. |
| Examples/MSRV/docs | All sixteen examples pass both feature modes on Rust 1.99.1; library/examples pass both on Rust 1.88. API docs build with warnings denied. |
| Clean publish dry run | Passes without `--allow-dirty`; Cargo aborts upload. The resulting archive matches the tested package exactly. |
| Repository consistency | Formatting, coverage report/eight tests, version/date consistency and 266 documentation links/anchors pass. |

Tested clean archive SHA-256:
`15ee6c6a802f7f46ce5a60d3c2d9a4ceefdb6d06d4b523debb446224580061ad`.
The normalized manifest has no workspace/path dependencies or build script.
The archive contains the intended sources, guides, examples, fixtures and licenses.

The candidate changes documentation and coverage-report wording only. Its SDK,
test fixtures, live runners, workflows and server baseline are unchanged from the
successful hosted compatibility revision below, so those live results still apply.

Hosted validation passed on **2026-11-03** for pushed commit
`75f0f2a9e67b06c0e4b80dd7134cac03b060d01e`:

| Workflow | Result |
| --- | --- |
| [CI 41679740026](https://github.com/a-poor/opencoders/actions/runs/41679740026) | All seven jobs pass: default and HTTP-only tests on Linux/macOS, quality checks, Rust 1.88 library checks and all nine generation checks. |
| [Pinned server compatibility 41680124254](https://github.com/a-poor/opencoders/actions/runs/41680124254) | All four jobs pass with `active_execution=true`: smoke and eight active scenarios on both Linux x86-64 and macOS arm64, using OpenCode v2.2.5 and Rust 1.99.1. |

The documentation revision `a2a389c` also passed
[ordinary CI](https://github.com/a-poor/opencoders/actions/runs/41676975364) and
[all four compatibility jobs](https://github.com/a-poor/opencoders/actions/runs/41677452946)
before the action update. No SDK fixes were required. Commit `75f0f2a` updates
both workflows to the reviewed, commit-pinned checkout v7.0.1 using Node.js 24.
The final CI and compatibility logs contain no action-runtime deprecation warnings.
Detailed platform/job evidence is in [the compatibility record](tools/live/README.md#hosted-validation).
Ordinary CI skips both live tests; only the separate compatibility runs establish
server behavior. These results do not cover subsequent release documentation edits.

Publishing and tagging are the remaining separate actions. Repository-only audit
commits do not change the tested package source, but repackaging records their new
Git revision and changes the archive checksum. At handoff, verify the clean final
archive against the tested contents, record its actual checksum/revision, and
confirm CI on that revision. Repeat affected checks if packaged content changes;
rerun live compatibility if SDK, fixture, workflow or baseline changes warrant it.

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
