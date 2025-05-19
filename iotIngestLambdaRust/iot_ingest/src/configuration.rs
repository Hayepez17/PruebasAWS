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
    pub database_name: String,
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

pub fn get_bucket_key(
    datetime: chrono::DateTime<chrono::Utc>,
) -> String {
    let year = datetime.format("%Y").to_string();
    let month = datetime.format("%m").to_string();
    let day = datetime.format("%d").to_string();
    format!(
        "data-lake-metrics-teg-hy/year={}/month={}/day={}/data-lake-metrics-{}.json",
        year,
        month,
        day,
        datetime.format("%Y-%m-%dT%H:%M:%S").to_string()
    )
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}?charset=utf8mb4&collation=utf8mb4_unicode_ci",
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
}
