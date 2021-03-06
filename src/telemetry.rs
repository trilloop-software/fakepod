use anyhow::{Result};
use rand::Rng;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::dummy_reader;

use super::pod_packet::PodPacket;
use super::pod_packet_payload::*;

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
    pub async fn run(mut self, dummy : dummy_reader::Dummy) -> Result<()> {
        println!("tele_svc: service running");
        let field_names = dummy.fields;

        loop {
            tokio::select! {
                //handle commands from pod_conn
                cmd = self.rx.recv() => {

                    let mut packet = cmd.unwrap();

                    match packet.cmd_type {
                        //command for retrieving fields for device telemetry
                        //part of the discovery packet
                        1 =>{
                            // extract the payload from the packet
                            // store a Vec<String> of field names inside it
                            let field_names = field_names.clone();
                            let mut resp_payload = decode_payload(packet.payload);
                            resp_payload.field_names = field_names;

                            //modify the packet to:
                            // -have an updated id
                            // -contain the requested data in the payload
                            packet.payload = encode_payload(resp_payload);

                            //send the packet back to pod_conn
                            self.tx.send(packet).await;

                        }
                        //command for retrieving telemetry data
                        128 =>{
                            //package the data into a Vec<String>
                            //store the Vec inside the pod packet payload
                        }
                        _ => ()
                    }
                }
            }
        }

        // print all telemetry data
        println!("{:#?}", self.data.clone());
        self.data = TelemetryData::random_data(self.data);
        println!("{:#?}", self.data.clone());

        Ok(())
    }
}
