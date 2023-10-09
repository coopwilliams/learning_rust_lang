// demo channels, which pass messages between threads

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // multiple threads sending to the same consumer

    // Note that a channel can only send messages of one type,
    // unless we specify an enum or use the `Any` trait.
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
                
            }
        });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

    for received in rx {
        println!("Got: {}", received);
    };

}
