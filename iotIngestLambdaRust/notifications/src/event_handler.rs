use aws_config;
use aws_lambda_events::{ sqs::SqsEventObj};
use aws_sdk_ses::Client;
use common_lib::SensorDeviceInsert;
use lambda_runtime::{tracing, Error, LambdaEvent};
use sqlx::{MySql, Pool};
use std::env;

use crate::configuration::get_configuration;


pub async fn function_handler(event: LambdaEvent<SqsEventObj<SensorDeviceInsert>>, pool: Pool<MySql>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // // Usa variables de entorno para los correos
    let from_email = get_configuration()
        .expect("Failed to read configuration")
        .email_from;

    // Puedes extraer el asunto y cuerpo del mensaje, aquí se usa el body completo como ejemplo
    let subject = "Notificación desde Lambda";
    let body = "Hola, este es un mensaje de prueba desde AWS Lambda.";

    let result = client
        .send_email()
        .destination(
            aws_sdk_ses::types::Destination::builder()
                .to_addresses(&from_email)
                .build(),
        )
        .message(
            aws_sdk_ses::types::Message::builder()
                .subject(
                    aws_sdk_ses::types::Content::builder()
                        .data(subject)
                        .build()?,
                )
                .body(
                    aws_sdk_ses::types::Body::builder()
                        .text(aws_sdk_ses::types::Content::builder().data(body).build()?)
                        .build(),
                )
                .build(),
        )
        .source(&from_email)
        .send()
        .await?;

    println!("Email sent: {:?}", result);

    Ok(())
}
