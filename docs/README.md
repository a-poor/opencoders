# Documentation

Start with the [installation and quick start](../README.md). These guides cover
the library's implemented APIs for GUI/TUI and automated clients.
Applications own their Tokio runtime, rendering, state and approval decisions.

| Guide | Topics |
| --- | --- |
| [Configuration](configuration.md) | Authentication, transport, deadlines, features and errors. |
| [Discovery](discovery.md) | Server locations, agents, models and session selections. |
| [Sessions and history](sessions.md) | Pagination, creation, forks, updates, deletion and messages. |
| [Session workflows](workflows.md) | Commands/skills, context/diffs, compaction and revert lifecycle. |
| [Input and execution](execution.md) | Prompt admission, inbox management, interruption, waiting and bounded automation. |
| [Permissions and forms](requests.md) | Pending requests, replies, cancellation and server validation. |
| [Events and recovery](events.md) | Live SSE, background observation, snapshot refresh and experimental logs. |
| [Workspace APIs](workspace.md) | Filesystem reads, VCS snapshots/initialization and project worktree lifecycle. |
| [Ordinary PTYs](terminals.md) | Lifecycle, resize, tickets and optional WebSocket I/O. |
| [Development and compatibility](development.md) | Repository checks, pinned server evidence, coverage and generation. |

## Running examples

From the repository or extracted crate, run `cargo run --example NAME` against a
server you configure. Each guide describes the example's environment variables
and effects. Start with `server_info` to check the connection:

```sh
export OPENCODE_BASE_URL=http://localhost:4096
# Set OPENCODE_PASSWORD in your environment to your server password.
cargo run --example server_info
```

Management examples mutate server state. The `automation` example requires
`-- --execute` and can run tools and spend provider credit; try those workflows
in a disposable workspace. The [execution guide](execution.md#bounded-automation)
explains its limits and retained-session cleanup.

## Project references

- [Operation coverage](https://github.com/a-poor/opencoders/blob/main/API_COVERAGE.md)
- [Release notes](../CHANGELOG.md)
- [Design and rationale](https://github.com/a-poor/opencoders/blob/main/DESIGN.md)
- [Contributor guidelines](https://github.com/a-poor/opencoders/blob/main/AGENTS.md)
- [Tasks and backlog](https://github.com/a-poor/opencoders/blob/main/TODO.md)
