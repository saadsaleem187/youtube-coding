use std::{io::{self, stdin, BufRead, BufReader}, net::TcpStream};

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

    let stream = TcpStream::connect(SERVER_ADDR)?;

    println!("Connected! Type a message or (CTRL+C) to exit:");

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;

    let stdin = stdin();

    for line in stdin.lock().lines() {
        let message = line?;

        if message.trim().is_empty() {
            continue;
        }

        send_message(&mut writer, &message)?;

        match read_message(&mut reader)? {
            Some(reply) => println!("Server: {}", reply),
            None => {
                println!("Server disconnected");
                break;
            }
        }
    }

    Ok(())
}
