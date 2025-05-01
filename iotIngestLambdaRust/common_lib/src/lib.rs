use chrono::NaiveDateTime;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing_subscriber;

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDeviceAlarm {
    pub sensor_device_setting: SensorDevicesSettings,
    pub sensor_device_insert: SensorDeviceInsert,
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
    pub notify_every: u32,
    pub min: f64,
    pub max: f64,
    pub warning: f64,
    pub critical: f64,
    pub offset: f64,
    pub calibration_factor: f64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDeviceRecive {
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

#[derive(Debug ,Serialize, Deserialize)]
pub struct ModbusRemoteFields {
    pub number: u8,
    pub value: Vec<String>,
    pub slave: Vec<u8>,
    pub funct: Vec<u8>,
    pub addr: Vec<u16>,
    pub bytes: Vec<u8>,
}

impl SensorDeviceRecive {
    pub fn new(sensor_id: String, model_mac: String, value: String, ip: String, time_stamp: String) -> Self {
        SensorDeviceRecive {
            sensor_id,
            model_mac,
            value,
            ip,
            time_stamp,
        }
    }
}

impl SensorDeviceData {
    pub fn generate_sensor_map(sensor_data: SensorDeviceData) -> HashMap<String, SensorDeviceRecive>{

        let mut sensor_map: HashMap<String, SensorDeviceRecive> = HashMap::new();

    // Procesar el campo 'temp'
    if let Some(temp_fields) = sensor_data.temp {
        for (i,sn) in temp_fields.sn.iter().enumerate() {
            let sensor_id = format!("ds18@{}", sn);
            let model_mac = format!("{}@{}", sensor_data.model, sensor_data.mac);
            let value = temp_fields.value.get(i).map_or_else(|| "N/A".to_string(), |v| v.to_string());

            let entry = SensorDeviceRecive {
            sensor_id: sensor_id.clone(),
            model_mac: model_mac.clone(),
            value,
            ip: sensor_data.ip.clone(),
            time_stamp: sensor_data.time_stamp.clone(),
            };

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
    tracing_subscriber::fmt().json()
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