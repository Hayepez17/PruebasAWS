use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use tracing_subscriber;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct SensorDeviceAlarm {
//     pub sensor_device_setting: SensorDevicesSettings,
//     pub sensor_device_insert: SensorDeviceInsert,
// }

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LoggerDeviceLocation {
  pub id: i32,
  pub device_location_id: i32,
  pub client_name: String,
  pub location_name: String,
  pub alarm_level: String,
  pub msg: String,
  pub recognized: i32,
  pub last_update: NaiveDateTime,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub endpoint: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SensorDevicesSettings {
    pub device_location_id: i32,
    pub model_mac: String,
    pub sensor_id: String,
    pub location_name: String,
    pub client_name: String,
    pub variable_name: String,
    pub unit: String,
    pub min: f64,
    pub max: f64,
    pub offset: f64,
    pub calibration_factor: f64,
    pub alarms_details: Option<Vec<AlarmsDetails>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AlarmsDetails {
    pub alarm_id: i32,
    pub device_location_id: i32,
    pub type_: u32,
    pub severity: u32,
    pub alarm_role: u32,
    pub set_point: f64,
    pub every: f64,
    pub status: u8,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct SensorDeviceInsert {
    pub device_location_id: i32,
    pub model_mac: String,
    pub sensor_name: String,
    pub location_name: String,
    pub client_name: String,
    pub variable_name: String,
    pub unit: String,
    pub state: String,
    pub ip: String,
    pub value: f64,
    pub timestamp: NaiveDateTime,
    pub alarm_type: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDeviceData {
    pub mac: String,
    pub model: String,
    pub ip: String,
    pub batery: u8,
    pub time_stamp: String,
    pub temp: Option<TempSensorFields>,
    pub modbus: Option<ModbusRemoteFields>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorDeviceRecive {
    pub mac: String,
    pub sensor_id: String, // "ds18@<sn>" || "<mac>@<byte_array>" || "<mac>@batery"
    pub model_mac: String, // "<model>@<mac>"
    pub value: String,
    pub ip: String,
    pub time_stamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempSensorFields {
    pub number: u8,
    pub value: Vec<f64>,
    pub sn: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModbusRemoteFields {
    pub number: u8,
    pub value: Vec<String>,
    pub slave: Vec<u8>,
    pub funct: Vec<u8>,
    pub addr: Vec<u16>,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SensorDeviceAlarm {
    ERROR(String),
    NORMAL(String),
    MAJOR(String),
    MINOR(String),
    WARNING(String),
    CRITICAL(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SensorAlarmSeverity {
    NORMAL(u8),
    MINOR(u8),
    MAJOR(u8),
    WARNING(u8),
    CRITICAL(u8),
}

impl SensorDeviceRecive {
    pub fn new(
        device_mac: String,
        sensor_id: String,
        model_mac: String,
        value: String,
        ip: String,
        time_stamp: String,
    ) -> Self {
        SensorDeviceRecive {
            mac: device_mac,
            sensor_id,
            model_mac,
            value,
            ip,
            time_stamp,
        }
    }
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

impl SensorDeviceData {
    pub fn generate_sensor_map(
        sensor_data: SensorDeviceData,
    ) -> HashMap<String, SensorDeviceRecive> {
        let mut sensor_map: HashMap<String, SensorDeviceRecive> = HashMap::new();

        // Procesar el campo 'temp'
        if let Some(temp_fields) = sensor_data.temp {
            for (i, sn) in temp_fields.sn.iter().enumerate() {
                let sensor_id = format!("ds18@{}", sn);
                let model_mac = format!("{}@{}", sensor_data.model, sensor_data.mac);
                let value = temp_fields
                    .value
                    .get(i)
                    .map_or_else(|| "N/A".to_string(), |v| v.to_string());

                let entry = SensorDeviceRecive::new(
                    sensor_data.mac.clone(),
                    sensor_id.clone(),
                    model_mac.clone(),
                    value.clone(),
                    sensor_data.ip.clone(),
                    sensor_data.time_stamp.clone(),
                );

                sensor_map.insert(sensor_id, entry);
            }
        }

        // Procesar el campo 'modbus'
        if let Some(modbus_fields) = sensor_data.modbus {
            for (i, value) in modbus_fields.value.iter().enumerate() {
                let sensor_id = format!(
                    "modbus@{}@{:02X}{:02X}{:04X}{:02X}",
                    sensor_data.mac,
                    modbus_fields.slave[i],
                    modbus_fields.funct[i],
                    modbus_fields.addr[i],
                    modbus_fields.bytes[i]
                );

                let model_mac = format!("{}@{}", sensor_data.model, sensor_data.mac);

                let entry = SensorDeviceRecive::new(
                    sensor_data.mac.clone(),
                    sensor_id.clone(),
                    model_mac.clone(),
                    value.clone(),
                    sensor_data.ip.clone(),
                    sensor_data.time_stamp.clone(),
                );

                sensor_map.insert(sensor_id, entry);
            }

            // Procesar el campo 'batery'

            let sensor_id = format!("batery@{}", sensor_data.mac);
            let model_mac = format!("{}@{}", sensor_data.model, sensor_data.mac);
            let value = sensor_data.batery.to_string();
            let entry = SensorDeviceRecive::new(
                sensor_data.mac.clone(),
                sensor_id.clone(),
                model_mac.clone(),
                value.clone(),
                sensor_data.ip.clone(),
                sensor_data.time_stamp.clone(),
            );
            sensor_map.insert(sensor_id, entry);
        }

        sensor_map
    }
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        // This needs to be set to remove duplicated information in the log.
        .with_current_span(false)
        // This needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // Disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        // Remove the name of the function from every log entry.
        .with_target(false)
        .init();
}
