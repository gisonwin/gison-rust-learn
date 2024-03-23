use axum::{extract::Query,response::Html,routing::get,Router};
use rand::{thread_rng,Rng};
use serde::Deserialize;

async fn handler(Query(range):Query<RangeParameters>) -> Html<String>{
    let random_number = thread_rng().gen_range(range.start..range.end);
    Html(format!("<h1>Random Number:{}</h1>",random_number))
}
async fn handler_html() -> Html<&'static str>{
    Html(include_str!("../sina.html"))
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler_html));
    let listener
        = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {:?}", listener);
    axum::serve(listener,app).await.unwrap();
}
#[derive(Deserialize)]
struct RangeParameters{
    start:usize,
    end:usize,
}
