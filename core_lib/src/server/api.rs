use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use std::io;
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
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error handling connection: {:?}", e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    loop {
        // Read data from the socket
        let bytes_read = socket.read(&mut buffer).await?;
        if bytes_read == 0 {
            // Connection was closed
            break;
        }
        // Convert the buffer to a string (assuming UTF-8 encoded data)
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        // Handle the request and generate a response
        let response = handle_request(request.to_string()).await;

        // Send the response back to the client
        socket.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

async fn handle_request(request: String) -> String {
    // Example: Simple echo server, responds with the same data received
    format!("Echo: {}", request)
}

