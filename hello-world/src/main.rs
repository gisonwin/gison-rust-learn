use axum::{http::StatusCode, Json, response::Html, Router, routing::get, routing::post};
use serde::{Deserialize, Serialize};
use tracing::info;

#[tokio::main]
async fn main() {
    //init tracing
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(handler));
    let listener
        = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("server listening on {}",listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
async fn handler() -> Html<&'static str>{
    Html("<h1>Hello Axum</h1>")
}