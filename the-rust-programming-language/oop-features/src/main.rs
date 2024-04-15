use blog::Post;
use blog_encoding_state::Post as EncodingPost;
use gui::Draw;
use gui::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    run_gui();
    run_blog();
    run_blog_encoding_state();
}

fn run_gui() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // This won't work because the trait Draw is not implemented for String
            // Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}

fn run_blog() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("{}", post.content());
}

fn run_blog_encoding_state() {
    let mut post = EncodingPost::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

    println!("{}", post.content());
}
