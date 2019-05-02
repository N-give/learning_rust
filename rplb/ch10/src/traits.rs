// each type implementing this trait must provide its own custom behavior
// the compiler will enforce this
pub trait Summary {
    fn summarize(&self) -> String;
    // can also implement a default behavior
    fn summarize_default(&self) -> String {
        format!("{} (Read more...)", self.summarize())
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
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let t1 = Tweet {
        username: String::from("ECE_Shop"),
        content: String::from("Welcome to the ECE shop"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", t1.summarize_default());
    notify(t1);
}

// passing a trait as a parameter
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
