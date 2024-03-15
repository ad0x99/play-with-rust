use crate::structs::MyBox;

pub fn custom_smart_pointer_with_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    println!("The value of x = {x}");
    println!("The value of y = {y:?}");

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
