//! Async Rust client for the OpenCode v2 HTTP API.
//!
//! Supports server information, location resolution, and agent discovery. Run
//! requests on a caller-owned Tokio runtime. Clones share the HTTP connection pool.
//!
//! ```no_run
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = opencoders::Client::builder("http://localhost:4096")
//!     .bearer_auth("your-server-token")
//!     .build()?;
//! let info = client.server().info().await?;
//! println!("OpenCode {}", info.version);
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

mod agents;
mod client;
mod error;
mod location;
pub mod models;
mod server;

pub use agents::Agents;
pub use client::{Client, ClientBuilder};
pub use error::{
    ApiError, BuildError, DecodeError, Error, HttpError, ResponseTooLargeError, TransportError,
};
pub use location::{Location, Locations};
pub use reqwest::{StatusCode, header};
pub use server::Server;

/// Result returned by API operations.
pub type Result<T> = std::result::Result<T, Error>;
