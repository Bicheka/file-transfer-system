use fts::p2p::get_public_ip;


#[tokio::test(flavor = "multi_thread")]
async fn test_public_ip_addr(){
    let ip = get_public_ip().await.unwrap();
    println!("{ip}")
}