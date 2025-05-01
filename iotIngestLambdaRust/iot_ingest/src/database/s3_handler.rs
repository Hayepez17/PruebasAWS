use aws_sdk_s3::{Client, Error};
use aws_sdk_s3::primitives::ByteStream;
// use crate::configuration::get_configuration;

pub async fn client() -> Result<Client, Error> {
    // let config = get_configuration().expect("Failed to read configuration");
    let aws_config = aws_config::load_from_env().await;
    let s3_client = Client::new(&aws_config);
    Ok(s3_client)
}

pub async fn put_metrics_to_s3(
    bucket: &str,
    key: &str,
    body: &[u8],
) -> Result<(), Error> {
    let s3_client = client().await?;
    let byte_stream = ByteStream::from(body.to_vec());

    s3_client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(byte_stream)
        .send()
        .await?;

    Ok(())
}

