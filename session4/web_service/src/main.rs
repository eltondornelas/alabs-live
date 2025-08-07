use axum::{
    Router,
    routing::{get, post},
};
// use std::net::SocketAddr;
use axum::response::Html;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/json", get(hello_json))
        .route("/post", post(hello_post)); // without the () it's a fuction pointer
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap(); // since not using anyhow
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn hello_json() -> axum::Json<HelloJson> {
    let message = HelloJson {
        message: "Hi from JSON".to_string(),
    };
    axum::Json(message)
}

// async fn say_hello_text() -> Html<&'static str> {
async fn say_hello_text() -> Html<String> {
    let path = std::path::Path::new("./src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)

    // const HTML: &str = include_str!("hello.html");
    // Html(HTML)
    // Html("<h1>Hello, world!</h1>")
}

async fn hello_post() -> Html<String> {
    Html("Hello from post".to_string())
}

// cargo add axum
// https://github.com/thebracket/ArdanUltimateRust-5Days/blob/main/03-Async/Axum.md
