#![allow(unused)]

use iot_ingest::{configuration, database, handlers, services};

use handlers::sqs_handler::handle_event;
use common_lib::{init_tracing, SensorDeviceData, SensorDeviceInsert, SensorDevicesSettings};
use aws_lambda_events::{event::sqs::SqsEvent, sqs::SqsEventObj};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlRow};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Inicializa el logger
    init_tracing();

    let config_aws = aws_config::load_from_env().await;

    print!("Config AWS: {:?}", config_aws);

    let config = configuration::get_configuration().expect("Failed to read configuration");

    let _db_pool = database::db_handler::establish_connection(&config.database).await?;

    run(service_fn(
        move |event: LambdaEvent<SqsEventObj<SensorDeviceData>>| {
            handle_event(event, _db_pool.clone())
        },
    ))
    .await
}