use axum::{routing::get,routing::post,http::StatusCode,Json,Router};
use serde::{Deserialize,Serialize};

#[tokio::main]
async fn main() {
    //init tracing
    tracing_subscriber::fmt::init();
}
