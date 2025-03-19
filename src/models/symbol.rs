use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: i64,
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub previous_close: f64,
    pub last_updated: DateTime<Utc>,
}

impl Symbol {
    pub fn new(
        id: i64,
        symbol: String,
        price: f64,
        change: f64,
        change_percent: f64,
        high_price: f64,
        low_price: f64,
        open_price: f64,
        previous_close: f64,
        last_updated: DateTime<Utc>
    ) -> Self {
        Self {
            id,
            symbol,
            price,
            change,
            change_percent,
            high_price,
            low_price,
            open_price,
            previous_close,
            last_updated,
        }
    }
}