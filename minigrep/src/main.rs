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
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
