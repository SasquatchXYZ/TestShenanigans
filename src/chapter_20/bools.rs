use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn send_data_to_user() {}

pub fn test_bools() {
    let bool_vec = vec![true, false, true, false, false];

    let result_vec = bool_vec
        .into_iter()
        .enumerate()
        .map(|(index, b)| {
            b.then(|| {
                let timestamp = timestamp();
                send_data_to_user();
                timestamp
            })
                .ok_or_else(|| {
                    let time = timestamp();
                    format!("Error with item {index} at {time}")
                })
        })
        .collect::<Vec<_>>();
    println!("{result_vec:#?}");
}