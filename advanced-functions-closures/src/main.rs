
// function pointers implement all three
// of the closure traits (Fn, FnMut, and FnOnce).
fn add_one(x: i32) -> i32 {
    x + 1
}

// Here we have a function pointer that 
// gets passed to another function.
// Since fn is a type rather than a trait,
// we specify it as a param type directly
// rather than declaring a generic type parameter.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    // demo passing a function to a function
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // -----------
    // We can do this simple conversion two ways.
    let list_of_numbers = vec![1, 2, 3];
    
    // this way uses a closure.
    let list_of_strings_from_closure: Vec<String> =
    list_of_numbers.iter().map(|i| i.to_string()).collect();
    
    // If we use a function, we have to use fully-qualified
    // syntax because there are multiple to_string() functions.
    let list_of_strings_from_function: Vec<String> =
    list_of_numbers.iter().map(ToString::to_string).collect();
    
    enum Status {
        Value(u32),
        Stop,
    }

    // Recall that each enum variant becomes an initializer function.
    // So we can use those as function pointers that implement
    // closure traits, and pass them to functions that take closures.
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    
    
    // -----------
    // Function pointers cannot be returned from a function.
    // Closures can be returned. But since they're not Sized,
    // the return type must be a trait object.

    // Can't do `fn returns_closure() -> dyn Fn(i32) -> i32 {`
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }


}
