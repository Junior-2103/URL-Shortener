use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    Html("Hello <strong>World</strong>")
}
