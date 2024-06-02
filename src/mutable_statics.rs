use std::os::macos::fs::MetadataExt;
use std::sync::Mutex;

#[derive(Debug)]
struct Log {
    date: &'static str,
    message: String,
}

static GLOBAL_LOGGER: Mutex<Vec<Log>> = Mutex::new(Vec::new());

fn add_message(date: &'static str) {
    GLOBAL_LOGGER.lock().unwrap().push(Log {
        date,
        message: "Everything's fine".to_string(),
    });
}

pub fn test_mutable_statics() {
    add_message("2022-12-12");
    add_message("2023-05-05");
    println!("{GLOBAL_LOGGER:#?}");
}
