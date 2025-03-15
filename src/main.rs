mod core;
mod config;
mod utils;
mod pages;
mod tests;

use anyhow::Result;
use log::info;
use std::env;

use crate::tests::login_test;
use crate::config::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация логгера
    crate::utils::logger::init_logger();
    
    // Если не задан CONFIG_PATH, используем локальную конфигурацию для разработки
    if env::var("CONFIG_PATH").is_err() {
        env::set_var("CONFIG_PATH", "config/local.toml");
    }
    
    // Загрузка конфигурации для вывода информации
    let config = AppConfig::new()?;
    
    println!("Фреймворк для автоматизации UI-тестирования на Rust");
    println!("Запуск тестов для сайта SauceDemo");
    println!("Используемый браузер: {}", config.browser.name);
    println!("URL Selenium: {}", config.environment.selenium_url);
    
    // Запуск тестов
    info!("Запуск тестов входа в систему");
    login_test::test_valid_login().await?;
    login_test::test_invalid_login().await?;
    
    println!("Все тесты успешно выполнены!");
    
    Ok(())
}