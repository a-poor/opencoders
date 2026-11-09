# Permissions and forms

[Documentation index](README.md)

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
management is available through `client.permissions().saved()` below; UI approval
policy remains caller-owned.

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

## Saved project allow rules

```rust
use opencoders::{Client, SavedPermissionListOptions, RequestField,
    models::{PermissionSavedId, PermissionSavedUpdateRequest}};

async fn inspect(client: &Client) -> opencoders::Result<()> {
    let project = client.location().get().await?.project.id;
    let rules = client.permissions().saved()
        .list(&SavedPermissionListOptions::default().project(project)).await?;
    Ok(())
}

// The application supplies an explicitly selected rule and authorized pattern.
async fn change(client: &Client, id: &PermissionSavedId, pattern: &str)
    -> opencoders::Result<()>
{
    let mut patch = PermissionSavedUpdateRequest::default().resource(pattern);
    patch.action = RequestField::Null; // Leaves the action unchanged on v2.2.5.
    let updated = client.permissions().saved().update(id, &patch).await?;
    // Removing a chosen grant is a separate application decision:
    // client.permissions().saved().remove(id).await?;
    Ok(())
}
```

Saved permissions are durable **project allow rules** created by an `Always`
reply with save patterns. These APIs cannot create a rule, change its project,
or turn it into a deny. Lists and update results preserve `data` and additional
fields; timestamps retain JSON number precision. Update requires HTTP 200;
remove requires HTTP 204. Ordinary deadlines and bounded errors apply.

The list route exposes only `projectID` in the pinned schema. These methods
ignore the SDK's default location. Use a concrete discovered project ID for an
explicit scope. On v2.2.5, an omitted project selects the server-resolved location's
project (including caller location headers); it does **not** list every project.
An explicitly empty project string is sent unchanged, and the pinned server
returns an unfiltered database list. The library adds no all-project convenience.
Update/remove address a saved rule globally by its ID, with no project guard.
Empty IDs and URL dot segments fail locally rather than addressing a different
route; other strings are encoded as a single segment.

`PermissionSavedUpdateRequest::default()` sends `{}`. Both `action` and `resource`
allow omission, explicit null and strings; on v2.2.5 omission/null leave existing
values intact, while `""` is an actual replacement. Changing patterns can broaden
or narrow future grants. A duplicate project/action/resource combination returns
`ApiError::Conflict` (409); updating a missing ID returns
`ApiError::PermissionSavedNotFound` (404). Removing a missing ID succeeds on the
pinned server. No mutation retries, rolls back, or follows up automatically.

The server reads saved rules during future permission evaluations. Editing or
removing one does not settle already-pending requests, stop active execution, or
undo completed work. Configured denies still win. After a lost response, refresh
saved rules and reconcile before deciding whether to repeat a mutation.

Run `cargo run --example saved_permissions` with `OPENCODE_BASE_URL`, optional
`OPENCODE_DIRECTORY` and `OPENCODE_PASSWORD`. It discovers the location's concrete
project and lists its rules without saving, editing or deleting them.

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
starts model work. See [live compatibility notes](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md).
