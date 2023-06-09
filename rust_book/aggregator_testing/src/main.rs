use std::fmt::{Debug, Display};

pub trait Summary {
    fn summerize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summerize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summerize_author(&self) -> String {
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
    fn summerize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle{
            headline: String::from(
                "Penguins win the stanley Cup championship",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again \
                are the best hockey team in the NHL."
            ),
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            }
        }
    }
}

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburg, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again \
            are the best hockey team in the NHL"
        ),
    };

    println!("New article available! {}", article.summarize());
}
