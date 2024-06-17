pub async fn test_request_with_tokio() {
    let client = reqwest::Client::default();
    let response = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .unwrap();
    println!("{}", response.text().await.unwrap());
}