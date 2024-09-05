use fts::network::{get_local_ip, get_public_ip, IpType};

#[tokio::test]
async fn test_get_public_ip(){
    println!("{:?}", get_public_ip(IpType::IPv6).await.unwrap());
    println!("{:?}", get_public_ip(IpType::IPv4).await.unwrap());
}

#[test]
fn test_get_local_ip(){
    println!("{:?}", get_local_ip().unwrap())
}