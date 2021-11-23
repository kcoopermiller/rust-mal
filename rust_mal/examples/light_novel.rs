use dotenv::dotenv;
use rust_mal::mal;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token: String = env::var("TOKEN")?;

    let client = mal::Client::new(token);

    for n in 3479..5000 { // Not run
        client.get_light_novels(n).await?;
    }

    Ok(())
}