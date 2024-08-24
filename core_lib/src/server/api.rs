use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use std::io;
use bincode;
use crate::Request;
pub async fn run(addr: &str) -> io::Result<()>{
    // listen for client connection
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on {}",addr);

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

// handles connections and reads the data transmited through the socket
async fn handle_request(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
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

        // Send the response back to the client
        socket.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

// handle the request depeding on what the request is asking for
async fn match_request(request: &Request) -> String {
    match request {
        Request::List => {
            println!("Handling List request");
            "Available items: item1, item2, item3".to_owned()
        }
        Request::Get(path) => {
            println!("Handling Get request for: {}", path);
            format!("Content of {}", path)
        }
    }
}