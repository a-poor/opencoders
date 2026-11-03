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
```

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
session list before writing fixtures. It creates eight initially empty sessions through the
SDK and imports one additional transcript with seven fixed messages through raw
HTTP solely for fixture setup. The log test also seeds captured rename events
into the runner-owned SQLite database (details below). All covered operations use
the SDK, and all nine
sessions are deleted before completion. Prompt fixtures explicitly use `resume(false)`
and inbox updates only queue work, so the test does not wake model execution or connect
to an existing server. Do not manually point its environment
variables at your normal server. `cargo test` compiles but skips this test.

## Verified behavior

Local run: **2026-11-02, macOS arm64, OpenCode 2.2.5, Rust 1.99.1**. The full
smoke test passes. Linux is configured in the manual workflow but has not yet
been exercised in this repository. Hosted workflow runs require pushing the files.

| Operation | Observed and asserted |
| --- | --- |
| `server.info` | Typed version/PID/paths; Basic authentication with username `opencode`; unauthenticated calls return typed HTTP 401. |
| `location.get` | Nested query encoding, spaces/Unicode, per-call overrides, `x-opencode-directory` fallback, and query precedence over that header. |
| `agent.list` | Nonempty typed agent list with resolved location metadata. |
| `agent.get` | Listed agent round-trip, query/header location selection, and typed missing-agent HTTP 404. |
| `model.list` | Typed catalog/location envelope, default chat filter, repeated type parameters, deduplication/group order, unknown-type empty lists, and directory overrides. |
| `model.default` | Typed default-model envelope, resolved location, header fallback, and query precedence. Null default data is covered by mocks. |
| `session.create` | Typed JSON requests/responses, titles, metadata, empty permissions, parent location precedence, missing-parent HTTP 404, `{}` defaults, and all eight fields explicitly null. |
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
Live message snapshots during active execution, concurrent timeline changes, and
experimental media-reference negotiation remain unverified.

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
Waking, delivery during execution, already delivered/cancelled ID reuse, synthetic/
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

All permission fixtures are manually created for idle disposable sessions with
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

Live checks exercise only idle sessions: empty inboxes with every resume query
choice, a held queued prompt with resume true, and a held steering prompt with
omitted/false resume. Wait returns HTTP 204 while those prompts remain pending and
message history remains empty. Fixtures are then cancelled through the SDK. No
resume-true call has steering/control work eligible to run, and no model is invoked.

Active interruption/cleanup, concurrent wakeups, failure outcomes, and actual
steering resumption are source-reviewed but not live-validated. Deterministic
protocol tests cover both response booleans, statuses/errors, total/error-body
deadlines, injected timeout policies, cancellation of one independent wait,
connection cleanup, empty keep-alive responses, and no automatic retry.

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
provider execution, and other platforms remain unverified.

## CI

The ordinary [`ci.yml`](../../.github/workflows/ci.yml) checks formatting, Clippy,
docs/examples, default and HTTP-only tests on Linux/macOS, both library feature
sets on Rust 1.88, Python runner tests, and all eight checked-in generation targets.
Regeneration pins Rust/rustfmt 1.99.1 and uses the locked dependencies and local
schema; it never refreshes upstream input. The full-schema experiment remains a
separate optional check.

[`live.yml`](../../.github/workflows/live.yml) is manual (`workflow_dispatch`) and
runs this same runner on Linux x86-64 and macOS arm64. No model secrets are needed.
Both workflows use read-only repository permissions and a pinned checkout action.
Validate workflow edits with `actionlint`; runner checks use:

```sh
python3 -m unittest discover -s tools/live -p 'test_*.py'
```
