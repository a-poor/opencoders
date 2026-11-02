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
session list before writing fixtures. It creates eight empty sessions through the
SDK and imports one additional transcript with seven fixed messages through raw
HTTP solely for fixture setup. All covered operations use the SDK, and all nine
sessions are deleted before completion. It never submits a prompt, invokes a model, or connects
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
| `session.create` | Typed JSON requests/responses, titles, metadata, empty permissions, parent location precedence, missing-parent HTTP 404, `{}` defaults, and all eight fields explicitly null. |
| `session.get` | Created-session round trips, retrieval across directories independently of the SDK default location, and typed missing-session HTTP 404. |
| `session.list` | Real session decoding, server-wide scope, directory/search/root/child/archive filters, next/previous traversal, empty terminal page, invalid-cursor HTTP 400, and zero-limit rejection. |
| `session.update` | Empty HTTP 204; title, metadata/permission replacement and clearing; archive/restore; omitted/null no-ops; cross-directory IDs; typed missing-session HTTP 404. |
| `session.remove` | Empty HTTP 204; deletion of a parent, child, and grandchild while preserving unrelated sessions; repeated/missing deletion returns typed HTTP 404. |
| `session.message.list` | Empty/populated history, newest-first default, both ordering directions and next/previous traversal, type filters, empty boundary pages, invalid cursor and cursor/order conflicts, zero-limit rejection, and missing-session errors. |
| `session.message.get` | Every imported message round-trips, including user attachments and assistant reasoning/tool content; missing and wrong-session message errors are typed. |

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
tests. Agent/model selection and explicit IDs are covered by request-shape checks,
not live execution tests.

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

The existing [`openapi/provenance.json`](../../openapi/provenance.json) describes
the original schema download, whose originating server release remains unknown.
Its `info.version: 0.0.1` is not this server's version. Testing v2.2.5 does not
retroactively identify that schema's source release. Concurrent session changes,
maximum page sizes, project/subpath filtering, explicit-null discovery queries, SSE,
provider execution, and other platforms remain unverified.

## CI

The ordinary [`ci.yml`](../../.github/workflows/ci.yml) checks formatting, Clippy,
docs/examples, default and HTTP-only tests on Linux/macOS, both library feature
sets on Rust 1.88, Python runner tests, and all five checked-in generation targets.
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
