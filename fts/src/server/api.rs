use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use std::{io, net::{IpAddr, SocketAddr}};
use bincode;
use crate::network::{Request, Response};

/// Starts server by listening for incomming connections
pub async fn run_api(ip: &IpAddr, port: u16) -> io::Result<()>{
    // listen for client connection
    let listener = TcpListener::bind(SocketAddr::new(ip.to_owned(), port)).await?;
    println!("Server running on {}",ip);

    loop {
        // Accept incoming connections
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        
        // Handle the connection in a new task
        tokio::spawn(async move {
            if let Err(e) = handle_request(socket).await {
                eprintln!("Error handling connection: {:?}", e);
            }
        });
    }
}

/// handles connections and reads the data transmited through the socket
pub async fn handle_request(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    loop {
        // Read data from the socket
        let bytes_read = socket.read(&mut buffer).await?; // if client closes connection it reads 0
        if bytes_read == 0 {
            // Connection was closed
            break;
        }
        // Convert the buffer to a string (assuming UTF-8 encoded data)
        // requests can only be made from a client which serializes the request
        let request: Request = match bincode::deserialize(&buffer[..bytes_read]) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Failed to deserialize request: {:?}", e);
                continue;
            }
        };

        // Handle the request and generate a response
        let response = match_request(&request).await;

        // serialize response
        let response = bincode::serialize(&response)?;

        // Send the response back to the client
        socket.write_all(&response).await?;
    }

    Ok(())
}

/// handle the request depeding on what the request is asking for
async fn match_request(request: &Request) -> Response {
    match request {
        Request::List => {
            println!("Handling List request");
            Response::Ok("Available items: item1, item2, item3".to_owned())
        }
        Request::Get(path) => {
            println!("Handling Get request for: {}", path);
            let response = format!("Content of {}", path);
            Response::Ok(response)
        }
    }
}