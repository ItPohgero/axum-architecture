use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    name: String,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
    username: String,
    password: String,
}

#[test]
fn environment() {
    let config = Config::builder()
        .add_source(File::new("application.yaml", FileFormat::Yaml))
        .build().unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap();

    assert_eq!(app_config.name, "My Application");
    assert_eq!(app_config.database.host, "localhost");
    assert_eq!(app_config.database.port, 5432);
    assert_eq!(app_config.database.name, "rust");
    assert_eq!(app_config.database.username, "asani");
    assert_eq!(app_config.database.password, "root");
}