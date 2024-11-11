use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::path::{Path, PathBuf};
use std::io::Error as IoError;
use serde::{Serialize, Deserialize};
use futures::future::BoxFuture;
use crate::compression::{start_compressing, unzip_file};

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
}

impl From<IoError> for TransferError {
    fn from(err: IoError) -> TransferError {
        TransferError::IoError(err.to_string())
    }
}

/// Specifies whether a path is a file or a directory.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PathType {
    /// Represents a file.
    File,
    /// Represents a directory.
    Directory,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMetadata {
    pub name: String,      // Name of the file or directory (relative path)
    pub is_dir: bool,      // Whether it's a directory
    pub size: u64,         // Size of the file (in bytes, 0 for directories)
    pub checksum: Option<String>, // Optional checksum to verify integrity
}

impl FileMetadata {
    pub fn new(path: &Path) -> FileMetadata {
        let name = path.to_string_lossy().into_owned();
        let is_dir = path.is_dir();
        let size = if is_dir {
            0 // No size for directories
        } else {
            path.metadata().map(|meta| meta.len()).unwrap_or(0)
        };
        FileMetadata {
            name,
            is_dir,
            size,
            checksum: None, // Can be calculated if needed
        }
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
            .read_exact(buffer)
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
    pub fn new(path: &str, chunk_size: u64) -> Self {
        FileTransferProtocol {
            path: PathBuf::from(path),
            chunk_size,
        }
    }

    /// Initiates sending a file or directory based on the `path` provided.
    pub async fn init_send(&self, connection : &mut Connection<'_>) -> Result<(), TransferError> {
        let is_file = self.path.is_file();
        if is_file{
            self.send_file(&self.path, connection).await.expect("Could not send file");
        }
        else{
            self.send_dir(connection).await.unwrap();
        }

        Ok(())
    }

    pub async fn send_metadata(&self, connection: &mut Connection<'_>, metadata: &FileMetadata) -> Result<(), TransferError> {
        let serialized_metadata = serde_json::to_vec(metadata)
            .map_err(|e| TransferError::IoError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Send the metadata (size first, then the actual metadata)
        connection.write(&(serialized_metadata.len() as u64).to_le_bytes()).await?; // Send size of metadata
        connection.write(&serialized_metadata).await?; // Send the metadata
        
        Ok(())
    }

    pub async fn receive_metadata(&self, connection: &mut Connection<'_>) -> Result<FileMetadata, TransferError> {
        let mut size_buffer = [0u8; 8];
        connection.read(&mut size_buffer).await?;  // Read the size of the metadata
        let metadata_size = u64::from_le_bytes(size_buffer);

        let mut metadata_buffer = vec![0u8; metadata_size as usize];
        connection.read(&mut metadata_buffer).await?;  // Read the metadata itself

        let metadata: FileMetadata = serde_json::from_slice(&metadata_buffer)
            .map_err(|e| TransferError::IoError(format!("Failed to deserialize metadata: {}", e)))?;

        Ok(metadata)
    }



    /// Sends a single file in chunks over the TCP connection.
    pub async fn send_file(&self, file_path: &Path, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        let mut file = tokio::fs::File::open(file_path).await.map_err(|_| TransferError::FileNotFound)?;

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

    /// Sends a directory and its contents recursively over the TCP connection.
    pub fn send_dir<'a>(
        &'a self,
        connection: &'a mut Connection<'_>,
    ) -> BoxFuture<'a, Result<(), TransferError>> {
        Box::pin(async move {
            let path = self.path.clone();  // Clone the path here

            // send file metadata to receiving side
            self.send_metadata(connection, &FileMetadata::new(&path)).await?;
            
            let zip_path = path.with_extension("zip");
            let zip_clone = zip_path.clone();
            let handle = tokio::task::spawn_blocking( move || {
                start_compressing(&path, &zip_path, zip::CompressionMethod::Deflated).expect("Could not compress directory");
            });
            handle.await.unwrap();
            self.send_file(&zip_clone, connection).await?;
            Ok(())
        })
    }

    /// Receives a file in chunks and writes it to disk.
    pub async fn receive_file(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        let mut file = tokio::fs::File::create(&self.path).await?;
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

    /// Receives a directory and its contents recursively from the TCP connection.
    pub async fn receive_dir(&self, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        println!("Recieving directory to path: {:?}", self.path);
        let metadata = self.receive_metadata(connection).await?;
        self.receive_file(connection).await?;
        unzip_file(
            self.path
                .with_file_name(metadata.name)
                .with_extension("zip").to_str()
                .unwrap(), 
            self.path.to_str().unwrap()).unwrap();
        Ok(())
    }


}