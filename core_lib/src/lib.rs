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
