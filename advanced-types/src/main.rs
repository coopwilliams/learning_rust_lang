use std::fmt;
use std::io::Result;

type Kilometers = i32;

// Since std::io has this type alias declaration:
//  `type Result<T> = std::result::Result<T, std::io::Error>;`
// ... we can use the fully qualified alias from that module
// to avoid writing overly word code.
pub trait Write {
    fn write(&mut self, bug: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {

    // Kilometers is not actually a different type
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
    

    // writing a lengthy type multiple times can be
    // annoying and error prone. e.g.:
    // `let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));`
    
    // Instead we can use a type alias,
    // which is much easier to read and write.
    type Thunk = Box<dyn Fn() + Send + Sync + 'static>; 
    let _f: Thunk = Box::new(|| println!("hi"));

    // Generic functions have an implicit trait bound:
    //      fn generic<T: Sized>(t: T) { //...// }
    // But you can get around that with this:
    //      fn generic<T: ?Sized>(t: &T) { //..// }
    // indicating that the trait may or may not be sized.
    // In that case, it will have to be behind some
    // kind of pointer, hence the &T parameter.
    fn is_equal<T: Eq + ?Sized>(t1: &T, t2: &T) -> bool {
        t1 == t2
    }
          
    // Note how these two `str` values are differently sized
    println!("{}", is_equal("Hello", "world!!!"));
      

}
