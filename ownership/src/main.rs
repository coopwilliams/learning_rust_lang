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

    let _reference_to_nothing = dangle();

    // access original string when slice is freed?
    let original_string = String::from("Hello my darlin");
    let slice = first_word(&original_string);
    println!("{}", slice);
    println!("{}", original_string);

    let hello = &s[0..5]; // string slice
    let world = &s[6..s.len()]; // just like in python!
    println!("The words are {} and {}!", hello, world);

    let mut lowercased: Vec<char> = vec!['a', 'b', 'c'];
    let mut uppercased: Vec<char> = vec!['A', 'b', 'c'];
    ascii_capitalize(&mut lowercased);
    ascii_capitalize(&mut uppercased);
    // how many times can we dereference the same data?
    let mut first_char = &mut lowercased[0];
    println!("{}", *first_char);
    // first_char = &mut char!['B']; // this doesn't work for many reasons.
    *first_char = 'B';
    println!("{}", *first_char);
    println!("{:?}", lowercased);

    // immutable stuff can be moved into a mutable var
    let immutable_me = vec![1,2,3];
    let mut mutable_you = immutable_me;
    println!("mutable now: {:?}", mutable_you);

    drop_example();
    tuple_modifying_example();
    mutating_different_elements_example();
    prevent_double_free();

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

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        v[0] = c.to_ascii_uppercase();
        // let up = c.to_ascii_uppercase();
        // v[0] = up;
        println!("Newly capitalized: {:?}", v);
    } else {
        println!("Already capitalized: {:?}", v);
    }
}


fn drop_example() {
    let s = String::from("This data gets dropped after it's no longer used");
    let s_ref = &s;
    println!("{}", s_ref);
    drop(s); // we can only drop after all refs are gone
    }

fn tuple_modifying_example() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
    }

fn mutating_different_elements_example() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0];
    let y = &a[1];
    // *x += *y; // this can't be done. Even though
                 // it's safe, Rust removes the permissions
    }

fn prevent_double_free() {
    let mut v = vec![1, 2, 3];
    let n = &v[0];
    println!("{n}");
    v.push(4); // this has to go after the println call
    }