# Workspace reads

These APIs are unreleased additions after 0.1.0; use the repository checkout
containing them until the next crate release.

`Client::files()` lists and finds paths and streams file bytes. `Client::vcs()`
reads branch information, review bases, status and diffs. `Client::worktrees()`
lists the directories registered to a project. Filesystem writes/watch, worktree
mutations and VCS initialization are outside this slice.

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
