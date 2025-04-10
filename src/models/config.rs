use crate::constants::error::ErrorMessages;
use serde::Deserialize;
use serde_json::from_str;
use std::fs::read_to_string;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load() -> Self {
        let config = read_to_string("assets/config.json").expect(ErrorMessages::OPEN_CONFIG);
        return from_str(&config).expect(ErrorMessages::READ_CONFIG);
    }
}
