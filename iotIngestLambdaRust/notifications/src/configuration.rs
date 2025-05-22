// src/configuration.rs
use std::env;
use common_lib::DatabaseSettings;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub email_from: String,
    pub telegram_token: String,
}

pub fn get_configuration() -> Result<Settings, std::env::VarError> {
    let database = DatabaseSettings {
        endpoint: env::var("DB_HOST_URL")?,
        port: env::var("DB_PORT")?.parse().unwrap_or(3306),
        user: env::var("DB_USERNAME")?,
        password: env::var("DB_PASSWORD")?,
        database_name: env::var("DB_NAME")?,
    };

    let email_from = env::var("EMAIL_FROM")?;
    let telegram_token = env::var("TELEGRAM_BOT_TOKEN")?;

    Ok(Settings { database, email_from, telegram_token })
    // Ok(Settings {database})
}
