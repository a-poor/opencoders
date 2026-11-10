//! Run one command, poll completion, print its first output page and remove capture.
use opencoders::{
    Client, Location, ShellOutputOptions,
    models::{ShellCreateRequest, ShellStatus},
};
use std::{env, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = env::args()
        .nth(1)
        .ok_or("usage: cargo run --example shell -- 'COMMAND'")?;
    let mut builder = Client::builder(env::var("OPENCODE_BASE_URL")?);
    if let Ok(password) = env::var("OPENCODE_PASSWORD") {
        builder = builder.basic_auth("opencode", password);
    }
    if let Ok(directory) = env::var("OPENCODE_DIRECTORY") {
        builder = builder.default_location(Location::directory(directory));
    }
    let client = builder.build()?;
    let started = client
        .shells()
        .create(&ShellCreateRequest::new(command).timeout_ms(10_000))
        .await?;
    let id = &started.data.id;
    let location = Location::directory(started.location.directory.as_str());
    let observation = tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            let info = client.shells().get_at(id, location.clone()).await?;
            if info.data.status != ShellStatus::Running {
                let page = client
                    .shells()
                    .output_at(id, &ShellOutputOptions::default(), location.clone())
                    .await?;
                println!(
                    "status={}, exit={:?}, captured bytes={}",
                    info.data.status,
                    info.data.exit,
                    page.data.size.get()
                );
                print!("{}", page.data.output);
                break Ok::<_, opencoders::Error>(());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await;
    // Cleanup is explicit, including when polling failed or exceeded its deadline.
    client.shells().remove_at(id, location).await?;
    observation??;
    Ok(())
}
