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
    let config = parse_config(&args);

    // read the file
    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");

println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path : String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config {query, file_path}
}