use std::error::Error;
use tokio::{io::AsyncReadExt, net::TcpStream}; 
use core_lib::{Request, client::send_request};

// starts client
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>{
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;
    let mut buffer: [u8; 1024] = [0; 1024];
    let request = Request::List;
    println!("Sending List request");
    send_request(&mut stream, &request).await?;

    let response  = stream.read(&mut buffer).await.unwrap();
    println!("{}",String::from_utf8_lossy(&buffer[..response]).trim().to_string());

    // Send Get request
    let request = Request::Get("elden ring".to_owned());
    println!("Sending get request");
    send_request(&mut stream, &request).await?;

    let response  = stream.read(&mut buffer).await.unwrap();
    println!("{}",String::from_utf8_lossy(&buffer[..response]).trim().to_string());
    Ok(())
}