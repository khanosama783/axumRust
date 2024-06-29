#![allow(unused)]
use axum::extract::Path;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{response::Html, routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_test());

    let addr = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Listing on {:?}", addr);

    axum::serve(addr, routes_all).await.unwrap();
}

fn routes_test() -> Router {
    Router::new()
        .route("/test", get(handler_hello))
        .route("/test2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("=> {:<12} - handler_hello - {params:?}", "HANDLER");
    let _name = params.name.as_deref().unwrap_or("Test-Unwrap");
    Html(format!("Test handler_hello function <h1>{_name}</h1>"))
}

async fn handler_hello2(Path(_params): Path<String>) -> impl IntoResponse {
    println!("=> {:<12} - handler_hello - {_params:?}", "HANDLER");
    Html(format!(
        "Test handler_hello2  function <strong>{_params}</strong>"
    ))
}
