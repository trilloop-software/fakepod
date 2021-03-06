use std::env;
use anyhow::Result;
use tokio::{spawn, sync::mpsc};

#[macro_use]
mod macros;

mod controls;
mod pod_packet;
mod pod_packet_payload;
mod pod_conn;
mod telemetry;
mod dummy_reader;

use pod_packet::*;
use pod_packet_payload::*;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect(); //get program args
    //arg0 = filepath

    if args.len() < 2 {panic!("Please supply a filepath to dummy device information")}
    let filepath = &args[1];

    let (tx_pod_to_tele, rx_pod_to_tele) = mpsc::channel::<PodPacket>(32);
    let (tx_pod_to_ctrl, rx_pod_to_ctrl) = mpsc::channel::<PodPacket>(32);
    let (tx_ctrl_to_pod, rx_ctrl_to_pod) = mpsc::channel::<PodPacket>(32);
    let (tx_tele_to_pod, rx_tele_to_pod) = mpsc::channel::<PodPacket>(32);

    let ctrl_svc = controls::ControlsSvc {
        rx: rx_pod_to_ctrl,
        tx: tx_ctrl_to_pod
    };

    let pod_conn_svc = pod_conn::PodConnSvc {
        rx_ctrl: rx_ctrl_to_pod,
        tx_ctrl: tx_pod_to_ctrl,
        rx_tele: rx_tele_to_pod,
        tx_tele: tx_pod_to_tele
    };

    let tele_svc = telemetry::TelemetrySvc {
        data: telemetry::TelemetryData::new(),
        rx: rx_pod_to_tele,
        tx: tx_tele_to_pod
    };

    let dummy = dummy_reader::load_dummy(filepath.to_string());

    spawn(ctrl_svc.run(dummy.clone()));
    //bind to port 0, so that it automatically binds to an available port
    spawn(pod_conn_svc.run(0));
    spawn(tele_svc.run(dummy.clone()));

    loop {

    }
}
