//! Shared bounded SSE transport; public resources choose their payload models.
use crate::{
    DecodeError, Error, Result, StatusCode, StreamError, StreamErrorKind, TransportError,
    sse::Decoder,
};
use futures_core::{Stream, stream::FusedStream};
use serde::de::DeserializeOwned;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub(crate) struct Message<T> {
    pub data: T,
    pub id: Option<String>,
    pub event_name: String,
    pub retry: Option<Duration>,
}
type BoxedStream<T> = Pin<Box<dyn Stream<Item = Result<Message<T>>> + Send>>;

pub(crate) struct SseStream<T> {
    inner: Option<BoxedStream<T>>,
}
impl<T: DeserializeOwned + Send + 'static> SseStream<T> {
    pub fn new(
        response: reqwest::Response,
        operation: &'static str,
        idle_timeout: Option<Duration>,
        frame_limit: usize,
        diagnostic_limit: usize,
    ) -> Self {
        let state = State {
            response,
            decoder: Decoder::new(frame_limit),
            pending: bytes::Bytes::new(),
        };
        let inner = futures_util::stream::try_unfold(state, move |mut state| async move {
            loop {
                while !state.pending.is_empty() {
                    let b = state.pending[0];
                    use bytes::Buf;
                    state.pending.advance(1);
                    if let Some(frame) = state.decoder.push(b).map_err(|()| {
                        Error::Stream(
                            StreamError::new(
                                operation,
                                StreamErrorKind::FrameTooLarge,
                                Some(StatusCode::OK),
                            )
                            .with_limit(frame_limit),
                        )
                    })? {
                        if frame.event == "effect/http-api/stream/failure" {
                            return Err(Error::Stream(
                                StreamError::new(
                                    operation,
                                    StreamErrorKind::ServerFailure,
                                    Some(StatusCode::OK),
                                )
                                .with_body(frame.data.as_bytes(), diagnostic_limit),
                            ));
                        }
                        let data = serde_json::from_str(&frame.data).map_err(|source| {
                            Error::Decode(DecodeError::new(
                                operation,
                                StatusCode::OK,
                                frame.data.as_bytes(),
                                diagnostic_limit,
                                source,
                            ))
                        })?;
                        return Ok(Some((
                            Message {
                                data,
                                id: frame.id,
                                event_name: frame.event,
                                retry: frame.retry,
                            },
                            state,
                        )));
                    }
                }
                let chunk = if let Some(timeout) = idle_timeout {
                    tokio::time::timeout(timeout, state.response.chunk())
                        .await
                        .map_err(|_| {
                            Error::Stream(StreamError::new(
                                operation,
                                StreamErrorKind::IdleTimeout,
                                Some(StatusCode::OK),
                            ))
                        })?
                } else {
                    state.response.chunk().await
                };
                match chunk.map_err(|source| {
                    Error::Transport(TransportError::new(operation, Some(StatusCode::OK), source))
                })? {
                    Some(chunk) => state.pending = chunk,
                    None => return Ok(None),
                }
            }
        });
        Self {
            inner: Some(Box::pin(inner)),
        }
    }
}
impl<T> SseStream<T> {
    pub fn close(&mut self) {
        self.inner = None;
    }
}
impl<T> Stream for SseStream<T> {
    type Item = Result<Message<T>>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let Some(inner) = this.inner.as_mut() else {
            return Poll::Ready(None);
        };
        let result = inner.as_mut().poll_next(cx);
        if matches!(&result, Poll::Ready(None | Some(Err(_)))) {
            this.inner = None;
        }
        result
    }
}
impl<T> FusedStream for SseStream<T> {
    fn is_terminated(&self) -> bool {
        self.inner.is_none()
    }
}
struct State {
    response: reqwest::Response,
    decoder: Decoder,
    pending: bytes::Bytes,
}
