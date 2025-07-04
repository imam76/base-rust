use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:5001")?;

    hc.do_get("/").await?.print().await?;
    hc.do_get("/version").await?.print().await?;

    let req_login = hc.do_post(
        "/api/auth",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/v1/hello",
        json!({
            "title": "Ticket AAA"
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_get("/api/v1/hello").await?.print().await?;

    Ok(())
}
