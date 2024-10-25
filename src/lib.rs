/// Contains functionality for the server component of the file transfer system.
pub mod server;

/// Contains networking utilities for handling connections and data transmission.
pub mod network;

/// Contains functionality for the client component of the file transfer system.
pub mod client;

/// Provides peer-to-peer transfer capabilities. This module is only available 
/// when the `p2p` feature is enabled.
#[cfg(feature = "p2p")]
pub mod p2p;

/// Enables graceful shutdown for the server on Windows systems. 
/// This module is available when the `graceful-shutdown` feature is enabled.
#[cfg(feature = "graceful-shutdown")]
#[cfg(windows)]
pub mod graceful_shutdown;

/// Core file transfer functionality for sending and receiving files and directories.
pub mod file_transfer;
