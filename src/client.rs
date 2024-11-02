use std::{ sync::Arc, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, sync::Mutex, time};
use bincode;
use crate::network::Request;

pub struct Client {
    server_address: String,
    timeout: Option<Duration>,
    connection: Arc<Mutex<Option<TcpStream>>>,  
}

impl Client {
    pub fn new(server_address: &str) -> Self {
        Self {
            server_address: server_address.to_string(),
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

    /// Reads a response from the server.
    pub async fn read_response(&mut self) -> Result<(), anyhow::Error> {
        let mut connection = self.connection.lock().await;
        if let Some(ref mut connection) = *connection {
            let mut buffer = [0; 1024];
            let timeout_duration = self.timeout.unwrap_or(Duration::from_secs(30)); // Default timeout
            
            // Apply timeout to the read operation
            let bytes_read = time::timeout(timeout_duration, connection.read(&mut buffer)).await??;
            let response = bincode::deserialize(&buffer[..bytes_read])?;
            Ok(response)
        }
         else {
            Err(anyhow::Error::msg("No active connection"))
        }
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