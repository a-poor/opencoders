# opencoders design and implementation plan

Status: proposed design, before implementation.

`opencoders` is an async Rust client for the OpenCode v2 HTTP API. Its initial
audience is developers building GUI, TUI, desktop, and other interactive OpenCode
clients. Automated workflows are a secondary audience and should be able to use
the same underlying API without UI-specific dependencies.

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

The documentation and OpenAPI schema reviewed on 2026-11-02 describe an
experimental HTTP API with 169 operations and 490 component schemas, using
OpenAPI 3.1.0. These counts describe that snapshot, not a permanent API size.

References:

- [HTTP API reference](https://opencode.ai/v2/docs/api)
- [OpenAPI JSON](https://opencode.ai/v2/openapi.json)
- [Official JavaScript client](https://opencode.ai/v2/docs/build/client/)
- [Official Effect client](https://opencode.ai/v2/docs/build/client/effect/)
- [Server and connection documentation](https://opencode.ai/v2/docs/cli/web/)

The reviewed schema has SHA-256
`2fdcda59d6c37f5dac904ab7b7aff05fb98465e23eb0fe8d3b9767151ad391de`.
Its `info.version` is `0.0.1`; that value should not be treated as the OpenCode
server release version.

Several contract details affect the client design:

- Session prompt submission durably admits an inbox item. Its response does not
  represent a completed assistant reply.
- The global event stream is live-only, spans server locations, and can fail when
  a consumer falls behind. Events during disconnection are missed.
- The experimental session log supports replay after an exclusive aggregate
  sequence and can follow subsequent events. Its durable event set excludes
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

Location defaults apply only where the route accepts a location. Session-bound
routes and server-wide event subscriptions must retain their upstream semantics.
Paths refer to the server's filesystem and should not require local filesystem
access or local path canonicalization.

Authentication headers must work consistently for ordinary requests and streams.
Convenience authentication and pairing helpers should follow verified upstream
behavior; the empty OpenAPI security section is not evidence that authentication
is unnecessary.

Long-lived streams and operations that wait for execution need timeout behavior
separate from ordinary requests. Dropping a request or stream cancels local
consumption; it does not promise to undo a request already accepted by the server.
Interrupting an agent is an explicit API operation.

Automatic retries are disabled initially. In particular, a transport error after
prompt submission must not silently cause duplicate input. Any future retry
policy must consider the operation's actual idempotency contract.

### Public Rust API

Group operations by resource rather than exposing hundreds of methods directly
on `Client`. For example, `client.sessions().create(request).await` illustrates
the intended shape; exact names and signatures remain subject to the first
implementation pass.

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

### Generated models and hand-written behavior

Use a hybrid approach: generate wire models from a pinned schema and hand-write
the ergonomic client methods, transport behavior, and stream handling. Evaluate
generation on difficult representative schemas before committing to a tool.

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

Document compatibility against tested OpenCode server releases and the pinned
schema. Experimental route names should remain discoverable in the Rust API and
documentation. Do not gate every experimental route behind a Cargo feature:
some are needed for the core session experience.

## Streaming and interactive state

Expose asynchronous streams with items shaped as `Result<Event, Error>` and
equivalent types for session log items. Preserve SSE identifiers and relevant
log watermark information alongside decoded data where necessary for recovery.

For the first implementation, each explicit subscription owns its connection.
Cloning a client shares the HTTP pool, not an implicit event subscription. A UI
can consume one stream in a background task and distribute updates to its views.
A shared subscription facility can follow if consumer experience justifies it;
it would need explicit buffering, subscriber lifetime, and lag behavior.

Streaming must handle chunk boundaries, multiline SSE data, comments/heartbeats,
invalid payloads, clean EOF, and upstream failure events. Avoid unbounded internal
queues. A slow consumer and a lost connection must be visible to the application.
Do not reconnect silently and imply that the live feed is continuous.

Library usage examples and documentation should demonstrate a recovery lifecycle:

1. Establish live observation and load authoritative snapshots for the selected
   views, reconciling updates that arrive while snapshots are loading.
2. Apply typed events to application-owned state and retain useful cursors.
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

Add opt-in integration tests against a pinned OpenCode server. Keep tests that
require model credentials, spend money, or mutate a workspace separate from the
default test suite. Use disposable server data and workspaces for those tests.

CI should run formatting, Clippy, unit/protocol tests, documentation and example
checks, supported feature combinations, and a regeneration consistency check.
Declare and test a minimum supported Rust version once dependencies are chosen.

Record endpoint coverage by upstream operation ID, including deliberate
deferrals and protocol-specific implementations. Review schema updates as explicit
changes: inspect the diff, regenerate, adjust adapters, and run compatibility
tests. Do not automatically replace the schema during normal builds or CI checks.

## Decisions to resolve during implementation

- Which generator produces maintainable models from this OpenAPI 3.1 schema, and
  which transformations require a small maintained override layer?
- What response and request-builder shapes provide convenient calls while
  retaining location metadata, pagination, and nullable update semantics?
- Which OpenCode release and Rust version form the initial tested baseline?
- What snapshot/event reconciliation behavior is actually guaranteed upstream,
  and which recovery steps must remain application-specific?
- Which terminal operations belong in the earliest WebSocket feature release?

Resolve these through representative models, protocol tests, and library usage
examples before treating the public API as stable.
