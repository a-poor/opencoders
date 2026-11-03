//! Async Rust client for the OpenCode v2 HTTP API.
//!
//! Supports server information, location/agent/model discovery, and session creation, retrieval,
//! updates, deletion, paginated listing, projected message history, prompt admission,
//! pending inbox management, interruption/waiting, permissions, forms, live event subscriptions, and experimental session logs. Run
//! requests on a caller-owned Tokio runtime. Clones share the HTTP connection pool.
//!
//! ```no_run
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = opencoders::Client::builder("http://localhost:4096")
//!     .basic_auth("opencode", "your-server-password")
//!     .build()?;
//! let info = client.server().info().await?;
//! println!("OpenCode {}", info.version);
//! # Ok(())
//! # }
//! ```

//! Create an empty session and retrieve it without submitting a prompt:
//! ```no_run
//! # async fn create(client: &opencoders::Client) -> opencoders::Result<()> {
//! use opencoders::models::{SessionCreateRequest, SessionLocationInput};
//! let request = SessionCreateRequest::default().title("New conversation")
//!     .location(SessionLocationInput::new("/remote/workspace"));
//! let created = client.sessions().create(&request).await?;
//! let retrieved = client.sessions().get(&created.data.id).await?;
//! // Creation may succeed even if its response is lost; do not retry blindly.
//! # Ok(())
//! # }
//! ```
//!
//! Fork an existing session's settled projected history:
//! ```no_run
//! # async fn fork(client: &opencoders::Client, source: &opencoders::models::SessionId) -> opencoders::Result<()> {
//! use opencoders::models::SessionForkRequest;
//! let fork = client.sessions().fork(source, &SessionForkRequest::default()).await?;
//! // Use .before(message_id) for an exclusive boundary. Empty sources fail.
//! // Fork provenance does not imply parent/child deletion ownership on v2.2.5.
//! // A lost response can leave a created fork; no automatic retry is performed.
//! # Ok(())
//! # }
//! ```
//!
//! Select a directory on the server, with optional per-call overrides:
//! ```no_run
//! # async fn discovery() -> Result<(), Box<dyn std::error::Error>> {
//! use opencoders::{Client, Location};
//! let client = Client::builder("http://localhost:4096")
//!     .default_location(Location::directory("/remote/workspace"))
//!     .build()?;
//! let location = client.location().get().await?;
//! let agents = client.agents().list().await?;
//! println!("{} agents in {}", agents.data.len(), agents.location.directory);
//! let other = client.agents().list_at(Location::directory("/remote/other")).await?;
//! // Omit the query and let server fallback rules (including caller headers) apply.
//! let fallback = client.location().get_at(Location::Unspecified).await?;
//! # Ok(())
//! # }
//! ```

//! Retrieve session pages with explicit filters and opaque cursors:
//! ```no_run
//! # async fn sessions() -> Result<(), Box<dyn std::error::Error>> {
//! use opencoders::{Client, ParentFilter, SessionListOptions, SessionOrder};
//! let client = Client::new("http://localhost:4096")?;
//! let mut options = SessionListOptions::default()
//!     .limit(20).order(SessionOrder::Desc).parent(ParentFilter::RootOnly);
//! let first = client.sessions().list(&options).await?;
//! if let Some(next) = first.cursor.next {
//!     options.cursor = Some(next);
//!     options.order = None;
//!     let second = client.sessions().list(&options).await?;
//!     // Use second.cursor.previous to request the preceding page.
//! }
//! # Ok(())
//! # }
//! ```

//! Update or remove a session, accepting the server's empty success response:
//! ```no_run
//! # async fn manage(client: &opencoders::Client, id: &opencoders::models::SessionId) -> opencoders::Result<()> {
//! use opencoders::models::SessionUpdateRequest;
//! client.sessions().update(id, &SessionUpdateRequest::default()
//!     .title("Renamed conversation").archived(true)).await?;
//! client.sessions().update(id, &SessionUpdateRequest::default().archived(false)).await?;
//! // Deletion also removes this session's descendants.
//! client.sessions().remove(id).await?;
//! # Ok(())
//! # }
//! ```

//! Read a session's projected message history without submitting input:
//! ```no_run
//! # async fn history(client: &opencoders::Client, id: &opencoders::models::SessionId) -> opencoders::Result<()> {
//! use opencoders::{MessageListOptions, MessageOrder, models::SessionMessageType};
//! let messages = client.sessions().messages(id);
//! let mut options = MessageListOptions::default().limit(20)
//!     .order(MessageOrder::Desc).message_type(SessionMessageType::User);
//! let first = messages.list(&options).await?;
//! if let Some(message) = first.data.first() {
//!     let one = messages.get(message.id()).await?;
//! }
//! if let Some(next) = first.cursor.next {
//!     options.cursor = Some(next);
//!     options.order = None; // Keep the type filter; do not combine cursor and order.
//!     let second = messages.list(&options).await?;
//! }
//! # Ok(())
//! # }
//! ```

//! Discover models and configure a session without submitting a prompt:
//! ```no_run
//! # async fn configure(client: &opencoders::Client, id: &opencoders::models::SessionId) -> opencoders::Result<()> {
//! use opencoders::{ModelListOptions, models::{SessionModelInput, SessionSwitchAgentRequest, SessionSwitchModelRequest}};
//! let catalog = client.models().list(&ModelListOptions::default().types(["chat"])).await?;
//! let agent = client.agents().get(&"build".to_owned().into()).await?;
//! if let Some(model) = client.models().default().await?.data {
//!     client.sessions().switch_agent(id, &SessionSwitchAgentRequest::new(agent.data.id)).await?;
//!     client.sessions().switch_model(id, &SessionSwitchModelRequest::new(
//!         SessionModelInput::new(model.provider_id, model.id),
//!     )).await?;
//! }
//! // A successful selection records a choice; it does not prove execution readiness.
//! # Ok(())
//! # }
//! ```

//! Admit pending input without waking execution, then inspect and cancel it:
//! ```no_run
//! # async fn pending(client: &opencoders::Client, id: &opencoders::models::SessionId) -> opencoders::Result<()> {
//! use opencoders::models::{SessionPromptRequest, SessionInboxDelivery, SessionInboxUpdateRequest};
//! let admitted = client.sessions().prompt(id, &SessionPromptRequest::new("Hello")
//!     .delivery(SessionInboxDelivery::Steer).resume(false)).await?;
//! let inbox = client.sessions().inbox(id);
//! let pending = inbox.list().await?;
//! inbox.update(&admitted.data.id, &SessionInboxUpdateRequest::new(SessionInboxDelivery::Queue)).await?;
//! inbox.cancel(&admitted.data.id).await?;
//! // Admission is not completion. resume(false) does not pause an already running loop.
//! # Ok(())
//! # }
//! ```

//! Request interruption, then separately wait for process-local execution to idle:
//! ```no_run
//! # async fn stop(client: &opencoders::Client, id: &opencoders::models::SessionId) -> opencoders::Result<()> {
//! use opencoders::{SessionInterruptOptions, SessionWaitOptions};
//! use std::time::Duration;
//! let response = client.sessions().interrupt(id,
//!     &SessionInterruptOptions::default().resume(false)).await?;
//! println!("Interruption accepted: {}", response.interrupted);
//! client.sessions().experimental_wait(id,
//!     &SessionWaitOptions::default().timeout(Duration::from_secs(30))).await?;
//! // Idle can follow failure/interruption and can coexist with pending input.
//! // Dropping the wait only cancels local waiting; it does not request a stop.
//! # Ok(())
//! # }
//! ```

//! Discover pending permissions and apply a caller-selected decision:
//! ```no_run
//! # async fn answer(client: &opencoders::Client, session: &opencoders::models::SessionId,
//! # request: &opencoders::models::PermissionId, decision: opencoders::models::PermissionReply) -> opencoders::Result<()> {
//! use opencoders::models::SessionPermissionReplyRequest;
//! let pending = client.permissions().list().await?;
//! let permissions = client.sessions().permissions(session);
//! let current = permissions.get(request).await?;
//! permissions.reply(request, &SessionPermissionReplyRequest::new(decision)).await?;
//! // Reject affects the session's pending batch; Always can save project rules
//! // and resolve matching requests in other sessions at the same location.
//! # Ok(())
//! # }
//! ```

//! Discover forms and submit caller-provided answers:
//! ```no_run
//! # async fn answer_form(client: &opencoders::Client, session: &opencoders::models::SessionId,
//! # id: &opencoders::models::FormId, answer: opencoders::models::FormAnswer) -> opencoders::Result<()> {
//! use opencoders::models::SessionFormReplyRequest;
//! let pending = client.forms().list().await?;
//! let forms = client.sessions().forms(session);
//! let current = forms.get(id).await?;
//! forms.reply(id, &SessionFormReplyRequest::new(answer)).await?;
//! // Applications render fields and collect answers; defaults are not submitted
//! // implicitly. A successful reply may unblock work but does not prove success.
//! # Ok(())
//! # }
//! ```

//! Consume a caller-owned, global live subscription:
//! ```no_run
//! # async fn events(client: &opencoders::Client) -> opencoders::Result<()> {
//! use opencoders::{EventOptions, models::Event};
//! let mut stream = client.events().subscribe(&EventOptions::default()
//!     .types(["session.text.delta", "session.renamed"])).await?;
//! while let Some(message) = stream.next().await {
//!     if let Event::SessionTextDelta(event) = message?.event {
//!         println!("{}", event.data.delta);
//!     }
//! }
//! // Reopen explicitly and refresh snapshots after observation stops.
//! # Ok(())
//! # }
//! ```

//! Read an experimental retained session log, with explicit resume/follow options:
//! ```no_run
//! # async fn log(client: &opencoders::Client, id: &opencoders::models::SessionId) -> opencoders::Result<()> {
//! use opencoders::{SessionLogOptions, models::SessionLogItem};
//! let mut stream = client.sessions().experimental_log(id,
//!     &SessionLogOptions::default().after(10).follow(false)).await?;
//! while let Some(message) = stream.next().await {
//!     if let SessionLogItem::Synced(marker) = message?.item {
//!         println!("Captured watermark: {:?}", marker.seq);
//!     }
//! }
//! // v2.2.5 CLI retention is disabled; always refresh authoritative snapshots.
//! # Ok(())
//! # }
//! ```

mod agents;
mod catalog;
mod client;
mod error;
mod events;
mod execution;
pub use execution::{SessionInterruptOptions, SessionWaitOptions};
mod inbox;
mod permissions;
pub use permissions::{Permissions, SessionPermissions};
mod forms;
pub use forms::{Forms, SessionFormCancelOptions, SessionForms};
mod session_log;
mod sse;
mod sse_stream;
pub use session_log::{SessionLogMessage, SessionLogOptions, SessionLogStream};
mod stream_error;
pub use events::{EventMessage, EventOptions, EventStream, Events};
pub use stream_error::{StreamError, StreamErrorKind};
mod location;
mod messages;
pub mod models;
mod request;
mod server;
mod sessions;

pub use agents::Agents;
pub use catalog::{ModelListOptions, Models};
pub use client::{Client, ClientBuilder};
pub use error::{
    ApiError, BuildError, DecodeError, EncodeError, Error, HttpError, ResponseTooLargeError,
    TransportError,
};
pub use inbox::SessionInbox;
pub use location::{Location, Locations};
pub use messages::{MessageListOptions, MessageOrder, SessionMessages};
pub use request::RequestField;
pub use reqwest::{StatusCode, header};
pub use server::Server;
pub use sessions::{ParentFilter, SessionListOptions, SessionOrder, Sessions};

/// Result returned by API operations.
pub type Result<T> = std::result::Result<T, Error>;
