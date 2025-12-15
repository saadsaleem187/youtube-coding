use std::{io::{self, stdin, BufRead}, net::TcpStream};

use tcp_demo::{read_message, send_message, SERVER_ADDR};

fn main() {
    if let Err(e) = run_client() {
        eprintln!("Client error: {}", e);
        std::process::exit(1);
    }

    println!("Client disconnected.");
}

fn run_client() -> io::Result<()> {
    println!("Connecting to {}...", SERVER_ADDR);

    let mut stream = TcpStream::connect(SERVER_ADDR)?;

    println!("Connected! Type a message or (CTRL+C) to exit:");

    let stdin = stdin();

    for line in stdin.lock().lines() {
        let message = line?;

        if message.trim().is_empty() {
            continue;
        }

        send_message(&mut stream, &message)?;

        let response = read_message(&mut stream)?;

        println!("Server: {}", response.trim());
    }

    Ok(())
}
