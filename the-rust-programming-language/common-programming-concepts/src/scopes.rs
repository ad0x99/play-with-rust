pub fn print_scopes() {
    variable_and_data_interacting_with_move()
}

#[warn(dead_code)]
fn basic_of_scope() {
    let _s = String::from("hello"); // s is valid from this point forward
                                    // do stuff with s
                                    // this scope is now over, and s is no longer valid
}

fn variable_and_data_interacting_with_move() {
    // Bind the value of 5 to x
    // Then make a copy of value in x and bind it to y
    // This will work
    let x = 5;
    let y = x;

    println!("x: {x}");
    println!("y: {y}");

    // An example with String
    // Bind the string `hello` to s1
    // And then make a copy of value s1 and bind it to s2
    // But in this case, because we're not copying the heap memory where the string is stored
    // Rust automatically moves the value of s1 to s2 and frees the s1 scope
    let s1 = String::from("hello");
    let s2 = s1;

    // Therefore, in this println will be failed because the s1 is no longer valid
    // println!("{s1}, world!");
    // The value of s1 now is moved to s2
    println!("{s2}, world!");

    // If you want to deeply copy the heap data of String, not just the stack data
    // You can use `clone` method
    let s3 = String::from("hi");
    let s4 = s3.clone();
    println!("{s4}, world!")
}
