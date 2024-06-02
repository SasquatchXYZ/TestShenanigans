const NUMBER: u8 = give_eight();

const fn give_eight() -> u8 {
    8
}

pub fn test_const_functions() {
    let mut my_vec = Vec::new();
    my_vec.push(give_eight());
}
