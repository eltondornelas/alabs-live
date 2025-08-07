use anyhow::Ok;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
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
