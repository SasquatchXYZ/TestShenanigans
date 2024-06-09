use std::mem::transmute;

struct User {
    name: String,
    number: u32,
}

pub fn test_transmute() {
    // let x = -19;
    // let y = unsafe { transmute::<i32, u32>(x) };
    // println!("{y}");
    //
    // println!("{:b}\n{:b}", -19, 4294967277u32);
    //
    // println!("{}", std::mem::size_of::<User>());

    let some_i32s = [1, 2, 3, 4, 5, 6, 7, 8];
    let user = unsafe { transmute::<[i32; 8], User>(some_i32s) };
}