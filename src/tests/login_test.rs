use anyhow::Result;
use log::info;

use crate::tests::test_base::TestContext;
use crate::pages::login_page::LoginPage;

pub async fn test_valid_login() -> Result<()> {
    info!("Запуск теста успешного входа в систему");
    
    // Инициализация контекста теста
    let context = TestContext::new().await?;
    
    // Получаем конфигурацию
    let base_url = &context.config.environment.base_url;
    
    // Создаем страницу логина, передавая ей WebDriver
    let login_page = LoginPage::new(context.driver_manager.get_driver().clone());
    
    // Выполняем действия теста
    login_page.open(base_url).await?;
    login_page.login("standard_user", "secret_sauce").await?;
    
    // Здесь должна быть проверка успешного входа
    // Например, проверка URL или наличие определенного элемента
    
    // Освобождаем ресурсы
    context.cleanup().await?;
    
    info!("Тест успешного входа завершен");
    Ok(())
}

pub async fn test_invalid_login() -> Result<()> {
    info!("Запуск теста неудачного входа в систему");
    
    // Инициализация контекста теста
    let context = TestContext::new().await?;
    
    // Получаем конфигурацию
    let base_url = &context.config.environment.base_url;
    
    // Создаем страницу логина, передавая ей WebDriver
    let login_page = LoginPage::new(context.driver_manager.get_driver().clone());
    
    // Выполняем действия теста
    login_page.open(base_url).await?;
    login_page.login("locked_out_user", "secret_sauce").await?;
    
    // Проверяем наличие сообщения об ошибке
    let error_message = login_page.get_error_message().await?;
    assert!(error_message.contains("locked out"), 
            "Ожидалось сообщение о блокировке, получено: {}", error_message);
    
    // Освобождаем ресурсы
    context.cleanup().await?;
    
    info!("Тест неудачного входа завершен");
    Ok(())
}