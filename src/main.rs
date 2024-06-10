use crate::chapter_18::async_without_join_macro::wait_and_give_u8;

mod chapter_18;

#[tokio::main]
async fn main() {
    let num1 = wait_and_give_u8(1).await;
    let num2 = wait_and_give_u8(2).await;
    let num3 = wait_and_give_u8(3).await;

    println!("{num1}, {num2}, {num3}")
}
