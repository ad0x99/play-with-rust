pub fn return_values_and_scope() {
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

pub fn dont_take_ownership() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

pub fn dont_take_ownership_with_reference() {
    let s1 = String::from("hello");

    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it
    // Because it does not own it, the value it points to will not be dropped when the reference stops being used.
    let len = calculate_length_with_reference(&s1);

    println!("The length of '{s1}' is {len}.");
}

// The signature of the function uses `&` to indicate that the type of the parameter s is a reference.
fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, the String is not dropped.
