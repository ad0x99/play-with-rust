pub fn print_variable() {
    let x = 5;
    let mut y = 5;
    println!("The value of x = {x}");

    // Error - because variables in Rust are immutable
    // x = 6;
    // println!("The value of x = {x}");

    // With mut keyword - the y variable might be mutable
    // y = 10;
    println!("The value of y = {y}");
}
