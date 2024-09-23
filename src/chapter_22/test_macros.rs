#[macro_export]
macro_rules! check {
    ($input1:ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}

#[macro_export]
macro_rules! print_anything {
    ($($input1:tt),*) => {
        let output = stringify!($($input1),*);
        println!("{}", output);
    };
}

#[macro_export]
macro_rules! make_a_function {
    ($name:ident, $($input:tt),+) => {
        fn $name() {
            let output = stringify!($($input),+);
            println!("{}", output);
        }
    };
}