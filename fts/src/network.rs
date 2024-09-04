//! networking logic
use std::{error::Error, net::{IpAddr, Ipv4Addr}};

use serde::{Deserialize, Serialize};
use local_ip_address::local_ip;
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub enum Request {
    List, // list of file names/paths
    Get(String), // get("filename/path")
}

pub enum IpType{
    IPv4,
    IPv6
}

/// gets local ip address as a string
pub fn get_local_ip_as_string() -> Result<String, String> {
    match local_ip() {
        Ok(ip) => Ok(ip.to_string()),
        Err(e) => Err(format!("Failed to get local IP address: {}", e)),
    }
}

pub async fn get_public_ip(ip_type: IpType) -> Result<String, Box<dyn Error>> {
    match ip_type{
        IpType::IPv4 => {
            let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
            return Ok(ip)
        },
        IpType::IPv6 => {
            let ip = reqwest::get("https://api64.ipify.org").await?.text().await?;
            return Ok(ip)
        }
    };
}