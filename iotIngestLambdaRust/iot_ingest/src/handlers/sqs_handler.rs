#![allow(unused)]

use crate::configuration as Config;
use crate::database::s3_handler::put_metrics_to_s3;
use crate::services::sensor_service;
use crate::{database::db_handler};

use aws_lambda_events::sqs::SqsEventObj;
use aws_sdk_s3::config;
use common_lib::{SensorDeviceData, SensorDeviceInsert};
use core::alloc;
use futures::future::join_all;
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
    // Procesa cada mensaje en paralelo
    let tasks = event.payload.records.into_iter().map(|record| {
        tokio::spawn(async move {
            let data = record.body;
            if data.mac.is_empty() {
                println!("Empty MAC address found in the data: {:?}", data);
                return None;
            }
            let sensor_map = SensorDeviceData::generate_sensor_map(data);
            Some(sensor_map)
        })
    });

    // Espera a que todos los tasks terminen y recoge los resultados
    let sensor_maps: Vec<_> = join_all(tasks)
        .await
        .into_iter()
        .filter_map(|res| match res {
            Ok(Some(map)) => Some(map),
            _ => None,
        })
        .collect();

    // Llama a la función del módulo `database`
    let mac_addresses: HashSet<String> = sensor_maps
        .iter()
        .flat_map(|map| map.values().map(|v| v.mac.clone()))
        .collect();

    let sensor_devices = db_handler::fetch_sensor_devices(&pool, mac_addresses).await?;
    let metrics_insert = sensor_service::process_metrics(&sensor_maps, &sensor_devices);

    // for (i, metric_insert) in metrics_insert.iter().enumerate() {
    //     println!("\nMetric insert row #{}: {:?}", i + 1, metric_insert);
    // }

    // Llama a la función del módulo `database` para insertar las métricas
    //db_handler::insert_metrics(&pool, &metrics_insert).await?;

    // let s3_client = Client::new(&aws_config::load_from_env().await);

    if !metrics_insert.is_empty() {
        let mut json_lines = String::new();

        for metric in metrics_insert.iter() {
            let json_data = to_string(&metric)?;
            json_lines.push_str(&json_data);
            json_lines.push('\n');
        }

        let config = Config::get_configuration().expect("Failed to read configuration");
        let bucket_name = config.aws.bucket_name.clone();
        let now = chrono::Utc::now();
        let s3_key = Config::get_bucket_key(now).clone();

        // Clona los datos necesarios para el proceso concurrente
        let pool_clone = pool.clone();
        let metrics_clone = metrics_insert.clone();

        let db_future = async move {
            db_handler::insert_metrics(&pool_clone, &metrics_clone).await
        };

        let s3_future = async move {
            put_metrics_to_s3(&bucket_name, &s3_key, &json_lines.into_bytes()).await
        };

        // Ejecuta ambos procesos concurrentemente
        let (db_result, s3_result) = tokio::join!(db_future, s3_future);

        db_result?; // Propaga el error si ocurre
        s3_result?; // Propaga el error si ocurre
    }

    //     s3_client
    //         .put_object()
    //         .bucket("<bucket_name>")
    //         .key(s3_key)
    //         .body(concatenated_json.into_bytes().into())
    //         .send()
    //         .await?;

    Ok(())
}
