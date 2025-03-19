use crate::models::symbol::Symbol;
use crate::utils::error::ApplicationError;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Row, Sqlite};
use sqlx::migrate::MigrateDatabase;
use tracing::info;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: String) -> Result<Self, ApplicationError> {
        if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
            info!("Creating database at {}", database_url);
            Sqlite::create_database(&database_url).await?;
        }

        let pool = sqlx::SqlitePool::connect(&database_url).await?;
        Self::init_database(&pool).await?;
        Ok(Self { pool })
    }

    async fn init_database(pool: &Pool<Sqlite>) -> Result<(), ApplicationError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS symbols (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                symbol TEXT NOT NULL UNIQUE,
                price REAL NOT NULL,
                change REAL NOT NULL,
                change_percent REAL NOT NULL,
                high_price REAL NOT NULL,
                low_price REAL NOT NULL,
                open_price REAL NOT NULL,
                previous_close REAL NOT NULL,
                last_updated TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn save_symbol(&self, symbol: &Symbol) -> Result<i64, ApplicationError> {
        // Convert DateTime<Utc> to ISO 8601 string
        let last_updated_str = symbol.last_updated.to_rfc3339();

        let result = sqlx::query(
            r#"
            INSERT INTO symbols (
                symbol, price, change, change_percent, high_price, low_price, open_price,
                previous_close, last_updated
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(symbol) DO UPDATE SET
                price = excluded.price,
                change = excluded.change,
                change_percent = excluded.change_percent,
                high_price = excluded.high_price,
                low_price = excluded.low_price,
                open_price = excluded.open_price,
                previous_close = excluded.previous_close,
                last_updated = excluded.last_updated
            "#,
        )
        .bind(&symbol.symbol)
        .bind(symbol.price)
        .bind(symbol.change)
        .bind(symbol.change_percent)
        .bind(symbol.high_price)
        .bind(symbol.low_price)
        .bind(symbol.open_price)
        .bind(symbol.previous_close)
        .bind(last_updated_str)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_all_symbols(&self) -> Result<Vec<Symbol>, ApplicationError> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, symbol, price, change, change_percent, high_price, low_price,
                open_price, previous_close, last_updated
            FROM symbols
            ORDER BY symbol
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut symbols = Vec::with_capacity(rows.len());

        for row in rows {
            let last_updated_str: String = row.get("last_updated");
            let last_updated = DateTime::parse_from_rfc3339(&last_updated_str)
                .map_err(|e| ApplicationError::DateParseError(e))?
                .with_timezone(&Utc);

            let symbol = Symbol {
                id: row.get("id"),
                symbol: row.get("symbol"),
                price: row.get("price"),
                change: row.get("change"),
                change_percent: row.get("change_percent"),
                high_price: row.get("high_price"),
                low_price: row.get("low_price"),
                open_price: row.get("open_price"),
                previous_close: row.get("previous_close"),
                last_updated,
            };

            symbols.push(symbol);
        }

        Ok(symbols)
    }

    pub async fn get_symbol_by_ticker(
        &self,
        ticker: &str,
    ) -> Result<Option<Symbol>, ApplicationError> {
        let row = sqlx::query(
            r#"
            SELECT
                id, symbol, price, change, change_percent, high_price, low_price,
                open_price, previous_close, last_updated
            FROM symbols
            WHERE symbol = ?
            "#,
        )
        .bind(ticker)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let last_updated_str: String = row.get("last_updated");
                let last_updated = DateTime::parse_from_rfc3339(&last_updated_str)
                    .map_err(|e| ApplicationError::DateParseError(e))?
                    .with_timezone(&Utc);

                let symbol = Symbol {
                    id: row.get("id"),
                    symbol: row.get("symbol"),
                    price: row.get("price"),
                    change: row.get("change"),
                    change_percent: row.get("change_percent"),
                    high_price: row.get("high_price"),
                    low_price: row.get("low_price"),
                    open_price: row.get("open_price"),
                    previous_close: row.get("previous_close"),
                    last_updated,
                };

                Ok(Some(symbol))
            }
            None => Ok(None),
        }
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}
