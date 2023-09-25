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

// the returned reference will be valid
// as long as both of the parameters are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// for demonstrating structs holding a ref
struct ImportantExcerpt<'a> {
    part: &'a str, // borrowed str slice w/ lifetime annotation
}

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

    // basic example of calling a function that uses lifetimes
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longer string is {}", result);

    // using longest() with refs to String values
    // with different concrete lifetimes
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // -----------
    // example showing the result lifetime (output lifetime) 
    // must be the smaller lifetime of the two args.
    let string1 = String::from("long string is long");
    let result;
    
    let mut string2 = String::from("xyz");
    // making the above mutable lets us test the last case
    // in the scope below.
    {
        // // if this is defined in the scope,
        // // it falls out at the end of the scope.
        // // Thus, the code would fail to compile
        // // because the shorter-lived of the two
        // // params doesn't live long enough.
        // let string2 = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str());
        
        // // however! This works.
        // // The borrow falls out of scope but the underlying
        // // string is still in scope above.
        // let string2_borrow = &string2;
        // result = longest(string1.as_str(), string2_borrow.as_str());
        
        // this also works.
        let mut string2_borrow = &mut string2;
        result = longest(string1.as_str(), string2_borrow.as_str());
    }
    // notice how this happens after the scope
    println!("The longest string is {}", result);
    // -----------

    // demonstrate using a struct that holds a ref
    let novel = String::from("call me ishmael. blah blah");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// generic type params,
// trait bounds, 
// and lifetimes...
// all in one function!
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}