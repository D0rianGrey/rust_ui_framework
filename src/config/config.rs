use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub browser: BrowserConfig,
    pub environment: EnvironmentConfig,
}

#[derive(Debug, Deserialize)]
pub struct BrowserConfig {
    pub name: String,
    pub headless: bool,
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentConfig {
    pub base_url: String,
    pub timeout: u64,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| String::from("config/default.toml"));

        let path = Path::new(&config_path);
        if !path.exists() {
            eprintln!("Файл конфигурации не найден: {}", config_path);
        }

        let config = Config::builder().add_source(File::from(path)).build()?;

        config.try_deserialize()
    }
}
