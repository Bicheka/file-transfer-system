use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;
use tokio::net::TcpStream;
use std::path::Path;
use std::io::Error as IoError;

// Define an error type for transfer errors
#[derive(Debug)]
pub enum TransferError {
    IoError(IoError),
    ConnectionClosed,
    FileNotFound,
    FileCorrupted,
    ChunkError,
    // Other errors can be added here
}

impl From<IoError> for TransferError {
    fn from(err: IoError) -> TransferError {
        TransferError::IoError(err)
    }
}

// Simplified Connection struct using only TcpStream
pub struct Connection {
    pub stream: TcpStream,
}

impl Connection {
    // Example of writing to the TCP connection
    pub async fn write(&mut self, data: &[u8]) -> Result<(), TransferError> {
        self.stream.write_all(data).await.map_err(TransferError::IoError)
    }

    // Example of reading from the TCP connection
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, TransferError> {
        self.stream.read(buffer).await.map_err(TransferError::IoError)
    }
}

// Define the file transfer protocol
pub struct FileTransferProtocol {
    pub filename: String,
    pub file_size: u64,
    pub chunk_size: u64,
    pub checksum: Option<String>,  // Optional checksum for integrity
}

impl FileTransferProtocol {
    // Initialize the file transfer protocol
    pub fn new(filename: String, file_size: u64, chunk_size: u64) -> Self {
        FileTransferProtocol {
            filename,
            file_size,
            chunk_size,
            checksum: None,
        }
    }

    // Send file logic over TCP
    pub async fn send_file(&self, connection: &mut Connection) -> Result<(), TransferError> {
        let path = Path::new(&self.filename);
        let mut file = File::open(path).await.map_err(|_| TransferError::FileNotFound)?;

        let mut buffer = vec![0u8; self.chunk_size as usize];
        let mut total_bytes_sent = 0;

        // Read the file chunk by chunk and send it over the connection
        loop {
            let n = file.read(&mut buffer).await.map_err(TransferError::from)?;
            if n == 0 {
                break;
            }

            // Send the chunk over the TCP connection
            connection.write(&buffer[..n]).await?;
            total_bytes_sent += n as u64;

            // Optional: You can implement progress reporting
            println!("Sent {} bytes of {} total.", total_bytes_sent, self.file_size);
        }

        Ok(())
    }

    // Receive file logic over TCP
    pub async fn receive_file(&self, connection: &mut Connection) -> Result<(), TransferError> {
        let path = Path::new(&self.filename);
        let mut file = File::create(path).await.map_err(TransferError::from)?;

        let mut buffer = vec![0u8; self.chunk_size as usize];
        let mut total_bytes_received = 0;

        // Receive the file chunk by chunk and write it to the file
        loop {
            let n = connection.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            // Write the chunk to the file
            file.write_all(&buffer[..n]).await.map_err(TransferError::from)?;
            total_bytes_received += n as u64;

            // Optional: You can implement progress reporting
            println!("Received {} bytes of {} total.", total_bytes_received, self.file_size);
        }

        Ok(())
    }
}
