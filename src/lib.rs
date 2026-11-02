//! Async Rust client for the OpenCode v2 HTTP API.
//!
//! Currently supports [`Client::server`] and [`Server::info`]. Run requests on a
//! caller-owned Tokio runtime. Clones share the HTTP connection pool.
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

mod client;
mod error;
pub mod models;
mod server;

pub use client::{Client, ClientBuilder};
pub use error::{
    ApiError, BuildError, DecodeError, Error, HttpError, ResponseTooLargeError, TransportError,
};
pub use reqwest::{StatusCode, header};
pub use server::Server;

/// Result returned by API operations.
pub type Result<T> = std::result::Result<T, Error>;
