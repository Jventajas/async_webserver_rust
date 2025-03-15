use dotenv::dotenv;

use std::error::Error;
use tracing::info;
use tokio::signal;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Spinning up server...");





    loop {
        tokio::select! {



            _ = signal::ctrl_c() => {
                info!("Shutting down server...");
                break;
            }
        }
    }





    Ok(())
}