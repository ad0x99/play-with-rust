#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// The lifetime parameter declaration after `impl` and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both `&self` and `announcement` their own lifetimes.
    // Then, because one of the parameters is `&self``, the return type gets the lifetime of `&self`, and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

pub fn lifetimes_annotation_in_method_definitions() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("The level value is {:?}", i.level());
    println!("{:?}", i.announce_and_return_part(first_sentence))
}
