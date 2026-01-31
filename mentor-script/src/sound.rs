use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

pub struct Audio {
    stream: OutputStream,
}

impl Audio {
    pub fn new() -> Option<Self> {
        let stream = OutputStreamBuilder::open_default_stream().ok()?;
        Some(Self { stream })
    }

    pub fn play_file(&self, path: PathBuf) -> Option<Sink> {
        let file = File::open(path).ok()?;
        let source = Decoder::new(BufReader::new(file)).ok()?;

        let sink = Sink::connect_new(self.stream.mixer());
        sink.append(source);
        Some(sink)
    }
}

// pub fn pick_random_song(urls: &[String]) -> Option<&String> {
//     let mut rng = rand::rng();
//     urls.choose(&mut rng)
// }
//
// /// Returns a sink. That's it.
// pub fn play_sound(stream: &OutputStream, url: &str) -> Sink{
//     let response = reqwest::blocking::get(url).unwrap();
//     let bytes = response.bytes().unwrap();
//
//     let cursor = Cursor::new(bytes);
//     let source = Decoder::new(cursor).unwrap();
//
//     let sink = Sink::connect_new(stream.mixer());
//     sink.append(source);
//     sink // sink
// }