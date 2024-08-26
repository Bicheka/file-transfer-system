use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
pub mod server;

pub mod client;
pub mod p2p;

#[derive(Serialize, Deserialize)]
pub enum Request {
    List, // list of file names/paths
    Get(String), // get("filename/path")
}

pub mod graceful_shutdown {
    use tokio::signal::windows;
    use std::future::Future;

    // The `on_exit` function is now a closure or function that returns a Future
    pub async fn exit<F, Fut>(on_exit: F) -> Result<(), std::io::Error>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        // Windows exit signals
        let mut ctrl_c = windows::ctrl_c()?;
        let mut ctrl_break = windows::ctrl_break()?;
        let mut ctrl_close = windows::ctrl_close()?;
        let mut ctrl_logoff = windows::ctrl_logoff()?;
        let mut ctrl_shutdown = windows::ctrl_shutdown()?;

        let shutdown_handle = tokio::task::spawn(async move {
            tokio::select! {
                _ = ctrl_c.recv() => {
                    println!("Received ctrlc, shutting down...");
                },
                _ = ctrl_break.recv() => {
                    println!("Received ctrl_break, shutting down...");
                },
                _ = ctrl_close.recv() => {
                    println!("Received ctrl_close, shutting down...");
                },
                _ = ctrl_logoff.recv() => {
                    println!("Received ctrl_logoff, shutting down...");
                },
                _ = ctrl_shutdown.recv() => {
                    println!("Received ctrl_shutdown, shutting down...");
                }
            }

            // Call the provided on_exit async function
            on_exit().await;
        });

        // Await the shutdown handle to ensure the on_exit function completes
        shutdown_handle.await?;

        Ok(())
    }
}