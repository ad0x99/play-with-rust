pub fn generic_lifetimes_in_fn() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let result = longest_without_dangling(string1.as_str(), string2);
    println!("{result}");
}

// Without lifetimes annotation, this will throw an error
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// With a lifetime annotation, this will work without any error
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest_with_dangling<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn longest_without_dangling(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result.to_string()
}
