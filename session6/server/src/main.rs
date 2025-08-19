mod collector;

use axum::{
    Extension, Json, Router,
    extract::Path,
    http::{self, Method},
    response::Html,
    routing::get,
};
use http::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read .env
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;
    let handle = tokio::spawn(collector::data_collector(pool.clone())); // connection pools are designed to be cloned

    // Launch Axum here
    // Start the web server
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);
    // .allow_methods(Any)
    // .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .route("/collector.html", get(collector))
        .route("/api/all", get(show_all))
        .route("/api/collectors", get(show_collectors))
        .route("/api/collector/{uuid}", get(collector_data))
        .layer(cors)
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.

    Ok(())
}

async fn index() -> Html<String> {
    let path = std::path::Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn collector() -> Html<String> {
    let path = std::path::Path::new("src/collector.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

use sqlx::FromRow;
// use futures::TryStreamExt;
use serde::Serialize;

#[derive(FromRow, Debug, Serialize)]
pub struct DataPoint {
    id: i32,
    collector_id: String,
    received: i64,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
        // .fetch(&pool);
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)

    // while let Some(row) = rows.try_next().await.unwrap() {
    //     println!("{:?}", row);
    // }
}

#[derive(FromRow, Debug, Serialize)]
pub struct Collector {
    id: i32,
    collector_id: String,
    last_seen: i64,
}

pub async fn show_collectors(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<Collector>> {
    const SQL: &str = "SELECT 
    DISTINCT(id) AS id, 
    collector_id, 
    (SELECT MAX(received) FROM timeseries WHERE collector_id = ts.collector_id) AS last_seen 
    FROM timeseries ts";
    Json(
        sqlx::query_as::<_, Collector>(SQL)
            .fetch_all(&pool)
            .await
            .unwrap(),
    )
}

pub async fn collector_data(
    Extension(pool): Extension<sqlx::SqlitePool>,
    uuid: Path<String>,
) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>(
        "SELECT * FROM timeseries WHERE collector_id = ? ORDER BY received",
    )
    .bind(uuid.as_str())
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(rows)
}

// data collection server as high performance server
// need to run here as server and run the ../collector project to see it working

// cargo add uuid -> no features since is not generating, only handling
// cargo add dotenv
// cargo add axum
// cargo add futures
// cargo add serde -F derive
// cargo add tower-http -F cors 
