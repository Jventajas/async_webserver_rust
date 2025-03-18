use crate::services::database::Database;
use crate::services::stock_client::StockClient;
use std::env;
use tokio::time;
use tracing::info;

const SYMBOL_FETCH_DELAY: u64 = 12;

pub struct DataSyncService {
    stock_client: StockClient,
    symbols: Vec<String>,
    database: Database,
}

impl DataSyncService {
    pub fn new(database: Database, symbols: Vec<String>) -> Self {
        Self {
            stock_client: StockClient::new(),
            symbols,
            database,
        }
    }

    pub async fn sync_data(&self, interval_seconds: u64) {
        let stock_client = self.stock_client.clone();
        let symbols = self.symbols.clone();
        let database = self.database.clone();

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

                                    match database.save_symbol(&symbol_data).await {
                                        Ok(id) => {
                                            info!("Saved symbol data to database with ID: {}", id)
                                        }
                                        Err(e) => info!(
                                            "Failed to save data for {} to database: {}",
                                            symbol, e
                                        ),
                                    }
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
