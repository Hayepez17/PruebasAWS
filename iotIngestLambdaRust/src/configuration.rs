// src/configuration.rs
use std::env;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub endpoint: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database_name: String
}

pub fn get_configuration() -> Result<Settings, std::env::VarError> {
    let database = DatabaseSettings {
        endpoint: env::var("DB_HOST_URL")?,
        port: env::var("DB_PORT")?.parse().unwrap_or(5432),
        user: env::var("DB_USERNAME")?,
        password: env::var("DB_PASSWORD")?,
        database_name: env::var("DB_NAME")?,
    };

    Ok(Settings { database })
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.endpoint, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password, self.endpoint, self.port
        )
    }
    
}