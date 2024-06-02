use std::ops::Deref;

#[derive(Debug)]
struct HoldsANumber(u8);

impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn test_deref() {
    let my_number = HoldsANumber(20);
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();
}
