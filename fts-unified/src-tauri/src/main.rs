// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::{join, runtime::Builder, task};
use core_lib::{unified, graceful_shutdown::exit};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tokio::main]
async fn main() {
    // use std::thread;
    // thread::spawn(|| {
        
    // });

    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::spawn(unified::start_server());
            tauri::async_runtime::spawn(exit());

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running the application");
}
