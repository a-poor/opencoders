# Client configuration and errors

[Documentation index](README.md)

## Authentication

OpenCode v2.2.5 uses HTTP Basic authentication with username `opencode` and the
server password. Use HTTPS when connecting to a remote server. `bearer_auth`
remains available for proxies that require it; the stock server does not accept
Bearer tokens. Pairing/cookie management is not implemented.

Custom authentication schemes and application headers can be supplied through
`ClientBuilder::default_headers`, using the re-exported `opencoders::header` types.
No credentials are discovered from local OpenCode files.

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
- Optional `websocket` enables PTY connections through the same HTTP client.
  PTY JSON management and file streaming require no WebSocket dependency.
- Success responses are limited to 8 MiB by default. HTTP/decode diagnostic
  prefixes are limited to 8 KiB; both limits are configurable. Error accessors
  indicate truncation. The limit applies to the accumulated body; the HTTP stack
  may deliver a larger individual chunk. File bodies and PTY sockets have separate
  per-call limits/deadlines; see [workspace reads](workspace.md) and [PTYs](terminals.md).
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
`Error::Http`, `Error::Decode`, `Error::ResponseTooLarge`, or `Error::Stream`;
the `websocket` feature adds `Error::PtySocket`. Use `operation()`,
`status()`, and `is_timeout()` for diagnostics and control flow. JSON response
methods expect exactly HTTP 200; session update/remove/switch/wait, permission reply,
form reply/cancel, PTY remove, and inbox update/cancel methods expect HTTP 204 with no
JSON decoding. Unexpected redirects and other success statuses are HTTP errors
too. Encoding errors happen before any request is sent and have no status.
JSON-body operations set `Content-Type: application/json`, overriding defaults.

HTTP errors retain a bounded raw body and recognize `InvalidRequestError`,
`UnauthorizedError`, `LocationNotFoundError`, `ServiceUnavailableError`, and
`InvalidCursorError`, `SessionNotFoundError` (including missing parents), and
`MessageNotFoundError` (also used when a message belongs to another session), and
`AgentNotFoundError`, `PermissionNotFoundError`, `FormNotFoundError`,
`FormAlreadySettledError`, `FormInvalidAnswerError`, `FileNotFoundError`,
`ProjectNotFoundError`, `PtyNotFoundError`, `ForbiddenError`, and `ConflictError`
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
