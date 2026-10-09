use anyhow::Result;
use httpc_test::new_client;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = new_client("http://localhost:8080/")?;

    hc.do_get("/").await?.print().await?;

    Ok(())
}
