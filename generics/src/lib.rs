use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
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
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // overriding the default behavior
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // overriding the default behavior
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct BlogPost {
    pub title: String,
    pub body: String,
    pub author: String,
}

// use default implementation
impl Summary for BlogPost {
    // we have to define this for summarize()
    // to use.
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// function with one parameter having a trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// function where both parameters implement trait
// and have the same type.
pub fn notify_two_same_type<T: Summary>(item1: &T, item2: &T) {
    println!("Two things of the same media type: {}, {}", item1.summarize(), item2.summarize());
}

// function where both parameters implement trait
// and can have any types
pub fn notify_any_two_types(item1: &impl Summary, item2: &impl Summary) {
    println!("Two things of any types: {}, {}", item1.summarize(), item2.summarize());
}

// function specifying two trait bounds
pub fn notify_and_display(item: &(impl Summary + Display)) {}

// function with a ton of trait bounds,
// formatted in a more readable way
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{0}

// function that puts a trait bound
// on the return type.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you already know",
        ),
        reply: false,
        retweet: false,
    }
}

// conditionally implemented methods
// using trait bounds
struct Pair<T> {
    x: T,
    y: T,
}

// new() is implemented for all Pairs
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}

// cmp_display() is implemented only if the inner type T
// has two specific traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}