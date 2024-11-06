use std::{net::IpAddr, str::FromStr, sync::Arc};

use file_transfer_system::{client:: Client, server::Server};
use tokio::sync::Notify;

#[tokio::test]
async fn test_client_connect(){
    let stop_signal = Arc::new(Notify::new());
    let stop_signal_clone = Arc::clone(&stop_signal);
    tokio::spawn(async move{
        let mut server = Server::new( IpAddr::from_str("127.0.0.1").unwrap(), 8080, "/desktop",  4096, stop_signal_clone);
        println!("{}", server.ip);
        server.start_server().await.unwrap();
    });
    
    let ip_address = IpAddr::from_str("127.0.0.1").unwrap();
    let mut client = Client::new("desktop", ip_address);
    client.connect().await.expect("could not connect to server");
}