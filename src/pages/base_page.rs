use anyhow::Result;
use log::info;
use std::time::Duration;
use thirtyfour::prelude::*;
use std::future::Future;

// Трейт для всех страниц
pub trait Page {
    fn get_driver(&self) -> &WebDriver;
    
    // Базовые методы навигации
    fn navigate<'a>(&'a self, url: &'a str) -> impl Future<Output = Result<()>> + Send + 'a {
        async move {
            info!("Переход на страницу: {}", url);
            self.get_driver().goto(url).await?;
            Ok(())
        }
    }
    
    // Ожидание загрузки страницы
    fn wait_for_page_load<'a>(&'a self) -> impl Future<Output = Result<()>> + Send + 'a {
        async move {
            // Ждем, пока document.readyState не станет 'complete'
            let script = r#"return document.readyState === 'complete';"#;
            let driver = self.get_driver();
            
            // Выполняем JavaScript и проверяем, что страница загружена
            let result = driver.execute(script, vec![]).await?;
            let value = result.json();
            
            if let Some(is_complete) = value.as_bool() {
                if !is_complete {
                    // Ждем еще немного, если страница не загружена
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
                
            Ok(())
        }
    }
    
    // И так далее для остальных методов...
}