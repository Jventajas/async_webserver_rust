mod server;
mod routes;
mod models;
mod services;

use dotenv::dotenv;
use tracing::{info, error};
use std::sync::Arc;
use std::error::Error;
use tokio::signal;
use tokio::net::TcpListener;

use crate::server::server::HttpServer;
use crate::services::data_sync::DataSyncService;
use crate::services::database::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")
        .expect("API_KEY environment variable not set, server cannot start");
    let ip_address = std::env::var("IP_ADDRESS")
        .expect("IP_ADDRESS environment variable not set, server cannot start");
    let port = std::env::var("PORT")
        .expect("PORT environment variable not set, server cannot start");
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable not set, server cannot start");

    info!("Initializing database...");
    let database = Database::new(database_url).await?;

    info!("Starting data sync service...");
    DataSyncService::new().sync_data(30).await;

    info!("Spinning up server...");

    let http_server = HttpServer::new();
    let http_server = Arc::new(http_server);
    let listener = TcpListener::bind(format!("{}:{}", ip_address, port)).await?;


    loop {
        tokio::select! {
            Ok((stream, _)) = listener.accept() => {
                let server = Arc::clone(&http_server);

                tokio::spawn(async move {
                    if let Err(e) = server.handle_connection(stream).await {
                        error!("Error handling connection: {}", e);
                    }
                });
            }

            _ = signal::ctrl_c() => {
                info!("Shutting down server...");
                break;
            }
        }
    }

    info!("Server shutdown complete");
    Ok(())
}