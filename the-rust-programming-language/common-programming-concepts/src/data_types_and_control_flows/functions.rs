pub fn print_func() {
    let x = five();
    println!("The value of x from five func is: {x}");
}

fn five() -> i32 {
    5
}

