//! Audio playback functionality
//!
//! Handles playing sound files when reminders trigger.

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

/// Audio output stream handler
pub struct Audio {
    stream: OutputStream,
}

impl Audio {
    /// Creates a new audio output stream
    pub fn new() -> Option<Self> {
        let stream = OutputStreamBuilder::open_default_stream().ok()?;
        Some(Self { stream })
    }

    /// Plays an audio file and returns a sink for controlling playback
    pub fn play_file(&self, path: PathBuf) -> Option<Sink> {
        let file = File::open(path).ok()?;
        let source = Decoder::new(BufReader::new(file)).ok()?;

        let sink = Sink::connect_new(self.stream.mixer());
        sink.append(source);
        Some(sink)
    }
}