use std::sync::Arc;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::Json;
use log::{debug, info};
use serde_json::Value;
use tokio::sync::Mutex;
use crate::device::{Device, StandbyStatus};
use crate::device::StandbyStatus::Off;

pub async fn list_devices(devices: Arc<Mutex<Vec<Device>>>) -> Response<String> {
    let devices = devices.lock().await;
    match serde_json::to_string(&*devices) {
        Ok(json_response) => {
            Response::builder()
                .header("content-type", "application/json")
                .body(json_response)
                .unwrap()
        }
        Err(err) => {
            eprintln!("Error serializing resource: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal Server Error".to_string())
                .unwrap()
        }
    }
}

pub async fn get_device(Path(device_id): Path<String>, devices: Arc<Mutex<Vec<Device>>>) -> Response<String> {
    let device_id: Result<u16, _> = device_id.parse();
    if let Ok(device_id) = device_id {
        let devices = devices.lock().await;

        for device in devices.iter() {
            if device.id == device_id {
                return match serde_json::to_string(device) {
                    Ok(json_response) => {
                        Response::builder()
                            .header("content-type", "application/json")
                            .body(json_response)
                            .unwrap()
                    }
                    Err(err) => {
                        eprintln!("Error serializing resource: {:?}", err);
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body("Internal Server Error".to_string())
                            .unwrap()
                    }
                }
            }
        }
    }
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not found".to_string())
        .unwrap()
}

pub async fn update_device_power(Path(device_id): Path<u16>, devices: Arc<Mutex<Vec<Device>>>, new_power_status: Json<Value>) -> Response<String> {
    // Deserialize payload
    let power_status = match new_power_status.get("power").and_then(Value::as_str) {
        Some("On") => StandbyStatus::On,
        Some("Off") => StandbyStatus::Off,
        _ => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid 'power' value in JSON payload".to_string())
                .unwrap();
        }
    };

    let mut devices = devices.lock().await;

    // Find the device by ID
    if let Some(device) = devices.iter_mut().find(|d| d.id == device_id) {
        device.power = power_status;
        info!("{} was set to {:?}", device.name, device.power);
        if device.power == Off {
            device.total_consumption = 0.0;
            debug!("{} total_consumption set to 0.0", device.name);
        }
        // Respond with the updated device JSON
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&device).unwrap())
            .unwrap()
    } else {
        // Device with the given ID not found
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Device Not Found".to_string())
            .unwrap()
    }
}