use anyhow::{anyhow, Result};
use log::info;
use thirtyfour::prelude::*;
use std::time::Duration;

pub struct DriverManager {
    driver: WebDriver,
}

impl DriverManager {
    pub async fn new(browser_name: &str) -> Result<Self> {
        info!("Инициализация драйвера для браузера: {}", browser_name);
        
        // Создаем общие capabilities
        let mut caps = Capabilities::new();
        
        // Устанавливаем имя браузера
        match browser_name.to_lowercase().as_str() {
            "chrome" => {
                caps.insert("browserName".to_string(), "chrome".into());
            },
            "firefox" => {
                caps.insert("browserName".to_string(), "firefox".into());
            },
            _ => return Err(anyhow!("Неподдерживаемый браузер: {}", browser_name)),
        };
        
        // Создаем WebDriver с выбранными capabilities
        let driver = WebDriver::new("http://localhost:4444", caps).await?;
        
        // Установка таймаутов
        driver.set_implicit_wait_timeout(Duration::from_secs(10)).await?;
        driver.set_page_load_timeout(Duration::from_secs(30)).await?;
        
        Ok(Self { driver })
    }
    
    pub async fn navigate(&self, url: &str) -> Result<()> {
        info!("Переход по URL: {}", url);
        self.driver.goto(url).await?;
        Ok(())
    }
    
    pub async fn close(self) -> Result<()> {
        info!("Закрытие браузера");
        self.driver.quit().await?;
        Ok(())
    }
    
    pub fn get_driver(&self) -> &WebDriver {
        &self.driver
    }
}