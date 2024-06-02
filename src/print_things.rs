use std::fmt::Display;

pub fn prints_one_thing<T: Display>(input: T) {
    println!("{input}");
}

#[derive(Debug)]
pub struct Billy {
    name: String,
    pub times_to_print: u32,
}

impl Billy {
    pub fn new(times_to_print: u32) -> Self {
        Self {
            name: "Billy".to_string(),
            times_to_print,
        }
    }
    pub fn print_billy(&self) {
        for _ in 0..self.times_to_print {
            println!("{}", self.name);
        }
    }
}