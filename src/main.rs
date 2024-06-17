use crate::chapter_19::async_with_try_join::test_async_with_try_join;

mod chapter_19;

#[tokio::main]
async fn main() {
    test_async_with_try_join().await
}
