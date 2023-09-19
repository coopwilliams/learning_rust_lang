use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread that sends data
    thread::spawn(move || {
        let data = "Hello from the spawned thread!";
        tx.send(data).unwrap();
    });

    // Receive the data in the main thread
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
