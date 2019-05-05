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
    notify1(t1);
}

// passing a trait as a parameter
pub fn notify1(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
pub fn notify2<T: Summary>(item: T) {
    println!("breaking news! {}", item.summarize());
}

// multiple trait parameters
pub fn notify3(item: impl Summary + std::fmt::Display) {
    println!("breaking news! {}", item.summarize());
}

// specify multiple trait bounds with '+' syntax
pub fn notify4<T: Summary + std::fmt::Display>(item: T) {
    println!("breaking news! {}", item.summarize());
}

// clearer trait bounds with where clauses
fn some_fn1<T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug>(t: T, u: U){
    // fn body
}

fn some_fn2<T, U>(t: T, u: U)
    where T: std::fmt::Display + Clone,
          U: Clone + std::fmt::Debug
{
    // fn body
}

// returning types that implement traits
// this is only allowed if the function will only be returning one impl Trait
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you proabaly already know, people"),
        reply: false,
        retweet: false,
    }
}
