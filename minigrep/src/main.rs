use std::{env, process};

use minigrep::Config;

fn main() {

    // according to general convention,
    // the responsibilities of the main()
    // function in a complicated program are:

    // - Calling the command line parsing logic with the argument values
    // - Setting up any other configuration
    // - Calling a run function in lib.rs
    // - Handling the error if run returns an error

    // parse args
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });

    if let Err(_e) = minigrep::run(config) {
        process::exit(1);
    }
}
