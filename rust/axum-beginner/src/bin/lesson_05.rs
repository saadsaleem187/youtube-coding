use std::{collections::HashMap, error};

use axum::{
    Router,
    extract::{Path, Query},
    routing::get,
};
use serde::Deserialize;

const URL: &str = "127.0.0.1:5000";

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    page: Option<u32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app = Router::new()
        .route("/hello/{name}", get(greet))
        .route("/users/{user_id}/posts/{post_id}", get(user_post))
        .route("/search", get(search))
        .route("/debug", get(debug_search));

    let listener = tokio::net::TcpListener::bind(URL).await?;

    println!("Server running on {URL}");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}\n")
}

async fn user_post(Path((user_id, post_id)): Path<(u32, u32)>) -> String {
    format!("User {} is viewing the post {}\n", user_id, post_id)
}

async fn search(Query(params): Query<SearchParams>) -> String {
    let page = params.page.unwrap_or(1);

    format!("User searching {} on page: {}\n", params.q, page)
}

async fn debug_search(Query(params): Query<HashMap<String, String>>) -> String {
    format!("User passes the query string: {params:?}\n")
}
