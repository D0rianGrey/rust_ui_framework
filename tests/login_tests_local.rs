use anyhow::Result;
use std::env;

// Импортируем тесты
use rust_ui_framework::tests::login_test;

#[tokio::test]
async fn test_valid_login_works() -> Result<()> {
    // Устанавливаем путь к конфигурации для теста
    env::set_var("CONFIG_PATH", "config/local.toml");
    
    login_test::test_valid_login().await
}

#[tokio::test]
async fn test_invalid_login_shows_error() -> Result<()> {
    // Устанавливаем путь к конфигурации для теста
    env::set_var("CONFIG_PATH", "config/local.toml");
    
    login_test::test_invalid_login().await
}