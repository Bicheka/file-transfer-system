use num_cpus;
use tokio::{runtime::Builder, task, signal};

mod server;
mod client;

fn main() {
    let num_cores = num_cpus::get(); // Get the number of logical cores

    // Decide the number of worker threads based on the number of cores
    let worker_threads = num_cores * 2; // Example: Use twice the number of cores for IO-bound tasks

    let rt = Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let server_task = task::spawn(server::start_server());
        let client_task = task::spawn(client::start_client());
        tokio::select! {
            _ = server_task => {},  // If the server task ends, handle it
            _ = client_task => {},  // If the client task ends, handle it
            _ = signal::ctrl_c() => {  // Wait for a Ctrl+C signal to gracefully shutdown
                println!("Shutting down...");
            }
        }
    });
}