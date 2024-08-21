use core_lib::server::api;

// starts server
#[tokio::main]
pub(crate) async fn main(){
    api().await.unwrap();
}
