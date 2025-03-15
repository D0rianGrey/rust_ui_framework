
use anyhow::Result;
use rust_ui_framework::tests::login_test;

#[tokio::test]
async fn test_valid_login_works() -> Result<()> {
    login_test::test_valid_login().await
}

#[tokio::test]
async fn test_invalid_login_shows_error() -> Result<()> {
    login_test::test_invalid_login().await
}