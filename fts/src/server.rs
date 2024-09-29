//! Contains logic for listening for incomming connections

use std::{io, net::{IpAddr, SocketAddr}, sync::Arc};

use tokio::{net::{TcpListener, TcpStream}, sync::Notify};

/// Module for everything related to administrate server side
pub mod admin;

pub struct Server{
    pub is_server_running: bool,
    pub ip: IpAddr,
    pub port: u16,
    stop_signal: Arc<Notify>, // Add a stop signal
}

impl Server{
    /// Creates new instance of server
    pub fn new(ip: IpAddr, port: u16, stop_signal: Arc<Notify>) -> Self{
        Self {
            is_server_running: false,
            ip,
            port,
            stop_signal,
        }
    }

    /// Starts server by listening for incomming connections
    pub async fn start_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(SocketAddr::new(self.ip.to_owned(), self.port)).await?;
            println!("Server running on {}", self.ip);

            loop {
                tokio::select! {
                    // Wait for an incoming connection
                    result = listener.accept() => {
                        match result {
                            Ok((socket, addr)) => {
                                println!("New connection from: {}", addr);
                                let stop_signal_clone = Arc::clone(&self.stop_signal);
                                tokio::spawn(async move {
                                    if let Err(e) = handle_request(socket, stop_signal_clone).await {
                                        eprintln!("Error handling connection: {:?}", e);
                                    }
                                });
                            }
                            Err(e) => {
                                eprintln!("Failed to accept connection: {:?}", e);
                            }
                        }
                    },

                    // Wait for the stop signal
                    _ = self.stop_signal.notified() => {
                        println!("Stopping server...");
                        break;
                    },
                }
            }
            println!("loop broken");
            Ok(())
    }

    pub async fn listen_connection(ip: IpAddr, port: u16) -> io::Result<(TcpStream, SocketAddr)>{
        let listener = TcpListener::bind(SocketAddr::new(ip.to_owned(), port)).await?;
        let result = listener.accept().await?;
        Ok(result)
    }
}