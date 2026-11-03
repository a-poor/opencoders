# Sessions and message history

[Documentation index](README.md)

## Session listing and pagination

```rust
use opencoders::{Client, ParentFilter, SessionListOptions, SessionOrder};

async fn browse(client: &Client) -> opencoders::Result<()> {
    let mut options = SessionListOptions::default()
        .limit(20).order(SessionOrder::Desc)
        .parent(ParentFilter::RootOnly).archived(false);
    let first = client.sessions().list(&options).await?;
    if let Some(next) = first.cursor.next {
        options.cursor = Some(next);
        options.order = None;
        let second = client.sessions().list(&options).await?;
        // second.cursor.previous can retrieve the preceding page.
    }
    Ok(())
}
```

`SessionListOptions::default()` omits all filters. Options cover limit, order,
search, parent, archived state, directory, project, subpath, and cursor. Root-only
selection sends `parentID=null`; selecting children uses a typed `SessionId`.
Omitted archive filtering differs from explicit `false`. Public option fields
can be cleared with `None`; provided empty strings are sent unchanged. The SDK
represents limits as `u32` and leaves accepted ranges to the server.

Session listing ignores the SDK's default location and uses explicit, direct
`directory`/`project` filters. Each call returns one `data`/`cursor` page in server
order. Pass either opaque cursor back unchanged, retaining applicable filters;
the order option is documented for the first page. Setting a cursor does not
silently change other options. Missing or null cursors become `None`; an empty
cursor remains `Some`. The SDK performs no automatic pagination or retries.

On tested OpenCode v2.2.5, cursors carry the original filters and ordering; changing
those requires clearing the cursor. `limit` still controls page size. A next or
previous cursor marks a boundary and can lead to an empty page, even when the
current page is shorter than the requested limit.

Run `cargo run --example sessions` with the same environment variables as the
discovery example. It follows up to three pages, keeping pagination in caller
code. Session models reuse discovery ID/location/model/permission types and
preserve added fields. Numeric timestamps, costs, and token values use
`serde_json::Number` to retain supported integer precision. Session/message ID
prefixes and nested file-diff bounds/flags are checked during decoding.

## Creating and retrieving sessions

```rust
use opencoders::{Client, RequestField, models::{SessionCreateRequest, SessionLocationInput}};

async fn create(client: &Client) -> opencoders::Result<()> {
    let mut request = SessionCreateRequest::default()
        .title("New conversation")
        .location(SessionLocationInput::new("/remote/workspace"));
    // Public fields also allow explicit null or omission:
    request.metadata = RequestField::Null;
    let created = client.sessions().create(&request).await?;
    let retrieved = client.sessions().get(&created.data.id).await?;
    println!("{}", retrieved.data.id.as_str());
    Ok(())
}
```

Both calls return `SessionResponse`, preserving `data` and envelope extensions.
Creation sends a JSON body (`{}` for a default request) and does not submit a
prompt. `RequestField::Omitted`, `Null`, and `Value(value)` preserve all three wire
states for the eight optional create fields. Fluent setters set values; public
fields can be reset to omission or null. Null does not imply a general clearing
policy. v2.2.5 accepts all eight fields as null; null/omitted location and parent
use the server working directory with no parent.

Creation uses the body's location or the parent's location when a concrete
`parentID` is supplied; the parent takes precedence even over a conflicting body
location on v2.2.5. Create/get ignore `ClientBuilder::default_location`. Retrieval
encodes the typed session ID as one path segment. Input location/model/permission
types are separate from tolerant response models, so response extensions are not
automatically echoed into writes. `SessionModelInput::variant` permits omission
or a string, not null; arbitrary JSON belongs in `SessionMetadata`.

The default transport does not retry creation or follow redirects. A timeout,
disconnect, or unreadable response can happen **after the session was created**.
Inspect server state before resubmitting; an explicit session ID is not a promise
of idempotency. An injected reqwest client's retry/redirect policy is retained.

Run `cargo run --example create_session` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, and optional `OPENCODE_PASSWORD`. It creates one empty
session, retrieves it, and leaves it on the server.

## Forking sessions

```rust
use opencoders::{Client, models::{SessionId, SessionMessageId, SessionForkRequest}};

async fn branch(client: &Client, source: &SessionId,
                before: SessionMessageId) -> opencoders::Result<()> {
    let fork = client.sessions().fork(source,
        &SessionForkRequest::default().before(before)).await?;
    println!("Created fork {}", fork.data.id.as_str());
    Ok(())
}
```

A default request sends `{}` and copies through the newest projected message.
`before(id)` excludes that message and copies earlier history; the ID must belong
to the source session. Public `before: RequestField<SessionMessageId>` preserves
omitted, null, and value. Null behaves like omission on v2.2.5. An entirely empty
source returns `InvalidRequest` with kind `empty_session`; forking before its
first existing message succeeds with an empty fork.

The result is a `SessionResponse`. On the pinned server, forks have `fork`
provenance (`session_id` plus a `before`/`through` boundary) and **no `parentID`**.
They appear as root sessions; deleting the source does not recursively delete
its forks. This differs from creating an owned child with `parent_id(...)`.
The event's `parentID` identifies the fork source, not deletion ownership.

Forks use the source's location and current agent/model, metadata, permissions,
and instruction values, even when copying an earlier boundary. They get a fresh
ID/title/timestamps; usage resets, and archived/execution/revert state is not
inherited. Discovery location defaults are ignored. Copied history receives new
message IDs; compaction `keepID` references are remapped. Pending inbox items are
not copied. Forking does not start model work.

v2.2.5 copies settled history, excluding unfinished assistant, shell, and
compaction projections. The SDK does not promise an atomic snapshot during
concurrent execution. Active-work exclusion and instruction inheritance are
source-reviewed; the disposable live checks cover settled fixtures and current
agent/model/metadata/permission inheritance.

The method accepts HTTP 200 and uses ordinary request deadlines and bounded
diagnostics. Missing sessions/boundaries expose existing typed errors. A failed
response can follow successful creation; the request has no idempotency key and
is never automatically retried. Reconcile server state before resubmitting.

Run `cargo run --example session_fork` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, and optional `OPENCODE_PASSWORD`. It creates a fresh session,
records an agent-selection history entry without model calls, creates full and
empty forks, and removes its known fixtures. See [compatibility notes](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md).

## Updating and deleting sessions

```rust
use opencoders::{Client, models::{SessionId, SessionUpdateRequest}};

async fn manage(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    client.sessions().update(id, &SessionUpdateRequest::default()
        .title("Renamed conversation").archived(true)).await?;
    client.sessions().update(id, &SessionUpdateRequest::default()
        .archived(false)).await?;
    // Removal also deletes this session's descendants.
    client.sessions().remove(id).await?;
    Ok(())
}
```

Both methods return `()` for HTTP 204, without decoding JSON. They encode the ID
as one path segment and ignore the SDK's discovery location default. Updates
support `title`, `metadata`, `permissions`, and `archived`, each using
`RequestField<T>` to preserve omission, null, and values.

On tested v2.2.5, omitted and null fields leave existing values unchanged.
Metadata and permission values replace their contents; an empty map or list
clears those contents. `archived(true)` archives, and `archived(false)` restores.
An empty title requests server-generated titling and can invoke a model when the
session has conversation history; it does not clear the title.

Deletion removes the session and its descendants; applications own confirmation
and recovery decisions. Repeated deletion returns a typed missing-session error
on v2.2.5. Updates can be partially applied before an error, and a lost response
can follow a completed update or deletion. The default transport does not retry
either method; inspect server state before deciding to repeat a mutation.

Run `cargo run --example session_lifecycle` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, and optional `OPENCODE_PASSWORD`. It creates a fresh empty
session, renames it, archives/restores it, and deletes it without a model call.
If a later step fails, the printed session ID is available for inspection.

## Message history and retrieval

```rust
use opencoders::{Client, MessageListOptions, MessageOrder,
    models::{SessionId, SessionMessageType}};

async fn history(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    let messages = client.sessions().messages(id);
    let mut options = MessageListOptions::default().limit(20)
        .order(MessageOrder::Desc).message_type(SessionMessageType::User);
    let first = messages.list(&options).await?;
    if let Some(message) = first.data.first() {
        let retrieved = messages.get(message.id()).await?;
        println!("{}", retrieved.data.message_type());
    }
    if let Some(next) = first.cursor.next {
        options.cursor = Some(next);
        options.order = None;
        let second = messages.list(&options).await?;
    }
    Ok(())
}
```

`list` returns `SessionMessagesResponse` with `data` and `cursor`; `get` returns
`SessionMessageResponse` with `data`. Both preserve additive fields and use
session/message IDs encoded as individual path segments. They ignore the SDK's
default discovery location and do not fetch extra pages automatically.

Options cover `limit`, `order`, `cursor`, and `message_type` (sent as `type`).
Defaults omit all parameters; v2.2.5 defaults to the newest 50 messages. Follow
either opaque cursor unchanged, **omit order and repeat the same type filter**.
The server rejects a nonempty cursor combined with order. Message cursors carry
ordering but not the type filter. Setting a cursor does not change other options;
public fields can be cleared with `None`. Boundary cursors can lead to empty pages.

`SessionMessageInfo` is an enum covering all eleven pinned timeline kinds, with
`id()` and `message_type()` helpers. Match `User`, `Assistant`, or other variants
to inspect their boxed payloads. Assistant content includes typed text, reasoning,
and tool states; compaction states and attachments remain typed. These are
projected history records, not live events or pending inbox items. Added object
fields are retained; unknown tags and malformed known variants are decode errors.
Message metadata and provider-native data may contain sensitive content.

Run `cargo run --example history` with `OPENCODE_BASE_URL`, `OPENCODE_SESSION_ID`,
and optional `OPENCODE_PASSWORD`. It reads at most three pages and retrieves one
message by ID, without submitting input or changing the session.
