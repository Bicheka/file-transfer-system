use crate::file_transfer::TransferError;

#[derive(Debug, Clone, Copy)]
/// Represents a configuration for the server.
pub struct ServerConfig {
    pub is_server_running: Arc<Mutex<bool>>,
    pub ip: IpAddr,
    pub port: u16,
    pub storage_path: String,
    pub buffer_size: u64,
    pub stop_signal: Arc<Notify>,
}

impl ServerConfig {
    pub fn new(ip: IpAddr, port: u16, path: &str, buffer_size: u64, stop_signal: Arc<Notify>) -> Self {
        let is_server_running = Arc::new(Mutex::new(false));
        Self {
            is_server_running,
            ip,
            port,
            storage_path: path.to_owned(),
            buffer_size,
            stop_signal,
        }
    }
}

pub trait Server{
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self);
    fn handle_request(&self, mut stream: TlsStream, shutdown_signal: Arc<Notify>) -> Result<(), Box<dyn std::error::Error>>;
    fn match_request(&self, request: &Request, stream: &mut TlsStream<TcpStream>) -> Result<(), TransferError>;
}