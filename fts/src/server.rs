//! Contains logic for listening for incomming connections

use std::io;

/// Contains client request handling logic
pub mod api;

/// Module for everything related to administrate server side
pub mod admin;

pub struct Server{
    pub is_server_running: bool,
    pub addr: String
}

impl Server{
    pub fn new(addr: String) -> Self{
        Self {
            is_server_running: false,
            addr
        }
    }
    pub async fn start_server(&mut self) -> Result<(), io::Error>{
        api::run_api(&self.addr).await?;
        self.is_server_running = true;
        Ok(())
    }
    // pub async fn stop_server(&self) -> Result<(), io::Error>{
        
    // }
}