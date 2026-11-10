# Development and compatibility

[Documentation index](README.md)

## Development checks

Run these commands from the repository checkout. The crate archive includes
source, guides, examples, tests and JSON fixtures; schema-generation, coverage and live
runner tools are repository-only. Package checks and the release handoff are
documented in [RELEASING.md](https://github.com/a-poor/opencoders/blob/main/RELEASING.md).

```sh
cargo test --workspace --locked
cargo test --locked --all-features
cargo test --locked --no-default-features --features websocket
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo doc --workspace --all-features --no-deps --locked
python3 tools/coverage/report.py --check
python3 -m unittest discover -s tools/coverage -p 'test_*.py'
python3 -m unittest discover -s tools/schema -p 'test_*.py'
```

Tests use local mock HTTP servers; they need permission to bind localhost ports
but do not require OpenCode, provider credentials, or model calls. The runnable
examples are built by the checks and contact a real server only when invoked.

Hosted CI and manual live-server jobs run on Linux. Development testing is
primarily on macOS; retain local macOS checks for releases and significant
dependency changes. Hosted macOS jobs may return if platform-specific behavior
justifies their cost and queue time.

## Compatibility

The pinned schema and provenance live in [`openapi/`](https://github.com/a-poor/opencoders/tree/main/openapi). The implemented
conversation operations pass an opt-in smoke test against **OpenCode v2.2.5 on macOS
arm64 and Linux x86-64**, using disposable data and no model calls. The 16 workspace/PTY operations pass locally on macOS arm64 and in Linux run
42707625108 at `88224a1` on 2026-11-09. The four newer worktree-lifecycle/VCS-init
operations and nine session-workflow operations also pass on macOS arm64 and
Linux at `3ff1edb` in run 42713795590. Saved-permission management also passes
on macOS arm64 and Linux at `ea1df6f` in run 42715208158.
Run `python3 tools/live/run.py --websocket`;
see [compatibility details and CI](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md) for prerequisites, release
checksums, verified behavior, and limitations. Other endpoints remain unimplemented. Log payload replay/follow uses explicitly seeded
fixtures because the stock CLI disables retention. Run `python3 tools/live/active.py`
separately for active execution with a scripted localhost provider and disposable
tools: streaming, interruption, failure and permission/form resumption pass on the
same two platforms. It also verifies controlled disconnect/reconnect and snapshot
recovery using the background observer. It uses no real credentials or paid calls.
Hosted smoke and active jobs passed on both platforms on 2026-11-03; see the
[recorded runs](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md#hosted-validation).
Remote providers, server overflow, restart recovery and other platforms remain
unverified.

## Coverage and generation

The [API coverage inventory](https://github.com/a-poor/opencoders/blob/main/API_COVERAGE.md) classifies all 170 pinned operations:
76 supported, 89 planned and 5 deferred. Each supported route links to its Rust
method, representative protocol tests and verification limits. See the
[milestone 2 readiness audit](https://github.com/a-poor/opencoders/blob/main/RELEASE_READINESS.md) for remaining integration,
execution/recovery and release work. The inventory checks are offline and do not
run during ordinary crate builds.

The private [`tools/model-spike/`](https://github.com/a-poor/opencoders/blob/main/tools/model-spike/README.md) workspace tool
contains the initial experiment and reviewed server-info/discovery/session/message/inbox/event/permission/form/workspace/PTY/workflow generation commands.
Models are checked in; normal builds do not generate code or fetch a schema.
Ordinary `cargo build` and `cargo test` select the root library; `--workspace`
also checks the tool.

A separate weekly/manual workflow checks the published OpenAPI JSON for content
drift without modifying the pin or generated code. See the
[schema maintenance guide](https://github.com/a-poor/opencoders/blob/main/openapi/README.md#weekly-upstream-check)
for comparison rules, results and manual commands. Ordinary CI tests the checker
offline and never fetches the upstream schema.
