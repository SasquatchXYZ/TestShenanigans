use crate::chapter_18::async_with_select::test_async_with_select;

mod chapter_18;

#[tokio::main]
async fn main() {
    test_async_with_select().await;
}
