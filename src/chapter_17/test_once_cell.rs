use once_cell::sync::OnceCell;
use reqwest::Client;
use std::sync::Mutex;

#[derive(Debug)]
struct Logger {
    logs: Mutex<Vec<Log>>,
    url: String,
    client: Client,
}

#[derive(Debug)]
struct Log {
    message: String,
    timestamp: i64,
}

static GLOBAL_LOGGER: OnceCell<Logger> = OnceCell::new();

fn fetch_url() -> String {
    "https://somethingsomething.com".to_string()
}

pub fn test_once_cell() {
    let url = fetch_url();

    GLOBAL_LOGGER
        .set(Logger {
            logs: Mutex::new(vec![]),
            url,
            client: Client::default(),
        })
        .unwrap();

    GLOBAL_LOGGER
        .get()
        .unwrap()
        .logs
        .lock()
        .unwrap()
        .push(Log {
            message: "Everything's going well".to_string(),
            timestamp: 1658930674,
        });
    println!("{GLOBAL_LOGGER:?}");
}