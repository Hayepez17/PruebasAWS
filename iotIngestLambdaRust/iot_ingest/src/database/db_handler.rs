// Project: iot_ingest
use common_lib::{DatabaseSettings, SensorDeviceInsert, SensorDevicesSettings, AlarmsDetails};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use sqlx::Row;
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
        r#"select dl.id device_location_id, CONCAT_WS('@', d.model, d.mac_address) model_mac, CONCAT_WS('@', s.name, s.serial) sensor_id, c.name client_name, l.name location_name, v.name variable_name, v.unit, dl.min, dl.max, dl.offset, dl.calibration_factor, case when COUNT(al.id)> 0 then JSON_ARRAYAGG(JSON_OBJECT('alarm_id', al.id, 'device_location_id', al.device_location_id, 'type', al.type, 'severity', al.severity, 'alarm_role', al.alarm_role, 'set_point', al.set_point, 'every', al.every, 'status', al.status)) else NULL end as alarms_details from device_locations dl 
        left join alarm_sensors al on al.device_location_id = dl.id 
        inner join clients c on dl.client_id = c.id 
        inner join devices d on dl.device_id = d.id 
        inner join locations l on dl.location_id = l.id 
        inner join variables v on dl.variable_id = v.id 
        inner join sensors s on dl.sensor_id = s.id where dl.status != 0 and d.mac_address IN ({}) group by dl.id;"#,
        placeholders
    );

    // let query_devices = format!(
    //     r#"SELECT dl.id device_location_id, CONCAT_WS('@',d.model,d.mac_address) model_mac, CONCAT_WS('@',s.name,s.serial) sensor_id, l.name location_name, c.name client_name, v.name variable_name, v.unit, dl.notify_every, dl.min, dl.max, dl.warning, dl.critical, dl.offset, dl.calibration_factor FROM device_locations dl 
    //     INNER JOIN variables v ON dl.variable_id = v.id 
    //     INNER JOIN devices d ON d.id = dl.device_id 
    //     INNER JOIN locations l ON dl.location_id = l.id AND d.client_id = l.client_id 
    //     INNER JOIN clients c ON dl.client_id = c.id 
    //     INNER JOIN sensors s ON dl.sensor_id = s.id 
    // WHERE dl.status != 0 AND d.mac_address IN ({});"#,
    //     placeholders
    // );

    // let mut select_query = sqlx::query_as::<_, SensorDevicesSettings>(&query_devices);

    // // Vincular las direcciones MAC
    // for mac in mac_vec.iter() {
    //     select_query = select_query.bind(mac);
    // }

    // // Ejecutar la consulta y devolver los resultados
    // let sensor_devices = select_query.fetch_all(pool).await?;

    // // for (i, sensor_device) in sensor_devices.iter().enumerate() {
    // //         println!("\nSensor device row #{}: {:?}", i + 1, sensor_device);
    // //     }

    let mut select_query = sqlx::query(&query_devices);

    // Bind MAC addresses
    for mac in mac_vec.iter() {
        select_query = select_query.bind(mac);
    }

    // Fetch rows and manually map to SensorDevicesSettings
    let rows = select_query.fetch_all(pool).await?;
    let mut sensor_devices = Vec::with_capacity(rows.len());
    for row in rows {
        // Adjust field extraction as per your struct definition
        let alarms_details_json: Option<String> = row.try_get("alarms_details").ok();
        let alarms_details: Option<Vec<AlarmsDetails>> = match alarms_details_json {
            Some(json) => serde_json::from_str(&json).ok(),
            None => None,
        };

        sensor_devices.push(SensorDevicesSettings {
            device_location_id: row.try_get("device_location_id")?,
            model_mac: row.try_get("model_mac")?,
            sensor_id: row.try_get("sensor_id")?,
            client_name: row.try_get("client_name")?,
            location_name: row.try_get("location_name")?,
            variable_name: row.try_get("variable_name")?,
            unit: row.try_get("unit")?,
            min: row.try_get("min")?,
            max: row.try_get("max")?,
            offset: row.try_get("offset")?,
            calibration_factor: row.try_get("calibration_factor")?,
            alarms_details,
        });
    }


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
