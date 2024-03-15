pub fn read_elements_of_vector() {
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
