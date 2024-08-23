use core_lib::server::api;

// starts server
#[tokio::main]
pub(crate) async fn main(){
    api::run("127.0.0.1:8000").await.unwrap();
}
