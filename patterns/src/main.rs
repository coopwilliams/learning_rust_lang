fn main() {
    // -- Demo how `if let` differs from `match`. --------------
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


    // -- Demo `while let` pattern matching --------------
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // -- In a for loop, the value directly following `for` is a pattern. --------------
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // -- Every usage of `let` involves a pattern! e.g.: --------------
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

    // -- A pattern that can fail to match for some possible value --------------
    // is "refutable".

    // Function parameters, `let` statements, and `for` loops
    // can only accept irrefutable patterns.

    // -- Here, stack.pop() returns an Option, which is expected by --------------
    // the Some(x) pattern.
    let x = if let Some(x) = stack.pop() {x} else {1};
    
    // // -- This code does not compile. Even though we provided an  --------------
    // // `else` block in expectation that the Some(x) pattern
    // // will not match, the code will not compile due to the
    // // `Some(x)`` pattern not matching `&point`.
    // let x = if let Some(a) = &point {a} else {&None};
  
    // -- demo shadowing inside the scope of a `match` expression. --------------
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // note how the `y` inside the Some() pattern is 
        // a shadowed variable, which replaces the original `y`
        // until the scope is ended.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
    // if we had wanted to compare the outer x and y
    // inside the match scope, we would need to use a
    // match guard conditional instead. To cover later.


    // -- Here we show multiple patterns used in the same match arm. --------------
    let x = 4;
    match x {
        1 | 2 | 4 => println!("one or two or four"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // -- Here we show how to match ranges of values. --------------
    // Note that ranges are only allowed with `char` or numeric values,
    // because those are the only types of ranges that Rust can check
    // for emptiness.
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // -- Demo destructuring structs using patterns. --------------
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };

    // Here we capture `a` and `b` from `p`.
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // We can also simply get the original fields by name.
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // We can also test some fields for particular values
    // while destructuring other fields.
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // -- Demo destructuring enums. --------------
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // -- Demo matching on nested items -------------
    // We've refactored the above to support colors in 
    // the ChangeColor message.
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    enum MessageTypeTwo {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    
    let msg = MessageTypeTwo::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        MessageTypeTwo::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        MessageTypeTwo::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // We can do destructuring in more complex ways.
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // We can use `..` to use specific parts and not others.
    // Note that your choice of what to skip must be unambiguous.
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }


    // -- Match guards are additional `if` conditions that -----------
    // come after the pattern in a match arm.
    let num = Some(4);

    match num {
        // This arithmetic test can't otherwise be expressed within
        // a pattern. The downside is that the compiler doesn't
        // attempt to check for exhaustiveness when match guard
        // expressions are involved.
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        // Remember that the match guard applies to
        // anything that comes through the pattern.
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // -- Demo using @ ---------------
    // The @ symbol is used when we're trying to capture a variable
    // while also testing it for a pattern match.
    enum MessageTypeThree {
        Hello { id: i32 },
    }

    let msg = MessageTypeThree::Hello { id: 5 };

    match msg {
        // Here, we capture in interior value in `id_variable`
        // while testing it against a range.
        MessageTypeThree::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        
        // Capture nothing, so we can't use the interior value.
        MessageTypeThree::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }

        // Capture without testing the interior value.
        MessageTypeThree::Hello { id } => println!("Found some other id: {}", id),
    }

}