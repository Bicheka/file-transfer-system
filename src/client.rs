use std::{ path::Path, sync::Arc, time::Duration};

use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex, time};
use bincode;
use crate::{file_transfer::{Connection, FileTransferProtocol, TransferError}, network::Request};

pub struct Client {
    client_storage_path: String,
    server_address: String,
    timeout: Option<Duration>,
    connection: Arc<Mutex<Option<TcpStream>>>,  
}

impl Client {
    pub fn new(client_storage_path: &str, server_address: &str) -> Self {
        Self {
            client_storage_path: client_storage_path.to_owned(),
            server_address: server_address.to_owned(),
            timeout: None,
            connection: Arc::new(Mutex::new(None))
        }
    }

    /// Sets a timeout duration for the client.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }

    /// Connects to the server.
    pub async fn connect(&mut self) -> Result<(), anyhow::Error> {
        let timeout_duration = self.timeout.unwrap_or(Duration::from_secs(30)); // Default timeout
        let connect_future = TcpStream::connect(&self.server_address);

        // Apply timeout to the connection attempt
        let stream = time::timeout(timeout_duration, connect_future).await??;
        let mut connection = self.connection.lock().await;
        *connection = Some(stream);
        Ok(())
    }

    /// Sends a request to the server.
    pub async fn send_request(&self, request: Request) -> Result<(), anyhow::Error> {
        let mut connection = self.connection.lock().await;
        if let Some(ref mut connection) = *connection {
            let request_bytes = bincode::serialize(&request)?;
            let timeout_duration = self.timeout.unwrap_or(Duration::from_secs(30)); // Default timeout
            
            // Apply timeout to the write operation
            time::timeout(timeout_duration, connection.write_all(&request_bytes)).await??;
        } else {
            return Err(anyhow::Error::msg("No active connection"))
        };
        Ok(())
    }

    /// Uses File Transfer Protocol to init a send to server through an already stablished connection
    pub async fn send(&self, path_to_send: &str) -> Result<(), TransferError> {
        let mut connection = self.connection.lock().await;
        let connection = connection.as_mut().expect("Connection is not stablished");
        FileTransferProtocol::new(&Path::new(path_to_send), 64 * 1024)
            .init_send(&mut Connection { stream: connection })
            .await?;
        Ok(())
    }

    /// Closes the connection to the server.
    pub async fn close(&mut self) -> Result<(), anyhow::Error> {
        let mut connection = self.connection.lock().await;
        if let Some(mut connection) = connection.take() {
            connection.shutdown().await?;
        }
        Ok(())
    }
}