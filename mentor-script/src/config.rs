//! Configuration loading from JSON file
//!
//! Loads application settings from config.json located next to the executable.

use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Application configuration loaded from config.json
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Message to display at the bottom of the window
    pub mentor_text: String,
    /// URL to open for hourly check-ins
    pub hourly_link: String,
    /// URL to open for 30-minute check-ins
    pub thirty_link: String,

    /// Folder containing audio files (can be anywhere).
    ///
    /// If relative, it is resolved relative to the executable's directory.
    #[serde(default, alias = "SONG_FOLDER")]
    pub songs_dir: PathBuf,

    /// Audio files discovered from `songs_dir`
    #[serde(skip)]
    pub songs: Vec<PathBuf>,
}

impl Config {
    /// Loads configuration from config.json and discovers audio files from songs_dir
    pub fn load() -> Option<Self> {
        let exe_dir: PathBuf = std::env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Executable must live in a directory")
            .to_path_buf();

        let path = exe_dir.join("config.json");

        let raw = fs::read_to_string(&path).unwrap();

        let mut config: Config =
            serde_json::from_str(&raw).expect("Invalid JSON in config.json");

        // Resolve songs_dir:
        // - if missing/empty => default to <exe_dir>/songs
        // - if relative => resolve relative to exe_dir
        // - always produce an absolute path
        config.songs_dir = Self::resolve_songs_dir(&exe_dir, &config.songs_dir);

        config.songs = Self::load_songs_from(&config.songs_dir);

        Some(config)
    }

    fn resolve_songs_dir(config_dir: &Path, configured: &Path) -> PathBuf {
        let resolved = if configured.as_os_str().is_empty() {
            config_dir.join("songs")
        } else if configured.is_relative() {
            config_dir.join(configured)
        } else {
            configured.to_path_buf()
        };

        // Canonicalize when possible for a normalized full path. If the folder
        // doesn't exist yet, keep the resolved absolute path as-is.
        fs::canonicalize(&resolved).unwrap_or(resolved)
    }

    /// Scans a folder for supported audio files (.mp3, .wav, .ogg, .flac)
    fn load_songs_from(dir: &Path) -> Vec<PathBuf> {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return Vec::new(), // folder missing/unreadable -> no sounds
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

    /// Opens the configured songs folder in the OS file explorer.
    pub fn open_songs_folder(&self) {
        let dir = &self.songs_dir;

        let spawn_result = if cfg!(target_os = "windows") {
            Command::new("explorer").arg(dir).spawn()
        } else if cfg!(target_os = "macos") {
            Command::new("open").arg(dir).spawn()
        } else if cfg!(target_os = "linux") {
            Command::new("xdg-open").arg(dir).spawn()
        } else {
            println!("Unsupported operating system for opening file explorer automatically.");
            return;
        };

        if let Err(e) = spawn_result {
            eprintln!("Failed to open songs folder: {e}");
        }
    }
}
