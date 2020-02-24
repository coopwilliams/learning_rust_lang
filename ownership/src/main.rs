fn main() {
    // String type value from string literal.
    // This is allocated on the heap.
    // The type can be mutable.
    let mut s = String::from("hello");

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
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
