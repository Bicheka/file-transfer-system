use upnp::upnp;

/// Attempts to traverse NAT using available methods.
///
/// Currently, only UPnP is implemented. If UPnP is successful, the function will return.
/// Otherwise, it will print an error message and may attempt other NAT traversal methods in the future.
pub async fn traverse_nat() {
    match upnp(8080).await {
        Ok(_) => return,
        Err(e) => println!("error setting up upnp: {}", e),
    }
}

/// Module containing UPnP (Universal Plug and Play) functionality for NAT traversal.
pub mod upnp {
    use igd::{search_gateway, Error, Gateway, PortMappingProtocol};
    use core::net::{SocketAddrV4, IpAddr};
    use std::net::Ipv4Addr;
    use crate::network::get_local_ip;

    /// Attempts to enable UPnP port mapping on the specified port for an IPv4 address.
    pub async fn upnp(port: u16) -> anyhow::Result<Ipv4Addr> {
        let ip = get_local_ip().unwrap();
        if let IpAddr::V4(ipv4) = ip {
            let socket = SocketAddrV4::new(ipv4, port);
            let ip = add_port_mapping(socket)?;
            Ok(ip)
        } else {
            Err(anyhow::Error::msg("Only IPv4 is supported for UPnP port mapping."))
        }
    }

    /// Discovers the gateway for UPnP-enabled networks.
    ///
    /// # Returns
    ///
    /// A result containing the gateway if successful, or an error if the discovery fails.
    pub fn discover_gateway() -> Result<Gateway, Error> {
        let gateway = search_gateway(Default::default())?;
        Ok(gateway)
    }

    /// Adds a port mapping for TCP on the specified port.
    ///
    /// # Parameters
    ///
    /// - `socket`: The local socket address, including IP and port.
    ///
    /// # Returns
    ///
    /// A result containing the external IP address if successful, or an error if the mapping fails.
    pub fn add_port_mapping(socket: SocketAddrV4) -> anyhow::Result<Ipv4Addr> {
        let gateway = discover_gateway()?;
        gateway.add_port(
            PortMappingProtocol::TCP,
            socket.port(),
            socket,
            0,
            "rust-upnp",
        )?;
        let ip = gateway.get_external_ip()?;
        Ok(ip)
    }

    /// Removes an existing port mapping for cleanup purposes.
    pub async fn remove_port_mapping(port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let gateway = discover_gateway()?;
        gateway.remove_port(PortMappingProtocol::TCP, port)?;
        Ok(())
    }
}

// TODO: Add STUN/TURN methods for further NAT traversal capabilities.

#[cfg(test)]
#[cfg(feature = "p2p")] // Ensure the test only runs when the `p2p` feature is enabled
mod tests {
    use super::*;
    use tokio;

    /// Tests the UPnP functionality by attempting to open a port mapping.
    #[tokio::test]
    async fn test_upnp() {
        let ip = upnp::upnp(8080).await;
        assert_eq!(ip.is_ok(), true);
    }
}
