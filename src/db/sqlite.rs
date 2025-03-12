use sqlx::sqlite::SqlitePool;
use std::path::Path;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let db_path = Path::new("./data/stock_tracker.db");
    let url= &format!("sqlite://{}", db_path.display());
    let pool = SqlitePool::connect(url).await?;

    // Run migrations
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}