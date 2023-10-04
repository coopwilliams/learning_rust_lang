// demonstrates how one package in a workspace
// can depend on another in the same workspace.

// these are both defined in adder/Cargo.toml
use add_one;
use rand;

fn main() {
    let num = 10;
    println!("Hello world! {num} plus one is {}!", add_one::add_one(num));
}
