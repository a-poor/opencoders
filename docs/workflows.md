# Session workflows

[Documentation index](README.md)

Commands and skills are location catalogs; session operations use the session's
stored location and ignore the client's default location. Applications own
selection, confirmation, execution observation, and recovery after uncertain
mutation results. No operation below retries automatically.

## Discovery and command execution

```rust
use opencoders::{Client, Location, models::{SessionId, SessionCommandRequest}};

async fn command(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    let commands = client.commands().list().await?;
    let skills = client.skills().list_at(Location::directory("/remote/workspace")).await?;
    // Both responses retain resolved location, data and additional fields.
    // After the application selects and authorizes a registered command:
    client.sessions().command(id, &SessionCommandRequest::new("review", "check the changes")).await?;
    Ok(())
}
```

`commands().list()` and `skills().list()` use the client default location;
`list_at` overrides it, and `Location::Unspecified` omits the location query.
Registration depends on configuration/plugins, so catalogs can be empty or change.
Skill entries include ID, name, server path, content, optional description and
`autoinvoke`; discovery does not itself invoke a skill.

`SessionCommandRequest` requires name and text. Empty text is sent unchanged.
Optional `files`, `agents`, and `skills` reuse strict prompt attachment types;
omitted and empty arrays remain distinct, while null arrays are rejected.
`delivery` supports `RequestField::Omitted`, `Null`, or `Value`; omitted/null
means steer on v2.2.5. A file attachment contains a server-resolved URI, with no
SDK file read or upload.

HTTP 204 means the registered callback returned. Callbacks may enqueue prompts,
switch selection, create child sessions, run shell substitutions or make other
changes. Observe execution events/history separately. `CommandNotFound` and
`CommandExecution` preserve server diagnostics; callback failures can leave partial
side effects. The ordinary request deadline applies, and losing a response does
not establish that the callback failed.

## Context and turn diffs

```rust
use opencoders::{Client, SessionDiffOptions, models::SessionId};

async fn inspect(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    let context = client.sessions().context(id).await?;
    let changes = client.sessions().diff(id, &SessionDiffOptions::default().context(3)).await?;
    Ok(())
}
```

Context reuses `SessionMessageInfo` and returns the active history window starting
at the latest completed compaction (including its summary/native window). It is
not the complete transcript or the final provider prompt. Use message pagination
for the complete projected history.

Diffs reuse `FileDiffInfo`. Omitted `from` selects the latest user turn; optional
`to` extends through another user turn. Both endpoints must name user messages,
and `to` cannot precede `from`. A turn spans prompts from the first one after idle
through the next idle marker, so steered prompts may share a turn. Legacy history
without idle markers uses the next user message as a boundary. Cross-location
ranges are rejected. Snapshot availability/configuration affects results; a
running step may compare against the current working copy. Omitted `context`
requests full-file patches; zero explicitly requests no context lines.

Run `cargo run --example session_workflows` with `OPENCODE_BASE_URL`, optional
`OPENCODE_PASSWORD`/`OPENCODE_DIRECTORY`, and `OPENCODE_SESSION_ID` to inspect
catalogs, context and diffs without submitting execution.

## Compaction admission

`client.sessions().compact(id, &SessionCompactRequest::default()).await?` returns
HTTP 200 with a durable `SessionInboxCompaction`, not a completed summary. An
optional `.id(message_id)` supplies a reconciliation key; `.delivery(...)` selects
steer/queue. Public fields preserve explicit null. The server defaults to steer
and wakes execution. It may call a provider and spend credit.

**On v2.2.5, compaction commits any staged revert before admission.** A subsequent
admission error therefore does not imply history stayed unchanged. Observe
compaction messages/events and execution outcomes before displaying completion.
The SDK does not retry even when an explicit input ID is supplied.

## Revert lifecycle

```rust
use opencoders::{Client, models::{SessionId, SessionMessageId, SessionRevertStageRequest}};

async fn stage_history_only(client: &Client, id: &SessionId, message: SessionMessageId)
    -> opencoders::Result<()>
{
    let staged = client.sessions().revert(id)
        .stage(&SessionRevertStageRequest::new(message).files(false)).await?;
    // Retain staged.data and present the boundary to the application/user.
    // Separately choose .clear().await? or .commit().await?.
    Ok(())
}
```

All three revert operations require an idle session; active execution returns
`ApiError::SessionBusy` (409). The boundary is **inclusive**:

- `stage` returns metadata and retains history. Omitted/null `files` restores file
  changes using server snapshots; `.files(false)` stages a history-only boundary.
  Moving an existing file revert first restores its original snapshot, even if
  the new request uses false. Staging is therefore not a pure preview.
- `clear` restores paths affected by staging, removes the boundary, and wakes
  queued execution. Restoring a snapshot can overwrite edits made after staging.
  With no staged boundary it still wakes execution on v2.2.5.
- `commit` permanently deletes messages and corresponding queued inputs at/after
  the boundary, rewinds instruction state and releases related media references.
  It retains staged file changes. `clear` cannot undo a commit. No staged boundary
  makes commit a no-op on v2.2.5.

Submitting a new prompt can automatically commit a staged revert after prompt
preparation succeeds. Compaction also commits it; command callbacks that submit
prompts inherit that effect. Inspect/reconcile server state after an uncertain
result instead of blindly retrying. `ApiError::Unknown` preserves optional server
failure references; raw error bodies retain additional details. Debug output
redacts diagnostic text.

The disposable active fixture verifies Git snapshot diffs/restoration, command
callbacks, busy errors, compaction completion/context, automatic and explicit
revert commits. Custom callbacks, remote providers, concurrent writers and
cross-location snapshot ranges remain outside that live evidence; see
[compatibility](development.md).
