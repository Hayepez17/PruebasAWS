use chrono::NaiveDateTime;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow)]
pub struct SensorDevicesDatabase {
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
