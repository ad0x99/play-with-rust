pub fn prevent_dangling_reference() {
    let r;

    // Error
    // {
    //     let x = 5;
    //     r = &x // `x` does not live long enough borrowed value does not live long enough
    // }

    let x = 5;
    r = &x;

    println!("Value of r is {}", r);
}
