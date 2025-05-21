#![allow(deprecated)]

use chrono::NaiveDateTime;
use common_lib::{
    SensorDeviceAlarm, SensorDeviceInsert, SensorDeviceRecive, SensorDevicesSettings,
};
use std::collections::HashMap;

pub fn process_metrics(
    sensor_maps: &Vec<HashMap<String, SensorDeviceRecive>>,
    sensor_devices: &Vec<SensorDevicesSettings>,
) -> Vec<SensorDeviceInsert> {
    let mut metrics_insert: Vec<SensorDeviceInsert> = Vec::new();

    for sensor_map in sensor_maps {
        //println!("Sensor map #{}: {:?} \n", index + 1, sensor_map);

        for (sensor_id, sensor_record) in sensor_map {
            if let Some(sensor_device) = sensor_devices.iter().find(|d| d.sensor_id == *sensor_id) {

                let value = match sensor_device.sensor_id.split('@').next().unwrap_or("") {
                    "ds18" => match sensor_record.value.as_str() {
                        "-127" => sensor_record.value.parse::<f64>().unwrap_or(0.0),
                        _ => apply_correction(
                            sensor_record.value.parse::<f64>().unwrap_or(0.0),
                            sensor_device.offset,
                            sensor_device.calibration_factor,
                        ),
                    },
                    "modbus" => {
                        let bytes = hex::decode(&sensor_record.value).unwrap_or_else(|_| vec![]);
                        match bytes.len() {
                            2 => {
                                let result = apply_correction(
                                    u16::from_be_bytes([bytes[0], bytes[1]]) as f64,
                                    sensor_device.offset,
                                    sensor_device.calibration_factor,
                                );
                                if result.is_nan() || result.is_infinite() {
                                    9999.0
                                } else {
                                    result
                                }
                            }
                            4 => {
                                let array = [bytes[0], bytes[1], bytes[2], bytes[3]];
                                let result = apply_correction(
                                    f32::from_be_bytes(array) as f64,
                                    sensor_device.offset,
                                    sensor_device.calibration_factor,
                                );
                                if result.is_nan() || result.is_infinite() {
                                    9999.0
                                } else {
                                    result
                                }
                            }
                            _ => 9999.0,
                        }
                    }
                    "battery" => sensor_record.value.parse::<f64>().unwrap_or(0.0),
                    _ => sensor_record.value.parse::<f64>().unwrap_or(0.0),
                };

                let state = match value {
                    -127.0 => SensorDeviceAlarm::DISCONNECT("DISCONNECT".to_string()),
                    9999.0 => SensorDeviceAlarm::DISCONNECT("TIMEOUT".to_string()),
                    _ => {
                        if value > sensor_device.warning {
                            SensorDeviceAlarm::WARNING("WARNING".to_string())
                        } else if value > sensor_device.critical {
                            SensorDeviceAlarm::CRITICAL("CRITICAL".to_string())
                        } else {
                            SensorDeviceAlarm::OK("OK".to_string())
                        }
                    }
                };

                metrics_insert.push(SensorDeviceInsert {
                    device_location_id: sensor_device.device_location_id,
                    model_mac: sensor_device.model_mac.clone(),
                    sensor_name: sensor_device.sensor_id.clone(),
                    location_name: sensor_device.location_name.clone(),
                    client_name: sensor_device.client_name.clone(),
                    variable_name: sensor_device.variable_name.clone(),
                    unit: sensor_device.unit.clone(),
                    ip: sensor_record.ip.clone(),
                    state: match &state {
                        SensorDeviceAlarm::DISCONNECT(s) => s.clone(),
                        SensorDeviceAlarm::WARNING(s) => s.clone(),
                        SensorDeviceAlarm::CRITICAL(s) => s.clone(),
                        SensorDeviceAlarm::OK(s) => s.clone(),
                    },
                    value,
                    alarm_type: match &state {
                        SensorDeviceAlarm::OK(_) => 0u16,
                        SensorDeviceAlarm::DISCONNECT(_) => 1u16,
                        SensorDeviceAlarm::WARNING(_) => 2u16,
                        SensorDeviceAlarm::CRITICAL(_) => 3u16,
                    },
                    timestamp: NaiveDateTime::parse_from_str(
                        &sensor_record.time_stamp,
                        "%Y-%m-%dT%H:%M:%S",
                    )
                    .unwrap_or_else(|_| NaiveDateTime::from_timestamp(0, 0)),
                });
            } else {
                println!("Sensor device not found for ID: {}", sensor_id);
            }
        }
    }

    metrics_insert
}

fn apply_correction(value: f64, offset: f64, calibration_factor: f64) -> f64 {
    value * calibration_factor + offset
}
