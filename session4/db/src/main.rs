use sqlx::Row;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?; // to get the connection string 

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    let messages = sqlx::query("SELECT id, message FROM messages")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let id: i64 = row.get(0);
            let message: String = row.get(1);
            (id, message)
        })
        .fetch_all(&pool)
        .await?;

    for (id, message) in messages {
        println!("{id} : {message}")
    }

    Ok(())
}

// cargo add dotenv
// cargo install sqlx-cli

//// to make it work
// sqlx = { version = "0.8.6", features = ["runtime-tokio-native-tls", "sqlite"]}
// openssl = { version = "0.10", features = ["vendored"] }
// sudo apt install libssl-dev
// sudo apt install pkg-config
// cargo install sqlx-cli

// sqlx database create
// sqlx migrate add initial
// sqlx migrate run
