use std::error::Error;
use tokio::{io::AsyncReadExt, net::TcpStream}; 
use core_lib::{Request, client::send_request};

// starts client
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>{
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;

    let request = Request::List;
    send_request(&mut stream, &request).await?;

    // Send Get request
    let request = Request::Get("elden ring".to_owned());
    println!("Sending get request");
    send_request(&mut stream, &request).await?;

    let mut buffer = [0; 1024];
    loop{
        let bytes_read = stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            println!("Connection closed by client");
            break;
        }
        println!("{}",String::from_utf8_lossy(&buffer[..bytes_read]));
    }

    Ok(())
}