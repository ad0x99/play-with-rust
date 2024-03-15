pub fn box_t_like_reference() {
    let x = 5;
    let y = Box::new(x);

    println!("The value of x = {x}");
    println!("The value of y = {y}");

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
