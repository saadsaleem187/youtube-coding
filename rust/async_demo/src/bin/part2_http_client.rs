// Part 2: Rust Async HTTP Client
// Cargo.toml dependencies
// [dependencies]
// tokio = { version = "1.49.0", features = ["full"] }
// reqwest = { version = "0.13", features = ["json"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0.149"

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::time::timeout;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Post {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

#[derive(Serialize)]
struct NewPost {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

const URL: &str = "https://jsonplaceholder.typicode.com/posts";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Async HTTP Client ===");

    simple_get_request().await?;
    fetch_json().await?;
    sequential_requests().await?;
    concurrent_requests_join().await?;
    concurrent_with_try_join().await?;
    dynamic_concurrent_requests(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).await?;
    create_post().await?;
    request_with_timeout().await?;

    Ok(())
}

async fn fetch_posts(id: u32) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
    let url: String = format!("{URL}/{}", id);
    let post: Post = reqwest::get(url).await?.json().await?;

    Ok(post)
}

// Example 1: Simple GET request 
async fn simple_get_request() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Example 1: Simple GET request ===");

    let url: String = format!("{URL}/1");

    println!("Fetching: {}", url);

    let response = reqwest::get(url).await?;

    println!("Response status: {}", response.status());
    let body = response.text().await?;
    println!("Body: {}", body);

    Ok(())
}

// Example 2: Fetch JSON 
async fn fetch_json() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Example 2: Fetch JSON ===");

    let url: String = format!("{URL}/1");

    let post: Post = reqwest::get(url).await?.json().await?;

    println!("Post: {:#?}", post);

    Ok(())
}

// Example 3: Multiple Sequential requests (SLOW)
async fn sequential_requests() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Example 3: Multiple Sequential Request ===");
    
    let start = Instant::now();

    println!("\nFetching posts sequentially...\n");

    for id in 1..=5 {
        let url: String = format!("{URL}/{}", id);
        let post: Post = reqwest::get(url).await?.json().await?;
        println!("Got Post {}: {}", id, post.title);
    }

    println!("\nSequential Time: {:?}", start.elapsed());

    Ok(())
}

// Example 4: Concurrent requests using join! (FAST)
async fn concurrent_requests_join() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Example 4: Concurrent Requests ===");

    let start = Instant::now();

    println!("\nFetching posts concurrently...\n");

    let (post1, post2, post3, post4, post5) = tokio::join!(
        fetch_posts(1),
        fetch_posts(2),
        fetch_posts(3),
        fetch_posts(4),
        fetch_posts(5),
    );

    for result in [post1, post2, post3, post4, post5] {
        match result {
            Ok(post) => println!("Got post: {}", post.title),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Concurrent Time: {:?}", start.elapsed());

    Ok(())
}

// Example 5: Concurrent requests with try_join!
async fn concurrent_with_try_join() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Example 5: Concurrent requests with try_join ===");
    println!("\nUsing try_join! (stops on first error)");

    let (post1, post2, post3) = tokio::try_join!(
        fetch_posts(1),
        fetch_posts(2),
        fetch_posts(3),
    )?;
    
    println!("All succeeded...");
    println!("Post 1: {}", post1.title);
    println!("Post 2: {}", post2.title);
    println!("Post 3: {}", post3.title);

    Ok(())
}

// Example 6: Dynamic concurrent Requests
async fn dynamic_concurrent_requests(ids: Vec<u32>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Example 6: Dynamic Concurrent Requests ===");
    println!("\nFetching posts {} concurrently...", ids.len());

    let start = Instant::now();
    let mut handles = vec![];

    for id in ids {
        let handle = tokio::spawn(async move {
            fetch_posts(id).await
        });

        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(Ok(post)) => println!("Got post: {}", post.title),
            Ok(Err(e)) => println!("Request error: {}", e),
            Err(e) => println!("Task error: {}", e),
        }
    }

    println!("Time: {:?}", start.elapsed());

    Ok(())
}

// Example 7: Create new post
async fn create_post() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Example 7: Create new post ===");
    
    let new_post: NewPost = NewPost {
        title: "Rust Async".to_string(),
        body: "Learning async http client".to_string(),
        user_id: 1,
    };

    let client = reqwest::Client::new();
    let response = client.post(URL).json(&new_post).send().await?;

    println!("Respone status: {}", response.status());
    let created: Post = response.json().await?;
    println!("Post created: {:#?}", created);

    Ok(())
}

// Example 8: Timeout handling
async fn request_with_timeout() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("\n=== Example 8: Request with timeout ===");
    println!("\nFetching with timeout...");

    let result = timeout(Duration::from_secs(5), fetch_posts(1)).await;

    match result {
        Ok(Ok(post)) => println!("Got post: {}", post.title),
        Ok(Err(e)) => println!("Request error: {}", e),
        Err(_) => println!("Timeout"),
    }

    Ok(())
}
