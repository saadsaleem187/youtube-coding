use std::time::{Duration, Instant};

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("1) one after another:");

    let start = Instant::now();
    let coffee = make_coffee().await;
    let toast = make_toast().await;

    println!("Got {coffee} and {toast} in {:?}\n", start.elapsed());
    println!("2) at the same time:");

    let start = Instant::now();
    let (coffee, toast) = tokio::join!(make_coffee(), make_toast());

    println!("Got {coffee} and {toast} in {:?}\n", start.elapsed());

    println!("3) on a separate task:");

    let handle = tokio::spawn(make_toast());
    println!("  main is free to work on another thing.");
    let toast = handle.await.unwrap();

    println!("Spawned task returned {toast}");
}

async fn make_coffee() -> String {
    println!("  Boiling water for coffee...");
    sleep(Duration::from_secs(2)).await;
    println!("  coffee finished");

    String::from("coffee")
}

async fn make_toast() -> String {
    println!("  Toasting bread...");
    sleep(Duration::from_secs(1)).await;
    println!("  Toasting finisehd");

    String::from("toast")
}
