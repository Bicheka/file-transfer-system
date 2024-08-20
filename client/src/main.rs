use tokio::time::Duration;

use tokio::time::sleep;


// starts client
#[tokio::main]
pub async fn main(){
    loop {
        sleep(Duration::from_millis(1000)).await;
        println!("client.....");
    }
}