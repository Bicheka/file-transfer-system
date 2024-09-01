use fts::unified;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use tauri::async_runtime::block_on;
use tokio::task; // Use tokio for async tasks

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_server])
    .setup(|app| {
        block_on(start_client());

        Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .run(tauri::generate_context!())
    .expect("error while running the application");
}

#[tauri::command]
async fn start_server() {
    task::spawn(unified::start_server());
}
// TODO create functionality to stop server

async fn start_client(){
    println!("starting client")
}