# Input, execution and automation

[Documentation index](README.md)

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
idle and parked input. A separate [active fixture](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md#active-execution)
verifies interruption of a held localhost provider stream and wait settlement.
Concurrent wakeups, tool interruption and steering resumption remain unverified.

## Bounded automation

[`examples/automation.rs`](https://github.com/a-poor/opencoders/blob/main/examples/automation.rs) creates a fresh session with
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

The [example-owned policy](https://github.com/a-poor/opencoders/blob/main/examples/support/automation.rs) uses a shared 30-second
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

[`tests/automation.rs`](https://github.com/a-poor/opencoders/blob/main/tests/automation.rs) verifies the policy and wire flow
with deterministic fixtures. A separate active fixture verifies SDK execution
outcomes and waiting-tool resumption against OpenCode with a scripted localhost
provider; it does not run this example's complete policy. See
[compatibility notes](https://github.com/a-poor/opencoders/blob/main/tools/live/README.md#active-execution).

Prompt admission on v2.2.5 can commit a staged revert after preparation succeeds.
See [session workflows](workflows.md#revert-lifecycle) before combining prompts
with a pending revert boundary.
