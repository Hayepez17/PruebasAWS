// src/configuration.rs
use std::env;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub aws: AwsSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub endpoint: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database_name: String
}

#[derive(serde::Deserialize)]
pub struct AwsSettings {
    pub bucket_name: String,
}

pub fn get_configuration() -> Result<Settings, std::env::VarError> {
    let database = DatabaseSettings {
        endpoint: env::var("DB_HOST_URL")?,
        port: env::var("DB_PORT")?.parse().unwrap_or(3306),
        user: env::var("DB_USERNAME")?,
        password: env::var("DB_PASSWORD")?,
        database_name: env::var("DB_NAME")?,
    };

    let aws = AwsSettings {
        bucket_name: env::var("AWS_BUCKET_NAME")?,
    };

    Ok(Settings { database, aws })
    // Ok(Settings {database})
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.endpoint, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            self.user, self.password, self.endpoint, self.port
        )
    }
}

impl AwsSettings {
    pub fn bucket_name(&self) -> String {
        self.bucket_name.clone()
    }

    pub fn get_bucket_key(&self, device_location_id: &str, year: &str, month: &str, day: &str) -> String {
        format!("{}/device_location_id={}/year={}/month={}/day={}/device_location_metrics.json", self.bucket_name,device_location_id, year, month, day)
    }
    
}