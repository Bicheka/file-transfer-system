//! Contains logic for listening for incomming connections

use std::{net::IpAddr, sync::Arc};

use tokio::sync::Notify;

/// Contains client request handling logic
pub mod api;

/// Module for everything related to administrate server side
pub mod admin;

pub struct Server{
    pub is_server_running: bool,
    pub ip: IpAddr,
    pub port: u16,
    stop_signal: Arc<Notify>, // Add a stop signal
}

impl Server{
    pub fn new(ip: IpAddr, port: u16, stop_signal: Arc<Notify>) -> Self{
        Self {
            is_server_running: false,
            ip,
            port,
            stop_signal,
        }
    }
    pub async fn start_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stop_signal = self.stop_signal.clone();
        api::run_api(&self.ip, self.port, stop_signal).await?;
        self.is_server_running = true;
        Ok(())
    }
}