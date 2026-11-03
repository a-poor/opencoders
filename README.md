# `opencoders`

`opencoders` is an async Rust client for connecting to the [OpenCode v2 HTTP API](https://opencode.ai/v2/docs/api).

The library currently supports server information, location/agent/model discovery,
session management/listing and agent/model selection, message history/retrieval,
prompt admission/inbox management, interruption/waiting, permission requests/replies, forms, live event subscriptions,
and experimental session logs. It targets developers building
GUI/TUI clients and also supports automated consumers. Applications own their UI
and state. See [DESIGN.md](DESIGN.md) and [TODO.md](TODO.md) for the plan and backlog.

## Usage

Requires Rust 1.88 or newer and a caller-owned Tokio runtime.

```rust
use opencoders::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("http://localhost:4096")
        .basic_auth("opencode", "your-server-password")
        .build()?;

    let info = client.server().info().await?;
    println!("OpenCode {} (pid {})", info.version, info.pid);
    Ok(())
}
```

Run the included example against a server you configure:

```sh
export OPENCODE_BASE_URL=http://localhost:4096
# Set OPENCODE_PASSWORD in your environment to your server password.
cargo run --example server_info
```

OpenCode v2.2.5 uses HTTP Basic authentication with username `opencode` and the
server password. Use HTTPS when connecting to a remote server. `bearer_auth`
remains available for proxies that require it; the stock server does not accept
Bearer tokens. Pairing/cookie management is not implemented.

Custom authentication schemes and application headers can be supplied through
`ClientBuilder::default_headers`, using the re-exported `opencoders::header` types.
No credentials are discovered from local OpenCode files.

## Location-aware discovery

```rust
use opencoders::{Client, Location};

async fn discover() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("http://localhost:4096")
        .default_location(Location::directory("/remote/workspace"))
        .build()?;
    let location = client.location().get().await?;
    let agents = client.agents().list().await?;
    println!("{} agents at {}", agents.data.len(), agents.location.directory);

    // Override only this call; clones retain the configured default.
    let other = client.agents().list_at(Location::directory("/remote/other")).await?;
    let fallback = client.location().get_at(Location::Unspecified).await?;
    Ok(())
}
```

`Location::Unspecified` omits the location query even when the client has a
configured default. Server fallback rules still apply, including any location
headers supplied by the application or injected HTTP client. It does not send
JSON null, an empty string, or the literal string `"null"`.

Directory values use the nested `location[directory]` query on supported routes
only. They refer to the server's filesystem; the SDK neither reads local paths
nor canonicalizes them. Server-wide calls such as `server.info()` ignore the
configured directory. Agent lists retain their `location`/`data` envelope and
include hidden agents; applications choose what to display.

Run `cargo run --example discovery` with `OPENCODE_BASE_URL`, optional
`OPENCODE_PASSWORD`, and optional `OPENCODE_DIRECTORY`. The example resolves a
location, then lists agents at that resolved directory.

## Model and agent selection

```rust
use opencoders::{Client, ModelListOptions,
    models::{SessionId, SessionModelInput, SessionSwitchAgentRequest, SessionSwitchModelRequest}};

async fn configure(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    let catalog = client.models().list(&ModelListOptions::default().types(["chat"])).await?;
    let agent = client.agents().get(&"build".to_owned().into()).await?;
    if let Some(model) = client.models().default().await?.data {
        client.sessions().switch_agent(id, &SessionSwitchAgentRequest::new(agent.data.id)).await?;
        client.sessions().switch_model(id, &SessionSwitchModelRequest::new(
            SessionModelInput::new(model.provider_id, model.id),
        )).await?;
    }
    Ok(())
}
```

`models().list`, `models().default`, and `agents().get` retain `location`/`data`
envelopes and support per-call `list_at`, `default_at`, and `get_at` overrides.
Default-model `data` is required but nullable: `None` means no default is available.
Lists preserve server order and expose a snapshot, without waiting for catalog
changes. Empty `ModelListOptions::types` omits the filter (chat on v2.2.5); nonempty
lists send repeated `types` parameters. Types are open strings, including plugin
types. The SDK preserves duplicates/order; the tested server deduplicates requested
types and concatenates matching groups. Agent IDs are encoded as one path segment;
empty IDs and `.`/`..` return a pre-send encoding error.

Selection requests use strict, required, non-null agent/model fields. Use the
catalog's public `id` and `provider_id`, not its provider-native `model_id`.
`SessionModelInput::variant` selects an optional variant. Both switch methods
return `()` for HTTP 204 and ignore the discovery location default. They record
choices for subsequent turns without executing or validating their availability:
v2.2.5 accepts even unknown agent/provider/model IDs. There is no null reset operation.

On tested v2.2.5, selecting the same agent again adds another history entry.
An omitted model variant is stored as `"default"`; repeating the same provider,
model, and variant creates no additional model-selection entry. The SDK performs
no local deduplication. A response can be lost after a selection is applied; the
default transport does not retry either operation.

Run `cargo run --example configure_session` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, `OPENCODE_AGENT_ID` (for example, `build`), and optional
`OPENCODE_PASSWORD`. It discovers the selected agent and default model, creates
a fresh session, records both choices, retrieves it, and leaves it on the server.
If no default model exists, it exits before creating a session. It makes no model calls.

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
empty forks, and removes its known fixtures. See [compatibility notes](tools/live/README.md).

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

## Prompt submission and pending input

```rust
use opencoders::{Client, models::{SessionId, SessionPromptRequest,
    SessionInboxDelivery, SessionInboxUpdateRequest}};

async fn pending(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    let admitted = client.sessions().prompt(id, &SessionPromptRequest::new("Hello")
        .delivery(SessionInboxDelivery::Steer).resume(false)).await?;
    let inbox = client.sessions().inbox(id);
    let pending = inbox.list().await?;
    inbox.update(&admitted.data.id,
        &SessionInboxUpdateRequest::new(SessionInboxDelivery::Queue)).await?;
    inbox.cancel(&admitted.data.id).await?;
    Ok(())
}
```

`prompt` returns a `SessionPromptResponse` containing an admitted `SessionInboxUser`.
HTTP 200 acknowledges durable admission, **not a completed assistant reply**.
Work may already have left the inbox when the response arrives; the admission
response does not establish that it is still pending.
On v2.2.5, omitted/null `delivery` means `steer`; omitted/null `resume` wakes the
agent loop. Use `resume(false)` to admit without waking it. This does not pause
an already running loop, and `queue` delivery alone does not prevent execution.
The pinned implementation considers steered work at step boundaries and queued
work at idle boundaries; execution scheduling itself is not live-tested here.

Text is required and can be empty. `id`, `delivery`, and `resume` use
`RequestField<T>` for omitted/null/value states. Optional `files`, `agents`,
`skills`, and `metadata` use `Option<T>`: `None` omits the property, while an empty
list/map is sent unchanged; these properties reject explicit null. Nested input
models are strict and separate from response models. File attachments take a
server-resolved `uri`; the SDK does not read or upload a local file. The server
prepares files and skills before admission, so returned payloads may differ from inputs.

`inbox.list()` returns the full `SessionInboxResponse` envelope in enqueue order,
without pagination. `SessionInboxInfo` distinguishes user, synthetic, compaction,
and move items, with shared `id()`, `session_id()`, and `delivery()` helpers.
These are pending items, distinct from projected message history. Unknown kinds
fail decoding; known payloads retain additional fields. Opaque compaction payloads
accept any non-null JSON through `NonNullValue`. Optional payload collections default to
empty when omitted and serialize empty collections as absent.

`inbox.update` changes only delivery mode and returns `()` for HTTP 204. Steering
wakes execution on v2.2.5; queueing does not wake it. Updating an unavailable item,
an item belonging to another session, or one already in the requested mode returns
typed `ApiError::Conflict`. `inbox.cancel` accepts HTTP 204; unavailable/wrong-session
items are no-ops, while missing sessions return 404. Cancellation does not interrupt
already delivered work. All four operations ignore the discovery location default.

With the same pending user ID and session, v2.2.5 returns the original admission
even when the new text/delivery differs; cross-session ID reuse conflicts. This is
not a general retry guarantee for every lifecycle state. A timeout, lost response,
or decode error can follow admission or another applied mutation. The SDK does
not retry; inspect pending work/history before deciding to resubmit. An injected
HTTP client's retry/redirect policies remain in effect.

Run `cargo run --example prompt_inbox` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, and optional `OPENCODE_PASSWORD`. It creates a fresh session,
admits with `resume(false)`, lists pending work, changes delivery to queue, cancels
the item, and removes the session. It makes no model calls. Waking execution,
delivery during active runs, and completed model responses remain unverified.

## Interruption and waiting

```rust
use opencoders::{Client, SessionInterruptOptions, SessionWaitOptions, models::SessionId};
use std::time::Duration;

async fn stop(client: &Client, id: &SessionId) -> opencoders::Result<()> {
    let response = client.sessions().interrupt(id,
        &SessionInterruptOptions::default().resume(false)).await?;
    println!("Interruption accepted: {}", response.interrupted);
    client.sessions().experimental_wait(id,
        &SessionWaitOptions::default().timeout(Duration::from_secs(30))).await?;
    // Idle is not a successful-execution result or a per-prompt receipt.
    let session = client.sessions().get(id).await?;
    println!("Latest recorded outcome: {:?}", session.data.outcome);
    Ok(())
}
```

`interrupt` sends a bodyless POST and returns `SessionInterruptResponse` for HTTP
200, preserving additional response fields. Its optional `resume` query preserves
omission, false, and true. On v2.2.5, omission/false requests no resumption;
`resume(true)` can wake eligible steering/control work even when `interrupted` is
false. Queued next-turn prompts remain parked, and interruption does not remove
pending input. Use inbox cancellation separately when that is intended.

The boolean describes interruption acceptance for execution owned by this server
process. `true` returns before asynchronous cleanup completes; `false` can mean
idle, already stopping, or settling. Neither boolean promises the session is idle.
New concurrent admissions can start further work.

`experimental_wait` sends a bodyless POST and accepts only HTTP 204. The pinned
server waits through cleanup and successor executions until it observes no active
execution for that session in this process. It never starts work. An idle session
can still have pending input, and idle can follow success, failure, or interruption.
Inspect session/history snapshots and `session.execution.*` live events for
outcomes. A new execution can begin after the wait returns.

Waiting has **no SDK deadline by default**, independent of the ordinary 30-second
request timeout. `SessionWaitOptions::timeout` sets a per-call total deadline,
including error-body reads; it overrides an injected reqwest total-request
timeout. `None`/`without_timeout()` leaves injected deadlines in effect. Other
transport policies, including read/connect timeouts, remain effective. Interruption
uses the ordinary request deadline. Both methods ignore the discovery location
default and retain bounded diagnostics with no automatic retries.

Dropping or timing out a wait cancels only local waiting; it does not interrupt
execution. Cancelling an interrupt request does not undo an accepted stop. The
application owns reconnection, retry decisions, and coordination with live events.

Run `cargo run --example execution` with `OPENCODE_BASE_URL`, `OPENCODE_SESSION_ID`,
and optional `OPENCODE_PASSWORD` to wait for up to 30 seconds and inspect snapshots.
Pass `-- --interrupt` to explicitly request a stop first. The smoke test covers
idle and parked input. A separate [active fixture](tools/live/README.md#active-execution)
verifies interruption of a held localhost provider stream and wait settlement.
Concurrent wakeups, tool interruption and steering resumption remain unverified.

## Bounded automation

[`examples/automation.rs`](examples/automation.rs) creates a fresh session with
an explicitly selected agent/model, subscribes to events, admits one identified
prompt with `resume(true)`, waits and inspects history plus pending work. Running
it can spend provider credit and invoke tools that modify the server workspace,
according to the server's agent/permission configuration. Normal tests use mocks.

Set `OPENCODE_BASE_URL`, `OPENCODE_DIRECTORY`, `OPENCODE_AGENT_ID`,
`OPENCODE_PROVIDER_ID`, `OPENCODE_MODEL_ID`, and `OPENCODE_PROMPT`;
`OPENCODE_PASSWORD` is optional. Use the public catalog model ID, not its native
provider `modelID`. Provider credentials must already be configured on the server.

```sh
cargo run --example automation -- --execute
```

The flag is required before any server request. Session and prompt IDs are chosen
and printed before mutations so a lost response can be investigated. They are not
idempotency keys. The example makes one attempt and never resubmits a mutation.
Selection/admission do not establish model availability or successful execution.

The [example-owned policy](examples/support/automation.rs) uses a shared 30-second
deadline for creation, subscription, admission and waiting, followed by a separate
10-second inspection window, including after a wait timeout. Ordinary requests
also have a 10-second limit. It consumes progress alongside the requests without
spawning a task; all global traffic counts toward a 1,024-event budget. Feed EOF,
decode errors and budget exhaustion are reported separately from execution.

Inspection reads the session, inbox, permissions, forms and up to three ascending
history pages of 50 entries. Responses are limited to 1 MiB and SSE blocks to
64 KiB; these are not process-memory caps. Cursors remain opaque, and reaching a
page budget with a next cursor leaves history incomplete, even for a short page.
Separate reads/pages are not an atomic snapshot.

| Observed outcome | Required evidence |
| --- | --- |
| `Succeeded` | Idle wait; complete bounded traversal containing the expected user ID and no competing input; completed, error-free assistant output; matching final idle marker and session outcome; no pending work. |
| `Failed` / `Interrupted` | Matching prompt history and corresponding terminal/session outcomes after an idle wait, without pending work. |
| `AwaitingInput` | Pending permission requests or forms; the example does not answer them. |
| `Pending` | The submitted prompt is still in the inbox, even if the session records an older success. |
| `Inconclusive` | Missing/conflicting evidence, incomplete history, unsupported/competing work, or failed/timed-out waits or inspection. |

This deliberately narrow policy assumes a fresh session used by this invocation.
It allows initial agent/model selection and recognized instruction-update notices;
other timeline kinds, concurrent work and multiple terminal markers require a
more specialized policy. Success describes observed completion, not answer
correctness or a per-prompt server receipt. Only stored assistant text is printed
on success; transient text fragments are progress counts, never appended to it.
An early idle response can still lead to `Inconclusive`; the example does not
silently poll or restart execution. Non-success outcomes exit unsuccessfully.

Every created session is retained, including on success. On errors/timeouts,
inspect the printed IDs before another invocation: server work may still be
running. If stopping it is intended, use the explicit interrupt/wait flow in
`examples/execution.rs`; cancel still-pending input through
`sessions().inbox(&id).cancel(&prompt_id)` separately. After inspecting settlement,
`sessions().remove(&id)` removes the session and its children. These actions do
not undo completed tool effects and are never automatic in this example.

[`tests/automation.rs`](tests/automation.rs) verifies the policy and wire flow
with deterministic fixtures. A separate active fixture verifies SDK execution
outcomes and waiting-tool resumption against OpenCode with a scripted localhost
provider; it does not run this example's complete policy. See
[compatibility notes](tools/live/README.md#active-execution).

## Permission requests and replies

```rust
use opencoders::{Client, models::{PermissionId, PermissionReply, SessionId,
    SessionPermissionReplyRequest}};

async fn answer(client: &Client, session: &SessionId, request: &PermissionId,
    decision: PermissionReply) -> opencoders::Result<()> {
    let permissions = client.sessions().permissions(session);
    let pending = permissions.get(request).await?;
    println!("{}: {:?}", pending.data.action, pending.data.resources);
    // The application supplies the decision, after presenting the request.
    permissions.reply(request, &SessionPermissionReplyRequest::new(decision)).await
}
```

`client.permissions().list()` discovers pending requests at the client's default
location; `list_at(Location)` overrides it for one call. The result retains
`location`, ordered `data`, and additional fields. `Location::Unspecified` omits
the query without removing caller location headers.

`client.sessions().permissions(&id)` exposes `list`, `get`, `create`, and `reply`.
These use the session's server location and ignore the SDK discovery default.
Lists are snapshots without pagination. Missing, resolved, or wrong-session
requests return typed `ApiError::PermissionNotFound`; rediscover pending state
when another client or reply may have resolved the request.

`create(&SessionPermissionCreateRequest::new(action, resources))` evaluates an
action and returns the full `{ data: { id, effect } }` envelope. HTTP 200 can mean
`allow`, `deny`, or `ask`; only `ask` creates a pending request. Creation does not
execute the named action or wait for approval. Pending requests are held in memory
by the location's permission service, not durable session history.

Replies accept HTTP 204. On the tested v2.2.5 server:

| Decision | Effect |
| --- | --- |
| `Once` | Resolves this request without saving rules. |
| `Reject` | Rejects this and all other pending requests in the same session; nonempty feedback applies to the batch. |
| `Always` | Resolves this request and, if its `save` patterns are nonempty, saves project allow rules and resolves other now-allowed requests at this location, including other sessions. With no patterns it saves nothing. |

Configured denies still take precedence over saved allows. A successful reply
can unblock server work; it does not report that work's outcome. Saved-rule
management and UI approval policy remain outside these methods.

Create `id`/`agent` and reply `message` use `RequestField<T>` to preserve omission,
null, and a value; the pinned server treats null like omission. Optional non-null
`save`, `metadata`, and `source` use `Option<T>`; explicit empty collections are
sent. `PermissionSourceInput::tool(message_id, id)` keeps strict input fields
separate from tolerant response sources. Permission IDs and reply enums reuse
the types in live permission events. Response extensions are retained; optional
response collections default to empty and omit empty values on serialization.

All calls use ordinary deadlines and bounded diagnostics. No automatic retries
are performed; creation/replies can take effect before a failure reaches the
caller, and supplying an ID is not an idempotency guarantee. Duplicate pending
IDs are rejected by the pinned implementation. Dropping a request future does
not undo server-side effects. Injected HTTP-client policies remain in effect.

Run `cargo run --example permissions` with `OPENCODE_BASE_URL`,
`OPENCODE_DIRECTORY`, and optional `OPENCODE_PASSWORD`. It creates a fresh session,
evaluates a demonstration action, inspects/rejects its own pending request, and
removes that session. It submits no prompt or tool call and saves no allow rules.
Live tests also cover saved rules and cross-session effects in disposable data.
The active fixture verifies `once` resuming a waiting read; other waiting-tool
reply modes, plugins, races and restart behavior remain unverified.

## Forms

```rust
use opencoders::{Client, models::{FormAnswer, FormId, SessionId, SessionFormReplyRequest}};

async fn answer(client: &Client, session: &SessionId, id: &FormId,
                answer: FormAnswer) -> opencoders::Result<()> {
    let forms = client.sessions().forms(session);
    let current = forms.get(id).await?;
    // Present current.data.fields and collect explicit answers in your application.
    forms.reply(id, &SessionFormReplyRequest::new(answer)).await
}
```

`client.forms().list/list_at` discovers pending forms by location and preserves
resolved location metadata. `client.sessions().forms(&id)` exposes `list`, `get`,
`create`, `reply`, and `cancel`; session location comes from the server. Discovery
is a snapshot and forms may settle between listing and replying.

`SessionFormCreateRequest::new(title, fields)` accepts a nonempty `FormFieldsInput`.
Construct fields with `FormStringFieldInput::new`, `FormNumberFieldInput::new`,
`FormIntegerFieldInput::new`, `FormBooleanFieldInput::new`,
`FormMultiselectFieldInput::new`, and `FormExternalFieldInput::new`. Public optional
fields expose labels, constraints, options, defaults, and conditions. Creation
returns a pending form immediately, without a model call or waiting for an answer.
Optional ID uses `RequestField`; null acts like omission on v2.2.5. Duplicate IDs
conflict, including IDs of retained completed forms. No mutation is retried.

Answers map field keys to `FormValue`: strings, JSON numbers, booleans, or string
lists. Numeric values retain JSON integer precision. Special-number wire strings
remain strings in answers/conditions; numeric field limits/defaults use `FormNumber`.
The server validates answers. In v2.2.5:

- Defaults are presentation data, not automatically submitted answers.
- Conditions refer to earlier fields and are combined with AND. An unanswered
  reference makes either comparison false; inactive fields cannot be answered.
- `hidden` does not bypass validation. External fields require an explicit `true`
  acknowledgement; this library never opens their URLs.
- Invalid answers leave the form pending. Successful replies may unblock waiting
  server work; they do not prove that work succeeded.

`get` returns `FormState::Pending`, `Answered`, or `Cancelled`. Lists contain only
pending forms. Forms live in location-service memory; completed forms are retained
for ten minutes in the pinned server, not as durable history. Missing/expired or
wrong-session forms produce `ApiError::FormNotFound`; later replies/cancellations
produce `FormAlreadySettled`, and invalid answers produce `FormInvalidAnswer`.

Cancel with `SessionFormCancelOptions::default()` or `.message("reason")`. The
message goes in a bodyless DELETE query; absent and empty remain distinct. Feedback
is present in cancelled state but absent from the v2.2.5 cancellation event. HTTP
responses and form events share IDs, field models, and answer values. Refresh
snapshots after stream interruption; the library owns no UI or application state.
The active fixture verifies that answering a waiting `question` tool resumes it.
Cancelling with feedback instead produces a tool error that the provider can
respond to; execution can still succeed. Dismissal without feedback is a distinct,
source-reviewed path that has not been exercised live.

Run `cargo run --example forms` with `OPENCODE_BASE_URL`, `OPENCODE_DIRECTORY`, and
optional `OPENCODE_PASSWORD`. It creates a fresh session, answers and cancels only
its own manual fixtures, then removes the session. It neither renders a UI nor
starts model work. See [live compatibility notes](tools/live/README.md).

## Live events

```rust
use opencoders::{Client, EventOptions, models::Event};

async fn observe(client: &Client) -> opencoders::Result<()> {
    let mut events = client.events().subscribe(&EventOptions::default()
        .types(["session.text.delta", "session.renamed"])).await?;
    while let Some(message) = events.next().await {
        match message?.event {
            Event::SessionTextDelta(event) => println!("{}", event.data.delta),
            Event::SessionRenamed(event) => println!("{}", event.data.title),
            Event::Unknown(raw) => { /* Preserve or inspect future event kinds. */ }
            _ => {}
        }
    }
    // EOF means observation stopped. Refresh authoritative state before reopening.
    Ok(())
}
```

`events().subscribe` opens `GET /api/event`, spanning all server locations and
ignoring the SDK's default directory. Filters send repeated `types` parameters;
an empty list requests everything. The initial `server.connected` marker arrives
even with filters. No model calls are needed to subscribe. Run the read-only
`cargo run --example events` with `OPENCODE_BASE_URL`, optional
`OPENCODE_PASSWORD`, and optional `OPENCODE_EVENT_TYPE`; stop it with Ctrl-C.

`models::Event` covers 62 kinds across live feeds and durable logs: connection/RPC, session lifecycle and selections,
inbox, execution, steps, text/reasoning, tools, retries, compaction, permissions,
forms, and durable history changes. The exact set is in [event-roots.json](tools/model-spike/event-roots.json).
Envelope/payload types live in `models::events`. Other kinds, including currently
unimplemented native events, retain their complete JSON in `Event::Unknown`.
Malformed implemented kinds produce `Error::Decode`. Nested closed enums remain
strict. IDs, timestamps, durable metadata, locations, and object extensions are
retained. Optional nulls generally normalize to omission; constrained optional
integers and nonempty collections reject null.

`EventMessage` separates JSON payloads from SSE metadata (`id`, `event_name`,
`retry`). An SSE ID is not a durable sequence or JSON event ID. Retry hints are
informational. The pinned server sends data-only frames and heartbeat comments;
this API does not synthesize `Last-Event-ID`, replay, or reconnect automatically.

Each `EventStream` owns an independent connection and implements `Stream` plus
an inherent async `next()`. No background task or event queue is created. Dropping
or calling `close()` releases the subscription; it does not interrupt server work.
Cancelling a pending `next()` retains parser/read state. Clean EOF returns `None`;
a failure returns one error then `None`. A slow reader can overflow the server's
queue and lose its connection. Recover by reopening and refreshing authoritative
snapshots; application snapshot ordering/deduplication remains separate work.
Experimental durable log replay is available subject to server retention (below).

The handshake deadline defaults to 30 seconds, including bounded HTTP error-body
reads. Body inactivity timeout is opt-in; heartbeat bytes count as activity.
Ordinary `request_timeout` and `max_response_bytes` do not apply. Injected reqwest
deadlines still apply. `max_event_bytes` defaults to 8 MiB per SSE block, including
comments/ignored fields and treating CRLF as one terminator. Invalid UTF-8 uses
replacement characters, which can expand strings by up to 3×; the HTTP stack's
current chunk allocation is outside this budget. Error diagnostics retain at most
`max_error_body_bytes` (8 KiB by default).

The decoder handles fragmented UTF-8, BOM, CR/LF/CRLF, multiline data, comments,
IDs, and retry hints. Unterminated events at EOF are discarded. A special
`effect/http-api/stream/failure` frame becomes `Error::Stream`, as do handshake/
idle timeouts, invalid content types, and oversized frames. Default error formatting
is redacted; event/model `Debug` and explicit diagnostic access can contain private
conversation data.

## Background session observation

[`examples/background_session.rs`](examples/background_session.rs) demonstrates
a background reader and minimal application-owned state without a UI framework.
With `OPENCODE_BASE_URL` and `OPENCODE_SESSION_ID` set (and optionally
`OPENCODE_PASSWORD`), run:

```sh
cargo run --example background_session
```

The read-only example runs for 30 seconds with at most three connection attempts.
It opens the live subscription before fetching session, recent-history, inbox,
permission and form snapshots. It refreshes those snapshots every second and
after every explicit reconnect. History is one page of at most 50 messages;
the response's opaque cursor remains available for caller-controlled pagination.
Each snapshot response is limited to 1 MiB. The views are separate reads, not an
atomic snapshot or a complete transcript.

The [example's state and task code](examples/support/session_observer.rs) shows
the integration policy:

- A reader task feeds a bounded queue of 32 events, with a 64 KiB SSE block limit.
  When the queue fills, the reader closes and reports lag through its task result,
  independently of the full queue. These limits exclude HTTP chunk allocation
  and decoded-object overhead; they are not a process-memory cap.
- Relevant events invalidate snapshots. Text deltas update only a separate
  256-character preview of the latest fragment, associated with its message ID.
  They are never appended to saved history. Previews may overlap, repeat or miss
  output; the example prints counts rather than conversation content.
- Events buffered before/during a refresh request another refresh instead of
  patching the new snapshot. This avoids duplicate text and stale overwrites.
  Unknown or unhandled transient kinds conservatively invalidate the view;
  unknown payloads are counted without being retained by the example.
- EOF, lag and errors leave the last successful snapshot visibly stale. Partial
  refresh results are not published. The caller explicitly reconnects, subscribes
  and refreshes again; no SSE ID, log watermark or retry hint supplies continuity.
- Shutdown cancels and joins the reader. Dropping the connection aborts it as a
  fallback, rather than detaching a task. Cancelling local observation does not
  interrupt server execution or reply to pending requests.

[`tests/background_session.rs`](tests/background_session.rs) uses controlled
HTTP/SSE fixtures to verify missed-change convergence, overlapping snapshots,
incremental previews, queue overflow, failed/cancelled refreshes and independent
shutdown. The [active recovery fixture](tools/live/README.md#controlled-live-recovery)
also runs the shared observer against v2.2.5 with forced SSE transport cuts,
missed text/form events and a delayed old history response. Explicit reconnect
and refresh converge after execution settles. Missing text deltas cannot be
recovered from history until the text block ends; previews stay separate.
Concurrent reads are still not atomic. Server overflow, restart recovery and
the example CLI's timer/backoff loop remain untested live.
The SDK itself still creates no tasks, queues or automatic reconnect loop.

## Experimental durable session logs

```rust
use opencoders::{Client, SessionLogOptions, models::{SessionId, SessionLogItem}};

async fn replay(client: &Client, id: &SessionId, saved: Option<u64>) -> opencoders::Result<()> {
    let mut options = SessionLogOptions::default().follow(false);
    options.after = saved;
    let mut log = client.sessions().experimental_log(id, &options).await?;
    while let Some(message) = log.next().await {
        match message?.item {
            SessionLogItem::Event(event) => println!("{}", event.kind()),
            SessionLogItem::Synced(marker) => println!("Replay boundary: {:?}", marker.seq),
            _ => { /* Handle unknown items before acknowledging a later watermark. */ }
        }
    }
    Ok(())
}
```

`experimental_log` opens `GET /api/experimental/session/{sessionID}/log`, ignoring
the discovery location default. `after` is an exclusive aggregate sequence:
omission includes sequence zero; `after(0)` excludes it. The server validates
its nonnegative safe-integer range. `follow` defaults to omission (false on
v2.2.5); `follow(true)` reads retained history, emits one captured watermark, then
tails retained events. Both modes return an independent `SessionLogStream`.

**The v2.2.5 CLI does not enable event payload retention.** Its sequence counters
still advance, but log reads can return only a marker; following need not deliver
new payloads either. The SDK does not turn retention on. Refresh authoritative
session/message/inbox snapshots after reconnecting, and verify retention for the
server deployment before relying on replay. The pinned server library has an
`events.persist` option, but the CLI does not forward it. See the
[verified scope and upstream sources](tools/live/README.md).

`SessionLogItem` distinguishes all 47 reviewed durable session events from
`log.synced` markers and unknown JSON. It reuses `models::Event`, adding typed
instruction, synthetic, skill, shell, revert, usage, and message-content events.
Known transient events and malformed known log items fail decoding. Unknown kinds
retain their full JSON; `sequence()` and `aggregate_id()` do not infer fields from
them. Handle unknown data deliberately before advancing a checkpoint.

A marker describes the captured read watermark over retained rows. Its optional
sequence can be absent, greater than the last emitted event, or lower than a
requested future cursor. Gaps are valid: the server can skip non-session or unknown
stored event kinds, and payload retention may be disabled. Do not interpret the
marker as a coherent transaction with REST snapshots or evidence that all session
state can be reconstructed. EOF without a marker does not prove replay completion.

`SessionLogMessage` preserves SSE IDs, event names, and retry hints separately
from JSON event IDs and sequences. The tested v2.2.5 encoder omits SSE IDs for
these log payloads. The SDK neither converts IDs to `after` nor synthesizes
`Last-Event-ID`. `sequence()` is a convenience accessor, not automatic cursor
management. Acknowledge only successfully handled items; never replace an absent
watermark with zero or regress a saved cursor to an older marker.

Log streams share the live feed's bounded SSE parser, separate 30-second handshake
deadline, optional idle deadline, terminal errors/EOF, and close/drop behavior.
No automatic retries, reconnections, collection of the entire log, or background
tasks are added. The log need not emit heartbeats; an idle timeout is a caller
policy. Dropping the stream does not interrupt session execution.

Run `cargo run --example session_log` with `OPENCODE_BASE_URL`,
`OPENCODE_SESSION_ID`, and optional `OPENCODE_PASSWORD`. This read-only example
handles replay, saves an in-memory acknowledged sequence, explicitly closes and
reopens with following, and refreshes snapshots. Observation is limited to ten events and a ten-second read
window; snapshot requests use ordinary request deadlines. Deduplication, durable
checkpoint storage, and snapshot reconciliation remain application decisions. Transient text deltas are not replayable log events.

## Configuration and behavior

- Base URLs accept an optional reverse-proxy prefix: `https://host/opencode`
  resolves to `https://host/opencode/api/info`. The trailing slash is optional.
  Supply the server base, without an `/api` suffix. URL credentials, queries, and
  fragments are rejected; use headers for authentication.
- Client clones share the transport. The default transport has a 10-second
  connection timeout, a 30-second ordinary request deadline including body reads, and no
  redirects or automatic retries.
- `http_client` accepts a configured `reqwest::Client`. Its DNS, proxy, TLS,
  redirect, retry, and connection policies remain in effect. SDK headers and the
  SDK ordinary request timeout override transport defaults. Configure connection timeouts
  on the injected client; combining it with `connect_timeout` returns a build
  error. `without_request_timeout` removes only the SDK deadline.
- The default `rustls` feature enables HTTPS. `--no-default-features` supports
  HTTP or a TLS backend enabled through the supplied reqwest dependency.
- Success responses are limited to 8 MiB by default. HTTP/decode diagnostic
  prefixes are limited to 8 KiB; both limits are configurable. Error accessors
  indicate truncation. The limit applies to the accumulated body; the HTTP stack
  may deliver a larger individual chunk.
- Server-info models preserve unknown fields in `extra` maps, including nested
  paths and capabilities. Deserialization enforces the schema's nonnegative
  safe-integer range for `pid`. Optional null and absent response values become
  `None` and serialize as absent.
- Discovery models preserve additional object fields and typed agent/provider
  configuration. Agent `steps`, when present, must be a positive safe integer;
  provider timeout values accept numbers or `false`. Unknown closed enum values
  remain decoding errors; model types and reasoning-field names are open strings.
  Model limits and cost-tier sizes enforce the signed safe-integer range in the
  schema. Generated optional fields generally accept null as absent; `steps` and
  optional model input limits reject explicit null. These are response-only models.

## Errors

`BuildError` covers invalid configuration. API calls return `Error::Encode`, `Error::Transport`,
`Error::Http`, `Error::Decode`, `Error::ResponseTooLarge`, or `Error::Stream`. Use `operation()`,
`status()`, and `is_timeout()` for diagnostics and control flow. JSON response
methods expect exactly HTTP 200; session update/remove/switch/wait, permission reply,
form reply/cancel, and inbox update/cancel methods expect HTTP 204 with no
JSON decoding. Unexpected redirects and other success statuses are HTTP errors
too. Encoding errors happen before any request is sent and have no status.
JSON-body operations set `Content-Type: application/json`, overriding defaults.

HTTP errors retain a bounded raw body and recognize `InvalidRequestError`,
`UnauthorizedError`, `LocationNotFoundError`, `ServiceUnavailableError`, and
`InvalidCursorError`, `SessionNotFoundError` (including missing parents), and
`MessageNotFoundError` (also used when a message belongs to another session), and
`AgentNotFoundError`, `PermissionNotFoundError`, `FormNotFoundError`,
`FormAlreadySettledError`, `FormInvalidAnswerError`, and `ConflictError`
(with an optional resource identifier).
Other JSON, including malformed known error shapes, is
available as `ApiError::Unrecognized`. Known payloads' additional fields remain
available in the raw body. Truncated or non-JSON bodies have no parsed API error.
Body-read failures are transport errors and retain any received HTTP status.

Default client/error `Debug` and error `Display` omit URLs, headers, response
bodies, and source messages. Explicit body access and error sources may contain
sensitive request or server-supplied text; inspect them deliberately. Dropping a request
future cancels local consumption and does not undo server-side work.

`ProviderRequest`, `ModelInfo`, and `ModelVariant` also redact their `Debug` output
because provider configuration can contain credentials. Their fields and serialization preserve the original
values; treat explicitly accessed or serialized response data as sensitive.

## Development and compatibility

```sh
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo doc --workspace --no-deps --locked
python3 tools/coverage/report.py --check
python3 -m unittest discover -s tools/coverage -p 'test_*.py'
```

Tests use local mock HTTP servers; they need permission to bind localhost ports
but do not require OpenCode, provider credentials, or model calls. The runnable
examples are built by the checks and contact a real server only when invoked.

The pinned schema and provenance live in [`openapi/`](openapi/). The implemented
operations also pass an opt-in smoke test against **OpenCode v2.2.5 on macOS
arm64**, using disposable data and no model calls. Run `python3 tools/live/run.py`;
see [compatibility details and CI](tools/live/README.md) for prerequisites, release
checksums, verified behavior, and limitations. Other endpoints remain unimplemented. Log payload replay/follow uses explicitly seeded
fixtures because the stock CLI disables retention. Run `python3 tools/live/active.py`
separately for active execution with a scripted localhost provider and disposable
tools: streaming, interruption, failure and permission/form resumption pass on the
same macOS baseline. It also verifies controlled disconnect/reconnect and snapshot
recovery using the background observer. It uses no real credentials or paid calls.
Remote providers, server overflow, restart recovery and Linux remain unverified.

The [API coverage inventory](API_COVERAGE.md) classifies all 169 pinned operations:
35 supported, 129 planned and 5 deferred. Each supported route links to its Rust
method, representative protocol tests and verification limits. See the
[milestone 2 readiness audit](RELEASE_READINESS.md) for remaining integration,
execution/recovery and release work. The inventory checks are offline and do not
run during ordinary crate builds.

The private [`tools/model-spike/`](tools/model-spike/README.md) workspace tool
contains the initial experiment and reviewed server-info/discovery/session/message/inbox/event/permission/form generation commands.
Models are checked in; normal builds do not generate code or fetch a schema.
Ordinary `cargo build` and `cargo test` select the root library; `--workspace`
also checks the tool.
