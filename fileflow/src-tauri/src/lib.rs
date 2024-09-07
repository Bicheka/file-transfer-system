use fts::server;
use fts::network;
use fts::p2p::upnp::upnp;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use tauri::async_runtime::block_on;
use tokio::task; // Use tokio for async tasks

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_server])
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
    task::spawn(async{
        let port: u16 = 8080;
        //gets local ip address creates a new socket and adds a port mapping with it
        if let Ok(ip) = upnp(port).await{
            println!("public ip: {}", ip);
            let mut server = server::Server::new(network::get_local_ip().expect("failed to get local ip address"), port);
            server.start_server().await.expect("failed to start server with upnp");
        } else {
            
            println!("continuing using ipv6");
            
            let ip = network::get_public_ip(network::IpType::IPv6).await.expect("failed to start server with ipv6");
            server::Server::new(ip, port);
        }
    });
}

// TODO create functionality to stop server
// #[tauri::command]
// async fn stop_server(){
//     println!("stoping server")
// }


async fn start_client(){
    println!("starting client")
}
