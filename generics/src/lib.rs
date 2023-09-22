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

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}