use std::error;

use axum::{
    Router,
    routing::{get, post},
};

const URL: &str = "127.0.0.1:5000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let api_routes = Router::new()
        .route("/status", get(api_status))
        .route("/version", get(api_version));

    let admin_routes = Router::new().route("/dashboard", get(admin_dashboard));

    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/contact", post(send_contact))
        .route(
            "/messages",
            get(list_messages)
                .post(create_message)
                .delete(delete_messages),
        )
        .nest("/api", api_routes)
        .nest("/admin", admin_routes);

    let listener = tokio::net::TcpListener::bind(URL).await?;

    println!("Server is running on {URL}");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn home() -> &'static str {
    "Welcome to the home page\n"
}

async fn about() -> &'static str {
    "This is the axum server\n"
}

async fn send_contact() -> &'static str {
    "Contact request received\n"
}

async fn list_messages() -> &'static str {
    "All messages\n"
}

async fn create_message() -> &'static str {
    "New message created\n"
}

async fn api_status() -> &'static str {
    "API is active\n"
}

async fn api_version() -> &'static str {
    "API version 1.0\n"
}

async fn admin_dashboard() -> &'static str {
    "Welcome to the admin dashboard\n"
}

async fn delete_messages() -> &'static str {
    "All messages deleted\n"
}
