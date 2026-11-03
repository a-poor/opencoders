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
starts model work. See [live compatibility notes](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md).
