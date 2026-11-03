use crate::{
    Result, Sessions,
    models::{SessionId, SessionLogItem},
    sse_stream::SseStream,
};
use futures_core::{Stream, stream::FusedStream};
use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub(crate) const OPERATION: &str = "experimental.session.log";

/// Options for the experimental durable session log. Defaults omit both queries.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SessionLogOptions {
    /// Exclusive aggregate sequence. Omission includes sequence zero; `Some(0)`
    /// excludes it. The server validates the safe-integer maximum (2^53 - 1).
    /// This is not a JSON event ID or an opaque SSE ID.
    pub after: Option<u64>,
    /// Whether to tail subsequent retained events. Omission is false on v2.2.5.
    /// Both modes emit a synchronization marker at the captured replay watermark.
    pub follow: Option<bool>,
    /// Deadline for headers/error-body reads; default 30 seconds.
    pub handshake_timeout: Option<Duration>,
    /// Body inactivity deadline, disabled by default. A pending read's deadline
    /// survives cancellation of `next()`. The log need not emit heartbeats.
    pub idle_timeout: Option<Duration>,
    /// Per-SSE-block byte budget, default 8 MiB. Includes ignored fields/comments,
    /// counts CRLF once, and excludes HTTP chunk allocations. UTF-8 replacement
    /// can expand decoded strings by up to three times.
    pub max_event_bytes: usize,
}
impl Default for SessionLogOptions {
    fn default() -> Self {
        Self {
            after: None,
            follow: None,
            handshake_timeout: Some(Duration::from_secs(30)),
            idle_timeout: None,
            max_event_bytes: 8 * 1024 * 1024,
        }
    }
}
impl SessionLogOptions {
    /// Resume strictly after this aggregate sequence; the server validates bounds.
    pub fn after(mut self, seq: u64) -> Self {
        self.after = Some(seq);
        self
    }
    /// Explicitly select finite replay or replay followed by tailing.
    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = Some(follow);
        self
    }
    /// Set the handshake deadline without limiting the body lifetime.
    pub fn handshake_timeout(mut self, timeout: Duration) -> Self {
        self.handshake_timeout = Some(timeout);
        self
    }
    /// Remove the SDK handshake deadline; injected transport policies still apply.
    pub fn without_handshake_timeout(mut self) -> Self {
        self.handshake_timeout = None;
        self
    }
    /// Set a deadline for each pending body read, including waits while following.
    pub fn idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = Some(timeout);
        self
    }
    /// Set the maximum buffered SSE block size.
    pub fn max_event_bytes(mut self, limit: usize) -> Self {
        self.max_event_bytes = limit;
        self
    }
}
impl Sessions<'_> {
    /// Read `experimental.session.log`, independently of the default location.
    ///
    /// Reads retained durable events after an exclusive sequence, emits a typed
    /// `log.synced` watermark, and optionally follows subsequent retained events.
    /// Sequences can have gaps; the watermark may exceed the last returned event
    /// or be below a requested future cursor. No cursor is updated automatically.
    /// SSE IDs are preserved as metadata, never used as `after` or Last-Event-ID.
    ///
    /// **Retention is server-dependent.** The v2.2.5 CLI does not enable event
    /// payload persistence: markers can advance without events being replayable.
    /// Logs exclude transient deltas and do not replace authoritative snapshots.
    /// The marker is a read boundary, not proof of complete reconstructible state.
    ///
    /// The ordinary request deadline/response limit do not apply. SDK handshake,
    /// idle, and per-block limits are independent; injected reqwest policies remain
    /// effective. Dropping/closing cancels local consumption, not server work.
    pub async fn experimental_log(
        &self,
        id: &SessionId,
        options: &SessionLogOptions,
    ) -> Result<SessionLogStream> {
        let mut query = Vec::new();
        if let Some(after) = options.after {
            query.push(("after", after.to_string()));
        }
        if let Some(follow) = options.follow {
            query.push(("follow", follow.to_string()));
        }
        let (response, limit) = self
            .client
            .open_sse_response(
                "api/experimental/session",
                &[id.as_str(), "log"],
                OPERATION,
                &query,
                options.handshake_timeout,
            )
            .await?;
        Ok(SessionLogStream {
            inner: SseStream::new(
                response,
                OPERATION,
                options.idle_timeout,
                options.max_event_bytes,
                limit,
            ),
        })
    }
}

/// Log data and separate SSE framing metadata. Debug includes server payloads.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SessionLogMessage {
    /// Durable event, synchronization marker, or complete unknown JSON.
    pub item: SessionLogItem,
    /// Last valid SSE ID, including an explicit empty reset. Never inferred from
    /// the event ID or sequence; do not assume it is a resume cursor.
    pub id: Option<String>,
    /// SSE event name, independent of the JSON discriminator.
    pub event_name: String,
    /// Most recent valid retry hint, retained without automatic reconnection.
    pub retry: Option<Duration>,
}

/// Independent caller-driven log stream. No buffering queue or background task.
///
/// Clean EOF yields `None`; an error yields once then terminates. EOF alone does
/// not prove the watermark was reached: inspect `SessionLogItem::Synced`.
/// Cancelling a pending `next()` retains decoder state. Drop/close releases the
/// connection. Reopen explicitly with an application-owned acknowledged sequence.
pub struct SessionLogStream {
    inner: SseStream<SessionLogItem>,
}
impl SessionLogStream {
    /// Read the next log item. Also implements [`Stream`] for combinators.
    pub async fn next(&mut self) -> Option<Result<SessionLogMessage>> {
        futures_util::StreamExt::next(self).await
    }
    /// Release the connection immediately. Subsequent reads return `None`.
    pub fn close(&mut self) {
        self.inner.close();
    }
}
impl Stream for SessionLogStream {
    type Item = Result<SessionLogMessage>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.get_mut().inner)
            .poll_next(cx)
            .map(|item| {
                item.map(|result| {
                    result.map(|m| SessionLogMessage {
                        item: m.data,
                        id: m.id,
                        event_name: m.event_name,
                        retry: m.retry,
                    })
                })
            })
    }
}
impl FusedStream for SessionLogStream {
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
impl fmt::Debug for SessionLogStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionLogStream")
            .field("closed", &self.is_terminated())
            .finish_non_exhaustive()
    }
}
