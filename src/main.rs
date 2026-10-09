use axum::{Json, Router, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/shorten", post(shorten));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize, Serialize)]
struct Url {
    url: String,
}

async fn shorten(Json(url): Json<Url>) -> (StatusCode, Json<Value>) {
    (StatusCode::CREATED, Json(json!({"response":url.url})))
}
