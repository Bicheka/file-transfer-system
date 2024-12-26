use crate::file_transfer::TransferError;

/// Represents a configuration for the server.
pub struct ServerConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub storage_path: String,
    pub buffer_size: u64,
}

pub trait Server{
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self);
    fn handle_request(&self, mut stream: TlsStream, shutdown_signal: Arc<Notify>) -> Result<(), Box<dyn std::error::Error>>;
    fn match_request(&self, request: &Request, stream: &mut TlsStream<TcpStream>) -> Result<(), TransferError>;
}