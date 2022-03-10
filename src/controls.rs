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

        //create placeholder data for now
        //Assume both arrays are the same size for now
        let placeholder_cmd_names =vec![
            s!["Placeholder Cmd A"],
            s!["Placeholder Cmd B"],
            s!["Placeholder Cmd C"],
        ];
        let placeholder_cmd_codes =vec![2,3,4];

        loop {
            tokio::select! {
                //handle requests from pod_conn
                cmd = self.rx.recv() => {

                    let mut packet = cmd.unwrap();

                    match packet.cmd_type {

                        //255 is reserved for emergency stop packets
                        //check for this one first, in case of emergency
                        //will activate the device's braking sequence
                        255 =>{
                            //call function to handle braking sequence
                            ControlsSvc::brake_seq().await;

                            //return ACK packet
                            self.tx.send(packet).await;
                        }
                        //254 is reserved for launch packets
                        //will activate the device's launching sequence
                        254 =>{
                            //call function to handle launching sequence
                            ControlsSvc::launch_seq().await;

                            //return ACK packet
                            self.tx.send(packet).await;
                        }
                        //0 is reserved for error packets
                        0 =>{
                            
                        }
                        //handle request for retrieving available commands for device controls
                        //part of the discovery packet
                        1 =>{
                            //build and encode a payload containing:
                            //  -a Vec<String> of command names
                            //  -a Vec<u8> of commands
                            let commands = placeholder_cmd_codes.clone();
                            let names = placeholder_cmd_names.clone();
                            let mut resp_payload = decode_payload(packet.payload);
                            resp_payload.command_names = names;
                            resp_payload.command_codes = commands;

                            //modify the packet to:
                            // -have an updated id
                            // -contain the requested data in the payload
                            packet.payload = encode_payload(resp_payload);

                            //send the packet back to pod_conn
                            self.tx.send(packet).await;
                        }
                        // device specific command
                        2 =>{
                            println!("Received device specific command");
                            println!("Executing cmd. cmd_code = 2");
                            println!("NOTE: This is a placeholder command");
                        }
                        // device specific command
                        3 =>{
                            println!("Received device specific command");
                            println!("Executing cmd. cmd_code = 3");
                            println!("NOTE: This is a placeholder command");
                        }
                        // device specific command
                        4 =>{
                            println!("Received device specific command");
                            println!("Executing cmd. cmd_code = 4");
                            println!("NOTE: This is a placeholder command");
                        }
                        _ => {
                            println!("No matching command was found");
                        }
                    }
                }
            }
        }

        // send a success response for whatever cmd received
        Ok(())
    }

    async fn launch_seq(){
        println!("Activating Launch Sequence");
        println!("Launch Sequence Complete");
    }
    
    async fn brake_seq(){
        println!("Activating Brake Sequence");
        println!("Brake Sequence Complete");
    }
}