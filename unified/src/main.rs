use std::net::SocketAddrV4;
use core::net::IpAddr;
use core_lib::{graceful_shutdown::exit, p2p::upnp, server::api};
use local_ip_address::local_ip;

use tokio::{runtime::Builder, task};
fn main(){
    let rt = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let program = task::spawn(program());
        let shutdown = task::spawn(exit(on_exit));
        tokio::select! {
            _ = program => {},
            _ = shutdown => {}
        }
    });
}

async fn program(){
    upnp().await;
    let addr = get_local_ip_as_string().unwrap();
    let port = "8080";
    api::run(&format!("{addr}:{port}")).await.unwrap();
}

async fn upnp(){
    let ip = local_ip().unwrap();
    match ip {
        IpAddr::V4(ipv4) => {
            let socket = SocketAddrV4::new(ipv4, 8080);
            upnp::add_port_mapping(socket);
        },
        IpAddr::V6(_) => {
            println!("ipv6 not supported")
        }
    }
}

fn get_local_ip_as_string() -> Result<String, String> {
    match local_ip() {
        Ok(ip) => Ok(ip.to_string()),
        Err(e) => Err(format!("Failed to get local IP address: {}", e)),
    }
}

async fn on_exit(){
    println!("Performing cleanup operations...");
    upnp::remove_port_mapping().await;
    println!("Shutdown complete.");
}