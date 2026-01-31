use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub mentor_text: String,
    pub hourly_link: String,
    pub thirty_link: String,
    pub song_urls: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        let exe_dir: PathBuf = std::env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Executable must live in a directory")
            .to_path_buf();

        let path = exe_dir.join("links.json");

        let raw = fs::read_to_string(&path)
        .unwrap();

        serde_json::from_str(&raw)
            .expect("Invalid JSON in links.json")
    }
}