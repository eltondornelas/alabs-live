use anyhow::Ok;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    // Read the .env file and build environment variables
    dotenv::dotenv()?; // is not async but  potentially throws errors so needs the question mark
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?; // sqlx is always assync

    // Run Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;


    Ok(())
}

// cargo add tokio -F full
// cargo add serde -F derive
// cargo add axum -F multipart
// cargo add sqlx -F runtime-tokio-native-tls,sqlite or cargo add sqlx -F runtime-tokio-native-tls -F sqlite
// cargo add anyhow
// cargo add futures
// cargo add dotenv
// cargo add tokio_util -F io
// cargo add image

// stacking the cargo add => cargo add anyhow dotenv futures image
