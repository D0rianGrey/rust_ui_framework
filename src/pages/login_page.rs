use anyhow::Result;
use thirtyfour::prelude::*;

use crate::pages::base_page::Page;

pub struct LoginPage {
    driver: WebDriver,
    username_input: By,
    password_input: By,
    login_button: By,
    error_message: By,
}

impl LoginPage {
    pub fn new(driver: WebDriver) -> Self {
        Self {
            driver,
            username_input: By::Id("user-name"),
            password_input: By::Id("password"),
            login_button: By::Id("login-button"),
            error_message: By::Css("h3[data-test='error']"),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<()> {
        self.type_text(self.username_input.clone(), username)
            .await?;
        self.type_text(self.password_input.clone(), password)
            .await?;
        self.click(self.login_button.clone()).await?;
        Ok(())
    }

    pub async fn get_error_message(&self) -> Result<String> {
        self.get_text(self.error_message.clone()).await
    }

    pub async fn open(&self, base_url: &str) -> Result<()> {
        self.navigate(base_url).await?;
        self.wait_for_page_load().await?;
        Ok(())
    }
}

impl Page for LoginPage {
    fn get_driver(&self) -> &WebDriver {
        &self.driver
    }
}
