//! networking logic
use std::{error::Error, net::IpAddr};

use serde::{Deserialize, Serialize};
use local_ip_address::local_ip;

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
pub fn get_local_ip() -> Result<IpAddr, Box<dyn Error>> {
    return Ok(local_ip()?);
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