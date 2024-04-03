use tokio::time;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(1));
    let mut counter = 0;
    loop{
        println!("{}", counter);
        counter+=1;
        interval.tick().await;
    }
}
