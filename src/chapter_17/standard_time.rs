use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub fn test_standard_time_with_instant() {
    let start_of_main = Instant::now();
    let before_operation = Instant::now();

    let mut new_string = String::new();
    loop {
        new_string.push('^');
        if new_string.len() > 100_000 {
            break;
        }
    }
    let after_operation = Instant::now();
    println!("{:?}", before_operation - start_of_main);
    println!("{:?}", after_operation - start_of_main);
}

pub fn test_standard_time_with_elapsed() {
    let start = Instant::now();
    println!("Time elapsed before busy operation: {:?}", start.elapsed());

    let mut new_string = String::new();
    loop {
        new_string.push('^');
        if new_string.len() > 100_000 {
            break;
        }
    }
    println!("Operation complete. Time elapsed: {:?}", start.elapsed());
}

pub fn bad_random_number(digits: usize) {
    if digits > 9 {
        panic!("Random number can only be up to 9 digits");
    }
    let now_as_string = format!("{:?}", Instant::now());

    now_as_string
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .take(digits)
        .for_each(|character| print!("{}", character));
    println!();
}

pub fn test_system_time() {
    let instant = Instant::now();
    let system_time = SystemTime::now();
    println!("{instant:?}");
    println!("{system_time:?}");
    println!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
}