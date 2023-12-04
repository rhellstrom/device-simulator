use std::sync::Arc;
use std::time::Duration;
use rand::Rng;
use tokio::sync::Mutex;
use tokio::time;
use serde::{Deserialize, Serialize};
use crate::args::Args;
use crate::util::get_current_timestamp;

#[derive(Serialize, Clone, Debug)]
struct ConsumptionData {
    timestamp: String,
    /// Power in watts
    power_usage: f32,
    /// Energy consumed in kWh
    energy_consumed: f32,
}

impl ConsumptionData {
    /// Simulate an entry for consumption_data
    pub fn simulate_entry(interval: u64) -> ConsumptionData {
        let mut rng = rand::thread_rng();
        let timestamp = get_current_timestamp();
        let power_usage = rng.gen_range(50.0..200.0);
        let energy_consumed = (power_usage * (interval as f32) / 1000.0) * (interval as f32) / 3600.0; //Calculate kWh from power_usage

        ConsumptionData {
            timestamp,
            power_usage,
            energy_consumed,
        }
    }
}

// Instead of a boolean value if we were to extend status with e.g Low power mode or such
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum StandbyStatus {
    On,
    Off,
}

#[derive(Serialize, Clone, Debug)]
pub struct Device {
    pub id: u16,
    pub name: String,
    pub power: StandbyStatus,
    total_consumption: f32,
    consumption_data: Vec<ConsumptionData>,
    #[serde(skip_serializing)]
    update_interval: u64,
    #[serde(skip_serializing)]
    max_entries: usize,
}

impl Device {
    pub fn new(id: u16, name: String, power: StandbyStatus, update_interval: u64, max_entries: usize) -> Self {
        Device {
            id,
            name,
            power,
            total_consumption: 0.0,
            consumption_data: vec![],
            update_interval,
            max_entries,
        }
    }

    pub fn on_tick(&mut self) {
        if self.power == StandbyStatus::On {
            let new_data = ConsumptionData::simulate_entry(self.update_interval);
            self.total_consumption += new_data.energy_consumed;
            self.consumption_data.push(new_data);

            if self.consumption_data.len() >= self.max_entries {
                self.consumption_data.remove(0); // Remove oldest entry if maximum entries reached
            }
        }
    }
}

/// Iterate over devices and start simulation task
pub async fn update_devices(args: Args, devices: Arc<Mutex<Vec<Device>>>){
    let mut interval = time::interval(Duration::from_secs(args.update_interval));
    loop {
        interval.tick().await;

        //Acquire the lock
        let mut devices_mutex = devices.lock().await;
        for device in devices_mutex.iter_mut(){
            device.on_tick();
        }
        //Drops lock when we go out of scope
    }
}

/// Helper function to create a vector of hardcoded devices
pub fn create_default_devices(args: Args) -> Vec<Device> {
    let device_info = [
        (1, "Fridge"),
        (2, "Freezer"),
        (3, "EV-Charger"),
        (4, "Water heater"),
        (5, "Bathroom floor heater"),
    ];

    device_info
        .iter()
        .map(|&(id, name)| Device::new(id, name.to_string(), StandbyStatus::On, args.update_interval, args.max_entries))
        .collect()
}
