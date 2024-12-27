use super::common::{Server, ServerConfig};

#[derive(Clone, Copy, Debug)]
struct EnterpriseServer {
    pub config: ServerConfig
}
impl EnterpriseServer {
    pub fn new(config: ServerConfig);
}

impl Server for EnterpriseServer {
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