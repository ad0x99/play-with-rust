pub fn print_numeric_operations() {
    // addition
    let sum = 5 + 10;
    println!("addition - sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("subtraction - difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("multiplication - product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("division - quotient: {}", quotient);
    println!("division - truncated: {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);
}