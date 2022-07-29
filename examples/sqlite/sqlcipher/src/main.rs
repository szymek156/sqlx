use std::{env, error::Error, fs::File, str::FromStr};

use anyhow::Context;
use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    ConnectOptions, Connection,
};
use tempdir::TempDir;

/// Creates new example database with explicit sqlcipher v3 configuration
async fn create_db(url: &str) -> anyhow::Result<()> {
    println!("Creating example database, url {url}");

    let mut conn = SqliteConnectOptions::from_str(url)?
        .pragma("key", "the_password")
        .pragma("cipher_page_size", "1024")
        .pragma("kdf_iter", "64000")
        .pragma("cipher_hmac_algorithm", "HMAC_SHA1")
        .pragma("cipher_kdf_algorithm", "PBKDF2_HMAC_SHA1")
        .journal_mode(SqliteJournalMode::Delete)
        .foreign_keys(false)
        .connect()
        .await?;

    conn.transaction(|tx| {
        Box::pin(async move {
            query(
                "CREATE TABLE COMPANY(
            ID INT PRIMARY KEY     NOT NULL,
            NAME           TEXT    NOT NULL,
            AGE            INT     NOT NULL,
            ADDRESS        CHAR(50),
            SALARY         REAL
         );",
            )
            .execute(tx)
            .await
        })
    })
    .await?;

    println!("Database created");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = TempDir::new("sqlcipher_example")?;
    let filepath = dir.path().join("database.sqlite3");

    // Touch the file, so DB driver will not complain it does not exist
    File::create(filepath.as_path());

    let url = format!("sqlite://{}", filepath.display());

    // Create a database, with example data
    create_db(&url).await?;

    // Open new connection to it to read the data
    println!("Opening connection to read the data");
    let mut conn = SqliteConnectOptions::from_str(&url)?
        .pragma("key", "the_password")
        .pragma("cipher_page_size", "1024")
        .pragma("kdf_iter", "64000")
        .pragma("cipher_hmac_algorithm", "HMAC_SHA1")
        .pragma("cipher_kdf_algorithm", "PBKDF2_HMAC_SHA1")
        .journal_mode(SqliteJournalMode::Delete)
        .foreign_keys(false)
        .connect()
        .await?;

    Ok(())
}
