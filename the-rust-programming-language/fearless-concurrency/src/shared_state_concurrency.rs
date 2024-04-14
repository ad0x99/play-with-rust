use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_shared_state() {
    single_thread_mutex();
    multi_thread_mutex();
}

fn single_thread_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn multi_thread_mutex() {
    // Won't work because of ownership rules
    // let counter = Mutex::new(0);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Newly added when using Arc<T>
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
