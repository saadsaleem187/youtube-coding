use std::{
    error,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    Json, Router,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use serde_json::{Value, json};

const URL: &str = "127.0.0.1:5000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app = Router::new()
        .route("/text", get(plain_text))
        .route("/owned", get(owned_string))
        .route("/html", get(html_page))
        .route("/json", get(json_data))
        .route("/items", post(create_item))
        .route("/headers", get(with_headers))
        .route("/coin", get(coin_flip))
        .fallback(not_found);

    let listener = tokio::net::TcpListener::bind(URL).await?;

    println!("Server running on {URL}");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn plain_text() -> &'static str {
    "This is the plain text\n"
}

async fn owned_string() -> String {
    let result = 7 * 5;

    format!("The answer is {result}\n")
}

async fn html_page() -> Html<&'static str> {
    Html("<h1>Axum</h1><p>We are creating the AXUM API's</p>\n")
}

async fn json_data() -> Json<Value> {
    Json(json!({
        "framework": "axum",
        "language": "rust"
    }))
}

async fn create_item() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "New item created")
}

async fn with_headers() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("x-powered-by", "axum"), ("lesson", "4")],
        "Tuple with the status code, custom header and body\n",
    )
}

async fn coin_flip() -> Response {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    if nanos % 2 == 0 {
        Html("<h1>It's heads. You win</h1>\n").into_response()
    } else {
        (StatusCode::IM_A_TEAPOT, "Tails....You loose\n").into_response()
    }
}

async fn not_found() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::NOT_FOUND,
        Html("<h1>404</h1><p>Not in the listing</p>\n<a href=''>Go back to home page..</a>"),
    )
}

