//! Read global credentials and print counts only. This does retrieve secret values.
use opencoders::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = Client::builder(env::var("OPENCODE_BASE_URL")?);
    if let Ok(password) = env::var("OPENCODE_PASSWORD") {
        builder = builder.basic_auth("opencode", password);
    }
    let credentials = builder.build()?.credentials().list().await?;
    println!("Stored credentials: {}", credentials.data.len());
    println!(
        "Selected credentials: {}",
        credentials.data.iter().filter(|entry| entry.active).count()
    );
    Ok(())
}
