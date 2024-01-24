pub fn run_vector() {
    println!("=============initialize_a_new_vector============");
    initialize_a_new_vector();

    println!("=============update_value_to_vector============");
    update_value_to_vector();

    println!("=============read_elements_of_vector============");
    read_elements_of_vector();

    println!("=============ownership_and_borrowing_with_vector============");
    ownership_and_borrowing_with_vector();

    println!("=============iterate_through_vector============");
    iterate_through_vector();

    println!("=============store_multiple_types_with_enum============");
    store_multiple_types_with_enum();

    println!("=============drop_vector_element============");
    drop_vector_element()
}

fn initialize_a_new_vector() {
    // Initialize a new vector with type annotation i32
    // Because we aren’t inserting any values into this vector,
    // Rust doesn’t know what kind of elements we intend to store.
    let empty_vector: Vec<i32> = Vec::new();
    println!("empty_vector: {empty_vector:?}");

    // `vec!` macro will create a new vector that holds the values you give it.
    let vector_with_macro = vec![1, 2, 3];
    println!("vector_with_macro: {vector_with_macro:?}");
}

fn update_value_to_vector() {
    // Updating a Vector with `push`` method
    let mut updated_vector = Vec::new();

    updated_vector.push(5);
    updated_vector.push(6);
    updated_vector.push(7);
    updated_vector.push(8);

    println!("updated_vector: {updated_vector:?}");
}

fn read_elements_of_vector() {
    let v = vec![1, 2, 3, 4, 5];

    // Read vector's elements with indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Read vector's elements with get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Different between these 2 methods
    let v = vec![1, 2, 3, 4, 5];

    // This will panic your program
    // let does_not_exist_with_indexing = &v[100];
    // println!("The element does not exist: {does_not_exist_with_indexing}");

    // This will not panic your program and return None if there is no found element
    let does_not_exist_with_get = v.get(100);
    println!("The element does not exist: {does_not_exist_with_get:?}");
}

fn ownership_and_borrowing_with_vector() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // This won't work, because we hold an immutable reference to the first element in a vector and try to add an element to the end.
    // v.push(6);

    println!("The first element is: {first}");
}

fn iterate_through_vector() {
    // `for` loop to get immutable references to each element in the vector
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    // Iterate over mutable references to each element in a mutable vector
    let mut m_v = vec![100, 32, 57];

    for i in &mut m_v {
        *i += 50;
    }

    println!("The m_v's element is: {m_v:?}");
}

fn store_multiple_types_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("The row's element is: {row:?}");
}

fn drop_vector_element() {
    fn in_scope() {
        let v = vec![1, 2, 3, 4];
        println!("The v's elements are: {v:?}");
    }

    in_scope();
    // v goes out of scope and is freed here
    // println!("The v's elements are: {v:?}");
}
