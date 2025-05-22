// src/configuration.rs
use std::env;
use common_lib::DatabaseSettings;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub aws: AwsSettings,
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

impl AwsSettings {
    pub fn bucket_name(&self) -> String {
        self.bucket_name.clone()
    }
}
