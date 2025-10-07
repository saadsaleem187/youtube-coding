use std::io::Cursor;
use anyhow::Result;
use rodio::{Decoder, OutputStreamBuilder, Sink};

fn main() -> Result<()> {
    let url = "https://samplelib.com/lib/preview/mp3/sample-3s.mp3";

    let response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP error: {}", response.status());
    }

    let bytes = response.bytes()?;

    let device = OutputStreamBuilder::open_default_stream().expect("Failed to connect to audio device");
    let sink = Sink::connect_new(&device.mixer());

    let cursor = Cursor::new(bytes);
    let source = Decoder::new(cursor).unwrap();
    
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
