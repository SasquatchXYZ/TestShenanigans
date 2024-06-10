use std::time::Duration;
use rand::*;
use tokio::time::sleep;

pub async fn wait_and_give_u8(num: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let wait_time = rng.gen_range(1..100);
    sleep(Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");
    num
}
