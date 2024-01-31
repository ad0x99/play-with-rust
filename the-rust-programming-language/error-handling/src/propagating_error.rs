use std::fs::File;
use std::io;
use std::io::Read;
use std::error::Error;

pub fn run_propagating_error() -> Result<(), Box<dyn Error>> {
    // read_username_from_file();
    // read_username_from_file_with_question_mark_operator();
    main()
}

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_with_question_mark_operator() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // The `?` placed after a `Result` value is defined to work in almost the same way as the `match` expressions.
    // If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression,
    // and the program will continue.
    // If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword
    // so the error value gets propagated to the calling code.
    username_file.read_to_string(&mut username)?;
    Ok(username)
}