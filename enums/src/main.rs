fn main() {
    
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hi there"));
    m.call();


}

enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddrKind) { }

// This is like writing four different structs. 
// But it lets us define a function that easily takes any of them. 
// We can also give it a method. 
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// This enum can encode the concept of a value being present or absent. 
// But Rust does not have a Null type.
// Which is cool; I was thinking the other day that I'd like to avoid
// them entirely in my code. 
enum Option<T> {
    Some(T),
    None,
}

