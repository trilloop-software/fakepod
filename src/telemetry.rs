use anyhow::{Result};
use rand::Rng;
use tokio::sync::mpsc::{Receiver, Sender};
use super::packet::PodPacket;

#[derive(Debug, Clone)]
pub struct TelemetryData {
    accelerometer: u32,
    brake_temp: u32,
    battery_temp: u32,
    battery_current: u32,
}

impl TelemetryData {
    pub fn new() -> Self {
        Self {
            accelerometer: 0,
            brake_temp: 50,
            battery_temp: 30,
            battery_current: 12
        }
    }

    pub fn random_data(mut self) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            accelerometer: rng.gen_range(0..101),
            brake_temp: rng.gen_range(39..75),
            battery_temp: rng.gen_range(26..45),
            battery_current: rng.gen_range(9..15)
        }
        /*self.accelerometer = rng.gen_range(0..101);
        self.brake_temp = rng.gen_range(39..75);
        self.battery_temp = rng.gen_range(26..40);
        self.battery_current = rng.gen_range(9..15);*/
    }
}

pub struct TelemetrySvc {
    pub data: TelemetryData,
    pub rx: Receiver<PodPacket>,
    pub tx: Sender<PodPacket>
}

impl TelemetrySvc {
    pub async fn run(mut self) -> Result<()> {
        println!("tele_svc: service running");
        // send all telemetry data

        println!("{:#?}", self.data.clone());
        self.data = TelemetryData::random_data(self.data);
        println!("{:#?}", self.data.clone());

        Ok(())
    }
}
