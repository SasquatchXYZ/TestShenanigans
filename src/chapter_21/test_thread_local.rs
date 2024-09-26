use std::cell::RefCell;
use std::sync::Mutex;

//  Accessible to all threads.  We have to wrap it in a thread-safe Mutex or RwLock
lazy_static::lazy_static! {
    static ref INITIAL_VALUE: Mutex<i32> = Mutex::new(10);
}

// Static that is local to each thread - no need for a Mutex - a regular RefCell or Cell works just fine
thread_local! {
    static LOCAL_INITIAL_VALUE: RefCell<i32> = RefCell::new(10);
}

// Run with `cargo test -- --nocapture` so you can see the output
#[test]
fn one() {
    let mut lock = INITIAL_VALUE.lock().unwrap();
    println!("Test 1. Global value is {lock}");
    *lock += 1;
    println!("Test 1. Global value is now {lock}");

    LOCAL_INITIAL_VALUE.with(|cell| {
        let mut lock = cell.borrow_mut();
        println!("Test 1, Local value is {lock:?}");
        *lock += 1;
        println!("Test 1. Local value is now {lock:?}\n");
    });
}
#[test]
fn two() {
    let mut lock = INITIAL_VALUE.lock().unwrap();
    println!("Test 2. Global value is {lock}");
    *lock += 1;
    println!("Test 2. Global value is now {lock}");

    LOCAL_INITIAL_VALUE.with(|cell| {
        let mut lock = cell.borrow_mut();
        println!("Test 2, Local value is {lock:?}");
        *lock += 1;
        println!("Test 2. Local value is now {lock:?}\n");
    });
}
#[test]
fn three() {
    let mut lock = INITIAL_VALUE.lock().unwrap();
    println!("Test 3. Global value is {lock}");
    *lock += 1;
    println!("Test 3. Global value is now {lock}");

    LOCAL_INITIAL_VALUE.with(|cell| {
        let mut lock = cell.borrow_mut();
        println!("Test 3, Local value is {lock:?}");
        *lock += 1;
        println!("Test 3. Local value is now {lock:?}\n");
    });
}