pub trait Summary {
    fn summarize(&self) -> String;

    fn default_summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;

    fn call_summarize_with_author(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as parameters
// impl Trait example
pub fn notify(items: &impl Summary) {
    println!("Breaking news! {}", items.call_summarize_with_author());
}

// Trait bound example
pub fn notify_trait_bound<T: Summary>(items: &T) {
    println!("Breaking news! {}", items.call_summarize_with_author());
}

// If we have 2 parameters that implement the same trait, we can use the impl Trait syntax.
// But if we want to force both parameters to have the same type, we must use a trait bound.
pub fn notify_same_trait(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "Breaking news the same trait! {}",
        item1.call_summarize_with_author()
    );
    println!(
        "Breaking news the same trait! {}",
        item2.call_summarize_with_author()
    );
}

pub fn notify_force_same_type<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking news the same type! {}",
        item1.call_summarize_with_author()
    );
    println!(
        "Breaking news the same type! {}",
        item2.call_summarize_with_author()
    );
}

// Returning types that implement traits
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
