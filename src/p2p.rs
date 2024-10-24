//! Peer to peer connectivity logic including nat traversal
use upnp::upnp;

// TODO add other nat traversing methods
pub async fn traverse_nat(){
    // Try UPnP
    match upnp(8080).await {
        Ok(_) => return,
        Err(e) => println!("error setting up upnp: {}", e)
    }

}
/// maps local ip address to router gateway, automates the process of allowing an application to operate through NAT
pub mod upnp{
    use igd::{search_gateway, Error, Gateway, PortMappingProtocol};
    use core::net::SocketAddrV4;
    use core::net::IpAddr;
    use std::net::Ipv4Addr;
    use crate::network::get_local_ip;

    /// gets local ip address creates a new socket and adds a port mapping with it
    pub async fn upnp(port: u16) -> anyhow::Result<Ipv4Addr>{
        let ip = get_local_ip().unwrap();
        if let IpAddr::V4(ipv4) = ip{
            let socket = SocketAddrV4::new(ipv4, port);
            let ip = add_port_mapping(socket)?;
            Ok(ip)
        } else {
            // Return an error if the IP is not IPv4.
            Err(anyhow::Error::msg("Only IPv4 is supported for UPnP port mapping."))
        }
    }   

    pub fn discover_gateway() -> Result<Gateway, Error>{
        let gateway = search_gateway(Default::default())?;
        Ok(gateway)
    }

    /// Add a port mapping for TCP on specified port
    pub fn add_port_mapping(socket: SocketAddrV4) -> anyhow::Result<Ipv4Addr>{
        let gateway = discover_gateway()?;
        gateway.add_port(
                PortMappingProtocol::TCP,
                socket.port(),
                socket,
                0,
                "rust-upnp"
                )?;
        let ip = gateway.get_external_ip()?;
        Ok(ip)
    }
    /// removes port mapping for cleanup, call it when port mapping is not longer needed
    pub async fn remove_port_mapping(port: u16) -> Result<(), Box<dyn std::error::Error>> { 
        let gateway = discover_gateway()?;
        gateway.remove_port(PortMappingProtocol::TCP, port)?;
        Ok(())
    }

}

// TODO add STUN TURN

#[cfg(test)]
#[cfg(feature = "p2p")]  // Ensure the test only runs when the `p2p` feature is enabled
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_upnp() {
        let ip = upnp::upnp(8080).await;
        assert_eq!(ip.is_ok(), true);
    }
}