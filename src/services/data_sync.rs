use std::env;
use tokio::time;
use tracing::info;
use crate::services::stock_client::StockClient;

pub struct DataSyncService {
    stock_client: StockClient,
    symbols: Vec<String>,
}

impl DataSyncService {

    pub fn new() -> Self {
        let symbols = env::var("SYMBOLS").expect("SYMBOLS environment variable not set");
        let symbols = symbols.split(",").map(|s| s.to_string()).collect();

        Self {
            stock_client: StockClient::new(),
            symbols,
        }
    }


    pub async fn sync_data(&self, interval_seconds: u64) {

        tokio::spawn(async move {
           let mut interval = time::interval(time::Duration::from_secs(interval_seconds));

            loop {
                interval.tick().await;
                info!("Starting data sync");
            }

        });
    }

}