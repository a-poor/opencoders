//! Run with OPENCODE_BASE_URL and, if required, OPENCODE_TOKEN in the environment.

use opencoders::Client;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_url = env::var("OPENCODE_BASE_URL")
        .map_err(|_| "set OPENCODE_BASE_URL to your OpenCode server URL")?;
    let mut builder = Client::builder(base_url);
    match env::var("OPENCODE_TOKEN") {
        Ok(token) => builder = builder.bearer_auth(token),
        Err(env::VarError::NotPresent) => {}
        Err(env::VarError::NotUnicode(_)) => {
            return Err("OPENCODE_TOKEN must be valid Unicode".into());
        }
    }
    let info = builder.build()?.server().info().await?;
    println!("OpenCode {} (pid {})", info.version, info.pid);
    println!("Server temporary directory: {}", info.paths.tmp);
    Ok(())
}
