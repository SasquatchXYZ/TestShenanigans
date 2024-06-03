use rand::{random, Rng, thread_rng};

pub fn test_rand() {
    for _ in 0..5 {
        let random_u16 = random::<u16>();
        println!("{random_u16}");
    }

    let mut number_maker = thread_rng();
    for _ in 0..5 {
        print!("{} ", number_maker.gen_range(1..11));
    }
}