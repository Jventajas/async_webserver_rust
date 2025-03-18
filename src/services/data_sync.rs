use std::env;
use tokio::time;
use tracing::info;
use crate::services::stock_client::StockClient;

const SYMBOL_FETCH_DELAY: u64 = 12;

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
        let stock_client = self.stock_client.clone();
        let symbols = self.symbols.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(interval_seconds));

            loop {
                interval.tick().await;
                info!("Starting data sync for {} symbols", symbols.len());

                for symbol in &symbols {
                    match stock_client.fetch_symbol_quote(symbol).await {
                        Ok(quote_response) => {
                            match stock_client.parse_quote_to_symbol(quote_response) {
                                Ok(symbol_data) => {
                                    info!(
                                    "Fetched data for {}: price=${}, change={}%, volume={}",
                                    symbol_data.symbol,
                                    symbol_data.price,
                                    symbol_data.change_percent,
                                    symbol_data.volume
                                );
                                }
                                Err(e) => info!("Failed to parse data for {}: {}", symbol, e),
                            }
                        }
                        Err(e) => info!("Failed to fetch data for {}: {}", symbol, e),
                    }

                    time::sleep(time::Duration::from_secs(SYMBOL_FETCH_DELAY)).await;
                }

                info!("Data sync completed");
            }
        });
    }
}