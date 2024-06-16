use crate::chapter_18::async_without_join_macro::{test_async_with_join, test_async_without_join};

mod chapter_18;

#[tokio::main]
async fn main() {
    test_async_without_join().await;

    test_async_with_join().await;
}
