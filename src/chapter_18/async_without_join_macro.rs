use std::time::Duration;
use rand::*;
use tokio::join;
use tokio::time::sleep;

async fn wait_and_give_u8(num: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let wait_time = rng.gen_range(1..100);
    sleep(Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");
    num
}

pub async fn test_async_without_join() {
    let num1 = wait_and_give_u8(1).await;
    let num2 = wait_and_give_u8(2).await;
    let num3 = wait_and_give_u8(3).await;

    println!("{num1}, {num2}, {num3}")
}

pub async fn test_async_with_join() {
    let nums = join!(
        wait_and_give_u8(1),
        wait_and_give_u8(2),
        wait_and_give_u8(3)
    );

    println!("{nums:?}")
}