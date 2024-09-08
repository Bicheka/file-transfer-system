use std::{net::IpAddr, str::FromStr, time::Duration};

use fts::{client, network::{Request, Response}, server::Server};
#[tokio::test]
async fn test_client(){
    // Create a shutdown channel
    let mut server = Server::new(IpAddr::from_str("0.0.0.0").expect("could create IpAddr from str"), 8080).await.unwrap();
    
    server.start_server().await.unwrap();

    let mut client = client::Client::new("10.0.0.123:8080");
    client.set_timeout(Duration::from_secs(10));
    client.connect().await.unwrap();
    client.send_request(&Request::List).await.unwrap();

    let response = client.read_response().await.unwrap();
    match response {
        Response::Ok(s) => println!("{s}"),
        Response::Err(s) => eprintln!("{s}")
    }
    
    client.send_request(&Request::Get("call of duty".to_owned())).await.unwrap();
    let response = client.read_response().await.unwrap();
    match response {
        Response::Ok(s) => println!("{s}"),
        Response::Err(s) => eprintln!("{s}")
    }
    server.stop();
    client.send_request(&Request::List).await.unwrap();
    client.read_response().await.unwrap();

    client.close().await.unwrap();
    
    
}