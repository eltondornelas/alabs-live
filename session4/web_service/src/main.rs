use axum::{Router, routing::get};
// use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(say_hello_text));
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap(); // since not using anyhow
}

async fn say_hello_text() -> &'static str {
    "Hello, world!"
}

// cargo add axum
