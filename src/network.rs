use std::{net::IpAddr, path::Path, str::FromStr};
use serde::{Deserialize, Serialize};
use local_ip_address::local_ip;
use crate::file_transfer::PathType;

/// Enum representing various types of requests that can be made in the system.
#[derive(Serialize, Deserialize)]
pub enum Request {
    /// Request to retrieve a file or directory located at a given path.
    Get(Box<Path>),
    /// Request to upload a file or directory, along with its `PathType`.
    Upload(PathType),
}

/// Enum representing the types of IP addresses.
pub enum IpType {
    /// Represents an IPv4 address.
    IPv4,
    /// Represents an IPv6 address.
    IPv6,
}

/// Gets the local IP address as a string.
pub fn get_local_ip() -> anyhow::Result<IpAddr> {
    Ok(local_ip()?)
}

/// Retrieves the public IP address of the specified type (IPv4 or IPv6).

pub async fn get_public_ip(ip_type: IpType) -> anyhow::Result<IpAddr> {
    match ip_type {
        IpType::IPv4 => {
            let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
            Ok(IpAddr::from_str(&ip)?)
        },
        IpType::IPv6 => {
            let ip = reqwest::get("https://api64.ipify.org").await?.text().await?;
            Ok(IpAddr::from_str(&ip)?)
        }
    }
}
