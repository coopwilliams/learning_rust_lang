fn main() {
    // String type value from string literal.
    // This is allocated on the heap.
    // The type can be mutable.
    let mut s = String::from("greetings dudes");

    // string append method
    s.push_str(", world");
    println!("{}", s);

    // variables and scope
    let x = s;
    println!("x = {}", x);
    // s is now out of scope. Its string on the heap is moved to x ptr.

    let y = 5;
    let z = y;
    println!("y = {}", y);
    let y = 4;
    println!("y = {}, z = {}", y, z);
    // both variables remain in scope because integers have known size
    // at compile time. 

    takes_ownership(x);        // x moves into the function
                               // ... and is out of scope here
    let x = 5;

    makes_copy(x);             // i32 is Copy, so still usable afterwards

    let mut s = String::from("another string");
    
    let r1 = &s; //immutable reference
    let r2 = &s; // and another
    println!("{} and {}", r1, r2);
    // the above two variables are not used after this point

    let r3 = &mut s; // mutable reference?
    println!("{}", r3);

    let reference_to_nothing = dangle();
    println!("{}", first_word(&s));

    let hello = &s[0..5]; // string slice
    let world = &s[6..s.len()]; // just like in python!
    println!("The words are {} and {}!", hello, world);

}


fn dangle() -> String {
    let s = String::from("hello my darlin");

    s
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
