use axum::{
    body::{Body,Bytes},
    extract::Request,
    http::StatusCode,
    routing::get,routing::post,Json,Router,response::Html
};
use serde::{Deserialize,Serialize};

#[tokio::main]
async fn main() {
    //init tracing
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(handler));
    let listener
        = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
     axum::serve(listener, app).await.unwrap();
}
async fn handler() -> Html<&'static str>{
    Html("<h1>Hello Axum</h1>")
}