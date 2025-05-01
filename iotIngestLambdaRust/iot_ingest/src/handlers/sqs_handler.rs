#![allow(unused)]

use crate::database::s3_handler::put_metrics_to_s3;
use crate::services::sensor_service;
use crate::{configuration::get_configuration, database::db_handler};

use aws_lambda_events::sqs::SqsEventObj;
use aws_sdk_s3::config;
use common_lib::{SensorDeviceData, SensorDeviceInsert};
use core::alloc;
use lambda_runtime::{Error, LambdaEvent};
use sqlx::MySqlPool;
use std::collections::HashSet;
// use aws_sdk_s3::Client;
use serde_json::to_string;
// use chrono::Utc;

pub async fn handle_event(
    event: LambdaEvent<SqsEventObj<SensorDeviceData>>,
    pool: MySqlPool,
) -> Result<(), Error> {
    // Procesa el evento SQS
    //println!("Received event: {:?}", event);

    let mut mac_addresses: HashSet<String> = HashSet::new();
    let mut sensor_maps = Vec::new();
    let mut metrics_insert: Vec<SensorDeviceInsert> = Vec::new();

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

    // Llama a la función del módulo `database`
    let sensor_devices = db_handler::fetch_sensor_devices(&pool, unique_macs).await?;

    // Llama a la función del módulo `db_handler` para procesar las métricas
    metrics_insert = sensor_service::process_metrics(&sensor_maps, &sensor_devices);

    for (i, metric_insert) in metrics_insert.iter().enumerate() {
        println!("\nMetric insert row #{}: {:?}", i + 1, metric_insert);
    }

    // Llama a la función del módulo `database` para insertar las métricas
    db_handler::insert_metrics(&pool, &metrics_insert).await?;

    // let s3_client = Client::new(&aws_config::load_from_env().await);
    
    
    if !metrics_insert.is_empty() {
        let mut json_objects = Vec::new();
        
        for metric in metrics_insert.iter() {
            let json_data = to_string(&metric)?;
            json_objects.push(json_data);
        }
        
        let concatenated_json = format!("[{}]", json_objects.join(","));
        let config = get_configuration().expect("Failed to read configuration");
        let bucket_name = config.aws.bucket_name.clone();
        let s3_key = config
            .aws
            .get_bucket_key(
                &metrics_insert[0].device_location_id.to_string(),
                &metrics_insert[0].timestamp.format("%Y").to_string(),
                &metrics_insert[0].timestamp.format("%m").to_string(),
                &metrics_insert[0].timestamp.format("%d").to_string(),
            )
            .clone();

        put_metrics_to_s3(&bucket_name, &s3_key, &concatenated_json.into_bytes())
            .await?;
    }

    //     s3_client
    //         .put_object()
    //         .bucket("<bucket_name>")
    //         .key(s3_key)
    //         .body(concatenated_json.into_bytes().into())
    //         .send()
    //         .await?;
    // }

    Ok(())
}
