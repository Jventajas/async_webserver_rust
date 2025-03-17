use reqwest::Client;
use std::env;


pub struct StockClient {
    http_client: Client,
    api_key: String,
    base_url: String,
}

impl StockClient {

    pub fn new() -> Self {
        let api_key = env::var("ALPHA_VANTAGE_API_KEY")
            .expect("ALPHA_VANTAGE_API_KEY must be set");
        let base_url = env::var("ALPHA_VANTAGE_BASE_URL")
            .expect("ALPHA_VANTAGE_BASE_URL must be set");

        Self {
            http_client: Client::new(),
            api_key,
            base_url,
        }
    }

}