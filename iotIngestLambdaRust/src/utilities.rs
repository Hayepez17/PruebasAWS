use std::collections::HashMap;

use chrono::NaiveDateTime;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow)]
pub struct SensorDevicesSettings {
    pub mac_address: String,
    pub model: String,
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
    pub status: bool,
    pub date_created: NaiveDateTime,
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
    pub bytes: Vec<u8>,
    pub addr: Vec<u16>,
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
    pub fn generate_sensor_map(sensor_data: SensorDeviceData) -> HashMap<String, Vec<SensorDeviceRecive>>{

        let mut sensor_map: HashMap<String, Vec<SensorDeviceRecive>> = HashMap::new();

    // Procesar el campo 'temp'
    if let Some(temp_fields) = sensor_data.temp {
        for sn in temp_fields.sn {
            let sensor_id = format!("ds18@{}", sn);
            let model_mac = format!("{}@{}", sensor_data.model, sensor_data.mac);
            let value = temp_fields.value.get(0).map_or_else(|| "N/A".to_string(), |v| v.to_string());

            let entry = SensorDeviceRecive {
                sensor_id: sensor_id.clone(),
                model_mac: model_mac.clone(),
                value,
                ip: sensor_data.ip.clone(),
                time_stamp: sensor_data.time_stamp.clone(),
            };

            sensor_map.entry(sensor_id).or_insert_with(Vec::new).push(entry);
        }
    }

    // Procesar el campo 'modbus'
    if let Some(modbus_fields) = sensor_data.modbus {
        for (i, value) in modbus_fields.value.iter().enumerate() {
            let sensor_id = format!(
                "{}@{:02X}{:02X}{:04X}{:02X}",
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

            sensor_map.entry(sensor_id).or_insert_with(Vec::new).push(entry);
        }

        // Procesar el campo 'batery'

        let sensor_id = format!("{}@batery", sensor_data.mac);
        let model_mac = format!("{}@{}", sensor_data.model, sensor_data.mac);
        let value = sensor_data.batery.to_string();
        let entry = SensorDeviceRecive::new(
            sensor_id.clone(),
            model_mac.clone(),
            value.clone(),
            sensor_data.ip.clone(),
            sensor_data.time_stamp.clone(),
        );
        sensor_map.entry(sensor_id).or_insert_with(Vec::new).push(entry);
    }

    sensor_map
    }
}