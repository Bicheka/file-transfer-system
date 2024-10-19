use std::{net::IpAddr, str::FromStr, sync::Arc, time::Duration};

use file_transfer_system::{client, file_transfer::FileMetadata, network::{Request, Response}, server};
use tokio::sync::Notify;

#[tokio::test]
async fn test_client(){
    let stop_signal = Arc::new(Notify::new());
    let stop_signal_clone = Arc::clone(&stop_signal);

    tokio::spawn(async move{
        let mut server = server::Server::new(IpAddr::from_str("127.0.0.1").unwrap(), 8080, stop_signal_clone);
        server.start_server().await.unwrap();
    });

    let mut client = client::Client::new("127.0.0.1:8080");
    client.set_timeout(Duration::from_secs(10));
    client.connect().await.unwrap();
    client.send_request(&Request::List).await.unwrap();

    let response = client.read_response().await.unwrap();
    match response {
        Response::Ok(s) => println!("{s}"),
        Response::Err(s) => eprintln!("{s}"),
        Response::DirectoryListing(hm) => println!("{:?}", FileMetadata::from_bytes(hm.get("elden ring").unwrap()))
    }
    
    client.send_request(&Request::Get("call of duty".to_owned())).await.unwrap();
    let response = client.read_response().await.unwrap();
    match response {
        Response::Ok(s) => println!("{s}"),
        Response::Err(s) => eprintln!("{s}"),
        _ => println!("")
    }

    stop_signal.notify_waiters();

    client.send_request(&Request::Get("call of duty".to_owned())).await.unwrap();

    let response = client.read_response().await;
    assert_eq!(true, response.is_err());

    client.close().await.unwrap();
    
}