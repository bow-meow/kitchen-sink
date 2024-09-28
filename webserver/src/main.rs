use anyhow::Error;
use thiserror::Error;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};



#[tokio::main]
async fn main() {
    println!("a {} a {} a {}", Err::NoInternet, Err::NoInternet, Err::MissingAttribute("red".to_string()));

    let route = Router::new()
    .route("/", get(get_book));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, route).await;
}

async fn get_book() -> Json<BookListing> {
    return Json(reqwest::get("https://openlibrary.org/search.json?q=@213921@@@@@").await.unwrap().json::<BookListing>().await.unwrap());
}

#[derive(Deserialize, Serialize)]
struct BookListing
{
    numFound: i32
}

#[derive(Error, Debug)]
pub enum Err
{
    #[error("get some internet m8")]
    NoInternet,
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}
