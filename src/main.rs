use tokio::join;
use crate::chapter_18::async_without_join_macro::wait_and_give_u8;

mod chapter_18;

#[tokio::main]
async fn main() {
    let nums = join!(
        wait_and_give_u8(1),
        wait_and_give_u8(2),
        wait_and_give_u8(3)
    );

    println!("{nums:?}")
}
