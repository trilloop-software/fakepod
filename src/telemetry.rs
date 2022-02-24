use anyhow::{Result};
use tokio::sync::mpsc::{Receiver, Sender};
use super::packet::PodPacket;

pub struct TelemetryData {

}

impl TelemetryData {
    pub fn new() -> Self {
        Self {

        }
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

        Ok(())
    }
}
