use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;
use crate::models::symbol::Symbol;

#[derive(Debug, Error)]
pub enum StockClientError {
    #[error("API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolQuoteResponse {
    #[serde(rename = "Global Quote")]
    pub global_quote: GlobalQuote,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    pub symbol: String,
    #[serde(rename = "02. open")]
    pub open: String,
    #[serde(rename = "03. high")]
    pub high: String,
    #[serde(rename = "04. low")]
    pub low: String,
    #[serde(rename = "05. price")]
    pub price: String,
    #[serde(rename = "06. volume")]
    pub volume: String,
    #[serde(rename = "07. latest trading day")]
    pub latest_trading_day: String,
    #[serde(rename = "08. previous close")]
    pub previous_close: String,
    #[serde(rename = "09. change")]
    pub change: String,
    #[serde(rename = "10. change percent")]
    pub change_percent: String,
}


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

    pub async fn fetch_symbol_quote(&self, symbol: &str) -> Result<SymbolQuoteResponse, StockClientError> {
        let url = format!(
            "{}?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            self.base_url, symbol, self.api_key
        );

        let response = self.http_client
            .get(&url)
            .send()
            .await?
            .json::<SymbolQuoteResponse>()
            .await?;

        Ok(response)
    }

    pub fn parse_quote_to_symbol(&self, quote_response: SymbolQuoteResponse) -> Result<Symbol, StockClientError> {
        let quote = quote_response.global_quote;
        let price = quote.price.parse::<f64>().unwrap_or(0.0);

        let change_percent = quote.change_percent
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0);

        let previous_close = quote.previous_close.parse::<f64>().unwrap_or(0.0);
        let volume = quote.volume.parse::<i64>().unwrap_or(0);

        Ok(Symbol {
            id: 0, // Database will assign the ID
            symbol: quote.symbol,
            price,
            change_percent,
            previous_close,
            volume,
            trading_day: quote.latest_trading_day,
            last_updated: chrono::Utc::now(),
        })
    }
}

impl Clone for StockClient {
    fn clone(&self) -> Self {
        Self {
            http_client: Client::new(),
            api_key: self.api_key.clone(),
            base_url: self.base_url.clone(),
        }
    }
}