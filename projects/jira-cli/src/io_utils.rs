use std::io;

// Gets user input from the command line
pub fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

// Waits for the user to press a key
pub fn wait_for_key_press() {
    io::stdin().read_line(&mut String::new()).unwrap();
}
