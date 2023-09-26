use std::{env, fs, process};

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
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    // read the file
    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");

println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path : String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config {query, file_path})
    }
}