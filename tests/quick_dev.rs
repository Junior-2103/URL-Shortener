use anyhow::Result;
use httpc_test::new_client;
use serde_json::json;
use sqlx::{Row, sqlite::SqlitePoolOptions};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://data/urls.db?mode=rwc")
        .await?;

    let hc = new_client("http://localhost:8080")?;

    let res = hc
        .do_post("/shorten", json!({"url":"https://example.com"}))
        .await?;

    let body = res.json_body()?;

    let uuid = body["link_uuid"].as_str().unwrap();

    let res_db = sqlx::query("SELECT url FROM urls WHERE uuid = ?")
        .bind(uuid)
        .fetch_one(&pool)
        .await?;

    let url: String = res_db.get("url");

    assert_eq!(url, "https://example.com");

    Ok(())
}
