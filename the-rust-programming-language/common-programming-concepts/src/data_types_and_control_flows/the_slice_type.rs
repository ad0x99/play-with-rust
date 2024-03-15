pub fn run_slice_type() {
    run_first_word_error();
    signature_slice();
    other_slices()
}

fn run_first_word_error() {
    let mut s = String::from("hello world");

    let word = first_word_error(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("word: {word}");

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference.
    let word = first_word_with_slice(&s);

    // Because `clear` needs to truncate the String,
    // it needs to get a mutable reference
    // s.clear(); // Error

    // The `println!` after the call to clear uses the reference in word, so the immutable reference must still be active at that point. Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.
    println!("word: {word}");
}

fn first_word_error(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn signature_slice() {
    let my_string = String::from("hello world");

    // `first_word_with_slice` works on slices of `String`s, whether partial or whole.
    let word_0_6 = first_word_with_slice(&my_string[0..6]);
    let word_whole = first_word_with_slice(&my_string[..]);
    // `first_word_with_slice` also works on ownership_and_references to `String`s, which
    // are equivalent to whole slices of `String`s.
    let word_reference = first_word_with_slice(&my_string);

    println!("word_0_6: {word_0_6}");
    println!("word_whole: {word_whole}");
    println!("word_reference: {word_reference}");

    let my_string_literal = "hello world";

    // `first_word_with_slice` works on slices of string literals,
    // whether partial or whole.
    let word_string_literal_0_6 = first_word_with_slice(&my_string_literal[0..6]);
    let word_string_literal_whole = first_word_with_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word_string_slice = first_word_with_slice(my_string_literal);

    println!("word_string_literal_0_6: {word_string_literal_0_6}");
    println!("word_string_literal_whole: {word_string_literal_whole}");
    println!("word_string_slice: {word_string_slice}");
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
