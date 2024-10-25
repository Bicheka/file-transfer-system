//! File transfer library for handling file and directory transfer over a TCP connection.

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::{self, create_dir_all, File};
use tokio::net::TcpStream;
use std::path::{Path, PathBuf};
use std::io::Error as IoError;
use serde::{Serialize, Deserialize};
use futures::future::BoxFuture;

/// Represents various errors that can occur during file transfer operations.
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferError {
    /// An I/O error, storing the error message as a `String`.
    IoError(String),
    /// Error indicating that the connection has closed unexpectedly.
    ConnectionClosed,
    /// Error when the specified file is not found.
    FileNotFound,
    /// Error indicating file corruption.
    FileCorrupted,
    /// Error encountered when handling chunks.
    ChunkError,
    // Additional error types can be added here
}

impl From<IoError> for TransferError {
    fn from(err: IoError) -> TransferError {
        TransferError::IoError(err.to_string())
    }
}

/// Metadata structure for file system objects (files or directories).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FSObjectMetadata {
    /// The size of the file in bytes, if available.
    pub file_size: Option<u64>,
    /// The name of the file or directory.
    pub file_name: String,
    /// The type of the path, either a file or a directory.
    pub path_type: PathType,
}

/// Specifies whether a path is a file or a directory.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PathType {
    /// Represents a file.
    File,
    /// Represents a directory.
    Directory,
}

impl FSObjectMetadata {
    /// Creates a new `FSObjectMetadata` instance.
    fn new(file_size: Option<u64>, file_name: String, path_type: PathType) -> Self {
        Self { file_size, file_name, path_type }
    }

    /// Serializes the metadata to a byte vector for network transmission.
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    /// Deserializes metadata from a byte slice received over the network.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

/// Represents a connection over a TCP stream.
pub struct Connection<'a> {
    /// The underlying TCP stream.
    pub stream: &'a mut TcpStream,
}

impl<'a> Connection<'a> {
    /// Writes data to the TCP stream.
    pub async fn write(&mut self, data: &[u8]) -> Result<(), TransferError> {
        self.stream
            .write_all(data)
            .await
            .map_err(|e| TransferError::IoError(e.to_string()))
    }

    /// Reads data from the TCP stream into a buffer.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<usize, TransferError> {
        self.stream
            .read(buffer)
            .await
            .map_err(|e| TransferError::IoError(e.to_string()))
    }
}

/// Defines the file transfer protocol, allowing files and directories to be transferred.
pub struct FileTransferProtocol {
    /// The path of the file or directory to transfer.
    pub path: PathBuf,
    /// The chunk size in bytes for data transfer.
    pub chunk_size: u64,
}

impl FileTransferProtocol {
    /// Creates a new instance of `FileTransferProtocol`.
    pub fn new(path: &Path, chunk_size: u64) -> Self {
        FileTransferProtocol {
            path: path.to_owned(),
            chunk_size,
        }
    }

    /// Initiates sending a file or directory based on the `path` provided.
    pub async fn init_send(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        if self.path.is_dir() {
            self.send_directory(connection).await?;
        } else {
            self.send_file(connection).await?;
        }
        Ok(())
    }

    /// Initiates receiving a file or directory based on the `path_type` provided.
    pub async fn init_receive(&self, connection: &mut Connection<'_>, path_type: &PathType) -> Result<(), TransferError> {
        match path_type {
            PathType::File => self.receive_file(connection).await?,
            PathType::Directory => self.receive_directory(connection).await?
        }
        Ok(())
    }

    /// Sends a file in chunks over the TCP connection.
    pub async fn send_file(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        let mut file = File::open(&self.path).await.map_err(|_| TransferError::FileNotFound)?;

        let mut buffer = vec![0u8; self.chunk_size as usize];
        let mut total_bytes_sent = 0;

        loop {
            let n = file.read(&mut buffer).await.map_err(TransferError::from)?;
            if n == 0 {
                break;
            }

            connection.write(&buffer[..n]).await?;
            total_bytes_sent += n as u64;

            println!("Sent {} bytes", total_bytes_sent);
        }

        Ok(())
    }

    /// Receives a file in chunks and writes it to disk.
    pub async fn receive_file(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        let mut file = File::create(&self.path).await.map_err(TransferError::from)?;

        let mut buffer = vec![0u8; self.chunk_size as usize];
        let mut total_bytes_received = 0;

        loop {
            let n = connection.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            file.write_all(&buffer[..n]).await.map_err(TransferError::from)?;
            total_bytes_received += n as u64;

            println!("Received {} bytes", total_bytes_received);
        }

        Ok(())
    }

    /// Sends a directory and its contents recursively over the TCP connection.
    pub fn send_directory<'a>(&'a self, connection: &'a mut Connection) -> BoxFuture<'a, Result<(), TransferError>> {
        Box::pin(async move {
            let mut dir_entries = fs::read_dir(&self.path).await.map_err(|_| TransferError::FileNotFound)?;

            while let Some(entry) = dir_entries.next_entry().await.map_err(TransferError::from)? {
                let entry_path = entry.path();
                let metadata = entry.metadata().await.map_err(TransferError::from)?;

                if metadata.is_dir() {
                    let dir_metadata = FSObjectMetadata::new(None, entry_path.file_name().unwrap().to_string_lossy().to_string(), PathType::Directory);
                    connection.write(&dir_metadata.to_bytes()).await?;

                    self.send_directory(connection).await?;
                } else if metadata.is_file() {
                    let file_metadata = FSObjectMetadata::new(Some(metadata.len()), entry_path.file_name().unwrap().to_string_lossy().to_string(), PathType::File);
                    connection.write(&file_metadata.to_bytes()).await?;

                    let file_transfer = FileTransferProtocol::new(&entry_path, self.chunk_size);
                    file_transfer.send_file(connection).await?;
                }
            }
            Ok(())
        })
    }

    /// Receives a directory and its contents recursively from the TCP connection.
    pub async fn receive_directory(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        loop {
            let mut metadata_buffer = vec![0u8; 1024];
            let n = connection.read(&mut metadata_buffer).await?;
            if n == 0 {
                break;
            }

            let metadata = FSObjectMetadata::from_bytes(&metadata_buffer[..n]);

            match metadata.path_type {
                PathType::Directory => {
                    let dir_path = self.path.join(&metadata.file_name);
                    create_dir_all(&dir_path).await.map_err(TransferError::from)?;
                }
                PathType::File => {
                    let file_path = self.path.join(&metadata.file_name);
                    let file_transfer = FileTransferProtocol::new(&file_path, self.chunk_size);
                    file_transfer.receive_file(connection).await?;
                }
            }
        }

        Ok(())
    }
}
