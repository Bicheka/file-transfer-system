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
pub struct FileEntry {
    path: Vec<String>,
    is_dir: bool,
}

impl FileEntry {
    pub fn new(base: &Path, full: &Path) -> Result<FileEntry, String> {
        let path = Self::path_difference_to_vec(base, full)
            .ok_or_else(|| "Could not convert path to vector".to_owned())?;
        let is_dir = full.is_dir();
        
        Ok(FileEntry { path, is_dir })
    }

    pub fn path_difference_to_vec(base: &Path, full: &Path) -> Option<Vec<String>> {
        if let Ok(relative_path) = full.strip_prefix(base) {
            let mut path_vec: Vec<String> = vec![];

            // Add the last component of the base path as the "root"
            if let Some(root_name) = base.components().last() {
                path_vec.push(root_name.as_os_str().to_string_lossy().into_owned());
            }

            // Append the components of the relative path
            path_vec.extend(
                relative_path
                    .components()
                    .map(|component| component.as_os_str().to_string_lossy().into_owned())
            );

            Some(path_vec)
        } else {
            None
        }
    }

    pub fn vec_to_path(&self) -> PathBuf {
        self.path.iter().collect::<PathBuf>()
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
        let mut file = tokio::fs::File::create_new(&self.path).await?;
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
        self.receive_file(connection).await?;
        unzip_file(self.path.with_extension("zip").to_str().unwrap(), self.path.to_str().unwrap()).unwrap();
        Ok(())
    }


}