use fts::server;
use fts::network;
use fts::p2p::upnp::upnp;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use tauri::async_runtime::block_on;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let port: u16 = 8080;
    //gets local ip address creates a new socket and adds a port mapping with it
    let ip = if let Ok(ip) = upnp(port).await{
        println!("public ip: {}", ip);
        let ip = network::get_local_ip().expect("failed to get local ip address");
        ip
    } else {
        println!("continuing using ipv6");
        let ip = network::get_public_ip(network::IpType::IPv6).await.expect("failed to start server with ipv6");
        ip
    };

    let server = server::Server::new(ip, port);

    tauri::Builder::default()
    .manage(server)
    .invoke_handler(tauri::generate_handler![
        start_server,
        stop_server
    ])
    .setup(|_app| {
        block_on(start_client());

        Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .run(tauri::generate_context!())
    .expect("error while running the application");
}

#[tauri::command]
async fn start_server() {

}

#[tauri::command]
async fn stop_server(){
    println!("stoping server")
}


async fn start_client(){
    println!("starting client")
}
