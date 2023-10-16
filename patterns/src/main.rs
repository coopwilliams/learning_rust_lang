fn main() {
    // Demo how `if let` differs from `match`.
    // Where `match` only allows one pattern comparison, `if let`
    // allows multiple comparisons using `else if let`.

    let favorite_color: Option<&str> = None;
    // let favorite_color: Option<&str> = Some("turquoise");
    // let is_tuesday = true;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // we can't check the shadowed variable `age` on
        // the same line where we check `age > 30`. `age`
        // can't be evaluated until we enter the new scope.
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    }
    // // Note: the compiler won't check `if let` expressions
    // // for exhaustiveness. So this missing `else` block can
    // // cause logic bugs.
    // else {
    //     println!("Using blue as the background color");
    // }


    // Demo `while let` pattern matching
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // In a for loop, the value directly following `for` is a pattern.
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Every usage of `let` involves a pattern! e.g.:
    //  let PATTERN = EXPRESSION
    let (x, y, z) = (1, 2, 3);

    // Function parameters can also be patterns.
    // Note how we capture x and y from a single parameter
    // passed to the function.
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // A pattern that can fail to match for some possible value
    // is "refutable".

    // Function parameters, `let` statements, and `for` loops
    // can only accept irrefutable patterns.

    // Here, stack.pop() returns an Option, which is expected by
    // the Some(x) pattern.
    let x = if let Some(x) = stack.pop() {x} else {1};
    
    // // This code does not compile. Even though we provided an 
    // // `else` block in expectation that the Some(x) pattern
    // // will not match, the code will not compile due to the
    // // `Some(x)`` pattern not matching `&point`.
    // let x = if let Some(a) = &point {a} else {&None};
  
}