#![allow(unsafe_code)]
#![allow(unused_imports)]

use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let _hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    _hc.do_get("/test?name=Osama").await?.print().await?;

    _hc.do_get("/test2/Alpha").await?.print().await?;

    let req_login = _hc.do_post(
        "/api/login",
        json!({
            "username" : "username",
            "password" : "password"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
