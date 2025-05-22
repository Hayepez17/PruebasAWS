use aws_lambda_events::sqs::SqsEventObj;
use common_lib::{init_tracing, SensorDeviceInsert};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use notifications::{configuration, database};
use notifications::event_handler::function_handler;


#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let config = configuration::get_configuration().expect("Failed to read configuration");

    let _db_pool = database::db_handler::establish_connection(&config.database).await?;
    

    // run(service_fn(event_handler::function_handler)).await

    run(service_fn(
        move |event: LambdaEvent<SqsEventObj<SensorDeviceInsert>>| {
            function_handler(event, _db_pool.clone())
        },
    ))
    .await
}
