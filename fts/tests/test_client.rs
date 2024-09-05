use std::{net::IpAddr, str::FromStr, time::Duration};

use fts::{client, network::Request, network::Response, server};

#[tokio::test]
async fn test_client(){
    tokio::spawn(async{
        server::api::run_api(&IpAddr::from_str("0.0.0.0").unwrap(), 8080).await.unwrap();
    });

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
    client.close().await.unwrap();
}