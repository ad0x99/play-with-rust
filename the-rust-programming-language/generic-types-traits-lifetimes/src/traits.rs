mod aggregator_crate;

use crate::traits::aggregator_crate::{NewsArticle, Summary, Tweet};

pub fn run_traits() {
    // Default implementations of traits
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.call_summarize_with_author());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.default_summarize());
    println!(
        "New article available! {}",
        article.call_summarize_with_author()
    );

    // Traits in parameters
    println!("================Traits in Parameters================");
    aggregator_crate::notify(&article);
    aggregator_crate::notify_trait_bound(&tweet);
    aggregator_crate::notify_same_trait(&tweet, &article);
    aggregator_crate::notify_force_same_type(&tweet, &tweet);

    // Return types that implement traits
    println!("================Return types that implement traits================");
    aggregator_crate::returns_summarizable().call_summarize_with_author();
}
