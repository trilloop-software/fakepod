use anyhow::Result;
use tokio::sync::mpsc::{Receiver, Sender};
use super::pod_packet::PodPacket;
use super::pod_packet_payload::*;

pub struct ControlsSvc {
    pub rx: Receiver<PodPacket>,
    pub tx: Sender<PodPacket>
}

impl ControlsSvc {
    pub async fn run(mut self) -> Result<()> {
        println!("ctrl_svc: service running");
        let placeholder_cmds =vec![2,3,4];

        loop {
            tokio::select! {
                //handle commands from pod_conn
                cmd = self.rx.recv() => {

                    let mut packet = cmd.unwrap();

                    match packet.cmd_type {
                        //command for retrieving available commands for device controls
                        //part of the discovery packet
                        1 =>{
                            //build and encode a payload containing a Vec<u8> of commands
                            let commands = placeholder_cmds.clone();
                            let mut resp_payload = decode_payload(packet.payload);
                            resp_payload.commands = commands;

                            //modify the packet to:
                            // -have an updated id
                            // -contain the requested data in the payload
                            packet.payload = encode_payload(resp_payload);

                            //send the packet back to pod_conn
                            self.tx.send(packet).await;
                        }
                        _ => ()
                    }
                }
            }
        }

        // send a success response for whatever cmd received
        Ok(())
    }
}