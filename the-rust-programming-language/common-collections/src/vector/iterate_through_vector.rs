pub fn iterate_through_vector() {
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
