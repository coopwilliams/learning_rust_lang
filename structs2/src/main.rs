fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "rect1 is {:#?}", rect1
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    check_auto_dereferencing();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function is not a method because it doesn't take &self 
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn check_auto_dereferencing() {
    let r = &mut Box::new(Rectangle { 
        width: 1,
        height: 2
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    let nested_again = &r;
    // Rust dereferences pointers as many times as it takes.
    let area3 = nested_again.area(); 
    assert_eq!(area1, area2);
    assert_eq!(area1, area3);

}
