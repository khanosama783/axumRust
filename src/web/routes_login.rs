use crate::{Error, Result};
use axum::Router;
use axum::{
    body,
    routing::{post, Route},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("=> {:<12} - api_login", "HANDLER");

    if payload.username != "username" || payload.password != "password" {
        return Err(Error::LoginFaul);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
