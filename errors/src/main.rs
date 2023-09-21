use std::fs;
use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // all these functions do the same thing in progressively
    // shorter ways.
    let username1 = read_username_from_file_long_way();
    println!("{:?}", username1);
    let username2 = read_username_from_file_short_way();
    println!("{:?}", username2);
    let username3 = read_username_from_file_shorter_way();
    println!("{:?}", username3);
    let username4 = read_username_from_file_shortest_way();
    println!("{:?}", username4);

    let last_char = last_char_of_first_line(&username3.expect("_"));
    println!("{:?}", last_char.unwrap());
}

fn read_username_from_file_long_way() -> Result<String, io::Error> {
    let username_file_result = File::open("hello2.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short_way() -> Result<String, io::Error> {
    let mut username_file = File::open("hello3.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter_way() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello4.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_shortest_way() -> Result<String, io::Error> {
    fs::read_to_string("hello5.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}