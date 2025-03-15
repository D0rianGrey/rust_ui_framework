use anyhow::Result;
use log::info;
use std::time::Duration;
use thirtyfour::prelude::*;
use std::future::Future;

// Трейт для всех страниц с ограничением Sync
pub trait Page: Sync {
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
    
    // Ожидание видимости элемента с явным ожиданием
    fn wait_for_element_visible<'a>(&'a self, locator: By) -> impl Future<Output = Result<WebElement>> + Send + 'a {
        async move {
            info!("Ожидание видимости элемента: {:?}", locator);
            let driver = self.get_driver();
            
            // Начальное время для отслеживания таймаута
            let start = std::time::Instant::now();
            let timeout = Duration::from_secs(10);
            
            // Пытаемся найти элемент, пока не истечет таймаут
            loop {
                match driver.find(locator.clone()).await {
                    Ok(element) => {
                        // Проверяем, видим ли элемент
                        if let Ok(true) = element.is_displayed().await {
                            return Ok(element);
                        }
                    },
                    Err(_) => {}
                }
                
                // Проверяем, не истек ли таймаут
                if start.elapsed() > timeout {
                    return Err(anyhow::anyhow!("Таймаут ожидания видимости элемента"));
                }
                
                // Делаем небольшую паузу перед следующей попыткой
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
    
    // Ожидание кликабельности элемента
    fn wait_for_element_clickable<'a>(&'a self, locator: By) -> impl Future<Output = Result<WebElement>> + Send + 'a {
        async move {
            info!("Ожидание кликабельности элемента: {:?}", locator);
            let element = self.wait_for_element_visible(locator).await?;
            
            // Проверяем, что элемент доступен для клика
            if !element.is_enabled().await? {
                return Err(anyhow::anyhow!("Элемент не доступен для клика"));
            }
            
            Ok(element)
        }
    }
    
    // Клик по элементу с ожиданием
    fn click<'a>(&'a self, locator: By) -> impl Future<Output = Result<()>> + Send + 'a {
        async move {
            info!("Клик по элементу: {:?}", locator);
            let element = self.wait_for_element_clickable(locator).await?;
            element.click().await?;
            Ok(())
        }
    }
    
    // Ввод текста в элемент
    fn type_text<'a>(&'a self, locator: By, text: &'a str) -> impl Future<Output = Result<()>> + Send + 'a {
        async move {
            info!("Ввод текста '{}' в элемент: {:?}", text, locator);
            let element = self.wait_for_element_visible(locator).await?;
            element.clear().await?;
            element.send_keys(text).await?;
            Ok(())
        }
    }
    
    // Получение текста элемента
    fn get_text<'a>(&'a self, locator: By) -> impl Future<Output = Result<String>> + Send + 'a {
        async move {
            info!("Получение текста элемента: {:?}", locator);
            let element = self.wait_for_element_visible(locator).await?;
            let text = element.text().await?;
            Ok(text)
        }
    }
}