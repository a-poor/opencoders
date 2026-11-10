# Repository Guidelines

## Project Scope and Structure

`opencoders` is a Rust 2024 library for OpenCode v2, primarily serving GUI/TUI
developers. Applications own rendering and UI state; do not build an interface.

- `src/`: public client, transport/errors, resource methods, and generated models.
- `tools/model-spike/`: private generation tool, experiment, and tests.
- `tools/live/`: pinned server manifest, disposable runner, and compatibility notes.
- `openapi/`: pinned schema/provenance; `tools/schema/`: upstream drift checker.
- `DESIGN.md`: architecture and milestones; `TODO.md`: tasks, inbox, and backlog.
- `tests/`: model and mock HTTP tests; `examples/`: runnable library usage.
- `docs/`: detailed usage guides; keep `README.md` focused on installation and quick start.

## Build, Test, and Development Commands

- `cargo build --locked`: compile the root library.
- `cargo test --workspace --locked`: test the library and generation experiment.
- `cargo fmt --all -- --check`: check formatting; `cargo fmt --all` fixes it.
- `cargo clippy --workspace --all-targets --all-features --locked`: lint everything.
- `cargo doc --workspace --no-deps --locked`: build API documentation.

`cargo test` selects only the root crate. See
[generation checks](tools/model-spike/README.md#verify-checked-in-models) for all
fourteen pinned-toolchain checks; [RELEASING.md](RELEASING.md) covers package validation.

## Coding Style and Architecture

Use rustfmt defaults and four-space indentation. Use `snake_case` for modules and
functions, `PascalCase` for types, and `SCREAMING_SNAKE_CASE` for constants.
Document public APIs.

Follow `DESIGN.md`: typed resources, caller-owned Tokio runtime, generated wire
models, and hand-written transport/adapters. Never hand-edit generated output.
Preserve omitted-versus-null semantics and response metadata. Do not silently
retry mutations. Never commit credentials or expose them in logs or fixtures.

## Goals and Testing

Prefer scoped goals for substantive multi-step work. State the outcome and
completion checks, track progress in `TODO.md`, and finish a coherent goal before
expanding scope. Use the goal tool when the user has authorized goal-based work;
do not invent time or token budgets.

Prefer TDD where behavior is testable: write a meaningful failing test, implement,
then refactor. Research and documentation use concrete acceptance checks instead
of artificial tests. Use Rust's test framework and behavioral names such as
`prompt_preserves_delivery_mode`. No numeric coverage threshold is configured.
Prioritize deterministic protocol tests and regressions. Keep live-server/model
tests opt-in with pinned versions and disposable data.
Use `python3 tools/live/run.py` for the isolated smoke test and
`python3 tools/live/active.py` for scripted localhost execution; normal tests skip both.

## Commits and Pull Requests

History uses descriptive subjects, such as `Add repository contributor
guidelines`. Prefer imperative summaries; no Conventional Commits prefix is
required. Keep changes focused. PRs should explain behavior, link tasks/issues,
and report validation and compatibility limitations.

## Task Tracking and Inbox Triage

Agents may triage `TODO.md` during updates or when asked. Preserve intent/details,
merge duplicates, and categorize entries. Leave unclear items with clarification
notes. Retain a blank bullet in Inbox and Backlog. Update `DESIGN.md` when
architecture changes.
