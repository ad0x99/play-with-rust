#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn run_generic_in_struct() {
    generic_in_struct_with_the_same_type();
    generic_in_struct_with_different_type();
}

fn generic_in_struct_with_the_same_type() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // This won't work due to different type
    // let both = Point { x: 5, y: 4.0 };

    println!("The value of integer are {:?}", integer);
    println!("The value of integer are {:?}", float);
}

#[derive(Debug)]
struct DifferentPoint<T, U> {
    x: T,
    y: U,
}

fn generic_in_struct_with_different_type () {
    let integer_and_float = DifferentPoint { x: 5, y: 4.0 };
    println!("The value of both integer and float are {:?}", integer_and_float);
}