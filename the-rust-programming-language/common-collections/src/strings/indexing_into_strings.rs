pub fn indexing_into_strings() {
    // Rust doesn't support indexing
    let s1 = String::from("hello");
    // This code will result an error
    // let h = s1[0];

    let hello = String::from("Здравствуйте");
    // This will result 24 bytes it takes to decode `Здравствуйте` in UTF-8
    println!("String length: {}", hello.len());
}

pub fn slicing_strings() {
    let hello = "Здравствуйте";

    // Here, `s` will be a `&str` that contains the first 4 bytes of the string.
    // Earlier, we mentioned that each of these characters was 2 bytes, which means `s` will be `Зд`.
    let sliced_string = &hello[0..4];
    println!("Sliced string: {}", sliced_string);

    // This will result an error
    // byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
    // let error = &hello[0..1];
}
