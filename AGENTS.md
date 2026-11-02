# Repository Guidelines

## Project Scope and Structure

`opencoders` is a Rust 2024 library for the OpenCode v2 HTTP API, primarily serving
GUI/TUI developers. Do not build a UI; consuming applications own rendering and
state. The crate currently contains only starter code.

- `src/lib.rs`: library entry point and current unit tests. Add resource modules
  under `src/` as implementation grows.
- `DESIGN.md`: architecture, API constraints, and milestones.
- `TODO.md`: active tasks, inbox, backlog, bugs, and completed work.
- Add integration tests under `tests/` and runnable library examples under
  `examples/` when needed; neither directory exists yet.

## Build, Test, and Development Commands

- `cargo build`: compile the library.
- `cargo test`: run unit, integration, and documentation tests.
- `cargo fmt --all -- --check`: check formatting; use `cargo fmt --all` to fix it.
- `cargo clippy --all-targets --all-features`: lint supported targets and features.
- `cargo doc --no-deps`: build public API documentation.

There is no application entry point to run. CI is planned; run applicable checks
locally before submitting changes.

## Coding Style and Architecture

Use rustfmt defaults, including four-space indentation. Use `snake_case` for
modules/functions, `PascalCase` for types, and `SCREAMING_SNAKE_CASE` for constants.
Document public APIs and non-obvious protocol behavior.

Follow `DESIGN.md`: typed resource APIs, caller-owned Tokio runtime, generated wire
models, and hand-written transport/streaming behavior. Keep generation explicit
and reproducible; never hand-edit generated output. Preserve omitted-versus-null
semantics and meaningful response metadata. Do not silently retry mutations.

## Testing Guidelines

Use Rust's built-in test framework; add async tests as runtime support arrives.
Name tests after behavior, such as `prompt_preserves_delivery_mode`. No numeric
coverage threshold is configured.

Prioritize deterministic mock-server tests for encoding, errors, pagination, SSE
framing, cancellation, and disconnects. Add regression coverage for protocol bugs.
Keep live-server/model tests opt-in, with pinned server versions and disposable
data. Never commit credentials or expose them in logs or fixtures.

## Commit and Pull Request Guidelines

History uses short descriptive subjects, such as `Document client design and
development backlog`. Prefer imperative summaries; no Conventional Commits prefix
is required.

Keep changes focused. PR descriptions should explain behavior, link relevant
issues or tasks, list validation, and identify compatibility limitations.

## Task Tracking and Inbox Triage

Keep `TODO.md` current. Agents may triage its Inbox during updates or when asked:
preserve intent and details, merge duplicates, and move entries into appropriate
sections. Leave unclear entries with clarification notes. Always retain a blank
bullet in both Inbox and Backlog. Update `DESIGN.md` when architecture changes.
