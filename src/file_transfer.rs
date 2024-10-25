use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::{self, create_dir_all, File};
use tokio::net::TcpStream;
use std::path::{Path, PathBuf};
use std::io::Error as IoError;
use serde::{Serialize, Deserialize};
use futures::future::BoxFuture;

/// Error type for transfer errors
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferError {
    IoError(String), // Store the error message as a String
    ConnectionClosed,
    FileNotFound,
    FileCorrupted,
    ChunkError,
    // Other errors can be added here
}

impl From<IoError> for TransferError {
    fn from(err: IoError) -> TransferError {
        TransferError::IoError(err.to_string()) // Convert IoError to String
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FSObjectMetadata {
    pub file_size: Option<u64>,
    pub file_name: String,
    pub path_type: PathType
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PathType {
    File,
    Directory
}

impl FSObjectMetadata {

    fn new(file_size: Option<u64>, file_name: String, path_type: PathType) -> Self {
        Self {file_size, file_name, path_type}
    }

    // Serialize to bytes for sending over the network
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()  // Using `bincode` for serialization
    }

    // Deserialize from bytes for receiving over the network
    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

// Simplified Connection struct using only TcpStream
pub struct Connection<'a> {
    pub stream: &'a mut TcpStream,
}


impl<'a> Connection<'a> {
    // Example of writing to the TCP connection
    pub async fn write(&mut self, data: &[u8]) -> Result<(), TransferError> {
        self.stream
            .write_all(data)
            .await
            .map_err(|e| TransferError::IoError(e.to_string())) // Convert std::io::Error to String
    }

    // Example of reading from the TCP connection
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, TransferError> {
        self.stream
            .read(buffer)
            .await
            .map_err(|e| TransferError::IoError(e.to_string())) // Convert std::io::Error to String
    }
}

// Define the file transfer protocol
pub struct FileTransferProtocol {
    pub path: PathBuf,
    pub chunk_size: u64,
}

impl FileTransferProtocol {
    // Initialize the file transfer protocol
    pub fn new(path: &Path, chunk_size: u64) -> Self {
        FileTransferProtocol {
            path: path.to_owned(),
            chunk_size,
        }
    }

    pub async fn init_send(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        if self.path.is_dir() {
            self.send_directory(connection).await?;
        } else {
            self.send_file(connection).await?;
        }
        Ok(())
    }
    pub async fn init_receive(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        todo!()
    }
    // Send file logic over TCP
    pub async fn send_file(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        let mut file = File::open(&self.path).await.map_err(|_| TransferError::FileNotFound)?;

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

            // TODO implement progress reporting
            println!("Sent {} bytes", total_bytes_sent);
        }

        Ok(())
    }

    // Receive file logic over TCP
    pub async fn receive_file(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        let mut file = File::create(&self.path).await.map_err(TransferError::from)?;

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
            println!("Received {} bytes", total_bytes_received);
        }

        Ok(())
    }

    pub fn send_directory<'a>(&'a self, connection: &'a mut Connection) -> BoxFuture<'a, Result<(), TransferError>> {
        Box::pin(async move {
            let mut dir_entries = fs::read_dir(&self.path).await.map_err(|_| TransferError::FileNotFound)?;

            while let Some(entry) = dir_entries.next_entry().await.map_err(TransferError::from)? {
                let entry_path = entry.path();
                let metadata = entry.metadata().await.map_err(TransferError::from)?;

                if metadata.is_dir() {
                    // Send directory metadata
                    let dir_metadata = FSObjectMetadata::new(None, entry_path.file_name().unwrap().to_string_lossy().to_string(), PathType::Directory);
                    connection.write(&dir_metadata.to_bytes()).await?;

                    // Recursively send the directory's contents
                    self.send_directory(connection).await?;
                } else if metadata.is_file() {
                    // Send file metadata
                    let file_metadata = FSObjectMetadata::new(Some(metadata.len()), entry_path.file_name().unwrap().to_string_lossy().to_string(), PathType::File);
                    connection.write(&file_metadata.to_bytes()).await?;

                    // Send the file content
                    let file_transfer = FileTransferProtocol::new(&entry_path, self.chunk_size);
                    file_transfer.send_file(connection).await?;
                }
            }
            Ok(())
        })
    }

    pub async fn receive_directory(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        loop {
            // Receive metadata (directory or file)
            let mut metadata_buffer = vec![0u8; 1024];
            let n = connection.read(&mut metadata_buffer).await?;
            if n == 0 {
                break;  // End of transfer
            }

            let metadata = FSObjectMetadata::from_bytes(&metadata_buffer[..n]);

            match metadata.path_type {
                PathType::Directory => {
                    // Create directory
                    let dir_path = self.path.join(&metadata.file_name);
                    create_dir_all(&dir_path).await.map_err(TransferError::from)?;
                }
                PathType::File => {
                    // Receive the file
                    let file_path = self.path.join(&metadata.file_name);
                    let file_transfer = FileTransferProtocol::new(&file_path, self.chunk_size);
                    file_transfer.receive_file(connection).await?;
                }
            }
        }

        Ok(())
    }
}
