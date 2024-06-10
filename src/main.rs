use crate::chapter_18::test_reqwest_with_tokio::test_request_with_tokio;

mod chapter_18;

#[tokio::main]
async fn main() {
    test_request_with_tokio().await;
}
