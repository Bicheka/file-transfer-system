//! Contains the logic for the client, sending requests to a server

use std::{error::Error, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, time};
use bincode;
use crate::network::{Request, Response};

pub struct Client {
    server_address: String,
    timeout: Option<Duration>,
    connection: Option<TcpStream>,  
}

impl Client {
    pub fn new(server_address: &str) -> Self {
        Self {
            server_address: server_address.to_string(),
            timeout: None,
            connection: None
        }
    }

    /// Sets a timeout duration for the client.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }

    /// Connects to the server.
    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let timeout_duration = self.timeout.unwrap_or(Duration::from_secs(30)); // Default timeout
        let connect_future = TcpStream::connect(&self.server_address);

        // Apply timeout to the connection attempt
        let stream = time::timeout(timeout_duration, connect_future).await??;
        self.connection = Some(stream);
        Ok(())
    }

    /// Sends a request to the server.
    pub async fn send_request(&mut self, request: &Request) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut connection) = self.connection {
            let request_bytes = bincode::serialize(request)?;
            let timeout_duration = self.timeout.unwrap_or(Duration::from_secs(30)); // Default timeout
            
            // Apply timeout to the write operation
            time::timeout(timeout_duration, connection.write_all(&request_bytes)).await??;
        } else {
            return Err("No active connection".into());
        }
        Ok(())
    }

    /// Reads a response from the server.
    pub async fn read_response(&mut self) -> Result<Response, Box<dyn Error>> {
        if let Some(ref mut connection) = self.connection {
            let mut buffer = [0; 1024];
            let timeout_duration = self.timeout.unwrap_or(Duration::from_secs(30)); // Default timeout
            
            // Apply timeout to the read operation
            let bytes_read = time::timeout(timeout_duration, connection.read(&mut buffer)).await??;
            let response: Response = bincode::deserialize(&buffer[..bytes_read])?;
            Ok(response)
        }
         else {
            Err("No active connection".into())
        }
    }

    /// Closes the connection to the server.
    pub async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(mut connection) = self.connection.take() {
            connection.shutdown().await?;
        }
        Ok(())
    }
}