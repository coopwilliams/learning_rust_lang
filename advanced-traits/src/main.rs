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
    )
}
