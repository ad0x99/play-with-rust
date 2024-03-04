pub fn methods_produce_other_iterators() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // This code does not do anything
    // v1.iter().map(|x| x + 1);

    // collect() method consumes the iterator and collects the resultant values into a collection data type.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("Collected values: {:?}", v2);
    assert_eq!(v2, vec![2, 3, 4]);
}
