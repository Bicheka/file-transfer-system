//! networking logic
use std::{collections::HashMap, net::IpAddr, str::FromStr};

use serde::{Deserialize, Serialize};
use local_ip_address::local_ip;

use crate::file_transfer::FileMetadata;

#[derive(Serialize, Deserialize)]
pub enum Request {
    List, // list of file names/paths
    Get(String), // get("filename/path")
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Ok(String),
    DirectoryListing(HashMap<String, Vec<u8>>), // List of files in a director
    Err(String),                // In case of any error
}
pub enum IpType{
    IPv4,
    IPv6
}

/// gets local ip address as a string
pub fn get_local_ip() -> anyhow::Result<IpAddr> {
    return Ok(local_ip()?);
}

pub async fn get_public_ip(ip_type: IpType) -> anyhow::Result<IpAddr> {
    match ip_type{
        IpType::IPv4 => {
            let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
            return Ok(IpAddr::from_str(&ip)?)
        },
        IpType::IPv6 => {
            let ip = reqwest::get("https://api64.ipify.org").await?.text().await?;
            return Ok(IpAddr::from_str(&ip)?)
        }
    };
}