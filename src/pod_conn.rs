use anyhow::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::{net::TcpListener, sync::mpsc::{Receiver, Sender}, io::{AsyncReadExt, AsyncWriteExt}};

//access the packet struct to construct and parse packets
use super::pod_packet::*;

pub struct PodConnSvc {
    pub rx_ctrl: Receiver<PodPacket>,
    pub tx_ctrl: Sender<PodPacket>,
    pub rx_tele: Receiver<PodPacket>,
    pub tx_tele: Sender<PodPacket>
}

impl PodConnSvc {
    pub async fn run(mut self, mut port: u16) -> Result<()> {
        println!("pod_conn_svc: service running");
        let addr: SocketAddr = format!["0.0.0.0:{}", port.clone()].parse().unwrap();
        let listener = TcpListener::bind(addr).await.unwrap();

        //print the address that this thraed is listening on
        println!("pod_conn_svc: pod device listening on {}", listener.local_addr().unwrap().port());
        
        //handles pod establishing connection to this device
        while let Ok((mut stream, addr)) = listener.accept().await {
            println!("new client from {}", addr);
            let mut buf = vec![0; 1024];

            loop {
 
                let req = match stream.read(&mut buf).await{
                    Ok(size) => {
        
                        println!("received command");

                        //decode the command packet
                        let decoded =  decode(buf[0..size].to_vec());
                        println!("decoded command");

                        //check for disconnect command
                        if(decoded.cmd_type == 2){
                            //if the packet was a shutdown packet
                            //shutdown this client's thread
                            println!("disconnect command received from client");
                            break;
                        }

                        //handle the contents of the request packet
                        let pkt = self.handle_request(decoded).await.unwrap();

                        //detemrine if response packet is necessary

                        if(pkt.cmd_type != 2){
                            //otherwise, send back a reqsponse packet
                            //encode it into a vector of bytes
                            let resp = encode(pkt);
            
                            println!("response packet built");

                            //send the test packet back to the backend
                            stream.write_all(&resp).await?;

                            println!("response sent");

                        }
                        
                    },
                    Err(e) => println!("failed to receive command: {}", (e).to_string())
                };


            }
        }

        Ok(())
    }

    async fn handle_request(&mut self, pkt: PodPacket) -> Result<PodPacket> {

        //create response packet 
        //using request packet as a template
        let mut packet = pkt.clone();

        //starting with the command type
        let resp: PodPacket = match packet.cmd_type {

            //255 is reserved for emergency stop packets
            //check for this one first, in case of emergency
            255 =>{
                //call function to handle emergency actions. 'handle_emergency()'

                //return ACK packet
                packet
            }
            //0 is reserved for error packets
            0 =>{

                //return ACK packet
                packet
            },
            //1 is reserved for discovery packets
            1 =>{

                println!("discovery command received");

                //get device telemetry fields from telemtry_svc
                //telemtry_svc will add the new data to the PodPacketPayload object in the return packet
                self.tx_tele.send(packet).await;
                packet = self.rx_tele.recv().await
                    .unwrap_or(PodPacket::new(0,Vec::new())); //if podpacket was corrupted, create an error packet

                println!("obtained telemetry fields");

                //get commands from controls_svc
                //control_svc will add the new data to the PodPacketPayload object in the return packet
                self.tx_ctrl.send(packet).await;
                packet = self.rx_ctrl.recv().await
                    .unwrap_or(PodPacket::new(0,Vec::new())); //if podpacket was corrupted, create an error packet

                println!("obtained command list");

                println!("discovery command fulfilled");
                //return packet containing the data in its payload: list of commands, list of device fields
                packet
            },
            2 =>{
                //cmd #2 is reserved for disconnect packets
                //listener stream should be dropped before the code even reaches this point
                packet
            },
            //cmds for controlling device
            3..=127 => {
                // send to controls service
                packet
            },
            //cmds for telemetry of device
            128..=254 => {
                // send to telemetry service
                packet
            }
        };
        Ok(resp)
    }
}