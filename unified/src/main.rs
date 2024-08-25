use std::net::SocketAddrV4;
use core::net::IpAddr;
use core_lib::p2p::upnp;
use local_ip_address::local_ip;

fn main() {
    let gateway = upnp::discover_gateway().unwrap();
    let ip = local_ip().unwrap();
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
