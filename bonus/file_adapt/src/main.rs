use axum::{
    Router,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use pin_project_lite::pin_project;
use tokio::io::BufReader;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pin_project! {
    struct ToUpper { // the adapter
        #[pin]
        stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>,
    }
}

impl ToUpper {
    fn new(stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>) -> Self {
        Self { stream }
    }
}

impl tokio_stream::Stream for ToUpper {
    type Item = std::io::Result<String>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project()
            .stream
            .poll_next(cx)
            .map(|opt| opt.map(|res| res.map(|line| line.to_uppercase() + "\n")))
    }
}

async fn handler() -> impl IntoResponse {
    use tokio::io::AsyncBufReadExt;

    // File implements AsyncRead
    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    // convert the AsyncRead into a buffered reader, then a line stream, then your adapter
    let stream = BufReader::new(file).lines(); // read in chunks (async)
    let stream = tokio_stream::wrappers::LinesStream::new(stream); // feeding one line at time when await making a chain of adapters
    let stream = ToUpper::new(stream);

    // convert the Stream into an axum::body::HttpBody
    let body = axum::body::Body::from_stream(stream);
    // let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/toml; charset=utf-8"),
    );

    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str("attachment; filename=\"Cargo.toml\"").unwrap(),
    );

    Ok((headers, body))
}
