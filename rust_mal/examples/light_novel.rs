use dotenv::dotenv;
use rust_mal::mal;
use std::env;

// TODO: create lib.rs, create client struct, study unfamiliar terminology

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token: String = env::var("TOKEN")?;

    let client = mal::Client::new(token);

    client.get_light_novels().await?;

    Ok(())
}