use anyhow::Result;
use httpc_test::new_client;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = new_client("http://localhost:8080")?;

    let res = hc
        .do_post("/shorten", json!({"url":"https://example.com"}))
        .await?
        .print()
        .await?;

    Ok(())
}
