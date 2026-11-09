# Pinned server compatibility

The baseline is [OpenCode v2.2.5](https://github.com/anomalyco/opencode/releases/tag/v2.2.5),
commit `9bbb8ce18c80f099e0e448775f59d965ead40a21`.
[`server.json`](server.json) pins the official macOS arm64 and Linux x86-64
archives and their GitHub release SHA-256 digests. This is a test dependency;
the library does not download, launch, or manage OpenCode.

## Running

Requires Python 3.10+, Cargo, network access for the first download, and permission
to bind a localhost socket. Supported runner platforms: macOS arm64 and Linux
x86-64 (glibc). From the repository root:

```sh
cargo test --locked --test live --no-run
python3 tools/live/run.py
# Include optional ordinary PTY WebSocket I/O:
python3 tools/live/run.py --websocket
```

The smoke fixture also creates a local Git repository and requires `git` on PATH.
`--download-only` fetches and verifies the archive without starting a server.
Downloads are cached under `target/live/`; every run verifies the archive before
extracting a fresh executable. Updating a baseline requires reviewing the release,
commit, asset hashes, and smoke-test results together.

The runner creates a temporary home override, XDG directories, SQLite database,
and two workspaces. It supplies a fresh password, starts `serve --stdio` on
`127.0.0.1` with port `0`, and passes the resulting origin to the ignored Rust test.
It does not inherit provider credentials or OpenCode configuration. Model-catalog
fetches, project/Claude configuration, file watchers, and FFF indexing are disabled.
The server is stopped and temporary state removed on success, test failure, or
normal interruption. The verified archive cache remains. This isolates application
state; it is not an OS sandbox or a guarantee against upstream network activity.

The test checks the server PID, version, temporary path, and initially empty
session list before writing fixtures. It creates eight initially empty sessions
through the SDK, imports two additional transcripts with seven and three fixed
messages through raw HTTP solely for fixture setup, and creates six forks through
the SDK. The log test also seeds captured rename events into the runner-owned
SQLite database (details below). All covered operations use the SDK, and all
sixteen sessions are deleted before completion. Prompt fixtures explicitly use `resume(false)`
and inbox updates only queue work, so the test does not wake model execution or connect
to an existing server. Do not manually point its environment
variables at your normal server. `cargo test` compiles but skips this test.

## Active execution

The separate opt-in runner exercises the real pinned server's execution loop
with a **scripted localhost provider**, without real provider credentials or paid
model calls:

```sh
cargo test --locked --test live_active --no-run
python3 tools/live/active.py
```

Verified **2026-11-03, macOS arm64 and Linux x86-64, OpenCode 2.2.5,
Rust 1.99.1** in the [hosted runs](#hosted-validation), in addition to the local
macOS run.
All eight scenarios in [`tests/live_active.rs`](../../tests/live_active.rs) pass,
including the [recovery cases](../../tests/support/live_recovery.rs).
Ordinary `cargo test` compiles and skips this test. The manual compatibility
workflow has a separate `active_execution` checkbox, off by default; enable it
to include these checks.

[`active.py`](active.py) reuses the smoke runner's checksum-pinned archive,
environment allowlist, ephemeral password, loopback port allocation and process
cleanup. It creates a separate disposable Git workspace/database with snapshots
enabled for workflow checks. A local HTTP server
speaks the pinned bundled OpenAI-compatible chat streaming protocol. The runner
configures only this provider, denies other providers by policy, disables warming,
automatic compaction, updates and sharing, and removes the title agent. Named
sessions avoid automatic titling. No provider keys, user configuration, plugins
or proxies are inherited by OpenCode.

Tool permissions deny everything except asking permission to read `fixture.txt`
and using `question`. The fixture emits only those two built-in tool calls. The
file contains fixed test text; questions are answered/cancelled through the SDK.
No shell commands or external URLs are requested. This isolates application state
and routes the tested provider locally; it is **not an OS/network sandbox**.

The provider uses per-run authentication, bounded bodies/request counts and
stream deadlines. It rejects unexpected models, duplicate calls and incorrect
tool results. Output is gated by the test, so interruption and waiting checks
cannot accidentally exercise an already completed request. Both servers stop and
temporary data is removed on completion, failure or normal interruption.

| Scenario | Observed and asserted through the SDK |
| --- | --- |
| Output | `resume(true)` delivers the identified prompt; two gated text deltas decode; an incomplete assistant snapshot has empty text until the block ends; final text and successful session/history outcome agree. |
| Interruption | A held provider stream returns `interrupted: true` with `resume(false)`; the provider observes connection cancellation; the typed interruption reason is `user`; wait settles, history retains partial text, and a subsequent idle interrupt returns false. |
| Failure | One provider HTTP 400 produces a typed execution-failed event and failed session/idle history with an error; wait still returns successfully. No provider retry is observed for this case. |
| Permission | A real `read` call waits on a pending request with matching tool ID/resource. Session/location lists expose it. `once` unblocks the read; the provider verifies the file content in the next request; the tool and execution complete. |
| Question | A real `question` call creates a pending form tied to its tool ID. Replying with `q0` produces the expected tool result at the provider, completed tool history and successful execution. |
| Cancellation feedback | Cancelling a waiting question with a message produces cancelled form state/events, failed tool history/events and model-facing feedback; the next provider step succeeds. Tool failure alone does not imply execution failure. |
| Streaming recovery | Cutting the observer's SSE transport leaves the provider running. A reconnect during the unfinished block cannot recover a missed delta from history. A history response captured before settlement is held while execution completes; queued events invalidate it, and another refresh converges to the complete text without duplication. |
| Form recovery | A question is created while disconnected, then discovered as pending through refreshed snapshots. Its answer and execution completion occur during another disconnect; a further reconnect clears the stale pending form and recovers successful terminal history. |

Wait deadlines expire while the provider or a tool is held; those local timeouts
do not stop execution. After settlement, all eight cases have matching prompt and
terminal history, no pending inbox/permission/form entries, and the expected
session outcome. The runner asserts exactly twelve provider requests and removes
all eight sessions through the SDK.

The text-snapshot distinction agrees with the pinned
[message updater](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/message-updater.ts):
`session.text.started` projects an empty text part, `session.text.ended` stores
the complete block, and transient deltas do not update stored history. Keep live
previews separate from snapshots; repeatedly fetching history cannot reconstruct
an unfinished block. The
[question tool](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/tool/plugin/question.ts)
distinguishes dismissal without feedback from cancellation with feedback; only
the feedback path is exercised here.

These tests validate SDK/server/protocol integration, not actual model reasoning,
remote-provider compatibility, billing or provider authentication. Active tool
interruption, permission rejection/always with waiting tools, question dismissal
without feedback, steering/successor runs, retries, shutdown, concurrent clients,
server overflow, restart recovery and other platforms remain separate validation work.
The complete automation policy remains mock-tested. The background observer's
shared connection/state module is exercised live as described below.

### Controlled live recovery

[`tests/support/recovery_proxy.rs`](../../tests/support/recovery_proxy.rs) is a
test-only localhost GET relay. It forwards the pinned server's response bodies
and deliberately closes selected SSE responses without a terminating HTTP chunk.
The SDK reports a stream error; the example-owned reader marks its old snapshot
stale, clears previews and closes its reader. OpenCode and its provider connection
remain running. Two deterministic relay tests independently check forwarding,
transport cancellation, captured-history gates and rejection of mutations.

Recovery uses the same [`Connection` and `State`](../../examples/support/session_observer.rs)
as the background example. The test explicitly reconnects, subscribes before
refreshing, and fetches the session, one recent 50-message page and pending
inbox/permission/form snapshots. Every relay request is a GET; the test asserts
exactly two subscriptions for text recovery and three for form recovery, with no
`Last-Event-ID`. An independent direct subscription is only a test synchronization
barrier: none of its missed event payloads enter the recovering observer's state.

When disconnected midway through a text block, an empty stored text part and a
new subscription cannot reconstruct the missing fragment. Once the block ends,
snapshot refresh recovers the complete text. A delayed old history response can
arrive after execution settles; queued events are treated as invalidations, never
applied as patches. After the controlled execution stops changing, bounded repeated
refreshes converge. This does not establish an atomic snapshot boundary, general
ordering across resources, lossless transient recovery or convergence under
continuous writes. Server-originated overflow, restart recovery, remote-network
failures and the example CLI's timer/backoff loop remain untested live.

The unfiltered observer also exposed and now verifies a corrected generated
`session.instructions.updated` payload: delta values are scalar 64-character
lowercase hexadecimal hashes or `"removed"`, not objects. The generator now emits
a checked scalar enum after guarding the disjoint schema alternatives; malformed
hashes, nulls and objects still fail decoding. This corrects the public generated
`EventSessionInstructionsUpdatedDataDeltaValue` from a struct with `subtype_*`
fields to an enum (`InstructionHash` or `String(Removed)`). No endpoint or
orchestration API was added.

## Verified smoke behavior

The full smoke test passes on **macOS arm64 and Linux x86-64, OpenCode 2.2.5,
Rust 1.99.1**, in the [hosted runs](#hosted-validation) on **2026-11-03**.
The earlier local macOS run passed on 2026-11-02.

| Operation | Observed and asserted |
| --- | --- |
| `server.info` | Typed version/PID/paths; Basic authentication with username `opencode`; unauthenticated calls return typed HTTP 401. |
| `location.get` | Nested query encoding, spaces/Unicode, per-call overrides, `x-opencode-directory` fallback, and query precedence over that header. |
| `agent.list` | Nonempty typed agent list with resolved location metadata. |
| `agent.get` | Listed agent round-trip, query/header location selection, and typed missing-agent HTTP 404. |
| `model.list` | Typed catalog/location envelope, default chat filter, repeated type parameters, deduplication/group order, unknown-type empty lists, and directory overrides. |
| `model.default` | Typed default-model envelope, resolved location, header fallback, and query precedence. Null default data is covered by mocks. |
| `session.create` | Typed JSON requests/responses, titles, metadata, empty permissions, parent location precedence, missing-parent HTTP 404, `{}` defaults, and all eight fields explicitly null. |
| `session.fork` | Default/null full history; exclusive and before-first boundaries; empty/missing/wrong-session errors; current location/configuration inheritance; root/provenance distinction, new message IDs, attachment readability, reset usage/archive state, parked inbox exclusion, compaction keep-ID remapping, fork events, and independent source/fork deletion. |
| `session.get` | Created-session round trips, retrieval across directories independently of the SDK default location, and typed missing-session HTTP 404. |
| `session.list` | Real session decoding, server-wide scope, directory/search/root/child/archive filters, next/previous traversal, empty terminal page, invalid-cursor HTTP 400, and zero-limit rejection. |
| `session.update` | Empty HTTP 204; title, metadata/permission replacement and clearing; archive/restore; omitted/null no-ops; cross-directory IDs; typed missing-session HTTP 404. |
| `session.remove` | Empty HTTP 204; deletion of a parent, child, and grandchild while preserving unrelated sessions; repeated/missing deletion returns typed HTTP 404. |
| `session.switchAgent` | Empty HTTP 204; stored agent choice across directories, repeated-agent history entries, acceptance of unknown agents, and typed missing-session errors. |
| `session.switchModel` | Empty HTTP 204; stored catalog model/variant, omitted variant normalized to `default`, repeated-reference deduplication, history markers, unknown provider/model acceptance, and typed missing-session errors. |
| `session.message.list` | Empty/populated history, newest-first default, both ordering directions and next/previous traversal, type filters, empty boundary pages, invalid cursor and cursor/order conflicts, zero-limit rejection, and missing-session errors. |
| `session.message.get` | Every imported message round-trips, including user attachments and assistant reasoning/tool content; missing and wrong-session message errors are typed. |
| `event.subscribe` | Independent filtered SSE subscriptions, connected markers, typed renames across directories, inbox enqueue/cancel events, filter exclusion, and continued observation after dropping another subscription. |
| `experimental.session.log` | Stock CLI marker-only behavior; finite/default and follow modes; optional/future `after`, empty replay and watermark gaps; typed missing-session/bad-sequence errors. Seeded captured events verify exclusive replay, cross-aggregate isolation, follow, and resumption. |
| `session.prompt` | Durable admission with `resume(false)`, explicit/generated IDs, omitted/null delivery defaulting to steer, empty text, inline file preparation, agent attachments, metadata, duplicate-ID reconciliation, cross-session conflicts, and preparation/missing-session errors. |
| `session.inbox.list` | Pending user payloads, enqueue order independent of ID order, and no projected history before delivery. Other inbox kinds are covered by fixtures/mocks. |
| `session.inbox.update` | Empty HTTP 204 for steer-to-queue changes; order retained; same-mode, unavailable, and wrong-session items return typed HTTP 409; missing sessions return 404. |
| `session.inbox.cancel` | Empty HTTP 204, repeated/missing/wrong-session cancellation no-ops, pending item removal without creating history, and missing-session HTTP 404. |
| `session.interrupt` | HTTP 200 with `interrupted: false` on idle sessions; omitted/false/true resume on empty sessions, true leaves queued input parked, omitted/false leave held steering input pending; typed missing-session HTTP 404. |
| `experimental.session.wait` | HTTP 204 on idle sessions, including sessions with held queue/steer input; no input is delivered or started; typed missing-session HTTP 404. |
| `permission.request.list` | Resolved location/data envelope; ordered pending requests across sessions; per-call directory, header fallback, query precedence, and missing-location HTTP 404. |
| `session.permission.create` | Immediate allow/deny/ask results; only ask appears as pending; explicit/generated/null ID and null agent, tool source, metadata, resource order, save patterns, and missing-session errors. |
| `session.permission.list/get` | Session ownership and pending snapshots; missing/resolved/wrong-session request errors. |
| `session.permission.reply` | Once resolves one; reject resolves the session batch; always saves project rules when patterns exist and resolves matching pending requests in another session at the same location. Empty/absent patterns save nothing; configured denies win. Empty/null/nonempty feedback accepted; repeated/wrong-session replies fail. |
| `form.list` | Pending location snapshots, resolved metadata, explicit directory, header fallback and query precedence; missing-location errors. |
| `session.form.list/create/get` | All six field kinds, generated/null/explicit IDs, metadata, pending-only lists, session/location isolation, duplicate pending/settled IDs, wrong-session/missing IDs, and required field-definition validation. |
| `session.form.reply` | Scalar/list answers, required defaults not applied, finite/integer/closed-option/external acknowledgement validation, conditional inactive fields, hidden-field validation, invalid answers remaining pending, and answered-state retention. |
| `session.form.cancel` | Bodyless DELETE with omitted/empty/Unicode feedback; terminal state retains feedback; repeated cancellation/reply returns settled errors. |
| Form events | Typed `form.created`, `form.replied`, and `form.cancelled`; scalar conditions and answers decode; cancellation event omits feedback. |

Smoke form fixtures are manual and do not start model calls or open external URLs.
The [pinned service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/form.ts)
retains terminal forms in memory for ten minutes; the smoke run checks immediate
terminal retrieval, not timed expiry or survival across restart/location teardown.
Concurrent replies/cancellations remain unverified. The separate active fixture
checks waiting-question resumption. Snapshot discovery and live events do not
provide durable form history.

Fork semantics are reviewed against the pinned [session service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session.ts)
and [projector](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/projector.ts).
The service uses the latest source configuration/instructions while copying only
settled projected history before/through the selected message. Fork provenance
is separate from parent/child ownership; forks are independent roots.

The [import path](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/transfer.ts)
drops unfinished assistant/shell/compaction entries before storing fixtures, so
these checks do not verify fork filtering during real active execution. Instruction
inheritance, concurrent mutation, copies crossing the 500-row batch boundary,
and retained-log replay on forks remain unverified live. The test imports no
running work and makes no provider, shell, or compaction calls.

Two pagination details matter for consumers:

- v2.2.5 embeds filters and ordering in cursors. Valid conflicting filters/order
  supplied alongside a cursor are ignored; `limit` remains a separate page-size
  parameter. Clear the cursor when changing filters. The SDK still sends every
  explicitly supplied option and treats cursors as opaque.
- Nonempty pages return boundary cursors, including at the beginning/end and when
  the page is shorter than `limit`. Cursor presence does **not** prove that more
  rows exist. Following one can return an empty page without a next cursor.

The typed responses exercised here decode without changes to existing response
field adapters. Each resource adds reviewed generated models. This does
not establish compatibility for every optional field or operation.
The schema's empty security declarations omit authentication requirements;
v2.2.5 requires Basic credentials (or its pairing/session flow), not Bearer tokens.
The SDK now provides `ClientBuilder::basic_auth`; Bearer remains available for
proxies. Pairing/cookies are not covered. See the pinned upstream
[authorization middleware](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/middleware/authorization.ts)
and [session listing implementation](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session.ts).

For creation, a concrete parent selects its location even if the body names a
different directory. With no parent, omitted/null location uses the server's
working directory. v2.2.5 accepts explicit null for all eight create properties;
the SDK preserves these wire states rather than rewriting null into omission.
Create/get ignore the SDK discovery location default. See the pinned
[session handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/session.ts).
No retry/idempotency or execution semantics are inferred from these empty-session
tests. Explicit creation IDs are covered by request-shape checks. Selection is
covered as stored data, without executing the selected agent or model.

Model lists/defaults are current snapshots. Repeated `types` parameters are decoded
as a list; the server deduplicates types and concatenates their model groups in
requested order. The SDK preserves the response order and does not locally filter
or wait for catalog changes. Custom/unknown model types remain valid wire strings.
See the pinned [model handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/model.ts).

Selection accepts unknown agent/provider/model IDs, so success does not establish
availability or valid provider credentials. Reselecting the same agent adds another
history entry. Omitted model variants are stored as `"default"`; repeating the same
provider/model/variant adds no entry, while changing the variant does. The test
asserts these markers through message history and retrieves the final session.
See the pinned [selection implementation](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/session.ts).
Concurrent catalog changes and selections during execution remain unverified.

For updates, omission/null leave fields unchanged; `{}` is a no-op. Metadata and
permission lists replace existing contents rather than merging; `{}` metadata
and `[]` permissions clear those contents. Archive/restore and repeated archive
are verified. Empty title invokes the server's titling path: the test only checks
an empty session, where it leaves the title unchanged. The pinned
[title generator](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/title.ts)
returns before any model call when there is no user message; a populated session
can invoke a model and is not tested here. Updates run sequentially in the handler;
the SDK makes no atomicity guarantee. Deletion's recursive behavior is checked
on idle, empty sessions, not sessions with active execution or concurrent changes.
Nonempty permission replacement is verified as stored data, not enforcement.

Message history uses [`tests/fixtures/messages.json`](../../tests/fixtures/messages.json).
The pinned [import handler and transfer service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/transfer.ts)
store settled projected messages without scheduling execution. Assistant/model
and tool results in the fixture are synthetic data, not real calls. Import is not
part of the public SDK yet. History reads retain exact fixture payloads.

Message pagination differs from session-list pagination: a nonempty cursor plus
order is rejected, and the type filter must be repeated separately. The test
checks both directions, filtering before pagination, and dropping the filter on
a cursor request. See the pinned
[message handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/message.ts).
The active fixture checks streaming and waiting-tool snapshots. Concurrent timeline
changes during pagination and experimental media-reference negotiation remain unverified.

Prompt admission returns a durable pending user item, not an assistant response.
Tests explicitly use `resume(false)` on an idle session to avoid waking execution.
Queue delivery alone does not prevent execution; false also does not pause a loop
that is already running. The source defaults omitted/null delivery to steer and
omitted/null resume to waking; only the no-wake branch is exercised live.
See the pinned [session implementation](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/session.ts).

Repeating a pending prompt ID for the same session returns the original admission,
even when text or delivery changes. Cross-session reuse conflicts. This observation
does not establish retry behavior after delivery/cancellation or concurrent calls.
The SDK never retries these mutations automatically. File fixtures use inline data
URIs; invalid URI/missing-skill preparation returns 400 without an inbox item.

Inbox updates only change delivery. The pinned implementation requires a transition
from the opposite mode, so updating to the current mode returns a conflict.
Steering wakes execution; the live test only changes steer to queue. Cancellation
of an unavailable item succeeds without undoing delivered work. See the pinned
[inbox implementation](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/inbox.ts).
The active fixture verifies waking a fresh session and delivering its one prompt.
Delivery during execution, already delivered/cancelled ID reuse, synthetic/
compaction/move lifecycle, and experimental inbox media-reference negotiation remain
unverified. Runtime ordering is not inferred from the pending-list snapshot.

The live feed is global and live-only. The pinned [handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/event.ts)
sends an initial connected event and heartbeat comments; its [feed](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/event-feed.ts)
uses data-only SSE frames and a bounded subscriber queue. Live tests confirm
connected markers despite filters and independent streams across locations, then
observe no-wake inbox admission/cancellation. They do not exercise provider
execution, heartbeat timing, slow-consumer overflow, or reconnect reconciliation.
Fragmentation, heartbeats, EOF, transport failures, protocol failure frames, and
timeouts are deterministic mock/socket tests. The declared failure-frame contract
does not establish how every server disconnect or overflow will be signaled.

### Permission request scope and reply effects

The pinned [permission service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/permission.ts)
evaluates agent/session rules and saved project rules, creating an in-memory pending
request only for ask. The [HTTP handlers](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/permission.ts)
enforce session ownership for get/reply. Reject affects the whole session's pending
batch; always with save patterns re-evaluates pending requests at that location
after saving project allow rules. Configured denies retain precedence.

Smoke permission fixtures are manually created for idle disposable sessions with
explicit rules under `opencoders.live.*`. No tool/model waits on their replies.
The test observes permission asked/replied events, verifies once versus batch
rejection, preserves other sessions/locations, and exercises saved-rule effects
across two sessions at one location. Saved rules are confined to the runner's
temporary database, which is removed with the server. No user configuration changes.

Null create ID/agent and reply feedback are accepted by v2.2.5. Duplicate pending
ID rejection is source-reviewed, not exercised live. Plugin evaluation hooks,
parallel reply races/partial failures, actual tool resumption, and reload/restart
behavior are not validated by these fixtures. Pending lists must be refreshed;
they do not establish history, durability, or execution completion.

### Interruption and wait semantics

The pinned [execution service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/execution.ts)
and [coordinator](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/run-coordinator.ts)
acknowledge interruption before finalizers settle. False can mean idle, already
stopping, or the terminal settlement window. Resume true can wake eligible steering
or control work even when the interruption result is false; queued prompts stay
parked. The wait implementation observes process-local idleness, waits through
successors, and ignores execution failure/interruption as outcomes. It never starts
work and does not promise that the inbox is empty or a particular prompt succeeded.

Smoke checks exercise only idle sessions: empty inboxes with every resume query
choice, a held queued prompt with resume true, and a held steering prompt with
omitted/false resume. Wait returns HTTP 204 while those prompts remain pending and
message history remains empty. Fixtures are then cancelled through the SDK. No
resume-true call has steering/control work eligible to run, and no model is invoked.

The active fixture adds provider interruption, wait settlement and success/failure
outcomes. Concurrent wakeups and actual steering resumption remain unverified. Deterministic
protocol tests cover both response booleans, statuses/errors, total/error-body
deadlines, injected timeout policies, cancellation of one independent wait,
connection cleanup, empty keep-alive responses, and no automatic retry.

### Automation example evidence

The [automation example](../../examples/automation.rs) and its
[tests](../../tests/automation.rs) add a mock-tested fresh-session outcome policy;
they have not been run with a live provider or tools. Source review of the pinned
[message store](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/store.ts)
confirms message ordering uses stored sequence, not the lexical order of caller
IDs. The [message updater](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/message-updater.ts)
projects terminal executions into idle history markers (excluding shutdown
interruption), and instruction updates into system notices marked
`metadata.notice = "instructions"`. The classifier permits those notices but
requires matching prompt/terminal history and session outcome, an idle wait and
no pending work before declaring observed success. This is a conservative
application policy, not an atomic snapshot guarantee or per-prompt server receipt.
The separate active fixture checks SDK execution and waiting tools, but does not
run this example's complete policy. Shutdown and competing work remain unverified.

### Durable log retention and replay

The v2.2.5 CLI does **not** retain durable event payloads. The pinned
[CLI launch code](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/cli/src/server-process.ts)
does not pass `events.persist`; the [server route setup](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/routes.ts)
forwards this optional setting, and the [bus implementation](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/bus.ts)
defaults persistence to false. Live checks confirm that a normal session log emits
only `log.synced`, its sequence advances after a rename, and following receives
no payload for that unretained change. The SDK does not change server retention.

To verify decoding and replay against the actual server encoder, the test captures
real SDK-triggered rename events from the live feed. [`seed_log.py`](seed_log.py)
inserts those exact event IDs, sequences, timestamps, and payloads into the owned
runner database, using the pinned [event table format](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/event/sql.ts).
It verifies the runner ownership marker, confines writes to its temporary root,
requires existing committed sequences, and leaves projections/counters untouched.
This is fixture preparation, not an SDK feature or a claim of normal CLI retention.

Seeded checks assert ordered replay, exclusive `after`, marker-only reads at/beyond
the watermark, cross-session isolation, explicit resumption, and follow delivery.
For the last case, the test seeds a captured event and uses a subsequent ordinary
rename to wake the DB-backed tail. Subsequent events and gaps remain observable;
the sync marker can exceed the last retained/returned event. The tested encoder
omits SSE IDs for both event and marker payloads; the SDK preserves IDs when present
in protocol tests without deriving a cursor from them.

The pinned [session implementation](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session.ts)
filters the bus log to durable session kinds. The bus subscribes before capturing
the watermark, reads retained rows in pages, emits one marker, and tails on durable
notifications. Missing/unknown stored kinds can produce gaps. This does not prove
REST snapshot consistency, persistence across restart, fork-inherited replay,
retention guarantees, or provider-generated history. Those remain unverified;
applications must refresh authoritative snapshots and own their checkpoints.

The existing [`openapi/provenance.json`](../../openapi/provenance.json) describes
the original schema download, whose originating server release remains unknown.
Its `info.version: 0.0.1` is not this server's version. Testing v2.2.5 does not
retroactively identify that schema's source release. Concurrent session changes,
maximum page sizes, project/subpath filtering, explicit-null discovery queries, durable log retention across restarts/forks,
remote-provider behavior, and other platforms remain unverified. Active local
provider execution is covered separately above.

## CI

The ordinary [`ci.yml`](../../.github/workflows/ci.yml) checks formatting, Clippy,
docs/examples, default and HTTP-only tests on Linux, both library feature
sets on Rust 1.88, Python runner/schema-checker tests, and all nine generation checks.
Before reducing the hosted matrix, all seven jobs passed for commit
`75f0f2a9e67b06c0e4b80dd7134cac03b060d01e`
on 2026-11-03 in
[run 41679740026](https://github.com/a-poor/opencoders/actions/runs/41679740026).
These jobs skip the two live tests; server compatibility is established separately
by the manual workflow below.
Regeneration pins Rust/rustfmt 1.99.1 and uses the locked dependencies and local
schema; it never refreshes upstream input. The full-schema experiment remains a
separate optional check.

[`live.yml`](../../.github/workflows/live.yml) is manual (`workflow_dispatch`) and
runs the smoke runner on Linux x86-64. Its optional `active_execution` input adds
the active localhost fixture on Linux. No model secrets are needed.

Hosted macOS jobs are disabled to avoid runner cost and queue delays. The same
smoke/active commands remain supported locally on macOS arm64, where most
development testing takes place. Use them for releases and relevant dependency
changes; the earlier hosted results below remain historical evidence. Revisit
the hosted matrix if platform-specific functionality makes it worthwhile.

### Hosted validation

On **2026-11-03**, [run 41677452946](https://github.com/a-poor/opencoders/actions/runs/41677452946)
passed all four smoke/active jobs for documentation commit
`a2a389c0990988deff39a00a0dd62dde306bead9`, with `active_execution=true`.
The runners used Linux x86-64 (`ubuntu-latest`) and macOS arm64 (`macos-14`),
Rust 1.99.1, and the checksum-pinned OpenCode v2.2.5 archives. Each active job
passed all eight scenarios: output, interruption, failure, permission, question,
cancellation feedback, text recovery and form recovery. The smoke test uses no
model calls; the active test uses only the scripted localhost provider.

After updating the pinned checkout action, all four jobs passed again for
`75f0f2a9e67b06c0e4b80dd7134cac03b060d01e` in
[run 41680124254](https://github.com/a-poor/opencoders/actions/runs/41680124254),
also with `active_execution=true`:

| Platform | Smoke | Active execution and recovery |
| --- | --- | --- |
| Linux x86-64 (`ubuntu-latest`) | [Pass](https://github.com/a-poor/opencoders/actions/runs/41680124254/job/126071258156) | [Pass, eight scenarios](https://github.com/a-poor/opencoders/actions/runs/41680124254/job/126071258171) |
| macOS arm64 (`macos-14`) | [Pass](https://github.com/a-poor/opencoders/actions/runs/41680124254/job/126071257635) | [Pass, eight scenarios](https://github.com/a-poor/opencoders/actions/runs/41680124254/job/126071258361) |

These results establish the documented fixture behavior on those two platforms,
not remote-provider compatibility, server-overflow handling or restart recovery.

Both workflows use read-only repository permissions and a pinned checkout action.
Validate workflow edits with `actionlint`; runner checks use:

```sh
python3 -m unittest discover -s tools/live -p 'test_*.py'
```

## Workspace reads and ordinary PTYs (2026-11-09)

`python3 tools/live/run.py --websocket` passes locally on macOS arm64 against the
same v2.2.5 archive. The new fixture is in `tests/support/workspace.rs`; the
existing parent verifies the server PID, paths and empty disposable state before
calling it. It creates binary fixtures and a separate Git repository with isolated
Git configuration/hooks, and starts only `/bin/cat` and `/usr/bin/false`. It does
not submit prompts, call providers or launch a user shell. The Linux smoke job
also passed with `--websocket` at `88224a1`; see the hosted follow-up below.
Earlier hosted results remain evidence for their original scope.

Observed assertions cover filesystem list/find, special-character filenames,
binary bytes, range 206, ETag and Last-Modified 304, empty 416 and missing files;
Git branch/status/branches/base envelope/working and committed diffs and worktree
listing; PTY create/list/get, accepted rename/resize, location isolation, ticket
lifetime/reuse rejection, UTF-8 I/O, replay cursor, detach with process retained,
explicit cursor reconnect, removal and retained exit code/4404 close.

The first VCS read returned empty branch information before asynchronous provider
activation. The fixture polls a bounded five seconds for its known Git branch;
the SDK does not add implicit polling. Worktree listing reads saved project
inventory rather than running discovery. Actual resized viewport dimensions,
nonempty inferred bases, branch-mode diff, multi-worktree inventories and remote
providers/workspaces remain unverified live.

Protocol/source review uses the pinned
[filesystem handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/fs.ts),
[filesystem service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/filesystem.ts),
[VCS service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/vcs.ts),
[PTY handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/pty.ts),
[PTY service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/pty.ts),
[PTY protocol](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/pty/protocol.ts)
and [ticket service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/pty/ticket.ts).
The server requires `x-opencode-ticket: 1`; tickets last 60 seconds and are
single-use/location-scoped. Socket success is HTTP 101, despite the schema's
boolean/200 description. Text is output, NUL-prefixed binary JSON carries the
cursor after replay, and cursor offsets count UTF-16 code units. The replay buffer
is bounded and supplies no explicit gap marker. Disconnect does not terminate the
process. These guarantees apply only to ordinary PTYs, not persistent PTYs.

Deterministic tests cover invalid upgrade headers/control frames, bounded HTTP
errors and message/byte limits, authentication/prefix/query preservation, ping,
peer close metadata, independent handshake/idle deadlines, read cancellation and
no mutation retries. TLS/proxy deployments, ticket expiry timing, buffer eviction,
slow consumers, server restarts and real network failure remain unverified live.

## Hosted workspace/PTY verification and lifecycle follow-up

At `88224a1`, [Linux CI 42707538439](https://github.com/a-poor/opencoders/actions/runs/42707538439)
passed all seven jobs, including all four test feature combinations, Rust 1.88,
quality and eleven generation targets. Both jobs in
[pinned compatibility 42707625108](https://github.com/a-poor/opencoders/actions/runs/42707625108)
passed on Linux x86-64 on 2026-11-09: the smoke job includes workspace reads and
ordinary PTY WebSockets; the active job uses only the scripted localhost provider.
These runs supersede the earlier Linux-pending note for the initial 16 operations.

The subsequent four-operation lifecycle slice adds
`tests/support/workspace_lifecycle.rs`. It passes locally on macOS arm64 with
`python3 tools/live/run.py --websocket`: unknown-provider rejection, Git
initialization, repeated-init conflict, detached checkout contents, collision
suffixes, accepted clean removal, dirty removal refused with `forceRequired`,
explicit forced cleanup of a known fixture, externally created worktree discovery,
stale-row removal, canonical-root rejection and missing-project errors. All
repository/filesystem changes are under the runner-owned temporary root; fixtures
use isolated Git configuration and no project setup scripts or remote operations.
This new lifecycle fixture has not yet run on Linux.

The pinned [worktree service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/worktree.ts),
[Git strategy](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/worktree/git.ts),
[worktree handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/worktree.ts)
and [VCS handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/vcs.ts)
were reviewed. Creation loads canonical configuration, uses the selected strategy,
persists the checkout and then runs any project setup script; failures can follow
partial effects. `directory` is a parent path and the returned path is authoritative.
Refresh reconciles inventory and can tolerate per-strategy discovery failures.
Removal requires an explicit force decision and uses the saved strategy. No SDK
mutation retry, force escalation or rollback is added. Custom strategies/providers,
Mercurial, setup-script failures, concurrent operations and permission/network
filesystem failures remain source-reviewed or unverified rather than live claims.


## Stable session workflow evidence (2026-11-09)

`python3 tools/live/active.py` now also runs
[`live_workflows.rs`](../../tests/support/live_workflows.rs). Locally on macOS
arm64 with v2.2.5 it verifies configured command/skill discovery (including skill
content/autoinvoke), missing/failing command callbacks, immediate callback return
followed by gated execution, context/diff reads and invalid turn anchors, and
409 busy responses for all revert mutations. The disposable Git workspace and
its two files belong to the runner. A controlled harness write after the first
text delta and before stream completion produces real server snapshots/diffs;
it does not claim model-selected edit-tool coverage.

History-only staging retains messages and file contents. Default file staging
restores the original file and clear restores its post-execution content. Clearing
also wakes a parked prompt. Compaction commits a staged boundary, admits an
identified input, obtains a summary from the scripted localhost provider and
exposes a context starting at the completed compaction. Explicit commit then
removes its boundary inclusively; empty commit/clear behavior and cleanup are
checked. Provider scenarios reject unexpected/repeated calls, and provider-use
policy denies every provider except the local fixture. No real model is called.

Reviewed pinned sources: [command callbacks](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/command.ts),
[session operations](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/session.ts),
[reverts](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/revert.ts),
[inclusive projections](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/projector.ts),
[turn diffs](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/diff.ts),
[history windows](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/history.ts),
and [compaction](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/compaction.ts).

These nine new methods have local macOS evidence only; the hosted `88224a1` runs
predate them. Linux, arbitrary command/plugin side effects, native compaction,
remote providers, cross-location ranges, snapshot failures, concurrent writers
and large histories remain unverified. The SDK adds no mutation retries or UI
state. See [workflow usage](../../docs/workflows.md).


## Hosted lifecycle/workflow verification (2026-11-09)

Commit `3ff1edb75d3f80d071baab288745270a3747445f` passed all seven jobs in
[CI 42713702937](https://github.com/a-poor/opencoders/actions/runs/42713702937)
and both jobs in [pinned compatibility 42713795590](https://github.com/a-poor/opencoders/actions/runs/42713795590).
This extends Linux evidence to the four workspace lifecycle/VCS-init methods and
nine stable session workflow methods described above, including snapshot restore,
parked-input wakeup, compaction and inclusive revert commit. Earlier local-only
notes describe the evidence at the time of initial implementation.

## Saved-permission lifecycle evidence (2026-11-09)

The smoke harness calls [`saved_permissions.rs`](../../tests/support/saved_permissions.rs)
after ownership verification and creation of its disposable Git project.
macOS arm64 and Linux x86-64 v2.2.5 checks pass for two-project isolation, explicit/empty project filters,
server/header fallback, unchanged omission/null patches and explicit empty values,
duplicate conflicts, missing updates and repeated removal. Edits affect future
evaluations immediately; existing pending requests stay pending, and configured
denies retain precedence. Rules are created through `Always` replies to manual
fixture requests, then removed. No model or tool executes.

The pinned [handler](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/handlers/permission.ts),
[saved store](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/permission/saved.ts)
and [evaluation service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/permission.ts)
explain the project fallback, global ID mutations, nullish update behavior,
uniqueness conflicts, missing-delete success and fresh saved-rule reads.
[Pinned compatibility 42715208158](https://github.com/a-poor/opencoders/actions/runs/42715208158)
passed smoke and active jobs at `ea1df6fe55a10c767da307603f4715908247d1b1`.
All seven jobs in [CI 42715169271](https://github.com/a-poor/opencoders/actions/runs/42715169271)
also passed on that revision.
Custom hooks, concurrent writes,
remote stores, restart durability and cancellation of active tools are unverified.

## Session activity and synthetic input (2026-11-09)

Both disposable suites pass locally on macOS arm64 with OpenCode v2.2.5.

The smoke fixture [`activity.rs`](../../tests/support/activity.rs) checks inactive
snapshots, no-idle view acknowledgements, parked empty synthetic input, repeated
ID reconciliation, cross-session/type conflicts, missing sessions and cancellation.
The active fixture [`live_activity.rs`](../../tests/support/live_activity.rs)
uses two separately gated localhost provider turns to check running/idle snapshots,
synthetic promotion and history reconciliation, future/duplicate/stale view
acknowledgements, viewed events, and unchanged `time.updated`. It also verifies
that synthetic admission preserves a staged revert and stays parked until cancelled.

The pinned [session service](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/session.ts),
[inbox](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/inbox.ts),
[coordinator](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/run-coordinator.ts)
and [projector](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session/projector.ts)
establish process ownership through cleanup, monotone viewed watermarks, first
admission winning and consumed-record reconstruction. Cancellation removes the
pending deduplication record. `resume(false)` suppresses waking an idle drain;
it does not pause an existing one. Explicit null defaults and numeric precision
are protocol-tested. Cross-process ownership, restarts, concurrent admission/viewers,
synthetic steering during execution, remote providers and synthetic tool execution
remain outside this fixture. See [usage](../../docs/workflows.md).
