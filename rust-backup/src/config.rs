use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Db {
    pub url: String,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize, Clone)]
pub struct Music {
    pub path: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub db: Db,
    pub music: Music,
}

impl Config {
    pub fn new() -> Self {
        toml::from_str(fs::read_to_string("config.toml").unwrap().as_str()).unwrap()
    }
}
