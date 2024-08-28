// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::async_runtime;
use tokio::{runtime::Builder, task};
use core_lib::{unified, graceful_shutdown::exit};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!!!!!!!! You've been greeted from Rust!", name)
}

fn main() {
    use std::thread;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            thread::spawn(|| {
                let rt = Builder::new_multi_thread()
                    .worker_threads(4)
                    .enable_all()
                    .build()
                    .unwrap();

                rt.block_on(async {
                    let program = task::spawn(unified::start_server());
                    let shutdown = task::spawn(exit());
                    tokio::select! {
                        _ = program => {},
                        _ = shutdown => {}
                    }
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running the application");
}