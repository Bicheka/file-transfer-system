use std::net:: SocketAddrV4;
use core::net::IpAddr;
use core_lib::{p2p::upnp, server::api};
use local_ip_address::local_ip;

#[tokio::main]
async fn main() {

    upnp().await;
    let addr = get_local_ip_as_string().unwrap();
    let port = "8080";
    api::run(&format!("{addr}:{port}")).await.unwrap();
}

async fn upnp(){
    let gateway = upnp::discover_gateway().unwrap();
    let ip = local_ip().unwrap();
    println!("{}", gateway.get_external_ip().unwrap());
    match ip {
        IpAddr::V4(ipv4) => {
            let socket = SocketAddrV4::new(ipv4, 8080);
            upnp::add_port_mapping(gateway, socket);
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
    