pub fn ownership_and_borrowing_with_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // This won't work, because we hold an immutable reference to the first element in a vector and try to add an element to the end.
    // v.push(6);

    println!("The first element is: {first}");
}
