use std::sync::Arc;
use std::time::Duration;
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
    tokio::spawn(async move {
        update_devices(args, Arc::clone(&devices)).await
    });


    tokio::time::sleep(Duration::from_secs(120)).await;

}


