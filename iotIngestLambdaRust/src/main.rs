#![allow(unused)]

use iotIngestLambdaRust::{
    utilities::{SensorDeviceData, SensorDeviceInsert, SensorDeviceRecive, SensorDevicesSettings},
    *,
};

use aws_lambda_events::{event::sqs::SqsEvent, sqs::SqsEventObj};
use hex;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlRow};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Inicializa el logger
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    let config = configuration::get_configuration().expect("Failed to read configuration");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database.connection_string())
        .await?;

    println!("Database connection established successfully.");

    run(service_fn(
        move |event: LambdaEvent<SqsEventObj<SensorDeviceData>>| {
            function_handler(event, pool.clone())
        },
    ))
    .await
}

fn apply_correction(value: f64, offset: f64, calibration_factor: f64) -> f64 {
    value * calibration_factor + offset
}

pub async fn function_handler(
    event: LambdaEvent<SqsEventObj<SensorDeviceData>>,
    pool: MySqlPool, // Agrega el pool como argumento
) -> Result<(), Error> {
    let mut mac_addresses: HashSet<String> = HashSet::new();

    let mut sensor_maps = Vec::new();

    for record in event.payload.records {
        let data = record.body;

        mac_addresses.insert(data.mac.clone());

        if data.mac.is_empty() {
            println!("Empty MAC address found in the data: {:?}", data);
            continue;
        }

        let sensor_map = SensorDeviceData::generate_sensor_map(data);
        sensor_maps.push(sensor_map);
    }

    let unique_macs: Vec<String> = mac_addresses.into_iter().collect();

    let placeholders = unique_macs
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let query_devices = format!(
        r#"SELECT dl.id device_location_id, CONCAT_WS('@',d.model,d.mac_address) model_mac, CONCAT_WS('@',s.name,s.serial) sensor_id, l.name location_name, c.name client_name, v.name variable_name, v.unit, dl.notify_every, dl.min, dl.max, dl.warning, dl.critical, dl.offset, dl.calibration_factor FROM device_locations dl 
    INNER JOIN variables v ON dl.variable_id = v.id 
    INNER JOIN devices d ON d.id = dl.device_id 
    INNER JOIN locations l ON dl.location_id = l.id AND d.client_id = l.client_id 
    INNER JOIN clients c ON dl.client_id = c.id 
    INNER JOIN sensors s ON dl.sensor_id = s.id 
WHERE dl.status != 0 AND d.mac_address IN ({});"#,
        placeholders
    );

    let mut select_query = sqlx::query_as::<_, SensorDevicesSettings>(&query_devices);

    // Vincular las direcciones MAC
    for mac in unique_macs.iter() {
        select_query = select_query.bind(mac);
    }

    let sensor_devices: Vec<SensorDevicesSettings> = select_query.fetch_all(&pool).await?;

    println!("Unique MACs: {:?}", unique_macs);

    let mut metrics_insert: Vec<SensorDeviceInsert> = Vec::new();

    for (index, sensor_map) in sensor_maps.iter().enumerate() {
        println!("Sensor map #{}: {:?} \n", index + 1, sensor_map);

        for (sensor_id, sensor_record) in sensor_map {
            if let Some(sensor_device) = sensor_devices.iter().find(|d| d.sensor_id == *sensor_id) {
                metrics_insert.push(SensorDeviceInsert {
                    device_location_id: sensor_device.device_location_id,
                    model_mac: sensor_device.model_mac.clone(),
                    sensor_name: sensor_device.sensor_id.clone(),
                    location_name: sensor_device.location_name.clone(),
                    client_name: sensor_device.client_name.clone(),
                    variable_name: sensor_device.variable_name.clone(),
                    unit: sensor_device.unit.clone(),
                    state: match sensor_device.sensor_id.split('@').next().unwrap_or("") {
                        "ds18" => {
                            if sensor_record.value == "-127" {
                                "DISCONNECT".to_string()
                            } else {
                                "OK".to_string()
                            }
                        }
                        "modbus" => match sensor_record.value.as_str() {
                            "FFFF" => "TIMEOUT".to_string(),
                            "FFFFFFFF" => "TIMEOUT".to_string(),
                            _ => "OK".to_string(),
                        },
                        "batery" => {
                            if sensor_record.value.parse().unwrap_or(0.0) < 70.0 {
                                "DISCHARGE".to_string()
                            } else {
                                "OK".to_string()
                            }
                        }
                        _ => "UNKNOWN".to_string(),
                    },
                    ip: sensor_record.ip.clone(),

                    value: match sensor_device.sensor_id.split('@').next().unwrap_or("") {
                        "modbus" => {
                            let bytes = hex::decode(&sensor_record.value).unwrap_or_else(|_| vec![]);
                            match bytes.len() {
                                2 => {
                                    let result = apply_correction(
                                        u16::from_be_bytes([bytes[0], bytes[1]]) as f64,
                                        sensor_device.offset,
                                        sensor_device.calibration_factor,
                                    );
                                    if result.is_nan() || result.is_infinite() || result < -3.402823466E+38 || result > 3.402823466E+38 {
                                        9999.0 // Valor de error
                                    } else {
                                        result
                                    }
                                },
                                4 => {
                                    let array = [bytes[0], bytes[1], bytes[2], bytes[3]];
                                    let result = apply_correction(
                                        f32::from_be_bytes(array) as f64,
                                        sensor_device.offset,
                                        sensor_device.calibration_factor,
                                    );
                                    if result.is_nan() || result.is_infinite() || result < -3.402823466E+38 || result > 3.402823466E+38 {
                                        9999.0 // Valor de error
                                    } else {
                                        result
                                    }
                                },
                                _ => 9999.0,
                            }
                        }
                        ("battery" | "ds18") => apply_correction(
                            sensor_record.value.parse::<f64>().unwrap_or(0.0),
                            sensor_device.offset,
                            sensor_device.calibration_factor,
                        ),
                        _ => 9999.0,
                    },
                    timestamp: chrono::NaiveDateTime::parse_from_str(
                        &sensor_record.time_stamp,
                        "%Y-%m-%dT%H:%M:%S",
                    )
                    .unwrap_or_else(|_| chrono::NaiveDateTime::from_timestamp(0, 0)),
                });
            } else {
                println!("Sensor device not found for ID: {}", sensor_id);
            }
        }
    }

    for (i, metric_insert) in metrics_insert.iter().enumerate() {
        println!("Metric insert row #{}: {:?} \n", i + 1, metric_insert);
    }

    if !metrics_insert.is_empty() {
        let insert_query = r#"
            INSERT INTO metrics (
                device_location_id,
                client_name,
                location_name,
                model_mac,
                sensor_name,
                variable_name,
                unit,
                state,
                ip,
                value,
                timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        for metric in metrics_insert {
            sqlx::query(insert_query)
                .bind(metric.device_location_id)
                .bind(&metric.client_name)
                .bind(&metric.location_name)
                .bind(&metric.model_mac)
                .bind(&metric.sensor_name)
                .bind(&metric.variable_name)
                .bind(&metric.unit)
                .bind(&metric.state)
                .bind(&metric.ip)
                .bind(metric.value)
                .bind(metric.timestamp)
                .execute(&pool)
                .await?;
        }

        println!("Metrics inserted successfully.");
    } else {
        println!("No metrics to insert.");
    }

    Ok(())
}
