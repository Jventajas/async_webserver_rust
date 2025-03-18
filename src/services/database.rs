use sqlx::{sqlite::SqlitePool, migrate::MigrateDatabase, Sqlite, Pool, Row};
use tracing::{info};
use crate::models::symbol::Symbol;
use chrono::{DateTime, Utc};
use crate::utils::error::ApplicationError;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: String) -> Result<Self, ApplicationError> {
        if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
            info!("Creating database at {}", database_url);
            Sqlite::create_database(&database_url).await?;
        }

        let pool = SqlitePool::connect(&database_url).await?;
        Self::init_database(&pool).await?;

        Ok(Self { pool })
    }

    async fn init_database(pool: &Pool<Sqlite>) -> Result<(), ApplicationError> {
        info!("Initializing database tables");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS symbols (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                symbol TEXT NOT NULL UNIQUE,
                price REAL NOT NULL,
                change_percent REAL NOT NULL,
                previous_close REAL NOT NULL,
                volume INTEGER NOT NULL,
                trading_day TEXT NOT NULL,
                last_updated TEXT NOT NULL
            )"
        )
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn save_symbol(&self, symbol: &Symbol) -> Result<i64, ApplicationError> {
        let last_updated_str = symbol.last_updated.to_rfc3339();

        let result = sqlx::query(
            "INSERT INTO symbols
            (symbol, price, change_percent, previous_close, volume, trading_day, last_updated)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(symbol) DO UPDATE SET
            price = excluded.price,
            change_percent = excluded.change_percent,
            previous_close = excluded.previous_close,
            volume = excluded.volume,
            trading_day = excluded.trading_day,
            last_updated = excluded.last_updated
            RETURNING id"
        )
            .bind(&symbol.symbol)
            .bind(symbol.price)
            .bind(symbol.change_percent)
            .bind(symbol.previous_close)
            .bind(symbol.volume)
            .bind(&symbol.trading_day)
            .bind(last_updated_str)
            .fetch_one(&self.pool)
            .await?;

        let id: i64 = result.get(0);
        Ok(id)
    }

    pub async fn get_all_symbols(&self) -> Result<Vec<Symbol>, ApplicationError> {
        let rows = sqlx::query(
            "SELECT id, symbol, price, change_percent, previous_close, volume, trading_day, last_updated
         FROM symbols"
        )
            .fetch_all(&self.pool)
            .await?;

        let mut symbols = Vec::with_capacity(rows.len());

        for row in rows {
            let last_updated = DateTime::parse_from_rfc3339(row.get::<String, _>("last_updated").as_str())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            symbols.push(Symbol {
                id: row.get("id"),
                symbol: row.get("symbol"),
                price: row.get("price"),
                change_percent: row.get("change_percent"),
                previous_close: row.get("previous_close"),
                volume: row.get("volume"),
                trading_day: row.get("trading_day"),
                last_updated,
            });
        }

        Ok(symbols)
    }


    pub async fn get_symbol_by_ticker(&self, ticker: &str) -> Result<Option<Symbol>, ApplicationError> {
        let row = sqlx::query(
            "SELECT id, symbol, price, change_percent, previous_close, volume, trading_day, last_updated
         FROM symbols WHERE symbol = ?"
        )
            .bind(ticker)
            .fetch_optional(&self.pool)
            .await?;

        let symbol = if let Some(row) = row {
            let last_updated = DateTime::parse_from_rfc3339(row.get::<String, _>("last_updated").as_str())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Some(Symbol {
                id: row.get("id"),
                symbol: row.get("symbol"),
                price: row.get("price"),
                change_percent: row.get("change_percent"),
                previous_close: row.get("previous_close"),
                volume: row.get("volume"),
                trading_day: row.get("trading_day"),
                last_updated,
            })
        } else {
            None
        };

        Ok(symbol)
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}