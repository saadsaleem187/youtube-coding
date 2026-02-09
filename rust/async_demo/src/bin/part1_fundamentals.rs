// Part 1: Rust Async Fundamentals
// Cargo.toml dependencies
// [dependencies]
// tokio = { version = "1.49.0", features = ["full"] }

use std::time::{Duration, Instant};

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("\n=== Example 1: Basic async function ===");
    say_hello().await;

    println!("\n=== Example 2: Delay async function ===");
    delayed_greeting("Saad", 1000).await;

    println!("\n=== Example 3: Lazy futures ===");
    let future = lazy_future(); // nothing prints yet
    println!("Future created but not awaited");
    let result = future.await;
    println!("Result: {}", result);

    println!("\n=== Example 4a: Sequential Execution ===");
    let start = Instant::now();
    let result1 = fetch_data(1).await;
    let result2 = fetch_data(2).await;
    let result3 = fetch_data(3).await;
    println!("Sequential results: {}, {}, {}", result1, result2, result3);
    println!("Time taken: {:?}", start.elapsed());

    println!("\n=== Example 4b: Concurrent Execution ===");
    let start = Instant::now();
    let (result1, result2, result3) = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3)
    );
    println!("Concurrent results: {}, {}, {}", result1, result2, result3);
    println!("Time taken: {:?}\n", start.elapsed());
}

// Example 1: Basic async function
async fn say_hello() {
    println!("Hello from the async function");
}

// Example 2: Async delayed
async fn delayed_greeting(name: &str, delay_ms: u64) {
    sleep(Duration::from_millis(delay_ms)).await;
    println!("Hello, {} (after {} ms)", name, delay_ms);
}

// Example 3: Demonstrating lazy futures
async fn lazy_future() -> String {
    println!("This only prints when awaited.");
    "Result".to_string()
}

// Example 4: Sequential vs Concurrent execution
async fn fetch_data(id: u32) -> u32 {
    println!("Fetching data for id: {}", id);
    sleep(Duration::from_secs(2)).await;
    println!("Data fetched for id: {}", id);
    id * 10
}
