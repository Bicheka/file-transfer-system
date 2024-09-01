use crate::{p2p::upnp, server::api};
use local_ip_address::local_ip;

pub async fn start_server(){
    upnp::upnp().await;
    let addr = get_local_ip_as_string().unwrap();
    let port = "8080";
    api::run(&format!("{addr}:{port}")).await.unwrap();
}

fn get_local_ip_as_string() -> Result<String, String> {
    match local_ip() {
        Ok(ip) => Ok(ip.to_string()),
        Err(e) => Err(format!("Failed to get local IP address: {}", e)),
    }
}