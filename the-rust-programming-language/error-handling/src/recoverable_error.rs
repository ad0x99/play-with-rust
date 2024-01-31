use std::fs::File;
use std::io::ErrorKind;

pub fn run_unrecoverable_error() {
    run_open_file_error();
    open_file_error_handling();
    expect_for_handling_panic();
    unwrap_for_handling_panic();
}


fn run_open_file_error() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
}

fn open_file_error_handling() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!(
                    "Problem creating the file: {:?}",
                    e
                ),
            }
        }
        other_error => {
            panic!(
                "Problem opening the file: {:?}",
                other_error
            );
        }
    });

    println!("Greeting file processing result: {:?}", greeting_file)
}

fn unwrap_for_handling_panic() {
    // The `unwrap` method is a shortcut method implemented just like the `match` expression.
    // If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`.
    // If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us.
    let greeting_file = File::open("hello.txt").unwrap();
}

fn expect_for_handling_panic() {
    // Similarly, the `expect` method lets us also choose the `panic!` error message.
    // Using `expect` instead of `unwrap` and providing good error messages can convey your intent
    // and make tracking down the source of a panic easier.
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}


