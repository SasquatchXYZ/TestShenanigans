#[derive(Debug)]
struct One;

#[derive(Debug)]
struct Two;

impl From<One> for Two {
    fn from(value: One) -> Self {
        Two
    }
}

pub fn test_two_blanket_trait_example() {
    let two: Two = One.into();
    let try_two = Two::try_from(One);
    println!("{two:?}, {try_two:?}");
}