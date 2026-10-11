use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use tokio::net::TcpListener;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://data/urls.db?mode=rwc")
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            uuid TEXT PRIMARY KEY,
            url TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    let router = Router::new()
        .route("/shorten", post(shorten))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize, Serialize)]
struct Url {
    url: String,
}

async fn shorten(
    State(pool): State<Pool<Sqlite>>,
    Json(url): Json<Url>,
) -> (StatusCode, Json<Value>) {
    let uuid = Uuid::new_v4();

    sqlx::query("INSERT INTO urls VALUES (?, ?)")
        .bind(uuid.to_string())
        .bind(url.url.to_string())
        .execute(&pool)
        .await
        .unwrap();
    (
        StatusCode::CREATED,
        Json(json!({"link_uuid":uuid.to_string()})),
    )
}
