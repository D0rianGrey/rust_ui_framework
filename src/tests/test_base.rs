use anyhow::Result;
use log::info;
use std::sync::Once;
use std::env;

use crate::core::driver::DriverManager;
use crate::config::config::AppConfig;

// Инициализация логгера только один раз
static INIT: Once = Once::new();

pub struct TestContext {
    pub driver_manager: DriverManager,
    pub config: AppConfig,
}

impl TestContext {
    pub async fn new() -> Result<Self> {
        // Инициализируем логгер
        INIT.call_once(|| {
            crate::utils::logger::init_logger();
        });
        
        info!("Инициализация контекста теста");
        
        // Устанавливаем переменную окружения CONFIG_PATH, если она не установлена
        if env::var("CONFIG_PATH").is_err() {
            env::set_var("CONFIG_PATH", "config/local.toml");
        }
        
        // Загружаем конфигурацию
        let config = AppConfig::new()?;
        info!("Конфигурация загружена: {:?}", config);
        
        // Создаем экземпляр драйвера с передачей конфигурации
        let driver_manager = DriverManager::new(&config).await?;
        
        Ok(Self {
            driver_manager,
            config,
        })
    }
    
    pub async fn cleanup(self) -> Result<()> {
        info!("Завершение теста, освобождение ресурсов");
        self.driver_manager.close().await?;
        Ok(())
    }
}