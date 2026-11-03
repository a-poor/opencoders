# Events and recovery

[Documentation index](README.md)

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
forms, and durable history changes. The exact set is in [event-roots.json](https://github.com/a-poor/opencoders/blob/main/tools/model-spike/event-roots.json).
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

[`examples/background_session.rs`](https://github.com/a-poor/opencoders/blob/main/examples/background_session.rs) demonstrates
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

The [example's state and task code](https://github.com/a-poor/opencoders/blob/main/examples/support/session_observer.rs) shows
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

[`tests/background_session.rs`](https://github.com/a-poor/opencoders/blob/main/tests/background_session.rs) uses controlled
HTTP/SSE fixtures to verify missed-change convergence, overlapping snapshots,
incremental previews, queue overflow, failed/cancelled refreshes and independent
shutdown. The [active recovery fixture](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md#controlled-live-recovery)
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
[verified scope and upstream sources](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md).

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
