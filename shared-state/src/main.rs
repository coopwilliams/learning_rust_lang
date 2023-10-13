// demonstrate shared state

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    // -----------------
    // demo basic use of a Mutex.

    // `m` is immutable, but we still change its value later,
    // because Mutex provides interior mutability.
    let m = Mutex::new(5);
    println!("m = {:?}", m);
    
    {
        let mut num = m.lock().unwrap();
        // data is <locked> here.
        println!("m = {:?}", m);
        *num = 6;
    }
    
    // data is accessible again here.
    println!("m = {:?}", m);
    
    
    // -----------------
    // demo sharing a Mutex between threads.

    // We use the Atomic version of Rc (Arc), which can be
    // shared between threads safely.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // .lock() blocks until it can acquire the lock.
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Number of threads that incremented the shared counter: {}", *counter.lock().unwrap());

}
