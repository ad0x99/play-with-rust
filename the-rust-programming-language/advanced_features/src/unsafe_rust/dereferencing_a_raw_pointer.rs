pub fn dereferencing_raw_pointer() {
    let mut num = 5;

    let raw_pointer_1 = &num as *const i32;
    let raw_pointer_2 = &mut num as *mut i32;

    unsafe {
        println!("Raw pointer 1 is: {}", *raw_pointer_1);
        println!("Raw pointer 2 is: {}", *raw_pointer_2);
    }

    // Uncomment the following line to see the error
    // let immutable_pointer_1 = &num;
    // let immutable_pointer_2 = &mut num;

    // println!("Immutable pointer 1 is: {}", *immutable_pointer_1);
    // println!("Immutable pointer 1 is: {}", *immutable_pointer_2);
}
