use file_transfer_system::p2p::upnp::upnp;
use file_transfer_system::server::Server;
use file_transfer_system::{network, server};
use tauri::State;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::sync::Arc;
use tauri::async_runtime::block_on;
use tokio::sync::{Mutex, Notify};

pub struct GlobalState {
    server: Arc<Mutex<Option<server::Server>>>,
    stop_signal: Arc<Notify>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            server: Arc::new(Mutex::new(None)),
            stop_signal: Arc::new(Notify::new()),
        }
    }

    pub async fn get_server(&self) -> Arc<Mutex<Option<server::Server>>> {
        self.server.clone()
    }

    pub fn get_stop_signal(&self) -> Arc<Notify> {
        Arc::clone(&self.stop_signal)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let global_state = GlobalState::new();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .manage(global_state)
        .invoke_handler(tauri::generate_handler![start_server, stop_server])
        .setup(|_app| {
            block_on(start_client());

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running the application");
}

#[tauri::command]
async fn start_server(global_state: State<'_, GlobalState>) -> Result<String, String> {
    create_server(&global_state).await.unwrap();

    let state_arc = global_state.get_server().await;
    let arc_clone = Arc::clone(&state_arc);

    tokio::spawn(async move {
        let mut lock = arc_clone.lock().await;
        if let Some(server) = lock.as_mut() {
            server.start_server().await.unwrap()
        }
    });

    Ok("Server Running".to_string())
}

async fn create_server(global_state: &State<'_, GlobalState>) -> Result<(), String> {
    let stop_signal = global_state.get_stop_signal();
    let stop_signal_clone = Arc::clone(&stop_signal);

    let port: u16 = 8080;
    let ip = if let Ok(ip) = upnp(port).await {
        println!("Public IP: {}", ip);
        network::get_local_ip().expect("failed to get local IP address")
    } else {
        println!("Continuing using IPv6");
        network::get_public_ip(network::IpType::IPv6)
            .await
            .expect("failed to start server with IPv6")
    };
    let state_arc = global_state.get_server().await;
    let mut arc_clone = state_arc.lock().await;
    if arc_clone.is_none() {
        let server = Server::new(ip, port, stop_signal_clone);
        *arc_clone = Some(server);
    } else {
        println!("server already exists")
    }
    Ok(())
}

#[tauri::command]
async fn stop_server(global_state: State<'_, GlobalState>) -> Result<(), String> {
    println!("Trying to stop server");
    // Notify the stop signal
    let stop_signal = global_state.get_stop_signal();
    stop_signal.notify_one(); // Notify the server to stop
    Ok(())
}

async fn start_client() {
    println!("starting client")
}
