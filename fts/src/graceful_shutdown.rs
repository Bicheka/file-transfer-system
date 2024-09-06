use tokio::signal::windows;

use crate::p2p::upnp;

// The `on_exit` function is now a closure or function that returns a Future
pub async fn exit() -> Result<(), std::io::Error>{
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

async fn on_exit(){
    println!("Performing cleanup operations...");
    upnp::remove_port_mapping(8080).await.unwrap();
    println!("Shutdown complete.");
}