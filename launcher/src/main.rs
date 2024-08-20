use std::process::{Command, Stdio};
use std::io;
use std::thread;

fn main() -> io::Result<()> {
    println!("Starting launcher...");

    // Launch the server in a separate thread
    let server_handle = thread::spawn(|| {
        println!("Starting server...");
        let mut server_process = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .current_dir("server")
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start server");

        println!("Server started with PID: {:?}", server_process.id());
        server_process.wait().expect("Server process failed");
        println!("Server process finished");
    });

    // Launch the client in a separate thread
    let client_handle = thread::spawn(|| {
        println!("Starting client...");
        let mut client_process = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .current_dir("client")
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start client");

        println!("Client started with PID: {:?}", client_process.id());
        client_process.wait().expect("Client process failed");
        println!("Client process finished");
    });

    // Wait for both threads to complete
    server_handle.join().expect("Server thread failed");
    client_handle.join().expect("Client thread failed");

    println!("Launcher finished successfully");

    Ok(())
}





// use num_cpus;
// use tokio::{runtime::Builder, task, signal};

// mod server;
// mod client;

// enum NetworkRole{
//     Client,
//     Server,
//     Both
// }

// fn main() {
//     let network_role = NetworkRole::Server;
//     let num_cores = num_cpus::get(); // Get the number of logical cores
//     let rt = Builder::new_multi_thread()
//         .worker_threads(num_cores)
//         .enable_all()
//         .build()
//         .unwrap();
//     match network_role{
//         NetworkRole::Client => {
//             rt.block_on(async {
//                 let client_task = task::spawn(client::start_client());
//                 tokio::select! {
//                     _ = client_task => {
//                         println!("Client task completed.");
//                     },
//                     _ = signal::ctrl_c() => {
//                         println!("Shutting down...");
//                     }
//                 }
//             });

//         },
//         NetworkRole::Server =>{
//             rt.block_on(async {
//                 let server_task = task::spawn(server::start_server());

//                 tokio::select! {
//                     _ = server_task => {
//                         println!("Server task completed.");
//                     },
//                     _ = signal::ctrl_c() => {
//                         println!("Shutting down...");
//                     }
//                 }
//             });
//         },
//         NetworkRole::Both =>{
//             rt.block_on(async {
//                 let server_task = task::spawn(server::start_server());
//                 let client_task = task::spawn(client::start_client());

//                 // Wait for either server or client tasks to complete
//                 // or a Ctrl+C signal to gracefully shutdown
//                 tokio::select! {
//                     _ = server_task => {
//                         println!("Server task completed.");
//                     },
//                     _ = client_task => {
//                         println!("Client task completed.");
//                     },
//                     _ = signal::ctrl_c() => {
//                         println!("Shutting down...");
//                     }
//                 }
//             });
//         }
//     }
// }