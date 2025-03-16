use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub id: i64,
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change_percent: f64,
    pub last_updated: DateTime<Utc>,
}