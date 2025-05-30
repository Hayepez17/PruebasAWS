// Project: iot_ingest
use common_lib::{DatabaseSettings, SensorDeviceInsert, SensorDevicesSettings};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::collections::HashSet;


pub async fn establish_connection(config: &DatabaseSettings) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection_string())
        .await?;

    println!("Database connection established successfully.");
    Ok(pool)
}

pub async fn fetch_sensor_devices(
    pool: &MySqlPool,
    mac_addresses: HashSet<String>,
) -> Result<Vec<SensorDevicesSettings>, sqlx::Error> {
    let mac_vec: Vec<String> = mac_addresses.into_iter().collect();

    let placeholders = mac_vec
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
    for mac in mac_vec.iter() {
        select_query = select_query.bind(mac);
    }

    // Ejecutar la consulta y devolver los resultados
    let sensor_devices = select_query.fetch_all(pool).await?;

    // for (i, sensor_device) in sensor_devices.iter().enumerate() {
    //         println!("\nSensor device row #{}: {:?}", i + 1, sensor_device);
    //     }

    Ok(sensor_devices)
}

pub async fn insert_metrics(
    pool: &MySqlPool,
    metrics: &Vec<SensorDeviceInsert>,
) -> Result<(), sqlx::Error> {
    if metrics.is_empty() {
        println!("No metrics to insert.");
        return Ok(());
    }

    // Construye la parte VALUES (?, ?, ..., ?) para cada registro
    let values_clause = (0..metrics.len())
        .map(|_| "(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .collect::<Vec<_>>()
        .join(", ");

    let insert_query = format!(
        "INSERT INTO metrics (
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
        ) VALUES {}",
        values_clause
    );

    let mut query = sqlx::query(&insert_query);

    // Enlaza todos los parámetros en orden
    for metric in metrics {
        query = query
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
            .bind(metric.timestamp);
    }

    query.execute(pool).await?;

    println!("Metrics inserted successfully (batch).");
    Ok(())
}
