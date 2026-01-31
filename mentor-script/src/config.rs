use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub mentor_text: String,
    pub hourly_link: String,
    pub thirty_link: String,
    #[serde(default)]
    pub songs: Vec<PathBuf>,
}

impl Config {
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