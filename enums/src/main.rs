fn main() {
    
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hi there"));
    m.call();

    value_in_cents(Coin::Quarter(UsState::Idaho));
    value_in_cents(Coin::Quarter(UsState::NonIdaho));
    value_in_cents(Coin::Sacagawea);
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

#[derive(Debug)]
enum UsState {
    Idaho,
    NonIdaho,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    HalfDollar,
    Sacagawea,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Idaho) => {
            println!("It's a beaut!");
            25
        }
        Coin::Quarter(UsState::NonIdaho) => {
            println!("meh.");
            25
        }
        // other coins not accepted.
        // we can use any name for
        // things not matched in the match.
        not_accepted => {println!("not accepted!"); 0} 
    }
}