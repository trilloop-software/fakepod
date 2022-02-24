use anyhow::Result;
use tokio::sync::mpsc::{Receiver, Sender};
use super::packet::PodPacket;

pub struct ControlsSvc {
    pub rx: Receiver<PodPacket>,
    pub tx: Sender<PodPacket>
}

impl ControlsSvc {
    pub async fn run(mut self) -> Result<()> {
        println!("ctrl_svc: service running");

        // send a success response for whatever cmd received
        Ok(())
    }
}