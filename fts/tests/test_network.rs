use fts::network::{get_ip, IpType};

#[tokio::test]
async fn test_get_ip(){
    println!("{:?}", get_ip(IpType::IPv6).await.unwrap());
    println!("{:?}", get_ip(IpType::IPv4).await.unwrap());
}