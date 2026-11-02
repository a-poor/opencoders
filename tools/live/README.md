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
session list before writing fixtures. It creates five empty sessions and archives
one through raw HTTP solely for fixture setup. It never submits a prompt, invokes
a model, or connects to an existing server. Do not manually point its environment
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
| `session.list` | Real session decoding, server-wide scope, directory/search/root/child/archive filters, next/previous traversal, empty terminal page, invalid-cursor HTTP 400, and zero-limit rejection. |

Two pagination details matter for consumers:

- v2.2.5 embeds filters and ordering in cursors. Valid conflicting filters/order
  supplied alongside a cursor are ignored; `limit` remains a separate page-size
  parameter. Clear the cursor when changing filters. The SDK still sends every
  explicitly supplied option and treats cursors as opaque.
- Nonempty pages return boundary cursors, including at the beginning/end and when
  the page is shorter than `limit`. Cursor presence does **not** prove that more
  rows exist. Following one can return an empty page without a next cursor.

The typed responses exercised here decode without generated-model changes.
This does not establish compatibility for every optional field or operation.
The schema's empty security declarations omit authentication requirements;
v2.2.5 requires Basic credentials (or its pairing/session flow), not Bearer tokens.
The SDK now provides `ClientBuilder::basic_auth`; Bearer remains available for
proxies. Pairing/cookies are not covered. See the pinned upstream
[authorization middleware](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/server/src/middleware/authorization.ts)
and [session listing implementation](https://github.com/anomalyco/opencode/blob/v2.2.5/packages/core/src/session.ts).

The existing [`openapi/provenance.json`](../../openapi/provenance.json) describes
the original schema download, whose originating server release remains unknown.
Its `info.version: 0.0.1` is not this server's version. Testing v2.2.5 does not
retroactively identify that schema's source release. Concurrent session changes,
maximum page sizes, project/subpath filtering, explicit-null locations, SSE,
provider execution, and other platforms remain unverified.

## CI

The ordinary [`ci.yml`](../../.github/workflows/ci.yml) checks formatting, Clippy,
docs/examples, default and HTTP-only tests on Linux/macOS, both library feature
sets on Rust 1.88, Python runner tests, and all four checked-in generation targets.
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
