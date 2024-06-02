#[derive(Debug)]
struct Buffers<T, const N: usize> {
    array_one: [T; N],
    array_two: [T; N],
}

pub fn test_const_generics() {
    let buffer_1 = Buffers {
        array_one: [0u8; 3],
        array_two: [0; 3],
    };
    let buffer_2 = Buffers {
        array_one: [0i32; 4],
        array_two: [10; 4],
    };
    println!("{buffer_1:#?}, {buffer_2:#?}");
}
