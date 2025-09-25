use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::{fs::File, io::BufReader};

fn main() {
    let stream_handler = OutputStreamBuilder::open_default_stream().expect("Failed to load the audio stream");
    let sink = Sink::connect_new(&stream_handler.mixer());

    let file = File::open("sound.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}
