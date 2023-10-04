use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
    
    fn new_nested(x: T) -> MyBox<MyBox<T>> {
        MyBox(MyBox(x))
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    let z = MyBox::new_nested(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, **z); // two boxes -> deref twice

    // here, the compiler performs Deref Coercion.
    // deref() is repeatedly called, first on
    // the MyBox and then on the String, to
    // produce the &str the function expects.
    let m = MyBox::new(String::from("Rust"));
    // without deref coercion, this would have to be:
    // hello(&(*m)[..]);
    hello(&m);
    
    // and here we show it goes as deep as necessary
    // without explicit derefs.
    let n = MyBox::new_nested(String::from("Rust"));
    hello(&n);
}
