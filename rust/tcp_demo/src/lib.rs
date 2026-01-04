use std::{io::{self, BufRead, BufReader, Write}, net::TcpStream};

pub const SERVER_ADDR: &str = "127.0.0.1:7878";

pub fn send_message(stream: &mut TcpStream, msg: &str) -> io::Result<()> {
    stream.write_all(msg.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    Ok(())
}

pub fn read_message(reader: &mut BufReader<TcpStream>) -> io::Result<Option<String>> {
    let mut message = String::new();
    let bytes = reader.read_line(&mut message)?;

    if bytes == 0 {
        return Ok(None);
    }

    Ok(Some(message.trim_end().to_string()))
}
