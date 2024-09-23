pub mod input_handling {
    pub struct User {
        pub name: String,
        pub bank: Bank,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Bank {
        BankOfAmerica,
        Hsbc,
        Citigroup,
        DeutscheBank,
        TorontoDominionBank,
        SiliconValleyBank,
    }

    pub mod user_input {
        use crate::chapter_21::input_handling::input_handling::{Bank, User};

        pub fn handle_user_input(user: &User) -> Result<(), ()> {
            match user.bank {
                Bank::SiliconValleyBank => {
                    println!(
                        "Darn it, looks like we have to handle this variant even though Silicon Valley Bank doesn't exist anymore: {}:{}:{}:{}",
                        module_path!(),
                        file!(),
                        column!(),
                        line!()
                    );
                    Ok(())
                }
                other_bank => {
                    println!("{other_bank:?}, no problem");
                    Ok(())
                }
            }
        }
    }
}