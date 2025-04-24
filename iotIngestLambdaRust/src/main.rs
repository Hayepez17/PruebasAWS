#![allow(unused)]

use iotIngestLambdaRust::{
    utilities::{SensorDeviceData, SensorDevicesDatabase},
    *,
};

use aws_lambda_events::{event::sqs::SqsEvent, sqs::SqsEventObj};
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

pub async fn function_handler(
    event: LambdaEvent<SqsEventObj<SensorDeviceData>>,
    pool: MySqlPool, // Agrega el pool como argumento
) -> Result<(), Error> {
    let mut mac_addresses: HashSet<String> = HashSet::new();

    for record in event.payload.records {
        let data = &record.body;

        mac_addresses.insert(data.mac.clone());

        if data.mac.is_empty() {
            println!("Empty MAC address found in the data: {:?}", data);
            continue;
        }
    }

    let unique_macs: Vec<String> = mac_addresses.into_iter().collect();

    let placeholders = unique_macs
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");

    let query_devices = format!(
        r#"SELECT d.mac_address, d.model, l.name location_name, c.name client_name, v.name variable_name, v.unit, dl.notify_every, dl.min, dl.max, dl.warning, dl.critical, dl.offset, dl.calibration_factor, dl.status, dl.date_created FROM device_locations dl 
    INNER JOIN variables v ON dl.variable_id = v.id 
    INNER JOIN devices d ON d.id = dl.device_id 
    INNER JOIN locations l ON dl.location_id = l.id AND d.client_id = l.client_id 
    INNER JOIN clients c ON dl.client_id = c.id 
WHERE dl.status != 0 AND d.mac_address IN ({});"#,
        placeholders
    );

    let mut select_query = sqlx::query_as::<_, SensorDevicesDatabase>(&query_devices);

    // Vincular las direcciones MAC
    for mac in unique_macs.iter() {
        select_query = select_query.bind(mac);
    }

    let sensor_devices: Vec<SensorDevicesDatabase> = select_query.fetch_all(&pool).await?;

    println!("Unique MACs: {:?}", unique_macs);

    let mut count = 0;
    for sensor_device in sensor_devices {
        count += 1;
        println!("Row #{count}: {:?} \n", sensor_device);
    }

    Ok(())
}
