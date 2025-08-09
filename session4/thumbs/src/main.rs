use anyhow::Ok;
use axum::{extract::{Multipart, Path}, http::{header, HeaderMap}, response::{Html, IntoResponse}, routing::{get, post}, Extension, Router};
use sqlx::{pool, Row};
use tokio_util::io::ReaderStream;
use axum::body::Body;

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

    // Build Axum with an "extension" to hold the database connection pool
    let app = Router::new()
        .route("/", get(index_page))
        .route("/upload", post(uploader))
        .route("/image/{id}", get(get_image))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn index_page() -> Html<String> {
    let path = std::path::Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn insert_image_into_database(pool: &sqlx::SqlitePool, tags: &str) -> anyhow::Result<i64>  {
    let row = sqlx::query("INSERT INTO images (tags) VALUES (?) RETURNING id")
        .bind(tags)
        .fetch_one(pool)
        .await?;

    Ok(row.get(0))
}

async fn save_image(id: i64, bytes: &[u8]) -> anyhow::Result<()> {
    // Check that the images folder exists and is a directory
    // If it doesn't, create it.
    let base_path = std::path::Path::new("images");
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    // Use "join" to create a path to the image file. Join is platform aware,
    // it will handle the differences between Windows and Linux.
    let image_path = base_path.join(format!("{id}.jpg"));
    if image_path.exists() {
        // The file exists. That shouldn't happen.
        anyhow::bail!("File already exists");
    }

    // Write the image to the file
    tokio::fs::write(image_path, bytes).await?;
    Ok(())
}

async fn uploader(
    Extension(pool): Extension<sqlx::SqlitePool>,
    mut multipart: Multipart,
) -> String {
    let mut tags = None; // "None" means "no tags yet"
    let mut image = None;
    
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        
        // println!("name: {name}, data: {}", data.len());

        match name.as_str() {
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()), // Using Some means we can check we received it
            "image" => image = Some(data.to_vec()),
            _ => panic!("Unknown field: {name}"),
        }
    }

    if let (Some(tags), Some(image)) = (tags, image) { // Destructuring both Options at once
        // println!("{tags} is {} bytes", image.len());
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
        save_image(new_image_id, &image).await.unwrap();  // just by addind the & turns the vector into a slice to borrow it
    } else {
        panic!("Missing field");
    }

    "Ok".to_string()
}

async fn get_image(Path(id): Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{id}.jpg");
    let attachment = format!("filename={filename}");
    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/jpeg"),
    );

    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment).unwrap()
    );

    let file = tokio::fs::File::open(&filename).await.unwrap();
    axum::response::Response::builder()
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("image/jpeg"))
        .header(header::CONTENT_DISPOSITION, header::HeaderValue::from_str(&attachment).unwrap())
        .body(Body::from_stream(ReaderStream::new(file)))
        .unwrap()
}

// async fn uploader(mut multipart: Multipart) -> String {
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let name = field.name().unwrap().to_string();
//         let data = field.bytes().await.unwrap();

//         println!("{name} is {} bytes", data.len());
//     }
//     "Ok".to_string()
// }


/* async fn test_database(Extension(pool): Extension<sqlx::SqlitePool>) -> String {
    let result = sqlx::query("SELECT COUNT(id) FROM images")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    let count = result.get::<i64, _>(0);
    format!("{count} images in the database")
} */


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
