fn never_type() -> ! {
    // The never type with the panic! advanced_macro
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }

    // The type ! is a loop
    print!("forever ");

    loop {
        print!("and ever ");
    }
}
