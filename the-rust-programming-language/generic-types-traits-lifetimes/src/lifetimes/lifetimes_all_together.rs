use std::fmt::Display;

pub fn find_longest_string_and_announce() {
    let x = "Hello";
    let y = "World";
    let ann = "Announcement! The longest string is";

    longest_with_an_announcement(x, y, ann);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Find the longest string between {x} and {y}");
    if x.len() > y.len() {
        println!("{ann} {x}");
        x
    } else {
        println!("{ann} {y}");
        y
    }
}
