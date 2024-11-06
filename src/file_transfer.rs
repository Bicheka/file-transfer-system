use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::{self, DirEntry, File, ReadDir};
use tokio::net::TcpStream;
use std::collections::VecDeque;
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

/// Specifies whether a path is a file or a directory.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PathType {
    /// Represents a file.
    File,
    /// Represents a directory.
    Directory,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileEntry {
    path: Vec<String>,
    is_dir: bool,
}

impl FileEntry {
    fn new(base: &Path, full: &Path) -> Result<FileEntry, String> {
        let path = Self::path_difference_to_vec(base, full)
            .ok_or_else(|| "Could not convert path to vector".to_owned())?;
        let is_dir = full.is_dir();
        
        Ok(FileEntry { path, is_dir })
    }

    fn path_difference_to_vec(base: &Path, full: &Path) -> Option<Vec<String>> {
        if let Ok(relative_path) = full.strip_prefix(base) {
            Some(
                relative_path
                    .components()
                    .map(|component| component.as_os_str().to_string_lossy().into_owned())
                    .collect(),
            )
        } else {
            None
        }
    }

    fn vec_to_path(&self) -> PathBuf {
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
    pub async fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, TransferError> {
        self.stream
            .read_to_end(buffer)
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
        // Convert `self.path` to a `Path` reference if it's not already.
        let path = Path::new(&self.path);

        if path.is_dir() {
            // If the path is a directory, initiate directory sending
            let read_dir = fs::read_dir(path).await?;
            let queue = VecDeque::from([read_dir]);  // Add initial directory to the queue
            self.send_dir(connection, queue).await?;
        } else {
            // If the path is a file, initiate file sending
            let mut rd = fs::read_dir(&self.path).await?;
            let dir = rd.next_entry().await.unwrap();
            let dir = dir.unwrap();
            self.send_file(&dir, connection).await?;
        }
        
        Ok(())
        
    }

    /// Sends a file in chunks over the TCP connection.
    pub async fn send_file(&self, entry: &DirEntry, connection: &mut Connection<'_>) -> Result<(), TransferError> {
        let mut file = File::open(entry.path()).await.map_err(|_| TransferError::FileNotFound)?;

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
        mut queue: VecDeque<ReadDir>,
    ) -> BoxFuture<'a, Result<(), TransferError>> {
        Box::pin(async move {
            let mut new_queue: VecDeque<ReadDir> = VecDeque::new();
            // for each dir in queue get all its inmediate subdirectories
            while let Some(dir) = queue.pop_front() {
                let mut subdirectories = get_inmediate_subdirectories_layer(dir).await;
                // for each subdirectory check if it is a file or a folder if it is a folder add it to the new_queue if it is a file send it
                while let Some(sub) = subdirectories.pop_front(){
                    let file_type = sub.file_type().await?;
                    
                    //Create metadata and send it over the connection
                    let entry_metadata = FileEntry::new(Path::new(&self.path), Path::new(&sub.path()));
                    let serialized = bincode::serialize(&entry_metadata)
                        .map_err(|_| TransferError::ChunkError)?;
                    connection.write(&serialized).await?;

                    if file_type.is_dir(){
                        let path = sub.path();
                        let read_dir = fs::read_dir(path).await?;
                        new_queue.push_back(read_dir);
                    }
                    else {
                        self.send_file(&sub, connection).await?;
                    }
                }
            }

            if !new_queue.is_empty(){
                self.send_dir(connection, new_queue).await?;
            }
            Ok(())
        })
    }

    /// Receives a file in chunks and writes it to disk.
    pub async fn receive_file(&self, connection: &mut Connection<'_>, file: &mut File) -> Result<(), TransferError> {
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
    pub async fn receive_directory(&self,  connection: &mut Connection<'_>) -> Result<(), TransferError> {
        loop {
            // Buffer for receiving serialized `FileEntry`
            let mut entry_buffer = vec![0; 1024];
            let bytes_read = connection.read(&mut entry_buffer).await?;
            if bytes_read == 0 {
                break; // Connection closed by sender, end of directory transfer
            }

            // Deserialize the FileEntry from the buffer
            let entry: FileEntry = bincode::deserialize(&entry_buffer[..bytes_read]).expect("Could not deserialize FileEntry");
            println!("Received entry: {:?}", entry);
            let entry_path = entry.vec_to_path();
            let base_path = &self.path;
            let full_path = base_path.join(entry_path);
            if entry.is_dir {
                let receiving_path = &self.path.join(full_path);
                fs::create_dir_all(receiving_path).await?;
            } else {
                // Create the file and receive its contents
                let mut file = File::create(full_path).await?;
                self.receive_file(connection, &mut file).await?;
            }
        }

        Ok(())
    }

}

// utils
pub async fn get_inmediate_subdirectories_layer(mut dir: ReadDir) -> VecDeque<DirEntry> {
    let mut subdirectories = VecDeque::new();
    while let Ok(Some(entry)) = dir.next_entry().await {
        if !entry.path().ends_with(".DS_Store") {
            subdirectories.push_back(entry);
        }
    }
    subdirectories
}

