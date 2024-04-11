use std::thread;
use std::time::Duration;

pub fn run_new_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1))
    }
}

pub fn wait_for_all_threads_to_finish_with_join_handles() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for the spawned threads to finish first and then run main thread.
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1))
    }

    // Block main thread to wait the spawned thread before continuing with the main thread
    // handle.join().unwrap();
}

pub fn move_closure_with_thread() {
    let v = vec![1, 2, 3];

    // Error
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // Fixed with move keyword before closure
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn invalid_reference() {
    let v = vec![1, 2, 3];

    // Error
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // Fixed with move keyword before closure
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Rust ownership rule disallowed us to use v because v's ownership has been moved to the spawned thread
    // Therefore, v is no longer valid here
    // drop(v); // oh no!
    handle.join().unwrap();
}
