use std::fs::File;
use std::io::BufReader;
use rand::seq::IndexedRandom;
use rand::thread_rng;
use rodio::{Decoder, OutputStream, Sink};

pub fn pick_random_song(urls: &[String]) -> Option<&String> {
    let mut rng = rand::rng();
    urls.choose(&mut rng)
}

/// Returns a sink. That's it.
pub fn play_sound(stream: &OutputStream, url: &str) -> Sink{
    let response = reqwest::blocking::get(url).unwrap();
    let bytes = response.bytes().unwrap();

    let cursor = std::io::Cursor::new(bytes);
    let source = Decoder::new(cursor).unwrap();

    let sink = Sink::connect_new(stream.mixer());
    sink.append(source);
    sink // sink
}