# Workspace reads and lifecycle

These APIs are unreleased additions after 0.1.0; use the repository checkout
containing them until the next crate release.

`Client::files()` lists and finds paths and streams file bytes. `Client::vcs()`
reads branch information, review bases, status and diffs. `Client::worktrees()`
lists and manages the directories registered to a project. `vcs().init()` explicitly
initializes version control. Experimental filesystem writes/watch remain outside
this slice.

All location-scoped methods have `_at` overrides. Resolve a location once with
`client.location().get()` and pass its directory when related calls must address
the same workspace. Responses retain their resolved location and additive fields.
Worktree listing takes a `ProjectId` and is not location-scoped.

```rust,no_run
# async fn example(client: &opencoders::Client) -> opencoders::Result<()> {
use opencoders::{FileFindOptions, FileListOptions, VcsDiffOptions, models::VcsMode};
let entries = client.files().list(&FileListOptions::default()).await?;
let found = client.files().find(&FileFindOptions::new("main").limit(20)).await?;
let diff = client.vcs().diff(&VcsDiffOptions::new(VcsMode::Working).context(3)).await?;
# Ok(()) }
```

Find searches paths, not file contents. `kind(FileSystemEntryType::File)` or
`Directory` narrows it. List/find order is retained as returned by the server.
Listing accepts server-defined relative or absolute paths; it is not a confinement
boundary. The pinned server confines file **reads** to the selected directory,
including realpath checks. The SDK encodes a read path as one URL component so
slashes, Unicode, percent signs, `?` and `#` reach the server intact; empty paths
and paths equal to `.` or `..` are rejected before HTTP. Embedded path components
are preserved for server-side resolution and confinement.

## Binary reads and metadata

```rust,no_run
# async fn example(client: &opencoders::Client) -> opencoders::Result<()> {
use opencoders::{FileReadOptions, StatusCode};
use std::time::Duration;
let response = client.files().read("assets/image.png", &FileReadOptions::default()
    .range("bytes=0-1023").max_bytes(1024)
    .idle_timeout(Duration::from_secs(10))).await?;
match response.status() {
    StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
        // Inspect headers() here before consuming the response.
        let mut body = response.into_body();
        while let Some(chunk) = body.next().await { let bytes = chunk?; /* consume bytes */ }
    }
    StatusCode::NOT_MODIFIED => { /* retain cached bytes */ }
    StatusCode::RANGE_NOT_SATISFIABLE => { /* inspect Content-Range and revise request */ }
    _ => unreachable!(),
}
# Ok(()) }
```

HTTP 200/206 return a byte stream; 304/416 return metadata and an empty stream.
Other statuses use the normal bounded HTTP error. Headers include the server's
ETag, Last-Modified, Content-Range and content type without reinterpretation.
`if_none_match`, `if_modified_since` and `if_range` send caller-supplied validators.
The pinned server supports a single range and can fall back to 200 for malformed
or multiple ranges; always inspect status. Invalid header values fail before send.

The default handshake deadline is 30 seconds, including HTTP error-body reading.
There is no default idle deadline or total byte limit. Set `idle_timeout` and
`max_bytes` for the application's needs; an over-limit chunk is not yielded.
Ordinary JSON response limits/deadlines do not govern file bodies. Policies on an
injected reqwest client still apply. `FileStream` implements `Stream` and has
`next()`/`close()`; it creates no task. Cancelling a pending read retains its idle
deadline. Closing or dropping releases the response. A stream error appears once,
then EOF. There is no automatic retry or range resumption.

## VCS semantics

`get()` returns optional current/default branch names. `base()` has required but
nullable `data`: `None` means no inferred review base. An initial VCS snapshot can
be empty while the pinned server activates its provider; the library does not
retry or activate it implicitly. Applications choose when to refresh.

`branches(VcsBranchOptions)` accepts search and limit; the pinned server defaults
to 50 and caps at 100. `diff(VcsDiffOptions)` requires `Working`, `Branch` or
`Committed`, with optional base, context and untracked-file/folder handling.
Working compares HEAD to the working tree; branch compares the base merge-base
to the working tree; committed compares that merge-base to HEAD. An explicit base
does not change repository configuration. Counts remain bounded integers; diff
objects retain patches and server truncation metadata. Snapshots across calls are
not atomic, and an empty status is not proof that a provider is ready.

Run `cargo run --example workspace` with `OPENCODE_BASE_URL`, optional
`OPENCODE_PASSWORD` and `OPENCODE_DIRECTORY`. Set `OPENCODE_FILE` to a relative
path to count up to 1 MiB of streamed bytes without printing file contents.
The example performs no repository or filesystem mutations through the API.

## Explicit lifecycle mutations

`Vcs::init(&VcsInitOptions)` sends a bodyless POST and accepts only HTTP 204.
Its `_at` override selects a per-call location. Omitted `provider` defaults to
`git` on v2.2.5; registered providers can have different effects. Initialization
uses the location's **project directory**, refreshes its location services and
creates no commit. Existing version control returns a conflict; unknown providers
return 400, unsupported initialization is a typed `VcsInitNotSupported` (501), and
provider failure can return 503. An error or cancelled request need not mean that
no files changed. The SDK does not retry initialization.

Worktree mutations take a `ProjectId` and ignore the client's default location:

```rust,no_run
# async fn example(client: &opencoders::Client, project: opencoders::models::ProjectId) -> opencoders::Result<()> {
use opencoders::models::{WorktreeCreateRequest, WorktreeRemoveRequest};
let created = client.worktrees().create(&WorktreeCreateRequest::new(project.clone())
    .directory("/server/checkouts".to_owned().into()).name("review").branch("HEAD")).await?;
client.worktrees().refresh(&project).await?;
client.worktrees().remove(&WorktreeRemoveRequest::new(project, created.directory, false)).await?;
# Ok(()) }
```

- **Create** loads canonical project configuration, selects its worktree strategy,
  creates a checkout and can execute the project's setup script. `from` selects a
  saved source checkout; omission uses the canonical project root. `directory`
  names the **parent directory**, not the final checkout. The server generates a
  name when omitted and can append collision suffixes. Always use the returned
  `WorktreeInfo.directory`. The built-in Git strategy checks out the requested
  revision (default HEAD) detached; `branch` does not create a named Git branch.
  Optional fields reject null and preserve explicit empty strings for server
  validation. A setup failure can occur after the checkout is created and saved.
- **Refresh** discovers through the configured strategies and reconciles saved
  inventory, including externally created checkouts and missing directories.
  It can load configuration/plugins and changes server state. It does not refresh
  a checkout's files from a remote. The pinned server can tolerate discovery
  errors, so HTTP 204 is not a guarantee that every strategy enumerated everything.
- **Remove** sends a JSON body on DELETE, including a required boolean `force`.
  The constructor requires that decision explicitly. The selected strategy can
  delete checkout files; the pinned server rejects the canonical/root checkout
  as an unmanaged directory. Dirty Git checkouts return `ApiError::Worktree` with
  `force_required: Some(true)`. The SDK never escalates force automatically.
  Optional/null force-error metadata maps to `None`; explicit false is preserved,
  and the bounded raw error body retains the original wire form.

Create expects HTTP 200; refresh/remove expect HTTP 204. Paths are server paths;
the SDK neither checks the local filesystem nor imposes a local root. Mutations
use the configured ordinary request deadline. Cancellation, transport loss or a
setup failure can leave partial/completed effects: inspect saved inventory and
server state before retrying. The SDK adds no retries, cleanup or rollback;
explicit retry policies on an injected HTTP client remain caller-owned.

`cargo run --example worktree -- --execute` requires `OPENCODE_BASE_URL` and
`OPENCODE_DIRECTORY` pointing to an existing committed project. Optional
`OPENCODE_PASSWORD` supplies auth and `OPENCODE_WORKTREE_PARENT` chooses a server
parent directory. It creates, refreshes and removes one checkout, always using
`force: false`; failures print the created path for inspection. Project setup
scripts may run and may leave dirty files that prevent cleanup. An ambiguous
create failure can require listing worktrees to find the result.
