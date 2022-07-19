use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Db {
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub db: Db,
}

impl Config {
    pub fn new() -> Self {
        toml::from_str(fs::read_to_string("config.toml").unwrap().as_str()).unwrap()
    }
}
