//! Contains logic for listening for incomming connections

use std::{io, net::IpAddr};

/// Contains client request handling logic
pub mod api;

/// Module for everything related to administrate server side
pub mod admin;

pub struct Server{
    pub is_server_running: bool,
    pub ip: IpAddr,
    pub port: u16
}

impl Server{
    pub fn new(ip: IpAddr, port: u16) -> Self{
        Self {
            is_server_running: false,
            ip,
            port,
        }
    }
    pub async fn start_server(&mut self) -> Result<(), io::Error>{
        api::run_api(&self.ip, self.port).await?;
        self.is_server_running = true;
        Ok(())
    }
    // pub async fn stop_server(&self) -> Result<(), io::Error>{
        
    // }
}