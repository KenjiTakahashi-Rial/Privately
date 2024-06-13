use httpc_test::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let c = httpc_test::new_client("http://localhost:8080")?;

    c.do_get("/index.html").await?.print().await?;

    let login_req = c.do_post("/api/login", json!({"username": "ob", "password": "ob"}));
    login_req.await?.print().await?;

    Ok(())
}
