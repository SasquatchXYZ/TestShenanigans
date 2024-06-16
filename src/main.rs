use crate::chapter_18::async_with_try_join::test_async_with_try_join;

mod chapter_18;

#[tokio::main]
async fn main() {
    test_async_with_try_join().await
}
