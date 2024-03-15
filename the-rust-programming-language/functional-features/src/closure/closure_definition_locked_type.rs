// The first time we call this function with the String value, the compiler infers the type of x and the return type of the closure to be String. Those types are then locked into the closure in run_closure_locked_type, and we get a type error when we next try to use a different type with the same closure.
pub fn run_closure_locked_type() {
    // We didn't add any type annotations to this definition
    let example_closure = |x| x;

    // This type is locked after the first run
    let s = example_closure(String::from("hello"));
    println!("The value of s is {s}")
    // let n = example_closure(5);
}
