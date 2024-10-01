//! Core file transfer logic

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::net::SocketAddr;
use std::path::Path;
use crate::network::Response;


pub struct FileSender;

impl FileSender {
    pub async fn send_file(path: &str, socket: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(path).await?;
        let mut buffer = [0; 1024];

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break; // End of file
            }

            let chunk = &buffer[..bytes_read];
            let serialized_chunk = bincode::serialize(&Response::FileChunk(chunk.to_vec()))?;
            socket.write_all(&serialized_chunk).await?;
        }

        // Indicate transfer completion
        let serialized_response = bincode::serialize(&Response::TransferComplete)?;
        socket.write_all(&serialized_response).await?;

        Ok(())
    }
}

pub async fn send_file_to_peer(addr: SocketAddr, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect(addr).await?;
    println!("Sending file to peer at {}", addr);

    let mut file = tokio::fs::File::open(file_path).await?;
    let mut buffer = [0; 1024];

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        socket.write_all(&buffer[..n]).await?;
    }

    println!("File sent successfully to peer");
    Ok(())
}



pub struct DirectoryReceiver;

impl DirectoryReceiver {
    pub async fn receive_directory(mut socket: TcpStream, save_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let save_path = Path::new(save_dir);

        loop {
            let mut buffer = [0; 1024];
            let bytes_read = socket.read(&mut buffer).await?;

            if bytes_read == 0 {
                break;
            }

            let response: Response = bincode::deserialize(&buffer[..bytes_read])?;

            match response {
                Response::FileChunk(chunk) => {
                    // Write each file chunk into the appropriate file
                    // Here, you could extract filenames and save each file accordingly
                    println!("Received file chunk");
                }
                Response::TransferComplete => {
                    println!("Directory transfer complete.");
                    break;
                }
                Response::Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
                _ => (),
            }
        }

        Ok(())
    }
}