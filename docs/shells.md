# Non-interactive shell commands

`Client::shells()` runs standalone commands at a location. `Sessions::shell()`
runs a user command in a session and records its history. Neither opens a PTY;
see [ordinary PTYs](terminals.md) for interactive terminals.
The SDK does not retry these mutations or start a polling task. A transport
failure after submission can leave the result unknown.

## Standalone commands

```rust,no_run
use opencoders::{Client, ShellOutputOptions, models::ShellCreateRequest};
# async fn example(client: &Client) -> opencoders::Result<()> {
let command = client.shells().create(
    &ShellCreateRequest::new("printf 'hello\\n'").timeout_ms(5_000),
).await?;
let record = client.shells().get(&command.data.id).await?;
let page = client.shells().output(
    &command.data.id, &ShellOutputOptions::default(),
).await?;
// Keep the ID and resolved location while your application observes completion.
client.shells().remove(&command.data.id).await?;
# Ok(()) }
```

Creation returns once the process is registered. `list` returns running commands
only; `get` reports `running`, `exited`, `timeout` or `killed`. A nonzero process
exit is data, not an HTTP error. Exit and timestamps use exact JSON numbers;
optional PID and output counters use checked safe integers. Responses retain
resolved location and additional fields. `ShellInfo` and `ShellOutput` are the
same types used by shell events. Request, response and shared record/output
Debug implementations hide payloads; explicit fields and serialization retain
the command, environment, metadata and output.

All five standalone operations have `_at` methods for an explicit `Location`.
`cwd` controls the command directory; an empty string is preserved on the wire
but v2.2.5 uses the location directory for it. An omitted/zero `timeout` disables
the server execution timer; positive values are **milliseconds**, distinct from
the client's HTTP deadline. Values must fit 0..=2^53−1.

Metadata is echoed on records and events. Despite its opaque schema, v2.2.5
interprets a valid `metadata.sessionID` to select an installed local session
environment. It does not associate the command with session history by itself.

## Output and retention

`output` pages the merged stdout/stderr capture by **absolute byte offset**.
Omission means cursor 0 and limit 65536; the SDK preserves explicit zero. Prefer
positive limits: the pinned implementation can fail on a zero-length read of
nonempty output. Numeric-string query parameters are refined to nonnegative
safe integers by the server, so the SDK exposes checked `u64` options.

Advance using the returned cursor and compare it with `size`. The `truncated`
flag is not a “more pages” indicator: v2.2.5 returns false even for partial pages.
A cursor beyond current output returns empty text with the cursor clamped to
the current size. While running, size can grow and capture-file writes can lag;
terminal status is published after output flushing completes.

The server decodes every page independently as UTF-8. Splitting a multibyte
character can return U+FFFD replacement characters. The SDK preserves those
strings and byte cursors; it cannot reconstruct bytes already replaced by the
server. For complete binary/byte fidelity use the separate file-read API on the
returned capture path where appropriate, subject to that API's access policy.

Records are process-local. v2.2.5 retains at most 25 exited commands per location,
evicting older records and their files; an hourly sweep also removes capture
files older than seven days. These are implementation limits, not durable job
storage. `ShellNotFound` can mean removal, eviction, restart, or a wrong location.
`remove` removes the record/output and terminates a running command; HTTP 204
can precede the end of process cleanup. Removing an unknown ID also returns 204
on v2.2.5. The SDK never automatically repeats the request.

## Session commands and environments

```rust,no_run
use opencoders::{Client, SessionShellOptions, models::{SessionId, SessionShellRequest,
    SessionEnvironmentRequest}};
use std::time::Duration;
# async fn example(client: &Client, id: &SessionId) -> opencoders::Result<()> {
client.sessions().environment(id, &SessionEnvironmentRequest::new(
    [("APP_MODE".into(), "test".into())].into(),
)).await?;
client.sessions().shell(id, &SessionShellRequest::new("printf '%s' \"$APP_MODE\""),
    &SessionShellOptions::default().timeout(Duration::from_secs(30)),
).await?;
# Ok(()) }
```

Session methods use the session's location, ignoring the discovery location
default. `session.shell` waits for completion and returns HTTP 204 even for a
nonzero exit. Subscribe before submitting if you need `session.shell.started`
and `session.shell.ended`; inspect the shell message in history for its output,
exit and completion time. The optional message `id` preserves omission, null and
a value, but does **not** establish a safe retry/deduplication contract.

The default `SessionShellOptions` adds no SDK HTTP deadline; injected HTTP-client
deadlines remain. An explicit timeout overrides the total request deadline for
this call. Dropping/timing out the request stops local waiting, while the server
continues the command and records completion. The shell ID from events/history
can be passed to `shells().remove_at` with the session location to terminate it.
The pinned session path sets no process execution timeout.

v2.2.5 also adds a synthetic inbox completion notification with `resume: false`.
This call alone does not wake a model and a user shell can run while the session
is absent from `sessions().active()`. The ended-event preview is capped at 1 MiB;
the synthetic notification uses the configured bounded tail. Use the retained
shell capture for further output, within its retention lifetime.

`environment` replaces the in-memory variable map for future local commands of
that session. Omitted variables from a subsequent map disappear; `{}` installs
an empty replacement instead of restoring the server environment. Before the
first replacement commands inherit the server environment. The server adds
`TERM=xterm-256color` and `OPENCODE_TERMINAL=1`; shells/plugins may further modify
values. Running processes are unchanged. Remote workspace execution does not
use this map, and persistence across server restarts is not provided.

See [the runnable example](../examples/shell.rs) and
[pinned compatibility evidence](../tools/live/README.md). Windows execution,
remote workspaces, plugin overrides and concurrent session moves remain outside
the live fixture. `session.background` and experimental job discovery are not
part of this slice.
