//! Peer to peer connectivity logic including nat traversal

// UPnP (automates the process of allowing an application to operate through NAT)
pub mod upnp{
    use igd::{search_gateway, Error, Gateway, PortMappingProtocol};
    use core::net::SocketAddrV4;

    fn discover_gateway() -> Result<Gateway, Error>{
        let gateway = search_gateway(Default::default())?;
        
        Ok(gateway)
    }

    // Add a port mapping for TCP on port 8080
    pub fn add_port_mapping(socket: SocketAddrV4){
        match discover_gateway(){
            Ok(gateway) => {
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
                // prints on current device external ip address
                println!("external ip address: {}", gateway.get_external_ip().unwrap());
            },
            Err(err) => eprintln!("Failed to find gateway: {}", err),
        }
        
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