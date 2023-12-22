use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Router};
use axum::routing::{get, patch};
use clap::Parser;
use tokio::sync::Mutex;
use log::{error, info};
use tokio::net::TcpListener;
use crate::device::{create_default_devices, Device, update_devices};
use crate::handlers::{get_device, list_devices, update_device_power};

mod device;
mod util;
mod args;
mod handlers;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = args::Args::parse();
    let init_devices = create_default_devices(args);
    let devices: Arc<Mutex<Vec<Device>>> = Arc::new(Mutex::new(init_devices));

    // Spawn a task to update the devices based on args.update_interval
    let update_clone = Arc::clone(&devices);
    tokio::spawn(async move {
        update_devices(args, update_clone).await
    });

    let app = Router::new()
        .route(
            "/devices",
            get({
                let devices = Arc::clone(&devices);
                move || list_devices(devices)
            }),
        )
        .route(
            "/devices/:device_id",
            get({
                let devices = Arc::clone(&devices);
                move |path| get_device(path, devices)
            }),
        )
        .route(
            "/devices/:device_id",
            patch({
                let devices = Arc::clone(&devices);
                |path, body| update_device_power(path, devices, body)
            }),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    if let Ok(listener) = TcpListener::bind(addr).await {
        info!("Server is running at {}", addr);
        axum::serve(listener, app).await.unwrap();
    } else {
        error!("Failed to bind to address {}", addr);
    }
}