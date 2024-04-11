use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run_channels() {
    println!("============Create a channel============");
    create_a_channel();

    println!("============Channel and Ownership============");
    channel_and_ownership();

    println!("============Sending Multiple Data In Channel============");
    sending_multiple_values();

    println!("============Creating Multiple Producers============");
    create_multiple_producers();
}

fn create_a_channel() {
    // Initialize a channel
    let (tx, rx) = mpsc::channel();

    // Move the transmitting end into a spawned thread and send a string message to the main thread
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap()
    });

    // Get the message from the main thread
    let received_message = rx.recv().unwrap();
    println!("Got: {}", received_message) // "Got: Hi"
}

fn channel_and_ownership() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");

        tx.send(val).unwrap();
        // The send() function takes ownership of its parameter
        // When the value is moved the receiver takes ownership of it
        // Therefore, the val will be invalid here because the val is moved to the main thread for usage of the receiver
        // println!("Value of val is: {val}")
    });

    let received_message = rx.recv().unwrap();
    println!("Received message: {:?}", received_message)
}

fn sending_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    for received_message in rx {
        println!("Received message: {}", received_message)
    }
}

fn create_multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    for received_message in rx {
        println!("Got: {}", received_message)
    }
}
