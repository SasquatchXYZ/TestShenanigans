use chrono::naive::{NaiveDate, NaiveTime};

pub fn test_test_naive() {
    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25));
    println!("{:?}", NaiveTime::from_hms_opt(12, 5, 30));
    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25).unwrap().and_hms_opt(12, 5, 30));
}