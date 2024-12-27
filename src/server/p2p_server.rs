use super::common::{Server, ServerConfig};

#[derive(Clone, Copy, Debug)]
struct P2PServer {
    pub config: ServerConfig
}
impl P2PServer {
    pub fn new(config: ServerConfig);
}

impl Server for P2PServer {
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn handle_request(&self, mut stream: TlsStream, shutdown_signal: Arc<Notify>) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn match_request(&self, request: &Request, stream: &mut TlsStream<TcpStream>) -> Result<(), crate::file_transfer::TransferError> {
        todo!()
    }
}