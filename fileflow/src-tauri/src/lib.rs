use fts::server;
use fts::network;
use fts::p2p::traverse_nat;
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
        traverse_nat().await;
        let ip = network::get_local_ip_as_string().unwrap();
        let port = "8080";
        let addr = format!("{ip}:{port}");
        let mut server = server::Server::new(addr);
        server.start_server().await.unwrap();
    });
}

// #[tauri::command]
// async fn stop_server(){
//     println!("stoping server")
// }

// TODO create functionality to stop server
async fn start_client(){
    println!("starting client")
}
