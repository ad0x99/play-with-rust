pub fn drop_vector_element() {
    fn in_scope() {
        let v = vec![1, 2, 3, 4];
        println!("The v's elements are: {v:?}");
    }

    in_scope();
    // v goes out of scope and is freed here
    // println!("The v's elements are: {v:?}");
}
