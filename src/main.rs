use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{Pool, Row, Sqlite, sqlite::SqlitePoolOptions};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct Url {
    url: String,
}

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:///data/urls.db?mode=rwc")
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
        .route("/link/{uuid}", get(link))
        .fallback_service(ServeDir::new("./static"))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn url_exists(pool: &Pool<Sqlite>, url: &Url) -> Option<String> {
    let res = sqlx::query("SELECT uuid FROM urls WHERE url = ?")
        .bind(url.url.to_string())
        .fetch_one(pool)
        .await;

    res.ok().map(|res| res.get("uuid"))
}

async fn shorten(
    State(pool): State<Pool<Sqlite>>,
    Json(url): Json<Url>,
) -> (StatusCode, Json<Value>) {
    let uuid = Uuid::new_v4();

    if let Some(link_uuid) = url_exists(&pool, &url).await {
        return (StatusCode::CONFLICT, Json(json!({"link_uuid":link_uuid})));
    }

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

async fn link(State(pool): State<Pool<Sqlite>>, Path(uuid): Path<String>) -> Redirect {
    let row = sqlx::query("SELECT url FROM urls WHERE uuid = ?")
        .bind(uuid)
        .fetch_one(&pool)
        .await
        .unwrap();

    let url: String = row.get("url");

    Redirect::permanent(url.as_str())
}
