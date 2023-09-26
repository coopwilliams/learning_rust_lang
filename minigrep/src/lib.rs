use std::{fs, error::Error};

pub struct Config {
    query: String,
    file_path : String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config {query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read the file
    let contents = fs::read_to_string(config.file_path)?;
        // .expect("Should have been able to read the file");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // huh. This compiles even if 
        // search() doesn't exist yet.
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}