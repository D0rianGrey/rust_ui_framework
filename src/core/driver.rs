use anyhow::{anyhow, Result};
use log::info;
use thirtyfour::prelude::*;
use std::time::Duration;

use crate::config::config::AppConfig;

pub struct DriverManager {
    driver: WebDriver,
}

impl DriverManager {
    pub async fn new(config: &AppConfig) -> Result<Self> {
        let browser_name = &config.browser.name;
        let selenium_url = match config.environment.selenium_url.as_ref() {
            "" => "http://localhost:4444",
            url => url,
        };
        
        info!("Инициализация драйвера для браузера: {} на {}", browser_name, selenium_url);
        
        let driver = match browser_name.to_lowercase().as_str() {
            "chrome" => {
                let mut caps = Capabilities::new();
                caps.insert("browserName".to_string(), "chrome".into());
                
                // Добавляем настройку headless, если указано в конфигурации
                if config.browser.headless {
                    let chrome_options = serde_json::json!({
                        "args": ["--headless", "--disable-gpu", "--no-sandbox"]
                    });
                    caps.insert("goog:chromeOptions".to_string(), chrome_options.into());
                }
                
                WebDriver::new(selenium_url, caps).await?
            },
            "firefox" => {
                let mut caps = Capabilities::new();
                caps.insert("browserName".to_string(), "firefox".into());
                
                // Добавляем настройку headless, если указано в конфигурации
                if config.browser.headless {
                    let firefox_options = serde_json::json!({
                        "args": ["-headless"]
                    });
                    caps.insert("moz:firefoxOptions".to_string(), firefox_options.into());
                }
                
                WebDriver::new(selenium_url, caps).await?
            },
            _ => return Err(anyhow!("Неподдерживаемый браузер: {}", browser_name)),
        };
        
        // Установка таймаутов
        let timeout = Duration::from_secs(config.environment.timeout);
        driver.set_implicit_wait_timeout(timeout).await?;
        driver.set_page_load_timeout(timeout).await?;
        
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