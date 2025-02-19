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
    port: i32,
    name: String,
    username: String,
    password: String,
}

#[test]
fn environment() {
    let config = Config::builder()
        .add_source(File::new("env.yaml", FileFormat::Yaml))
        .build().unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap();

    assert_eq!(app_config.name, "My Application");
    assert_eq!(app_config.database.host, "localhost");
    assert_eq!(app_config.database.port, 5432);
    assert_eq!(app_config.database.name, "my_database");
    assert_eq!(app_config.database.username, "user");
    assert_eq!(app_config.database.password, "password");
}