use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub mentor_text: String,
    pub hourly_link: String,
    pub thirty_link: String,
}

impl Config {
    pub fn load() -> Self {
        let raw = fs::read_to_string("links.json")
            .expect("Failed to read links.json");

        serde_json::from_str(&raw)
            .expect("Invalid JSON in links.json")
    }
}