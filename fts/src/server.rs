//! Contains logic for listening for incomming connections

use std::net::IpAddr;

use tokio::{io, sync::oneshot};

/// Contains client request handling logic
pub mod api;

/// Module for everything related to administrate server side
pub mod admin;

pub struct Server{
    pub is_server_running: bool,
    shutdown_signal: Option<oneshot::Sender<()>>,
    ip: IpAddr,
    port: u16
}

impl Server{
    pub async fn new(ip: IpAddr, port: u16) -> io::Result<Self>{
        
        Ok(Self {
            is_server_running: false,
            shutdown_signal: None,
            ip,
            port
        })
    }
    pub async fn start_server(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        // Create a shutdown channel
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        self.shutdown_signal = Some(shutdown_tx);

        api::run_api(shutdown_rx, self.ip, self.port).await;
    
        Ok(())
    }
    /// Stops the server gracefully
    pub fn stop(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_signal.take() {
            let _ = shutdown_tx.send(()); // Ignore the result as we don't care if the receiver was already dropped
        }
    }
}