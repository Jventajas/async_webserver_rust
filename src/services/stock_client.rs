use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use crate::models::symbol::Symbol;
use crate::utils::error::ApplicationError;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolQuoteResponse {
    pub c: f64, // Current price
    pub d: f64, // Change
    pub dp: f64, // Percent change
    pub h: f64, // High price of the day
    pub l: f64, // Low price of the day
    pub o: f64, // Open price of the day
    pub pc: f64, // Previous close price
    pub t: i64, // Timestamp
}

pub struct StockClient {
    http_client: Client,
    api_key: String,
    base_url: String,
}

impl StockClient {
    pub fn new() -> Self {
        let api_key = env::var("FINNHUB_API_KEY")
            .expect("FINNHUB_API_KEY must be set");
        let base_url = env::var("FINNHUB_BASE_URL")
            .unwrap_or_else(|_| String::from("https://finnhub.io/api/v1"));

        Self {
            http_client: Client::new(),
            api_key,
            base_url,
        }
    }

    pub async fn fetch_symbol_quote(&self, symbol: &str) -> Result<SymbolQuoteResponse, ApplicationError> {
        let url = format!(
            "{}/quote?symbol={}&token={}",
            self.base_url, symbol, self.api_key
        );

        let symbol_data = self.http_client
            .get(&url)
            .send()
            .await?
            .json::<SymbolQuoteResponse>()
            .await?;

        Ok(symbol_data)
    }

    pub fn parse_quote_to_symbol(&self, quote_response: SymbolQuoteResponse, symbol_name: &str) -> Result<Symbol, ApplicationError> {
        Ok(Symbol {
            id: 0, // Database will assign the ID
            symbol: symbol_name.to_string(),
            price: quote_response.c,
            change: quote_response.d,
            change_percent: quote_response.dp,
            high_price: quote_response.h,
            low_price: quote_response.l,
            open_price: quote_response.o,
            previous_close: quote_response.pc,
            last_updated: Utc::now(),
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