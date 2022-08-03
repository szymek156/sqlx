use std::{error::Error, str::FromStr};

use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    ConnectOptions, Connection, SqliteConnection,
};
use warp::{post, Filter};

async fn do_migration() -> Result<(), Box<dyn Error>> {
    let mut conn = SqliteConnectOptions::from_str(":memory:")?
        .connect()
        .await?;

    sqlx::migrate!("./migrations").run(&mut conn).await?;

    Ok(())
}

async fn do_migration_pool() -> Result<(), Box<dyn Error>> {
    let mut pool = SqlitePoolOptions::new().connect(":memory:").await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let endpoint = warp::path("v1")
        .and(warp::path!("create_item"))
        .then(|| async move {
            do_migration().await;

            // do_migration_pool().await;
        });

    Ok(())
}
