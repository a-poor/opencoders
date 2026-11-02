//! Async Rust client for the OpenCode v2 HTTP API.
//!
//! Supports server information, location/agent discovery, and session creation, retrieval,
//! updates, deletion, paginated listing, and projected message history/retrieval. Run
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

mod agents;
mod client;
mod error;
mod location;
mod messages;
pub mod models;
mod request;
mod server;
mod sessions;

pub use agents::Agents;
pub use client::{Client, ClientBuilder};
pub use error::{
    ApiError, BuildError, DecodeError, EncodeError, Error, HttpError, ResponseTooLargeError,
    TransportError,
};
pub use location::{Location, Locations};
pub use messages::{MessageListOptions, MessageOrder, SessionMessages};
pub use request::RequestField;
pub use reqwest::{StatusCode, header};
pub use server::Server;
pub use sessions::{ParentFilter, SessionListOptions, SessionOrder, Sessions};

/// Result returned by API operations.
pub type Result<T> = std::result::Result<T, Error>;
