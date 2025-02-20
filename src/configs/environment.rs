use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub port: u16,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}

pub fn main() -> AppConfig {
    let config = Config::builder()
        .add_source(File::new("env.yaml", FileFormat::Yaml))
        .build().unwrap();

    config.try_deserialize().unwrap()
}