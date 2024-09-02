//! networking logic

use serde::{Deserialize, Serialize};
use local_ip_address::local_ip;

#[derive(Serialize, Deserialize)]
pub enum Request {
    List, // list of file names/paths
    Get(String), // get("filename/path")
}
/// gets local ip address as a string
pub fn get_local_ip_as_string() -> Result<String, String> {
    match local_ip() {
        Ok(ip) => Ok(ip.to_string()),
        Err(e) => Err(format!("Failed to get local IP address: {}", e)),
    }
}