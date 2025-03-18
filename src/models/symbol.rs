use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: i64,
    pub symbol: String,
    pub price: f64,
    pub change_percent: f64,
    pub previous_close: f64,
    pub volume: i64,
    pub trading_day: String,
    pub last_updated: DateTime<Utc>,
}

impl Symbol {
    pub fn new(
        id: i64,
        symbol: String,
        price: f64,
        change_percent: f64,
        previous_close: f64,
        volume: i64,
        trading_day: String,
        last_updated: DateTime<Utc>
    ) -> Self {
        Self {
            id,
            symbol,
            price,
            change_percent,
            previous_close,
            volume,
            trading_day,
            last_updated,
        }
    }
}