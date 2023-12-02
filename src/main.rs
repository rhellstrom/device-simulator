use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router};
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum::routing::get;
use clap::Parser;
use tokio::sync::Mutex;
use crate::device::{create_default_devices, Device, update_devices};
mod device;
mod util;
mod args;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let init_devices = create_default_devices(args);
    let devices: Arc<Mutex<Vec<Device>>> = Arc::new(Mutex::new(init_devices));

    // Spawn a task to update the devices based on args.update_interval
    let update_clone = Arc::clone(&devices);
    tokio::spawn(async move {
        update_devices(args, update_clone).await
    });

    let app = Router::new()
        // Handler for GET /devices
        .route(
            "/devices", get({
            let devices = Arc::clone(&devices);
            move || list_devices(devices)
        }),
        )
        .route(
            "/devices/:device_id",
            get({
                let devices = Arc::clone(&devices);
                move |path| get_device(path, devices)
            }
        )
        );


    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
async fn list_devices(devices: Arc<Mutex<Vec<Device>>>) -> Response<String> {
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

async fn get_device(Path(device_id): Path<String>, devices: Arc<Mutex<Vec<Device>>>) -> Response<String> {
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
