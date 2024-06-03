static mut NUMBER: u32 = 0;

pub unsafe fn test_static_mut() {
    let mut join_handle_vec = vec![];
    for _ in 0..1000 {
        join_handle_vec.push(std::thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    NUMBER += 1;
                }
            }
        }));
    }
    for handle in join_handle_vec {
        handle.join().unwrap();
    }
    unsafe {
        println!("{NUMBER}");
    }
}