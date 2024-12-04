use std::{ net::{IpAddr, SocketAddr}, sync::Arc, time::Duration};

use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex, time};
use bincode;
use tokio_rustls::{rustls::pki_types::ServerName, TlsConnector, TlsStream};
use crate::{file_transfer::{Connection, FileTransferProtocol, TransferError}, network::Request};

/// Represents a client for managing file transfers over a TCP connection.
///
/// The `Client` struct encapsulates the necessary details for establishing and managing
/// connections to a server for file transfer operations. It holds connection details,
/// configuration options, and storage settings for the client.
///
/// # Fields
///
/// * `client_storage_path` - A `String` specifying the local directory path where
///   files will be stored or retrieved for transfer.
/// 
/// * `server_address` - A `String` containing the address (IP and port) of the server
///   to which the client will connect for file transfers.
/// 
/// * `timeout` - An `Option<Duration>` specifying the maximum amount of time to wait 
///   for connection attempts or operations before timing out. If `None`, the client 
///   will use a default timeout or no timeout, depending on the underlying connection 
///   logic.
///
/// * `connection` - An `Arc<Mutex<Option<TcpStream>>>` that holds the TCP connection
///   to the server. This field is wrapped in `Arc` and `Mutex` to allow for safe, 
///   concurrent access across async contexts. The `Option<TcpStream>` is `None` until
///   the client successfully connects to the server.
pub struct Client {
    client_storage_path: String,
    server_address: IpAddr,
    timeout: Option<Duration>,
    connection: Arc<Mutex<Option<TlsStream<TcpStream>>>>,  
}

impl Client {
    pub fn new(client_storage_path: &str, server_address: IpAddr) -> Self {
        Self {
            client_storage_path: client_storage_path.to_owned(),
            server_address,
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

        if let Err(err) = rustls_post_quantum::provider().install_default() {
            eprintln!("Failed to install default CryptoProvider: {:?}", err)
        }

        let root_store = rustls::RootCertStore::from_iter(
            webpki_roots::TLS_SERVER_ROOTS
                .iter()
                .cloned(),
        );

        let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
         
        let addr = SocketAddr::new(self.server_address, 8080);
       
        let tcp = TcpStream::connect(addr).await?;

        let connector: TlsConnector = TlsConnector::from(Arc::new(config));

        let tls = connector
            .connect(ServerName::IpAddress(self.server_address.into()), tcp)
            .await.expect("Could not connect with tls");

        let mut connection =  self.connection.lock().await;
        *connection = Some(tokio_rustls::TlsStream::Client(tls));

        Ok(())
    }

    /// Sends a request to the server. Ok if if ok to continue, Err if server declines for some reason
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

    /// Initiates a file transfer to the server using the File Transfer Protocol (FTP).
    /// 
    /// This asynchronous function sends a file located at `path_to_send` through an 
    /// existing connection to the server. It establishes the transfer by setting up
    /// the file path and buffer size, then utilizes the `init_send` function of 
    /// `FileTransferProtocol` to handle the transmission over the connection.
    /// 
    /// # Arguments
    /// 
    /// * `path_to_send` - A string slice that specifies the path to the file 
    ///   intended for transfer to the server.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the file transfer is successfully initiated and completes
    /// without errors, or `Err(TransferError)` if any issue arises during the process.
    /// 
    /// # Errors
    /// 
    /// This function will return an error in the following cases:
    /// 
    /// * The connection is not established, causing a "Connection is not established"
    ///   error to be raised.
    /// * The `init_send` function encounters an error while transferring the file.
    pub async fn send(&self, path_to_send: &str) -> Result<(), TransferError> {
        let mut connection = self.connection.lock().await;
        let connection = connection.as_mut().expect("Connection is not established");
        FileTransferProtocol::new(path_to_send, 64 * 1024)
            .init_send(&mut Connection { stream: connection })
            .await?;
        Ok(())
    }

    /// Downloads a file from the server to the client's storage path using the File Transfer Protocol.
    ///
    /// This asynchronous function initiates a file download from the server through an 
    /// already established connection. The file will be saved at the path specified by 
    /// `client_storage_path`, using a buffer size of 64 KB for efficient data transfer.
    ///
    /// # Arguments
    /// 
    /// This function does not take any additional arguments but relies on the `client_storage_path`
    /// field of the `Client` struct to determine the location where the file should be saved.
    ///
    /// # Returns
    /// 
    /// Returns `Ok(())` if the file is successfully downloaded, or `Err(TransferError)` if an error
    /// occurs during the download process.
    ///
    /// # Errors
    ///
    /// This function may return an error in the following cases:
    /// 
    /// * The connection is not established, resulting in an "Connection is not established" error.
    /// * The `init_receive` function encounters an issue during the download process, returning a 
    ///   `TransferError`.
    pub async fn download(&self) -> Result<(), TransferError>{
        let mut connection = self.connection.lock().await;
        let connection = connection.as_mut().expect("Connection is not established");
        let ftp = FileTransferProtocol::new(&self.client_storage_path, 64 * 1024);
        ftp.receive(&mut Connection {stream: connection}).await?;
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