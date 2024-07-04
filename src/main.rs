#![allow(unused)]
pub use self::error::{Error, Result};
use axum::extract::Path;
use axum::extract::Query;
use axum::middleware;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get_service;
use axum::{response::Html, routing::get, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;
use web::routes_login;

mod error;
mod web;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_test())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

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

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("=> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}
