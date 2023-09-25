use adder;

// example integration test.
// it calls the source code as an
// external library.

// NOTE we can't do this with a
// binary crate that only has a 
// src/main.rs file.

// That explains why lots of projects
// have a main.rs file that calls logic
// in the lib.rs file. That makes
// integration tests possible.
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add(2,2));
}

