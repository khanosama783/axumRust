#![allow(unused)]
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello"
    );
}
