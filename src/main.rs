use anyhow::Result;
use tokio::{spawn, sync::mpsc};

mod controls;
mod pod_packet;
mod pod_packet_payload;
mod pod_conn;
mod telemetry;

use pod_packet::*;
use pod_packet_payload::*;

#[tokio::main]
async fn main() -> Result<(), ()> {
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

    spawn(ctrl_svc.run());
    spawn(pod_conn_svc.run(14000));
    spawn(tele_svc.run());

    loop {

    }
}
