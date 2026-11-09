# opencoders design and implementation plan

Status: 35 pinned operations have public methods and protocol tests, covering
server/location/agent/model discovery, session lifecycle and forks, history,
prompt admission/inbox management, interruption/waiting, permissions/forms,
live events and experimental session logs. Background observation and automation
examples have deterministic tests. A separate localhost provider fixture verifies
active execution, waiting tools and controlled observer recovery on v2.2.5;
hosted smoke and active checks pass on macOS arm64 and Linux x86-64.
Version 0.1.0 was published on 2026-11-03 with a verified registry archive,
`v0.1.0` tag and successful docs.rs build, completing milestone 2. See
[API coverage](API_COVERAGE.md) for operation-level evidence and
[release readiness](RELEASE_READINESS.md) for the prioritized acceptance audit.
Broader APIs remain the implementation plan.

`opencoders` is an async Rust client for the OpenCode v2 HTTP API. Its initial
audience is developers building GUI, TUI, desktop, and other interactive OpenCode
clients. Automated workflows are a secondary audience and should be able to use
the same underlying API without UI-specific dependencies.

The README is the short installation/quick-start entry point. Put extensive
workflow documentation in [docs/](docs/README.md); this file records architecture,
contract decisions and the implementation plan.

## Goals

The first usable release should provide the APIs a consuming application needs
to support a complete interactive conversation:

1. Connect to an OpenCode server and select a working location.
2. Browse sessions and load a session's message history.
3. Observe live text, reasoning, tool activity, and execution status.
4. Submit or steer input, change the agent or model, and interrupt execution.
5. Display and answer permission requests and forms.
6. Recover a useful view after a connection failure.

The long-term goal is broad coverage of the documented v2 API. The implementation
should make endpoint coverage and compatibility visible as that coverage grows.

## Scope and boundaries

This project delivers a Rust library for UI developers. Building a GUI, TUI,
session browser, conversation interface, or terminal emulator is outside its
scope. Examples demonstrate library calls and event consumption without building
an interactive interface.

The crate owns HTTP transport, wire encoding, typed requests and responses,
protocol errors, pagination primitives, and streaming connections. Applications
own UI state, rendering, persistence, and decisions such as whether to approve a
permission request.

Initial support targets native Rust applications using Tokio. The library will
use the caller's runtime and will not create a runtime internally. Browser/WASM
support, a blocking API, local server discovery and process management, and a
reusable UI state store are possible later additions rather than initial release
requirements.

Framework-specific adapters should be separate from the core client. An automated
consumer should be able to create a session, submit input, and observe execution
without constructing a UI model or subscribing to unrelated events.

## Upstream contract

The OpenAPI schema reviewed on 2026-11-09 describes an
experimental HTTP API with 170 operations and 497 component schemas, using
OpenAPI 3.1.0. These counts describe that snapshot, not a permanent API size.
The [drift review](openapi/README.md#2026-11-09-drift-review) records changes from
the initial 2026-11-02 contract; the supported Rust models are unchanged.

References:

- [HTTP API reference](https://opencode.ai/v2/docs/api)
- [OpenAPI JSON](https://opencode.ai/v2/openapi.json)
- [Official JavaScript client](https://opencode.ai/v2/docs/build/client/)
- [Official Effect client](https://opencode.ai/v2/docs/build/client/effect/)
- [Server and connection documentation](https://opencode.ai/v2/docs/cli/web/)

The reviewed schema has SHA-256
`e54ea93eac5278b072769616478e94638c3ea3974b36ce5998b834aff0e3265f`.
Its `info.version` is `0.0.1`; that value should not be treated as the OpenCode
server release version.

Several contract details affect the client design:

- Session prompt submission durably admits an inbox item. Its response does not
  represent a completed assistant reply.
- The global event stream is live-only, spans server locations, and can fail when
  a consumer falls behind. Events during disconnection are missed.
- The experimental session log supports retained replay after an exclusive aggregate
  sequence and can follow retained events. The tested v2.2.5 CLI disables payload
  retention, so counters/markers can advance without replayable data. Its durable event set excludes
  transient text deltas, so it does not replace the live feed for incremental
  rendering.
- Many operations return data envelopes, sometimes with location information;
  paginated responses also carry cursors. Other operations return empty or binary
  bodies.
- Location query parameters require nested encoding such as
  `location[directory]`. Some schema entries omit the explicit encoding style.
- The schema's security declarations are empty despite the server's authentication
  requirements. WebSocket upgrade routes are described with JSON boolean success
  responses. Binary range and conditional-response behavior is partly documented
  in prose. These paths need verification beyond generic code generation.
- SSE endpoints declare a special `effect/http-api/stream/failure` event. This
  must be handled as a stream failure rather than an ordinary business event.

## Architecture

### Transport and client configuration

Use a reusable `reqwest::Client` with Tokio, Serde, and Rustls. The public client
should be cheap to clone and usable across background tasks. Clones share the
HTTP connection pool and immutable configuration.

A client builder should support:

- An explicit server base URL.
- Authentication and application headers.
- An optional default location, with per-request overrides on applicable routes.
- Connection and ordinary-request timeouts.
- An existing `reqwest::Client` for custom TLS, proxy, and transport configuration.

The initial implementation provides all of these configuration options.
It uses reqwest 0.13, supports Rust 1.88+, and enables HTTPS through the default
`rustls` feature. Base URLs preserve reverse-proxy path prefixes and reject URL
credentials, queries, and fragments. Basic/Bearer authentication and arbitrary headers
are explicit; the library does not discover local credentials.

Default connection and ordinary-request timeouts are 10 and 30 seconds,
respectively; the request deadline includes body reads. Supplying an HTTP client
preserves its connection, TLS, proxy, redirect, and retry policies. SDK headers
and its request deadline override supplied defaults. Connection timeout settings
must be configured on the supplied client; combining both settings is an error.
Removing the SDK deadline does not remove a supplied client's own deadlines.

Location defaults apply only where the route accepts a location. Session-bound
routes and server-wide event subscriptions must retain their upstream semantics.
Paths refer to the server's filesystem and should not require local filesystem
access or local path canonicalization.

`ClientBuilder::default_location(Location::directory(...))` selects a default for
location-aware routes. The discovery resources' `get_at`, `list_at`, and `default_at` methods
override it for one call without mutating shared configuration. Their argument
`Location::Unspecified` omits the query entirely, bypassing the SDK default while
retaining caller-supplied headers. The upstream client's documented fallback is
location headers followed by the server working directory. This does not promise
to clear location headers configured on an injected HTTP client.

The implemented discovery routes explicitly declare `deepObject`/`explode` in
the pinned schema. We encode a directory with the `location[directory]` key and
standard query percent encoding, retaining arbitrary server path syntax. Explicit
null query encoding is not exposed; no equivalence between omission, empty string,
and the literal string `null` is assumed. Routes with unspecified serialization
styles still need verification. The v2.2.5 smoke test verifies nested directory
queries, header fallback, and query precedence over the location header.

Authentication headers must work consistently for ordinary requests and streams.
The v2.2.5 baseline verifies Basic authentication with username `opencode`;
`ClientBuilder::basic_auth` provides it. Bearer remains useful for authenticating
proxies. Pairing/cookie helpers await their own verification. The empty OpenAPI
security section is not evidence that authentication is unnecessary.

Long-lived streams and operations that wait for execution need timeout behavior
separate from ordinary requests. Dropping a request or stream cancels local
consumption; it does not promise to undo a request already accepted by the server.
Interrupting an agent is an explicit API operation.
`SessionWaitOptions` supplies an independent optional total deadline for waits,
defaulting to none. A configured deadline overrides an injected reqwest total
timeout; omission leaves transport deadlines effective. Shared response handling
still bounds diagnostic bodies and preserves HTTP status on body-read failures.

The default transport disables redirects and automatic retries. In particular,
a transport error after prompt submission must not silently cause duplicate input. Any future retry
policy must consider the operation's actual idempotency contract.

### Public Rust API

Group operations by resource rather than exposing hundreds of methods directly
on `Client`. The implemented `client.server().info().await` returns typed server
information directly, matching the operation's unenveloped HTTP 200 body.
`client.location().get().await` also returns a direct response, with directory and
project information. `client.agents().list().await` returns an `AgentListResponse`
containing both the resolved `location` and the `data` list, without filtering
hidden agents. Location and agent discovery support per-call location overrides;
server information is server-wide and ignores the discovery location default.

`agents().get(&AgentId)` returns an `AgentResponse`; `models().list(&ModelListOptions)`
and `models().default()` return location/data envelopes. Missing agents have typed
errors. Agent IDs use one encoded path segment; empty IDs and `.`/`..` fail before
HTTP because URL normalization cannot preserve them as agent path segments.
Model filters use repeated `types` parameters, preserving caller values/order;
omitting the filter selects chat models on the pinned server. Model types and
reasoning-field names are open strings. Lists retain server order, and default
`data` is required but nullable. Discovery returns current snapshots; catalog
settlement and execution readiness are not inferred from them.

`sessions().switch_agent(&SessionId, &SessionSwitchAgentRequest)` and
`switch_model(&SessionId, &SessionSwitchModelRequest)` accept HTTP 204 without
decoding JSON, share the transport's no-retry policy, and ignore discovery defaults.
Requests require non-null selections and reuse strict `SessionModelInput`, using
public catalog IDs rather than provider-native IDs. The v2.2.5 live test confirms
selections are stored without availability validation or model execution, including
unknown IDs. Reselecting an agent adds a history marker; repeating the same model
reference does not. Omitted model variants are stored as `"default"`. Applications
own changes to existing sessions and reconciliation after ambiguous failures.

`client.sessions().create(&SessionCreateRequest)` and
`client.sessions().get(&SessionId)` return `SessionResponse`, retaining the `data`
envelope and additive fields. A default creation request sends `{}`. Create/get
ignore the discovery location default; creation location belongs in the JSON
body, and a concrete parent ID selects the parent's location. The live baseline
verifies parent precedence over a conflicting body location and server-cwd
fallback for omitted/null location without a parent.

Create and update fields use `RequestField<T>` with `Omitted`, `Null`, and `Value(T)`.
Constructors/fluent setters support ordinary values; public fields permit explicit
null and resetting to omission. Separate strict location/model/permission input
types prevent response extension maps from leaking into writes. Nested model
variant is optional but non-nullable; metadata retains arbitrary JSON. The wire
states are preserved without imposing a general server-side clearing policy.

JSON requests use an explicit content type and the shared bounded response/error
handling. Serialization failures are reported before sending. Dynamic session IDs
are appended as encoded path segments, preserving proxy prefixes and preventing
URL delimiters from changing the route. Creation is not retried by the default
transport: timeouts, disconnects, or unreadable responses may leave a successfully
created session. Callers must reconcile before retrying; a supplied ID alone does
not establish idempotency. Injected HTTP-client policies remain caller-controlled.

`client.sessions().fork(&SessionId, &SessionForkRequest)` uses the same
`SessionResponse` envelope and ordinary bounded transport, with explicit `{}` for
its default request. The reviewed `--sessions` generator guards the shared
create/get/fork envelope and optional nullable `before: SessionMessageId` field.
The handwritten setter preserves all three `RequestField` wire states.

On v2.2.5 omitted/null `before` selects through the newest projected message;
a supplied ID selects strictly before that message and must belong to the source.
An empty source yields typed `InvalidRequest { kind: Some("empty_session"), .. }`;
before-first is allowed and can yield empty history. `MessageNotFound` and
`SessionNotFound` cover missing boundaries and sources.

Forks are independent roots: `fork.sessionID` records origin, while `parentID` is
absent. Removing the source does not recursively remove forks. The source location
and current agent/model/metadata/permissions/instructions are inherited even at an
earlier history boundary. New message IDs and title/timestamps are assigned;
usage resets, and archived/execution/revert state and pending inbox work are not
copied. Compaction keep-ID references are remapped to copied messages. Applications
own any branch presentation and reconciliation; no UI state is introduced.

The server copies settled projections, excluding unfinished assistant, shell,
and compaction rows. It does not replay a retained event log to produce the fork.
No atomic snapshot or retry/idempotency guarantee is added by this client. Lost
responses may leave created sessions, and configured request deadlines apply to
large copies. Successful forking does not start provider execution.

Live fixtures verify default/null/explicit boundaries, before-first, ownership
errors, current configuration, new IDs, attachment readability, compaction remaps,
usage resets, parked inbox exclusion, independent deletion, and fork events.
The import API itself drops unfinished rows, so active-work filtering remains
source-reviewed; instruction inheritance, concurrent mutation, large batch copies,
and forked retained-log behavior remain separate compatibility work.

`client.sessions().update(&SessionId, &SessionUpdateRequest)` and
`client.sessions().remove(&SessionId)` return `()` for HTTP 204 without decoding
JSON. They share transport deadlines, bounded error handling, encoded ID paths,
and the default no-retry policy. Neither applies the discovery location default.
Updates support title, metadata, permissions, and archived state. On v2.2.5,
omitted/null fields are no-ops; metadata and permission values replace their
contents, including empty maps/lists. Archive `true`/`false` archives/restores.
An empty title requests generated titling and may invoke a model with conversation
history. The live test exercises it only on an empty session, where it is a no-op.

Updates are not promised atomic: failures can follow partial or complete changes.
Deletion recursively removes descendants and may have succeeded before a lost
response; repeated removal returns a missing-session error on v2.2.5. Application
code owns confirmation and reconciliation. The live test verifies a three-level
session tree is deleted while unrelated sessions remain; deletion during active
execution is not covered.

`client.sessions().list(&SessionListOptions)` retrieves one `SessionsResponse`.
Options are cloneable, default to omission, and have fluent setters plus public
fields for clearing values. `ParentFilter::RootOnly` encodes literal `parentID=null`;
`ChildrenOf(SessionId)` encodes a validated ID. Archive filters preserve omission
versus explicit false. Request limits use `u32` with server-side range validation.
The route uses direct directory/project/subpath filters and ignores the SDK's
default nested location. All supplied options are sent exactly once.

Session pages retain `data`, the cursor object, and additive metadata. Returned
order is preserved. Cursors stay opaque; either `next` or `previous` can be supplied
on a later call. Missing/null cursors mean `None`, while an empty string remains
present. The caller owns page progression and filter retention; the SDK neither
auto-fetches nor changes options when a cursor is set. Examples clear first-page
`order` while following cursors. Verified v2.2.5 cursors carry their original
filters/order and ignore valid conflicting filter/order parameters; `limit` still
controls page size. Clear the cursor to change filters. Nonempty pages return
boundary cursors even at the start/end; they can lead to empty pages. The live
test verifies forward/backward traversal and rejects a zero limit. Maximum limits
and consistency under concurrent session changes remain unverified.

`client.sessions().messages(&SessionId)` returns a lightweight borrowed resource.
Its `list(&MessageListOptions)` returns a `SessionMessagesResponse` data/cursor
page; `get(&SessionMessageId)` returns a `SessionMessageResponse` data envelope.
Both use HTTP 200, encoded path segments, existing transport policies, and no
discovery location default. `MessageListOptions` has optional limit, order, cursor,
and typed message-kind filters; all are sent as supplied. In v2.2.5, defaults are
50 newest messages. A nonempty cursor plus order is rejected; retain the same type
filter on later pages because it is not encoded in the cursor. Boundary cursors
can lead to empty pages. The SDK does not sort, auto-page, or interpret cursors.

Projected messages use the eleven-kind `SessionMessageInfo` enum, boxed payloads,
and common `id()`/`message_type()` accessors. Nested assistant content, tool states,
compaction states, and attachments are typed. These reads do not represent pending
inbox items or live events. Unknown kinds fail decoding, consistently with the
current model policy; live event streams have a separate unknown-event fallback. Applications own message presentation and state.

Use typed request structs with constructors or defaults for common cases. Provide
lightweight ID types where they prevent mixing session, message, and other IDs.
Preserve upstream terminology in documentation so users can map methods back to
the HTTP reference and operation IDs.

Responses should retain pagination cursors and meaningful location or response
metadata. Convenience access to a payload should not force callers to lose that
information. Start with explicit page requests; automatic pagination can be an
additional helper that preserves filters, ordering, and cursor rules.

Keep a supported path to arbitrary JSON for plugin RPC and genuinely dynamic
fields. Dynamic extension points should not cause otherwise known API models to
be represented entirely as `serde_json::Value`.

### Prompt admission and inbox lifecycle

`sessions().prompt(&SessionId, &SessionPromptRequest)` accepts HTTP 200 and returns
the `SessionPromptResponse` envelope with a typed admitted user item. This is
durable admission, not execution completion. Request text is required; nullable
optional ID/delivery/resume fields use `RequestField<T>`. Non-null optional
attachments/metadata use `Option<T>`, preserving omitted versus empty values.
Strict nested input models are separate from tolerant, prepared response payloads;
the SDK passes server-resolved URIs and does no local file I/O.

`sessions().inbox(&SessionId)` exposes `list`, `update`, and `cancel`. Listing keeps
the unpaginated envelope/enqueue order and dispatches user, synthetic, compaction,
and move items. Updates change only delivery mode; update/cancel accept HTTP 204.
All four methods encode IDs as individual path segments, ignore discovery defaults,
and reuse existing deadlines, limits, and the default no-retry policy.

The pinned server defaults omitted/null delivery to steer and wakes execution
unless resume is false. Queue alone does not suppress waking; resume(false) does
not pause existing execution. Inbox steering wakes execution; queue updates do
not. Same-mode and unavailable-item updates conflict. Unavailable cancellations
are no-ops, while a missing session fails. Repeating a pending user ID for the same
session returns the original payload; cross-session reuse conflicts. This is not
a blanket idempotency promise across lifecycle states. Smoke checks admit with
`resume(false)`; the active localhost fixture verifies fresh-session delivery
with `resume(true)`. Concurrent admissions, steering during existing execution
and ID reuse after delivery/cancellation remain separate validation work.

### Execution control

`sessions().interrupt(&id, &SessionInterruptOptions)` sends a bodyless POST with
an optional boolean `resume` query and returns the generated, extensible
`SessionInterruptResponse` for HTTP 200. Omitted/false resume requests no restart
on v2.2.5; true can wake eligible steering/control work even when nothing was
interrupted, while queued next-turn prompts remain parked. Interruption is an
acknowledgement before cleanup, not a settlement barrier. False also covers already
stopping/settling execution, so neither boolean establishes idle.

`sessions().experimental_wait(&id, &SessionWaitOptions)` sends a bodyless POST
and returns `()` only for HTTP 204. It observes process-local idleness without
starting work, rechecking after cleanup/successor executions. It does not propagate
execution outcomes and may return with pending input. New work can start afterward;
there is no per-prompt completion or permanent-idle guarantee. Snapshots and the
existing typed execution events remain the outcome observation primitives.

Both methods ignore discovery location defaults and use shared bounded response
handling. Interruption keeps ordinary request deadlines; waiting uses its per-call
deadline. Neither retries automatically. Dropping a wait cancels local observation,
not execution; dropping an interrupt cannot undo an accepted action. The example
requires an explicit `--interrupt` flag to request a stop before bounded waiting.

Live validation covers empty/idle sessions, parked queued work with resume true,
held steering work with omitted/false resume, and missing sessions, without model
calls. A separate opt-in active fixture verifies interruption of a held localhost
provider stream, connection cancellation, bounded waits and success/failure/
interrupted outcomes in events and history. Active tool interruption, concurrent
resumption and waits across successor runs remain unverified; mocks exercise both
acknowledgement booleans, deadlines and response/error contracts.

`examples/automation.rs` applies an example-owned, fresh-session policy: explicitly
select agent/model on creation, subscribe before one identified prompt, consume
progress alongside admission/waiting, then inspect session/history and pending
input/permission/form snapshots. It has a shared work deadline, a separate bounded
inspection window and event/history budgets. Admission, idleness, feed failure and
observed execution outcomes remain distinct. Success requires matching prompt and
terminal history, consistent session outcome, completed assistant output and no
pending work. Ambiguous, truncated or competing history stays inconclusive.
Recognized instruction-update notices are allowed without treating them as another
user prompt. The example retains sessions and never automatically retries,
interrupts, answers requests or deletes work. These are tested application choices,
not a public orchestration API or a live per-prompt completion guarantee.

### Permission requests and replies

`client.permissions().list/list_at` returns location-scoped pending discovery with
the resolved location/data envelope and server ordering. It follows the established
default/per-call/unspecified location rules. `sessions().permissions(&id)` exposes
`list`, `get`, `create`, and `reply`, resolving the session's location on the server
independently of SDK discovery defaults. IDs remain individual encoded segments.

Creation returns an immediate typed `allow`/`deny`/`ask` result, not a completed
action or approval wait. Only ask creates a pending request. Pending requests are
location-service memory; successful replies and snapshot lists are not durable
history. Live events retain shared permission IDs/reply/source types. Missing,
resolved, and wrong-session requests expose `ApiError::PermissionNotFound`.

On v2.2.5 once resolves one request; reject also rejects the session's pending batch
with nonempty feedback. Always with nonempty save patterns stores project allow
rules and re-evaluates pending requests at that location, including other sessions.
Without save patterns it saves nothing. Configured denies beat saved allows.
Applications choose decisions and refresh pending state after these broader effects;
the SDK imposes no policy and does not manage saved rules in this goal.

The eighth generation target (`--permissions`) extracts endpoint envelopes/inputs,
reuses reviewed scalar/event types, preserves response extensions, and separates
strict tool-source inputs from tolerant responses. Guarded adapters preserve three
states for create id/agent and reply message, while optional non-null save/metadata/
source distinguish omission from empty values and reject null. All methods share
ordinary deadlines, bounded errors, and the default no-retry policy. Errors can
follow applied effects; caller-supplied IDs are not an idempotency promise.

Disposable live tests cover immediate decisions, discovery location/header
precedence, session ownership, once/reject/always, saved-rule effects and deny
precedence, nullable inputs, and permission events without model/tool execution.
The example rejects its own manually created request in a fresh session. The active
fixture adds a built-in read waiting on `once`, then verifies file content reaches
the local provider and execution succeeds. Waiting-tool reject/always, plugin
hooks, concurrent decisions and reload/restart lifetime remain separate work.

### Forms and explicit answers

`client.forms().list/list_at` retains location-scoped pending discovery envelopes.
`sessions().forms(&id)` exposes pending lists, create/get/reply/cancel; the server
resolves session ownership/location. Cancellation is a bodyless DELETE with an
optional `message` query, preserving omission and empty strings. Reply/cancel
accept HTTP 204. All calls share ordinary deadlines, bounded diagnostics, encoded
ID segments, and no automatic mutation retries.

The ninth generation target (`--forms`) extracts endpoint envelopes, strict
creation/reply/field inputs, and explicit state dispatch. Response IDs, fields,
metadata, and answers reuse event models only after guarded schema-equivalence
checks. A scalar-union normalization removes a redundant special-number string
branch that caused Typify to flatten answer/condition scalars into unusable objects;
regressions cover primitive/list values and JSON integer precision. Numeric input
limits/defaults share `FormNumber`; count constraints use checked integers, and
`FormFieldsInput` enforces nonempty lists. Nullable create ID uses `RequestField`;
other optional input fields preserve omission/empty/false and reject null.

Applications own form rendering, default selection, condition display, external
navigation, and explicit answer collection. The server owns semantic validation;
the SDK does not duplicate its regex, condition, or option validator. Conditions
are ANDed and reference earlier fields; inactive fields cannot be answered.
Defaults are not answers, hidden fields still validate, and external fields require
true acknowledgement. Invalid answers leave forms pending.

The v2.2.5 service stores forms in location-scoped memory. Pending forms appear in
lists; answered/cancelled states remain retrievable for ten minutes. Caller IDs
are not idempotency keys, including while terminal entries remain cached. Typed
errors cover missing/wrong-session forms, settled forms, and invalid answers.
Replies may unblock server work; transport failures can follow applied effects.
Events share form data, but cancellation feedback requires fetching state.

Disposable live validation exercises all field kinds, conditions/defaults/hidden
semantics, ownership/location isolation, null IDs, duplicate IDs, errors, terminal
states, and creation/reply/cancellation events without model calls. The active
fixture verifies answering a waiting question and cancellation with model-facing
feedback; the latter records a tool error but permits successful execution.
Ten-minute expiry, restart/teardown lifetime, concurrent settlement and dismissal
without feedback remain source-reviewed or future validation work.

### Generated models and hand-written behavior

Use a hybrid approach: generate wire models from a pinned schema and hand-write
the ergonomic client methods, transport behavior, and stream handling. The
[initial generation experiment](tools/model-spike/README.md) selects Typify 0.8.0
with an explicit non-null JSON override: all 497 component schemas generate and
compile, and a smaller checked-in set has behavioral tests. The raw experimental
models remain private to that tool.

The tool's `--server-info` mode now generates the public `ServerInfo` response
models into `src/models/generated.rs`. Reviewed adaptations preserve additive
fields in flattened `extra` maps at every object level, mark response structs
non-exhaustive, and enforce the schema's nonnegative safe-integer PID range.
A schema guard requires review if that range changes. Absent and null optional
fields both decode to `None` for these response-only types; this does not define
request-field semantics. `--server-info --check` verifies
checked-in output without modifying it.

`--discovery` generates `src/models/discovery.rs`, extracting the inline
agent list/get and model list/default envelopes and their transitive models. It uses the same additive-field
policy and validates positive safe-integer agent step limits. A guarded schema
simplification represents `"auto" | string` cache intervals as strings; a guarded
named replacement represents provider timeouts as a JSON number or `false`.
`serde_json::Number` preserves integer values without conversion through `f64`.
Cache TTL patterns use the generator's `regress` validator. Provider request
`Debug` redacts configuration, as do model and model-variant responses, while explicit
field access and serialization retain values. Unknown closed enum variants produce
decode errors. Guarded open-string simplifications retain arbitrary model types and
reasoning-field names. Model limits and cost-tier sizes use signed safe integers;
optional input limits reject explicit null. A guarded adapter preserves required,
nullable default-model data and serializes `None` as JSON null.

`--sessions` generates the session response/input graph into `src/models/sessions.rs`.
Named replacements reuse the existing location, project, agent, model, and
permission types. Numeric values use `serde_json::Number`, retaining supported
integers rather than passing them through floating point. Generated ID parsers
enforce the schema's session/message prefixes. Guarded adapters validate
nonnegative safe-integer file-diff counts and the optional true-only `more` flag.
The same additive-field and unknown-enum policies apply; optional response fields
generally collapse absent/null except strict adapters such as `more`.

The same command extracts create/get response envelopes and create/update/selection requests
directly from their inline endpoint schemas. Matching response envelopes share
`SessionResponse`; strict request definitions are kept apart from response
tolerance adaptations. Guarded AST transformations replace the eight create and
four update nullable optional fields with `RequestField<T>` and reject explicit null for the
nested non-nullable model variant. Schema shape changes require adapter review.
Selection bodies are separately guarded strict objects with required, non-null fields.

`--messages` generates a separate response graph in `src/models/messages.rs`,
reusing shared IDs, location/model types, costs, and token counts. It extracts the
single-message envelope and type filter from inline schemas. A named replacement
avoids Typify's flattened top-level union: `SessionMessageInfo` and nested tagged
unions use explicit decoding in `src/models/message_adapters.rs`. Guards verify
union members and discriminators. Unknown or malformed known tags remain errors.
The larger tool-content payload is boxed, with its generated conversion adapted.

Message responses preserve extension fields and numeric precision. Guarded
adapters enforce safe-integer shell output counts/retry delays, positive retry
attempts, HTTP status bounds, and nonempty tool content when present. Constrained
optional scalar/content fields reject explicit null; other generated optional
fields follow the existing response policy. These response types are not mutation
inputs. Tests cover every message kind and nested tool/compaction states.

`--inbox` generates `src/models/inbox.rs` for inline prompt/update requests and
prompt/list envelopes. Shared IDs, locations, and prepared attachment response
types reuse existing models. Guarded adapters preserve nullable request fields,
omitted versus empty collections, and non-null optional nested inputs. The four-kind
`SessionInboxInfo` uses explicit tag dispatch; `NonNullValue` enforces the opaque
compaction payload constraint. A strict source decoder corrects Typify's permissive
adjacent-tag input decoding. Responses retain added fields and numeric precision.
`--events`, `--permissions` and `--forms` generate the remaining implemented
conversation models. Event dispatch preserves unknown kinds while rejecting
malformed known payloads; permission and form inputs retain their reviewed
validation and omission/null rules. See the [generation guide](tools/model-spike/README.md)
for each slice's adapters and schema guards. CI checks all nine targets: the raw
probe and eight production slices. Normal builds never run generation.

Production request models use three-state fields where the reviewed schema allows
omission, null and a value; non-nullable inputs retain their stricter contracts.
Response tolerance, full event/error coverage, numeric constraints and inline
schemas for additional endpoints still require review. Compilation alone does not
establish wire compatibility or complete schema validation.

The generation evaluation must cover tagged unions, nested references, optional
and nullable fields, recursive types, and JSON encoded inside SSE data. Many
upstream unions use `anyOf` without an OpenAPI discriminator even though their
variants contain identifying fields. Prefer explicit tag-based decoding when the
wire contract permits it.

Keep generated Rust in the repository. Regeneration should be an explicit
maintainer command with a pinned toolchain and reproducible output. Ordinary
crate builds must not download the schema or run an external generator.

Keep schema adaptations and naming overrides separate and reviewable; do not
rely on manual edits to generated output. If generation of routine endpoint
methods later becomes worthwhile, introduce it without coupling the public API
to a generator's default naming or transport abstractions.

### Compatibility and errors

Model omitted and explicit `null` independently where they have different
request semantics. Do not assume a plain `Option<T>` expresses every update.

Response decoding should tolerate additive fields. Preserve extension data where
callers need it, and provide a raw fallback for unknown event variants. A
malformed payload for a known event must still produce a decoding error rather
than being silently reclassified as an unknown event.

Expose distinct errors for transport, HTTP/API, response decoding, and stream
protocol failures. Preserve status, operation context, structured server error
details, and bounded diagnostic response content where useful. Unknown API errors
and non-JSON error bodies must remain inspectable. Authentication secrets must
not appear in default debug or tracing output.

The client implements `Encode`, `Transport`, `Http`, `Decode`, `ResponseTooLarge`
and `Stream` errors plus separate client-build failures. JSON response routes
accept HTTP 200. Session update/remove/switch/wait, inbox update/cancel,
permission reply and form reply/cancel accept only HTTP 204 without JSON decoding.
Known validation/authentication, missing-resource, conflict, form-settlement,
invalid-form-answer, unavailable-service and invalid-cursor errors expose typed details;
unknown or malformed error shapes retain their JSON and raw bytes. A body-read
failure remains a transport error and retains the received status when available.
Encode failures occur before a request is sent and have no HTTP status.

Success bodies are capped at 8 MiB and retained diagnostic prefixes at 8 KiB by
default, with configurable limits and explicit truncation. These bound collected
bodies; the HTTP stack can allocate an individual chunk before the limit check.
Default error formatting omits URLs, bodies, and sensitive source text. Explicit
body/detail accessors and error sources may contain sensitive server data.

Document compatibility against tested OpenCode server releases and the pinned
schema. Experimental route names should remain discoverable in the Rust API and
documentation. Do not gate every experimental route behind a Cargo feature:
some are needed for the core session experience.

## Streaming and interactive state

`client.events().subscribe(&EventOptions)` implements global, live-only
`event.subscribe`. Repeated type filters preserve caller values; an empty list
requests all events. Location defaults do not scope this route. Each call owns
an independent connection and returns `EventStream`, implementing
`Stream<Item = Result<EventMessage>>` and an inherent async `next()`.

Generated `models::events` envelopes cover the 62 reviewed live/log kinds listed in
`tools/model-spike/event-roots.json`; `models::Event` dispatches tags explicitly.
Unimplemented/future kinds retain raw JSON. Malformed implemented kinds fail.
Nested inbox, status, and form unions use explicit dispatch; checked wrappers
preserve safe integers, durable versions, and nonempty collections. SSE ID, event
name, and retry hints remain separate from the JSON event and durable metadata.

Subscriptions are caller-driven, with no background task, queue, reconnect, or
replay. Closing/dropping releases the connection without interrupting execution;
cancelling one pending read preserves partial decoder state. EOF yields `None`;
errors yield once then terminate. Cloning a client shares the HTTP pool. A UI can
own a background consumer and distribute updates itself.

`examples/background_session.rs` demonstrates that application boundary with a
bounded reader task and a separate example-only state module. It refreshes a
session, one recent history page and pending inbox/permission/form snapshots;
typed text fragments remain bounded transient previews, never history patches.
Buffered events invalidate refreshed state rather than being reapplied over it.
Queue overflow, EOF and failures mark the last successful bundle stale; explicit
reconnection subscribes before refreshing again. Shutdown joins the reader and
drop aborts it as a fallback. Deterministic tests establish convergence after
quiescence and cancellation/lag behavior. The active fixture now runs the shared
observer against forced SSE transport cuts, missed text and form changes, and an
old history response delayed across settlement. Explicit reconnect plus bounded
refresh converges once the controlled execution stops changing. This does not
establish arbitrary concurrent ordering, an atomic snapshot boundary, server
overflow behavior or restart recovery.
The active fixture verifies text deltas and confirms that v2.2.5 history stores
the text block on `text.ended`, not each transient delta. In-progress previews must
remain separate from stored snapshots. No state-store or reconnect policy is
added to the public SDK.

SSE framing handles arbitrary chunks, UTF-8 replacement, BOM, CR/LF/CRLF,
multiline data, comments, and metadata. EOF discards unfinished events. Blocks
are bounded (8 MiB default, including ignored fields, counting CRLF once); decoded
replacement characters may expand up to 3× and HTTP chunk allocations remain
outside that limit. Diagnostics use the client's bounded prefix limit. A server
failure frame is terminal, with redacted default diagnostics.

The handshake timeout defaults to 30 seconds and covers headers/error-body reads.
Body inactivity timeout is opt-in and resets on bytes, including heartbeats.
Ordinary request deadlines/response limits do not apply; injected HTTP-client
policies still apply. No automatic retry or `Last-Event-ID` is synthesized.
The server's bounded feed can overflow for slow consumers; no continuity is
promised, and overflow-specific signaling is not yet live-tested.

`sessions().experimental_log(id, options)` implements the experimental durable
session endpoint with typed `SessionLogItem::{Event, Synced, Unknown}` and
`SessionLogMessage` SSE metadata. All 47 pinned durable session kinds reuse the
shared `Event` model; known transient kinds and malformed known payloads fail.
Sequence/aggregate accessors expose reviewed data without inferring unknown
cursors. Missing marker sequences remain absent; they are never converted to zero.

`SessionLogOptions` omits `after`/`follow` by default. `after` is exclusive and
server-validated; omitted follow is finite replay on v2.2.5. Following subscribes
before the pinned implementation captures its watermark, reads retained rows in
sequence order, emits one marker, then uses durable notifications to read further
rows. Other/unknown stored kinds can be skipped, so sequence gaps are valid and
watermarks can exceed emitted session events. A future cursor can exceed the
captured watermark; client code must not move its checkpoint backwards.

Both resources share private `SseStream<T>` and the same handshake path, bounds,
error handling, and ownership behavior. Public stream types retain their existing
resource-specific fields. Stream errors now retain the originating operation ID.
No replay validation, automatic checkpointing, deduplication, or reconnection is
hidden in transport. Applications acknowledge successful handling and explicitly
reopen; EOF alone does not establish that a sync marker was received.

The stock v2.2.5 CLI advances durable counters with payload persistence disabled.
Its live compatibility check observes marker-only replay/follow. A separate fixture
adapter inserts actual captured rename events into only the runner-owned temporary
SQLite database, without changing projections/counters, to verify retained replay,
exclusive boundaries, aggregate isolation, and following. These checks do not
establish retention for normal CLI deployments. Keep snapshot refresh mandatory in
examples; a watermark is not proof of reconstructible state or snapshot consistency.

Library usage examples and documentation should demonstrate a recovery lifecycle:

1. Establish live observation and load authoritative snapshots for the selected
   views, reconciling updates that arrive while snapshots are loading.
2. Treat events as snapshot invalidations and keep transient previews separate
   from stored history. Retain explicit durable-log checkpoints only after
   successfully handling the corresponding events.
3. On disconnection, expose that live state may be stale.
4. Re-establish observation and refresh affected snapshots; use durable session
   replay where applicable, without assuming it restores transient deltas or
   unrelated server resources.

The exact ordering, deduplication, and snapshot reconciliation rules must be
verified against upstream behavior during implementation. A connection marker or
log watermark should not be interpreted as stronger consistency than the server
guarantees.

Permissions, forms, tool activity, and execution outcomes are first-class data.
The library supplies requests and events; applications choose presentation and
approval policy. Automated users can implement their own policy using those same
primitives.

## Terminal and binary support

PTY management and optional WebSocket support should follow the core conversation
workflow early, given the intended GUI/TUI audience. Verify connection-ticket
handling, input/output framing, resize behavior, closure, and any cursor semantics
against the server or official client. The OpenAPI success response alone is
insufficient to implement the socket protocol.

Keep WebSocket dependencies behind a Cargo feature. Ordinary JSON and SSE usage
should not require them. Persistent PTYs and client-hosted MCP WebSockets can use
the transport foundation as their protocols are covered.

Filesystem and media APIs should expose streamed bytes and relevant headers and
status rather than requiring every response to be buffered. Range and conditional
file responses need explicit handling and tests.

## Implementation milestones

| Milestone | Deliverable | Completion criteria |
| --- | --- | --- |
| 1. Contract and foundation | Pinned schema, generation evaluation, initial models, client builder, transport, authentication configuration, and errors. | Generated output is reproducible; server info and representative location-scoped requests work against mocks and a pinned server; encoding and response handling are verified. |
| 2. Interactive conversation APIs | Session listing and management, message history, agents/models, prompts and steering, interruption, permissions/forms, global SSE, and durable session logs. | Runnable library examples and protocol tests exercise session listing, history, incremental output, permission/form replies, execution outcomes, and recovery after disconnection. |
| 3. Terminal and workspace experience | PTY management, optional WebSocket connections, filesystem access, and worktree/VCS APIs. | Examples and tests cover terminal input/output and lifecycle, streamed file reads, and the workspace operations needed by an interactive client. |
| 4. Broader coverage and release hardening | Remaining configuration, integration, credential, MCP, plugin/RPC, generation, and other documented operations. | An operation-ID coverage report identifies every supported or deferred route; feature combinations, compatibility documentation, and release checks pass. |

The first usable release targets milestone 2. Terminal support follows closely;
complete HTTP coverage is a longer-term objective. Each milestone should leave a
usable client rather than requiring all endpoints before any real consumer can
exercise the design.

The operation inventory is already maintained across all milestones; it is not
deferred until milestone 4. The [readiness audit](RELEASE_READINESS.md) distinguishes
implemented methods from integrated workflow evidence. A usable snapshot-refresh
recovery path is required for the stock CLI baseline; naturally retained log
replay remains a separate validation gap rather than an assumed server guarantee.

Provide small runnable Rust examples that list sessions, retrieve history,
submit input, consume events, and reply to permissions or forms. Use ordinary
output or assertions to demonstrate results; no GUI toolkit, TUI framework, or
screen rendering is required. Examples can show a background event consumer and
minimal application-owned state to explain integration and recovery. Add an
automation example using the same methods to ensure the interface remains useful
outside interactive applications.

## Validation and maintenance

Use a local mock server for deterministic protocol tests covering:

- Authentication, nested location queries, URL/path encoding, and request bodies.
- Omitted versus null fields, empty responses, envelopes, and cursor pagination.
- Structured errors, unknown errors, non-JSON errors, and malformed known payloads.
- SSE framing across arbitrary HTTP chunks, unknown events, failure frames,
  cancellation, and disconnects.
- Log replay boundaries and markers, without asserting that transient events are
  replayable.
- WebSocket and binary-response behavior when those features are implemented.

Use opt-in integration tests against a pinned OpenCode server. Keep tests that
require model credentials, spend money, or mutate a workspace separate from the
default test suite. Use disposable server data and workspaces for those tests.

GitHub Actions workflows cover formatting, Clippy, unit/protocol tests, docs and
examples, default/HTTP-only features, Rust 1.88 library builds, and all checked-in
generation targets on Linux. Most development testing occurs locally on macOS;
hosted macOS jobs are disabled to avoid runner cost and queue delays. Retain local
macOS release/dependency checks and reconsider hosted coverage if OS-specific
functionality grows. Regeneration pins Rust/rustfmt 1.99.1. A separate manual
workflow runs the disposable server smoke test and optionally the separate active
fixture on Linux; ordinary tests skip both live tests.

OpenCode v2.2.5 is the initial live baseline, with official archive SHA-256 hashes
and commit pinned in `tools/live/server.json`. Server info, location/agent/model discovery,
and session creation/retrieval/listing/update/removal/selection, message list/get,
prompt admission, inbox list/update/cancel, and idle interruption/waiting pass
on macOS arm64 and Linux x86-64 in the hosted smoke workflow. The history test
imports a fixed transcript through the pinned experimental import route solely
for fixtures, without scheduling model
or tool execution; history reads use the SDK. This does not expose import publicly.
Smoke prompt checks use an idle disposable session with explicit `resume(false)` and
only queue-mode inbox updates. They verify admission/reconciliation, prepared
inline attachments, order, conflicts, cancellation, and absent history before delivery.
The opt-in `tools/live/active.py` runner adds a scripted authenticated localhost
provider and built-in read/question tools in disposable state. It verifies active
output, interruption, outcomes, waiting-tool resumption and controlled observer
recovery through the SDK, without real credentials or paid calls. Its scoped
configuration denies other providers/tools; application-state isolation is not
an OS sandbox.
The active fixture also passes on both hosted platforms. See
[compatibility results](tools/live/README.md#hosted-validation).
The unfiltered observer exposed an instruction-update delta bug: the generated
value now uses a checked scalar enum for a hash or `"removed"`, guarded against
schema drift, instead of an incorrect flattened object. Authentication and cursor
semantics required clearer guidance. The original schema's source release remains
unknown, independently of this tested baseline. No release is downloaded during
normal builds/tests.

Record endpoint coverage by upstream operation ID in `openapi/coverage.json`,
including deliberate deferrals and protocol-specific implementations. Regenerate
`API_COVERAGE.md` with `python3 tools/coverage/report.py --write`; CI checks it with
`--check`. Supported status requires a public method and protocol assertions;
live scope, source-reviewed behavior and unverified cases remain separate.
See [inventory maintenance](tools/coverage/README.md). Review schema updates as explicit
changes: inspect the diff, regenerate, adjust adapters, and run compatibility
tests. Do not automatically replace the schema during normal builds or CI checks.
A separate weekly/manual [drift workflow](.github/workflows/schema.yml) downloads
upstream JSON and compares its content, ignoring whitespace and object-key order.
It reports changes for review and never regenerates or edits the pin. Ordinary
CI tests the checker offline. See [schema maintenance](openapi/README.md#weekly-upstream-check).

## Decisions to resolve during implementation

- How should the production generation layer enforce response tolerance, numeric
  constraints, and complete event/error dispatch beyond the validated spike?
- What response and request-builder shapes provide convenient calls while
  retaining location metadata, pagination, and nullable update semantics?
- What snapshot/event reconciliation behavior is actually guaranteed upstream,
  and which recovery steps must remain application-specific?
- Which terminal operations belong in the earliest WebSocket feature release?

Resolve these through representative models, protocol tests, and library usage
examples before treating the public API as stable.
