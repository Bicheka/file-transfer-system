use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::{self, create_dir_all, File};
use tokio::net::TcpStream;
use std::path::Path;
use std::io::Error as IoError;
use serde::{Serialize, Deserialize};
use futures::future::BoxFuture;

/// Error type for transfer errors
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileSystemObjectMetadata {
    File { path: String, size_bytes: u64 },
    Directory { path: String },
}

impl FileSystemObjectMetadata {
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

    pub path: String,
    pub chunk_size: u64,
    pub checksum: Option<String>,  // Optional checksum for integrity
}

impl FileTransferProtocol {
    // Initialize the file transfer protocol
    pub fn new(path: &str, chunk_size: u64) -> Self {
        FileTransferProtocol {
            path: path.to_owned(),
            chunk_size,
            checksum: None
        }
    }
    // Send file logic over TCP
    pub async fn send_file(&self, connection: &mut Connection) -> Result<(), TransferError> {
        let path = Path::new(&self.path);
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

            // TODO implement progress reporting
            println!("Received {} bytes", total_bytes_sent);
        }

        Ok(())
    }

    // Receive file logic over TCP
    pub async fn receive_file(&self, connection: &mut Connection) -> Result<(), TransferError> {
        let path = Path::new(&self.path);
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
            println!("Received {} bytes", total_bytes_received);
        }

        Ok(())
    }

    pub fn send_directory<'a>(&'a self, connection: &'a mut Connection, dir_path: &'a Path,) -> BoxFuture<'a, Result<(), TransferError>> {
        Box::pin(async move {
            let mut dir_entries = fs::read_dir(dir_path).await.map_err(|_| TransferError::FileNotFound)?;

            while let Some(entry) = dir_entries.next_entry().await.map_err(TransferError::from)? {
                let entry_path = entry.path();
                let metadata = entry.metadata().await.map_err(TransferError::from)?;

                if metadata.is_dir() {
                    // Send directory metadata
                    let dir_metadata = FileSystemObjectMetadata::Directory {
                        path: entry_path.to_string_lossy().into(),
                    };
                    connection.write(&dir_metadata.to_bytes()).await?;

                    // Recursively send the directory's contents
                    self.send_directory(connection, &entry_path).await?;
                } else if metadata.is_file() {
                    // Send file metadata
                    let file_metadata = FileSystemObjectMetadata::File {
                        path: entry_path.to_string_lossy().into(),
                        size_bytes: metadata.len(),
                    };
                    connection.write(&file_metadata.to_bytes()).await?;

                    // Send the file content
                    let file_transfer = FileTransferProtocol::new(
                        entry_path.to_str().expect("could not parse entry path into str"),
                        self.chunk_size,
                    );
                    file_transfer.send_file(connection).await?;
                }
            }
            Ok(())
        })
    }

    pub async fn receive_directory(&self, connection: &mut Connection) -> Result<(), TransferError> {
        loop {
            // Receive metadata (directory or file)
            let mut metadata_buffer = vec![0u8; 1024];  // Adjust buffer size as needed
            let n = connection.read(&mut metadata_buffer).await?;
            if n == 0 {
                break;  // End of transfer
            }
            
            let metadata = FileSystemObjectMetadata::from_bytes(&metadata_buffer[..n]);

            match metadata {
                FileSystemObjectMetadata::Directory { path } => {
                    // Create directory
                    let dir_path = Path::new(&path);
                    create_dir_all(dir_path).await.map_err(TransferError::from)?;
                }
                FileSystemObjectMetadata::File { path, size_bytes } => {
                    // Receive the file
                    let file_transfer = FileTransferProtocol::new(path.clone().as_str(), self.chunk_size);
                    file_transfer.receive_file(connection).await?;
                }
            }
        }

        Ok(())
    }
}