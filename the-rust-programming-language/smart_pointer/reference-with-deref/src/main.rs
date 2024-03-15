use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    regular_reference();
    box_t_like_reference();
    custom_smart_pointer_with_mybox();
    deref_coercions_with_fn_and_method()
}

fn regular_reference() {
    let x = 5;
    let y = &x;

    println!("The value of x = {x}");
    println!("The value of y = {y}");

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn box_t_like_reference() {
    let x = 5;
    let y = Box::new(x);

    println!("The value of x = {x}");
    println!("The value of y = {y}");

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn custom_smart_pointer_with_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    println!("The value of x = {x}");
    println!("The value of y = {y:?}");

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_coercions_with_fn_and_method() {
    let m = MyBox::new(String::from("Rust"));
    // With deref coercions supported
    hello(&m);

    // Without deref coercions supported
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
