use generics::{*};
use std::fmt::Display;

// this Point struct can have two different
// types, indicated by two different generics
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// X1, Y1 generics are declared after 'impl' 
// because they go with the struct impl
impl<X1, Y1> Point<X1, Y1> {
    // X2, Y2 generics declared after 'fn' because they're
    // specific to the function
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn displayable<T: Display>(t: T) -> T { t }

fn main() {
    // demonstrate use of mixup() method
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    // demonstrate use of traits
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you already know",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("1 new article available! {}", article.summarize());

    let blogpost = BlogPost {
        title: String::from("My life is boring"),
        body: String::from("That's all."),
        author: String::from("sadguy"),
    };
    println!("1 new blog post: {}", blogpost.summarize());

    // demonstrate functions that take a trait param
    notify(&blogpost);
    notify_any_two_types(&blogpost, &article);
    notify_two_same_type(&article, &article);

    // demonstrate that we can only call push_str() on s2
    // because the function returns type T.
    // If it only returned "impl Display" then
    // the compiler cannot tell that the returned type
    // has the push_str() methods implemented.
    let s = String::from("hello");
    let mut s2 = displayable(s);
    s2.push_str(" world");
    println!("{s2}"); 
}