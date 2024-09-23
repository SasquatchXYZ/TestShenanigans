use crate::chapter_21::input_handling::input_handling::{Bank, User};
use crate::chapter_21::input_handling::input_handling::user_input::handle_user_input;

mod chapter_20;
mod chapter_21;

fn main() {
    let user = User {
        name: "SomeUser".to_string(),
        bank: Bank::SiliconValleyBank,
    };
    handle_user_input(&user).unwrap();

    let user2 = User {
        name: "SomeUser2".to_string(),
        bank: Bank::TorontoDominionBank,
    };
    handle_user_input(&user2).unwrap()
}
