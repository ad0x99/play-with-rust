use crate::struct_method_multiple_impl::run_method_multiple_impl;
use crate::struct_method_multiple_parameters::run_associated_function;
use crate::struct_method_multiple_parameters::run_method_multiple_parameters;

pub fn run_struct_methods() {
    println!("==========Struct - One Method=========");
    print_one_method();

    println!("==========Struct - Multiple Method Params=========");
    run_method_multiple_parameters();
    run_associated_function();

    println!("==========Struct - Multiple Impl Block=========");
    run_method_multiple_impl();
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // The `&self`` is actually short for `self: &Self``.
    // Within an `impl`` block, the type `Self`` is an alias for the type that the impl block is for.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn print_one_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )
}
