//! Configuration loading from JSON file
//!
//! Loads application settings from config.json located next to the executable.

use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

/// Application configuration loaded from config.json
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Message to display at the bottom of the window
    pub mentor_text: String,
    /// URL to open for hourly check-ins
    pub hourly_link: String,
    /// URL to open for 30-minute check-ins
    pub thirty_link: String,
    /// Audio files loaded from the songs folder
    #[serde(default)]
    pub songs: Vec<PathBuf>,
}

impl Config {
    /// Loads configuration from config.json and discovers audio files from songs folder
    pub fn load() -> Option<Self> {
        let exe_dir: PathBuf = std::env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Executable must live in a directory")
            .to_path_buf();

        let path = exe_dir.join("config.json");

        let raw = fs::read_to_string(&path)
            .unwrap();

        let mut config: Config = serde_json::from_str(&raw)
            .expect("Invalid JSON in config.json");

        config.songs = Self::load_songs();

        Some(config)
    }

    /// Scans the songs folder for supported audio files (.mp3, .wav, .ogg, .flac)
    fn load_songs() -> Vec<PathBuf> {
        let mut dir = std::env::current_exe().unwrap();
        dir.pop();
        dir.push("songs");

        let entries = match fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(_) => return Vec::new(), // songs folder missing -> no sounds
        };

        entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("mp3" | "wav" | "ogg" | "flac")
            )
            })
            .collect()
    }
}