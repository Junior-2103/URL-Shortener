use anyhow::Result;
use httpc_test::new_client;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = new_client("http://localhost:8080")?;

    let res = hc
        .do_post("/shorten", json!({"url":"https://example.com"}))
        .await?;
    let body = res.json_body()?;
    let uuid = body["link_uuid"].as_str().unwrap();

    let res = hc.do_get(format!("/link/{}", uuid).as_str()).await.unwrap();
    let body = res.json_body()?;
    let url = body["link"].as_str().unwrap();

    assert_eq!(url, "https://example.com");

    let res2 = hc
        .do_post("/shorten", json!({"url":"https://example.com"}))
        .await?;

    let body2 = res2.json_body()?;
    let uuid2 = body2["link_uuid"].as_str().unwrap();

    assert_eq!(uuid, uuid2);

    Ok(())
}
