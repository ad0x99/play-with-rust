use crate::structs::MyBox;

pub fn deref_coercions_with_fn_and_method() {
    let m = MyBox::new(String::from("Rust"));
    // With deref coercions supported
    hello(&m);

    // Without deref coercions supported
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
