// demonstrate the use of threads

use std::thread;
use std::time::Duration;

fn demonstrate_spawned_thread() {
    // // in the below, the ending of the main thread
    // // stops the spawned thread prematurely.
    // // The spawned thread usually doesn't print
    // // past 5 before the program ends.
    // // And there is no guarantee that the spawned
    // // thread will get to run at all!
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    
    // // We can fix that by saving the return
    // // value of the thread::spawn
    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));    
    };
    
    // this causes the main thread to block 
    // (not continue or exit)
    // until the spawned thread exits.
    // Putting this before the "main thread" loop
    // causes the whole "spawned" thread to complete
    // before the "main thread" loop even begins.
    handle.join().unwrap();
}

fn demonstrate_move_closures_with_threads() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn main() {
    demonstrate_spawned_thread();
    demonstrate_move_closures_with_threads();
}