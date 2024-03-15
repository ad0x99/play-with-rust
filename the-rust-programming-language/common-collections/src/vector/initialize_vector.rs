pub fn initialize_a_new_vector() {
    // Initialize a new vector with type annotation i32
    // Because we aren’t inserting any values into this vector,
    // Rust doesn’t know what kind of elements we intend to store.
    let empty_vector: Vec<i32> = Vec::new();
    println!("empty_vector: {empty_vector:?}");

    // `vec!` macro will create a new vector that holds the values you give it.
    let vector_with_macro = vec![1, 2, 3];
    println!("vector_with_macro: {vector_with_macro:?}");
}
