pub fn update_string() {
    let mut s = String::from("foo");
    println!("String `s` before updated: {s}");

    // push_str accepts a string
    s.push_str("bar");
    println!("String `s` after updated with push_str: {s}");

    // push only accepts a character
    s.push('!');
    println!("String `s` after updated with push: {s}");
}
