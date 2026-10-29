use std::error;

use axum::{Router, routing::get};

const URL: &str = "127.0.0.1:5000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind(URL).await?;

    println!("Server running on {}", URL);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello() -> &'static str {
    "Hello, Axum!\n"
}
