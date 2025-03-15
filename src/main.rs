mod core;
mod config;
mod utils;
mod pages;
mod tests;

use anyhow::Result;
use crate::config::config::AppConfig;
use crate::utils::logger;

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация логгера
    logger::init_logger();
    
    // Загрузка конфигурации
    let config = AppConfig::new()?;
    
    println!("Фреймворк для автоматизации UI-тестирования на Rust успешно инициализирован!");
    println!("Используется браузер: {}", config.browser.name);
    println!("Базовый URL: {}", config.environment.base_url);
    
    Ok(())
}