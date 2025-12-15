use std::{io::{self, Read, Write}, net::TcpStream};

pub const SERVER_ADDR: &str = "127.0.0.1:7878";

pub fn send_message(stream: &mut TcpStream, msg: &str) -> io::Result<()> {
    stream.write_all(msg.as_bytes())?;
    stream.flush()?;

    Ok(())
}

pub fn read_message(stream: &mut TcpStream) -> io::Result<String> {
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;

    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}
