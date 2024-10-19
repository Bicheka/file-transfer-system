//! Contains logic for listening for incomming connections

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::Notify};
use std::{collections::HashMap, net::{IpAddr, SocketAddr}};
use bincode;
use crate::{file_transfer::FileMetadata, network::{Request, Response}};
use std::sync::Arc;

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
                                    if let Err(e) = Self::handle_request(socket, stop_signal_clone).await {
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

    /// handles connections and reads the data transmited through the socket
    async fn handle_request(
        mut socket: TcpStream,
        shutdown_signal: Arc<Notify>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 1024];
        loop {
            tokio::select! {
                // Check if we have received a shutdown signal
                _ = shutdown_signal.notified() => {
                    println!("Shutdown signal received. Closing connection.");
                    break;
                }

                // Read data from the socket
                bytes_read = socket.read(&mut buffer) => {
                    match bytes_read {
                        Ok(0) => {
                            // Connection was closed
                            println!("Connection closed by client.");
                            break;
                        }
                        Ok(bytes_read) => {
                            // Convert the buffer to a string (assuming UTF-8 encoded data)
                            let request: Request = match bincode::deserialize(&buffer[..bytes_read]) {
                                Ok(req) => req,
                                Err(e) => {
                                    eprintln!("Failed to deserialize request: {:?}", e);
                                    continue;
                                }
                            };

                            // Handle the request and generate a response
                            let response = Self::match_request(&request).await;

                            // Serialize response
                            let response = bincode::serialize(&response)?;

                            // Send the response back to the client
                            socket.write_all(&response).await?;
                        }
                        Err(e) => {
                            eprintln!("Failed to read data from socket: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// handle the request depeding on what the request is asking for
    async fn match_request(request: &Request) -> Response {
        match request {
            Request::List => {
                // Call your get_list function here, for example:
                Response::DirectoryListing(Self::get_list().await)
            },
            Request::Get(path) => {
                println!("Handling Get request for: {}", path);
                let response = format!("Content of {}", path);
                Response::Ok(response)
            }
        }
    }
    // TODO allow to store a list of the files in the disk

    pub async fn get_list() -> HashMap<String, Vec<u8>>{
        let file = FileMetadata::File { path: "/example".to_owned(), size: 23 };
        HashMap::from([
            ("elden ring".to_owned(), file.to_bytes())
        ])
    }
}