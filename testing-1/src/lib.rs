#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn this_works_too() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    
    // #[test]
    // fn fail_me() {
    //     panic!("Make this fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle { 
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle { 
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // -----------------
    // demonstrate how #[should_panic]
    // makes a test expect a panic.
    pub struct Guess {
        value: i32,
    }
    
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
            
            Guess { value }
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        // we can specify a substring of
        // the error message we expect too
        #[test]
        #[should_panic(expected = "less than or equal to 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
    // -----------------

    // this tests an internal function.
    // works fine!  
    #[test]
    fn internal_test() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
