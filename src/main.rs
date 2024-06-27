#![allow(unused)]
use axum::extract::Query;
use axum::{response::Html, routing::get, Router};
use axum::response::IntoResponse;
use serde::Deserialize;


#[tokio::main]
async fn main() {
    let route_test = Router::new().route(
        "/test",
        get(handler_hello)
    );
    let addr =  tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listing on {:?}", addr);
    axum::serve(addr, route_test).await.unwrap();
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