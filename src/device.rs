use std::time::Duration;
use rand::Rng;
use tokio::time;
use crate::util::get_current_timestamp;

#[derive(Clone, Debug)]
struct ConsumptionData {
    timestamp: String,
    /// Power in watts
    power_usage: f32,
    /// Energy consumed in kWh
    energy_consumed: f32,
}

impl ConsumptionData {
    /// Simulate an entry for consumption_data.
    /// Parameter interval is given in minutes to calculate kWh per update
    pub fn simulate_entry(interval: u64) -> ConsumptionData {
        let mut rng = rand::thread_rng();
        let timestamp = get_current_timestamp();
        let power_usage = rng.gen_range(50.0..200.0);
        let energy_consumed = (power_usage * (interval as f32) / 1000.0) / 60.0; //Calculate kWh from power_usage

        ConsumptionData {
            timestamp,
            power_usage,
            energy_consumed,
        }
    }
}

// Instead of a boolean value if we were to extend status with e.g Low power mode or such
#[derive(Clone, Debug)]
pub enum StandbyStatus {
    On,
    Off,
}

#[derive(Clone, Debug)]
pub struct Device {
    id: u16,
    name: String,
    power: StandbyStatus,
    total_consumption: f32,
    consumption_data: Vec<ConsumptionData>,
    //Skip serialization
    update_interval: u64,
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

    pub async fn simulate_continuous_consumption(&mut self) {
        let update_interval_seconds = self.update_interval; // Convert update_interval to seconds
        let mut interval = time::interval(Duration::from_secs(update_interval_seconds));

        loop {
            interval.tick().await; // Wait for the interval

            self.on_tick();
            if self.consumption_data.len() >= self.max_entries {
                self.consumption_data.remove(0); // Remove oldest entry if maximum entries reached
            }
        }
    }

    fn on_tick(&mut self) {
        let new_data = ConsumptionData::simulate_entry(self.update_interval); // Simulate new consumption data
        self.total_consumption += new_data.energy_consumed; // Add energy consumed to total
        self.consumption_data.push(new_data); // Add new consumption data
        println!("{:?}", self);
    }
}
