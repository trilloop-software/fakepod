use anyhow::Result;
use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::mpsc::{Receiver, Sender}, io::{AsyncReadExt, AsyncWriteExt}};
use super::packet::*;

pub struct PodConnSvc {
    pub rx_ctrl: Receiver<PodPacket>,
    pub tx_ctrl: Sender<PodPacket>,
    pub rx_tele: Receiver<PodPacket>,
    pub tx_tele: Sender<PodPacket>
}

impl PodConnSvc {
    pub async fn run(mut self, port: u16) -> Result<()> {
        println!("pod_conn_svc: service running");
        let addr: SocketAddr = format!["0.0.0.0:{}", port.clone()].parse().unwrap();
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("pod_conn_svc: pod device listening on port {}", port);
        
        //handles pod establishing connection to this device
        while let Ok((mut stream, addr)) = listener.accept().await {
            println!("new client from {}", addr);
            let mut buf = vec![0; 1024];

            loop {
 
                let req = match stream.read(&mut buf).await{
                    Ok(size) => {
        
                        println!("received command");

                        //try to decode the command
                        let decoded =  decode(buf[0..size].to_vec());

                        //for now, resp Packet contains hard-coded data
                        let resp = encode(self.handle_request(decoded).await.unwrap());
        
                        //try to send the test packet back to the backend
                        stream.write_all(&resp).await?;

                        
                        println!("decoded command");
                    },
                    Err(e) => println!("failed to receive command: {}", (e).to_string())
                };


            }
        }

        Ok(())
    }

    async fn handle_request(&mut self, pkt: PodPacket) -> Result<PodPacket> {

        let resp = PodPacket::new(0, vec![("Test").to_string()]);

        //let resp: PodPacket = match pkt.cmd_type {
        //    0..=127 => {
        //        // send to controls service
        //        pkt
        //    },
        //    128..=255 => {
        //        // send to telemetry service
        //        pkt
        //    }
        //};
        Ok(resp)
    }
}