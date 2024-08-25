//! Peer to peer connectivity logic including nat traversal

// UPnP (automates the process of allowing an application to operate through NAT)
pub mod upnp{
    use igd::{search_gateway, Error, Gateway, PortMappingProtocol};
    use core::net::SocketAddrV4;

    pub fn discover_gateway() -> Result<Gateway, Error>{
        let gateway = search_gateway(Default::default())?;
        Ok(gateway)
    }

    // Add a port mapping for TCP on port 8080
    pub fn add_port_mapping(gateway: Gateway, socket: SocketAddrV4){
        match gateway.add_port(
            PortMappingProtocol::TCP,
            8080,
            socket,
            0,
            "rust-upnp"

        ) {
            Ok(_) => println!("Port mapping added successfully"),
            Err(err) => eprintln!("Failed to add port mapping: {}", err),
        }
    }
}