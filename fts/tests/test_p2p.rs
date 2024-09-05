use fts::p2p::upnp;

#[tokio::test]
async fn test_upnp(){
    let ip = upnp::upnp(8080).await.unwrap();
    println!("{ip}");
}