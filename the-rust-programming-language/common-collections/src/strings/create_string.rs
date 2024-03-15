pub fn create_new_string() {
    // Initialize a new empty string with `new` function
    let empty_string = String::new();
    println!("A new empty string: {empty_string}");

    // Add initial value to empty string with `to_string` method
    let data = "Initial string";
    let init_string = data.to_string();
    println!("A new string with `to_string` method: {init_string}");

    // Create a new string with initial value
    let string_with_initial_value = String::from("Initial string");
    println!("A initial value string with string literal: {string_with_initial_value}");
}
