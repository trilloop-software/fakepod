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
        
        while let Ok((mut stream, addr)) = listener.accept().await {
            println!("new client from {}", addr);
            let mut buf = vec![0; 1024];

            loop {
                let req = stream.read(&mut buf).await?;
                
                if req == 0 {
                    return Ok(());
                }

                let resp = self.handle_request(decode(buf.to_vec())).await.unwrap();
                let resp = encode(resp);
                stream.write_all(&resp).await?;
            }
        }

        Ok(())
    }

    async fn handle_request(&mut self, pkt: PodPacket) -> Result<PodPacket> {
        let resp: PodPacket = match pkt.cmd_type {
            0..=127 => {
                // send to controls service
                pkt
            },
            128..=255 => {
                // send to telemetry service
                pkt
            }
        };
        Ok(resp)
    }
}