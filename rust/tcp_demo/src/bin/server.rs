use std::{io::{self, Read, Write}, net::{TcpListener, TcpStream}, thread};

use tcp_demo::SERVER_ADDR;

fn main() {
    if let Err(e) = run_server() {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}

fn run_server() -> io::Result<()> {
    let listener = TcpListener::bind(SERVER_ADDR)?;

    println!("Server is listening on {}", SERVER_ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprint!("Error handling client: {}", e);
                    }
                });
            },
            Err(e) => {
                eprint!("Failed to connect: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;

    println!("New connection from: {}", peer_addr);

    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", peer_addr);
                break;
            },
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from {}: {}", peer_addr, received.trim());

                let response = format!("Echo: {}", received);

                stream.write_all(response.as_bytes())?;
            },
            Err(e) => {
                eprintln!("Error reading from {}: {}", peer_addr, e);
                return Err(e)
            }
        }
    }

    Ok(())
}
