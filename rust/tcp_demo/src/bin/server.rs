use std::{io::{self, BufReader}, net::{TcpListener, TcpStream}, thread};

use tcp_demo::{read_message, send_message, SERVER_ADDR};

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

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;

    println!("New connection from: {}", peer_addr);

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;

    loop {
        match read_message(&mut reader)? {
            Some(msg) => {
                println!("[{}]: {}", peer_addr, msg);

                let reply = format!("{}", msg);
                send_message(&mut writer, &reply)?;
            },
            None => {
                println!("Client disconnected");
                break;
            }
        }
    }

    Ok(())
}
