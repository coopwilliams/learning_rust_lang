use std::ops::Add;

// // Here is the definition of the Add trait.
// // Note how the default is to add two
// // things of the same type.
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// We can overload the `+` operator by implementing
// the trait associated with the operator.
impl Add for Point {
    // The associated type "Output" is needed
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


// We can implement Add to Add things 
// of different types.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

// We have to set the value of the `Rhs` type parameter
// to add differing types.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Returning to the Point struct, we can even
// specify a different implementation in addition
// to the existing implementation, and even with a
// different output type.
impl Add<i32> for Point {
    type Output = i32;

    fn add(self, other: i32) -> i32 {
            self.x + other + self.y + other
    }
}

//
//
// Calling Methods with the same named
//

// Here we show a Human trait with three
// different fly() methods defined on it.
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your cap'n speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waves arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

//
//
// Supertraits
//

use std::fmt;

// Here we define a trait that depends on another trait.
// It's necessary that we specify fmt::Display as a dependency.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}


//
//
// Newtype pattern to implement external traits on external types
//

// the orphan rule that states we’re only allowed to implement 
// a trait on a type if either the trait or the type are local 
// to our crate. It’s possible to get around this restriction 
// using the newtype pattern, which involves creating 
// a new type in a tuple struct.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x:1, y: 0 } + Point{ x: 2, y: 3 },
        Point { x: 3, y: 3},
    );
    
    assert_eq!(
        Point { x:1, y: 0 } + 2,
        5,
    );
    
    assert_eq!(
        Millimeters(1000) + Meters(1),
        Millimeters(2000),
    );

    // Calling a method defined multiple times on a type
    // causes the compiler to call the one that is
    // directly implemented on the type.
    let person = Human;
    person.fly();

    // To call the fly() methods from the Human traits,
    // we need to use explicit syntax.
    Pilot::fly(&person);
    Wizard::fly(&person);

    // To use the Dog-specific implementation of baby_name()
    // we need to use fully-qualified syntax:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Demo using a supertrait method on Point
    let demo_point = Point { x: 3, y: 3};
    demo_point.outline_print();

    // Demo using a Newtype called Wrapper.
    // Remember that Wrapper doesn't have the
    // methods of what it's holding.
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
