//! Peer to peer connectivity logic including nat traversal

use stun::stun;
use upnp::upnp;
pub async fn traverse_nat(){
    // Try UPnP
    match upnp().await {
        Ok(_) => return,
        Err(e) => println!("error setting up upnp: {}", e)
    }
    println!("UPnP failed, falling back to STUN.");

    // Try STUN
    match stun().await{
        Ok(_) => return,
        Err(e) => println!("Failed to establish stun connection: {}", e)
    }


}

/// automates the process of allowing an application to operate through NAT
pub mod upnp{
    use igd::{search_gateway, Error, Gateway, PortMappingProtocol};
    use core::net::SocketAddrV4;
    use core::net::IpAddr;
    use local_ip_address::local_ip;

    /// gets local ip address creates a new socket and adds a port mapping with it
    pub async fn upnp() -> Result<(), Box<dyn std::error::Error>>{
        let ip = local_ip().unwrap();
        match ip {
            IpAddr::V4(ipv4) => {
                let socket = SocketAddrV4::new(ipv4, 8080);
                add_port_mapping(socket)?;
            },
            IpAddr::V6(_) => {
                println!("ipv6 not supported");
            }
        }
        Ok(())
    }

    pub fn discover_gateway() -> Result<Gateway, Error>{
        let gateway = search_gateway(Default::default())?;
        
        Ok(gateway)
    }

    /// Add a port mapping for TCP on port 8080
    pub fn add_port_mapping(socket: SocketAddrV4) -> Result<(), Box<dyn std::error::Error>>{
        match discover_gateway(){
            Ok(gateway) => {
                gateway.add_port(
                PortMappingProtocol::TCP,
                8080,
                socket,
                0,
                "rust-upnp"
                )?;

                // prints on current device external ip address
                println!("external ip address: {}", gateway.get_external_ip().unwrap());
            },
            Err(err) => eprintln!("Failed to find gateway: {}", err),
        }

        Ok(())
        
    }
    
    pub async fn remove_port_mapping() {
        match discover_gateway() {
            Ok(gateway) => {
                match gateway.remove_port(PortMappingProtocol::TCP, 8080) {
                    Ok(_) => println!("Port mapping removed successfully"),
                    Err(err) => eprintln!("Failed to remove port mapping: {}", err),
                }
            }
            Err(err) => eprintln!("Failed to find gateway: {}", err),
        }   
    }

}


pub mod stun{
    pub async fn stun() -> Result<(), Box<dyn std::error::Error>>{
        // logic needs to be added
        println!("stun");
        Ok(())
    }
}

