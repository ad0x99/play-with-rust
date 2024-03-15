pub fn ownership_of_push_str() {
    let mut s1 = String::from("foo");
    let s2 = "bar";

    // The `push_str` method takes a string slice because we don’t necessarily want to take ownership of the parameter.
    s1.push_str(s2);

    // If the `push_str` method took ownership of `s2`, we wouldn’t be able to print its value on the this line.
    println!("s2 is {s2}");
}
