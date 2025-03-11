mod server;
mod router;
mod routes;
mod models;
mod db;
mod fetcher;
mod utils;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    println!("Starting portfolio tracker...");
    Ok(())
}