use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

// the return type has a lifetime annotation
// which says the returned closure depends
// on SOME lifetime. And the compiler figures
// out that it must be s_ref, the only param.
// It's equivalent to:
// fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {
    move || s_ref.to_string()
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //--------------
    // closure with immutable reference
    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    
    // closure with mutable reference
    let mut list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // can't print here because would require an immutable borrow
    // and we currently have a mutable borrow open.
    borrows_mutably();
    println!("After calling closure: {:?}", list);
    
    // explicitly moving ownership to a closure
    let mut list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);
    // move is required because main() could end before
    // the thread does, invalidating a reference to `list`.
    thread::spawn( move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    //--------------------
    // Closures must name captured lifetimes.
    let s_own = String::from("Hello World");
    let cloner = make_a_cloner(&s_own);
    
    // // The lifetime annotation in the signature
    // // of make_a_cloner() tells Rust that
    // // the closure can't outlive s_ref. 
    // // So this drop() call won't compile.
    // drop(s_own);
    cloner();
    
    
}