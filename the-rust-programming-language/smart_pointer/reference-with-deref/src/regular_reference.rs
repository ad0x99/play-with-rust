pub fn regular_reference() {
    let x = 5;
    let y = &x;

    println!("The value of x = {x}");
    println!("The value of y = {y}");

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
