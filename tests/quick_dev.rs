#![allow(unsafe_code)]
#![allow(unused_imports)]

use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let _hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    _hc.do_get("/test?name=Osama").await?.print().await?;

    _hc.do_get("/test2/Alpha").await?.print().await?;

    Ok(())
}
