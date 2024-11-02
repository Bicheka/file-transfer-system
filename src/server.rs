use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{Mutex, Notify},
};
use std::{
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    sync::Arc,
};
use bincode;
use crate::{
    file_transfer::{Connection, FileTransferProtocol, TransferError},
    network::Request,
};

/// Represents a file server that listens for incoming connections and handles file transfer requests.
#[derive(Clone)]
pub struct Server {
    /// Indicates if the server is currently running.
    pub is_server_running: Arc<Mutex<bool>>,
    /// The IP address on which the server listens.
    pub ip: IpAddr,
    /// The port on which the server listens.
    pub port: u16,
    /// The path to the directory where files are stored.
    pub path: PathBuf,
    /// Buffer size for file transfer operations.
    pub buffer_size: u64,
    /// Notification signal for stopping the server.
    pub stop_signal: Arc<Notify>,
}

impl Server {
    /// Creates a new instance of the `Server`.
    ///
    /// # Parameters
    ///
    /// - `ip`: IP address on which the server will listen.
    /// - `port`: Port on which the server will listen.
    /// - `path`: Directory path for file storage and retrieval.
    /// - `buffer_size`: Size of the buffer used for file transfers.
    pub fn new(ip: IpAddr, port: u16, path: &Path, buffer_size: u64, stop_signal: Arc<Notify>) -> Self {
        let is_server_running = Arc::new(Mutex::new(false));
        Self {
            is_server_running,
            ip,
            port,
            path: path.to_owned(),
            buffer_size,
            stop_signal,
        }
    }

    /// Starts the server, accepting and handling incoming connections.
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
                            let self_clone = self.clone();
                            tokio::spawn(async {
                                if let Err(e) = self_clone.handle_request(socket, stop_signal_clone).await {
                                    eprintln!("Error handling connection: {:?}", e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Failed to accept connection: {:?}", e);
                        }
                    }
                },
                _ = self.stop_signal.notified() => {
                    println!("Stopping server...");
                    break;
                },
            }
        }
        Ok(())
    }

    pub async fn stop_server(&self) {
        self.stop_signal.notify_waiters();
    }

    /// Handles an incoming connection by reading and processing client requests.
    async fn handle_request(
        self,
        mut socket: TcpStream,
        shutdown_signal: Arc<Notify>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 1024];
        loop {
            tokio::select! {
                _ = shutdown_signal.notified() => {
                    println!("Shutdown signal received. Closing connection.");
                    break;
                }
                bytes_read = socket.read(&mut buffer) => {
                    match bytes_read {
                        Ok(0) => {
                            println!("Connection closed by client.");
                            break;
                        }
                        Ok(bytes_read) => {
                            let request: Request = match bincode::deserialize(&buffer[..bytes_read]) {
                                Ok(req) => req,
                                Err(e) => {
                                    eprintln!("Failed to deserialize request: {:?}", e);
                                    continue;
                                }
                            };
                            let response = self.match_request(&request, &mut socket).await;

                            let response = bincode::serialize(&response)?;
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

    /// Matches the incoming request to the appropriate action and executes it.
    async fn match_request(&self, request: &Request, stream: &mut TcpStream) -> Result<(), TransferError> {
        match request {
            Request::Get(path) => {
                FileTransferProtocol::new(path, 64 * 1024)
                    .init_send(&mut Connection { stream })
                    .await?;
            }
            Request::Upload() => {
                FileTransferProtocol::new(&self.path, self.buffer_size)
                    .init_receive(&mut Connection { stream })
                    .await?;
            }
        }
        Ok(())
    }
}
