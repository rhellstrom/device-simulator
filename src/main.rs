use clap::Parser;
use crate::device::{Device, StandbyStatus};
mod device;
mod util;
mod args;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    let mut device1 = Device::new(1337,
                                  "Kylskåp".to_string(),
                                  StandbyStatus::On,
                                  args.update_interval,
                                  args.max_entries);
    println!("{:?}", device1);

    device1.simulate_continuous_consumption().await;
}

