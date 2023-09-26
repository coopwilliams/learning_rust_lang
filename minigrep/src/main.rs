use std::env;
use std::fs;

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
    let (query, file_path) = parse_config(&args);

    // read the file
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}