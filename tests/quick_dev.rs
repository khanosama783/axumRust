use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let _hc = httpc_test::new_client("http://localhost:8080")?;

    _hc.do_get("/test?name=Osama").await?.print().await?;

    _hc.do_get("/test2/Alpha").await?.print().await?;

    let req_login = _hc.do_post(
        "/api/login",
        json!({
            "username" : "username",
            "password" : "password",
        }),
    );

    req_login.await?.print().await?;

    let req_create_ticket = _hc.do_post(
        "api/tickets",
        json!({
            "title": "Ticket",
        }),
    );
    req_create_ticket.await?.print().await?;

    _hc.do_get("/api/tickets/").await?.print().await?;

    Ok(())
}
