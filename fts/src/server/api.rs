use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use std::net::{IpAddr, SocketAddr};
use bincode;
use crate::network::{Request, Response};
use tokio::sync::Notify;
use std::sync::Arc;



/// Starts server by listening for incomming connections
pub async fn run_api(ip: &IpAddr, port: u16, stop_signal: Arc<Notify>) -> io::Result<()> {
    let listener = TcpListener::bind(SocketAddr::new(ip.to_owned(), port)).await?;
    println!("Server running on {}", ip);

    loop {
        tokio::select! {
            // Wait for an incoming connection
            result = listener.accept() => {
                match result {
                    Ok((socket, addr)) => {
                        println!("New connection from: {}", addr);
                        let stop_signal_clone = Arc::clone(&stop_signal);
                        tokio::spawn(async move {
                            if let Err(e) = handle_request(socket, stop_signal_clone).await {
                                eprintln!("Error handling connection: {:?}", e);
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {:?}", e);
                    }
                }
            },

            // Wait for the stop signal
            _ = stop_signal.notified() => {
                println!("Stopping server...");
                break;
            },
        }
    }
    println!("loop broken");
    Ok(())
}

/// handles connections and reads the data transmited through the socket
pub async fn handle_request(
    mut socket: TcpStream,
    shutdown_signal: Arc<Notify>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    loop {
        tokio::select! {
            // Check if we have received a shutdown signal
            _ = shutdown_signal.notified() => {
                println!("Shutdown signal received. Closing connection.");
                break;
            }

            // Read data from the socket
            bytes_read = socket.read(&mut buffer) => {
                match bytes_read {
                    Ok(0) => {
                        // Connection was closed
                        println!("Connection closed by client.");
                        break;
                    }
                    Ok(bytes_read) => {
                        // Convert the buffer to a string (assuming UTF-8 encoded data)
                        let request: Request = match bincode::deserialize(&buffer[..bytes_read]) {
                            Ok(req) => req,
                            Err(e) => {
                                eprintln!("Failed to deserialize request: {:?}", e);
                                continue;
                            }
                        };

                        // Handle the request and generate a response
                        let response = match_request(&request).await;

                        // Serialize response
                        let response = bincode::serialize(&response)?;

                        // Send the response back to the client
                        socket.write_all(&response).await?;
                    }
                    Err(e) => {
                        eprintln!("Failed to read data from socket: {:?}", e);
                        break;
                    }
                }
            }
        }
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