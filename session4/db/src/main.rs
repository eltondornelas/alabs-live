use futures::TryStreamExt;
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?; // to get the connection string 

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Run Migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages") // usually ignore first parameter and the second is what you want the rows to be
        .fetch_all(&pool) // fetch_all is a iterator, for communicating with external systems as busy async server the ideal is go for streams rather than iterator
        .await?; // the iterator grabs the whole thing making pausing things

    // for working with data like objects (strong type), Diesel should be more effective than sqlx

    println!("{messages:?}");

    let mut message_stream =
        sqlx::query_as::<_, Message>("SELECT id, message FROM messages").fetch(&pool);

    while let Some(message) = message_stream.try_next().await? {
        // try_next is from the future crate; it will tell what failed in case of what went wrong;
        println!("{message:?}");
    }

    Ok(())
}

// use sqlx::Row;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     dotenv::dotenv()?;
//     let db_url = std::env::var("DATABASE_URL")?; // to get the connection string

//     let pool = sqlx::SqlitePool::connect(&db_url).await?;

//     let messages = sqlx::query("SELECT id, message FROM messages")
//         .map(|row: sqlx::sqlite::SqliteRow| {
//             let id: i64 = row.get(0);
//             let message: String = row.get(1);
//             (id, message)
//         })
//         .fetch_all(&pool)
//         .await?;

//     for (id, message) in messages {
//         println!("{id} : {message}")
//     }

//     Ok(())
// }

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

// sqlx migrate add more_messages
